#version 450
#extension GL_EXT_shader_atomic_float : require

layout(local_size_x = 256, local_size_y = 1, local_size_z = 1) in;

const uint EMPTY_KEY = 0xFFFFFFFF;
const uint SHARED_TABLE_SIZE = 1536;
const uint SHARED_PROBE = 32;
const uint GLOBAL_PROBE = 1000;

const int GRID_OFFSET_X = 512;
const int GRID_OFFSET_Y = 512;
const int GRID_OFFSET_Z = 512;

layout(push_constant) uniform PushConstants {
    int num_points;
    int table_size;
    float voxel_size;
    int _pad;
} pc;

layout(set = 0, binding = 0) buffer InputPoints { float in_points[]; };
layout(set = 0, binding = 1) buffer TableKeys   { uint table_keys[]; };
layout(set = 0, binding = 2) buffer TableCentroids { float table_centroids[]; };
layout(set = 0, binding = 3) buffer TableCounts { uint table_counts[]; };
//layout(set = 0, binding = 4) buffer OutputPoints { float out_points[]; };
//layout(set = 0, binding = 5) buffer OutputCount  { uint out_count; };
layout(set = 0, binding = 6) buffer TableSecondMoments { float table_m2[]; };


shared uint s_keys[SHARED_TABLE_SIZE];
shared float s_centroids[SHARED_TABLE_SIZE * 3];
shared int s_counts[SHARED_TABLE_SIZE];
shared float s_m2[SHARED_TABLE_SIZE * 6];


uint compute_voxel_key(float px, float py, float pz, float voxel_size) {
    float inv_voxel = 1.0 / voxel_size;
    int vx = int(floor(px * inv_voxel)) + GRID_OFFSET_X;
    int vy = int(floor(py * inv_voxel)) + GRID_OFFSET_Y;
    int vz = int(floor(pz * inv_voxel)) + GRID_OFFSET_Z;

    vx = clamp(vx, 0, 1023);
    vy = clamp(vy, 0, 1023);
    vz = clamp(vz, 0, 1023);

    return uint(vx) | (uint(vy) << 10) | (uint(vz) << 20);
}

void add_to_global(
    uint key, 
    float px, float py, float pz, 
    float m_xx, float m_xy, float m_xz,
    float m_yy, float m_yz, float m_zz, 
    int count
) {
    uint h = (key * 2654435761u);
    uint idx = h % uint(pc.table_size);

    for (uint i = 0; i < GLOBAL_PROBE; ++i) {
        uint old_key = atomicCompSwap(table_keys[idx], EMPTY_KEY, key);

        if (old_key == EMPTY_KEY || old_key == key) {
            atomicAdd(table_centroids[3 * idx + 0], px);
            atomicAdd(table_centroids[3 * idx + 1], py);
            atomicAdd(table_centroids[3 * idx + 2], pz);
            atomicAdd(table_counts[idx], count);

            atomicAdd(table_m2[6 * idx + 0], m_xx);
            atomicAdd(table_m2[6 * idx + 1], m_xy);
            atomicAdd(table_m2[6 * idx + 2], m_xz);
            atomicAdd(table_m2[6 * idx + 3], m_yy);
            atomicAdd(table_m2[6 * idx + 4], m_yz);
            atomicAdd(table_m2[6 * idx + 5], m_zz);
            return;
        }
        idx = (idx + 1) % uint(pc.table_size);
    }
} 

void main() {
    uint g_idx = gl_GlobalInvocationID.x;
    uint l_idx = gl_LocalInvocationID.x;
    uint l_size = gl_WorkGroupSize.x;

    for (uint i = l_idx; i < SHARED_TABLE_SIZE; i += l_size) {
        s_keys[i] = EMPTY_KEY;
        s_centroids[i * 3 + 0] = 0.0;
        s_centroids[i * 3 + 1] = 0.0;
        s_centroids[i * 3 + 2] = 0.0;
        s_counts[i] = 0;

        s_m2[i * 6 + 0] = 0.0;
        s_m2[i * 6 + 1] = 0.0;
        s_m2[i * 6 + 2] = 0.0;
        s_m2[i * 6 + 3] = 0.0;
        s_m2[i * 6 + 4] = 0.0;
        s_m2[i * 6 + 5] = 0.0;
    }
    barrier();

    if (g_idx < pc.num_points) {
        float px = in_points[g_idx * 3 + 0];
        float py = in_points[g_idx * 3 + 1];
        float pz = in_points[g_idx * 3 + 2];

        uint key = compute_voxel_key(px, py, pz, pc.voxel_size);
        uint s_idx = key % SHARED_TABLE_SIZE;

        bool stored = false;
        for (uint i = 0; i < SHARED_PROBE; ++i) {
            uint old = atomicCompSwap(s_keys[s_idx], EMPTY_KEY, key);
            
            if (old == EMPTY_KEY || old == key) {
                atomicAdd(s_centroids[s_idx * 3 + 0], px);
                atomicAdd(s_centroids[s_idx * 3 + 1], py);
                atomicAdd(s_centroids[s_idx * 3 + 2], pz);
                atomicAdd(s_counts[s_idx], 1);

                atomicAdd(s_m2[s_idx * 6 + 0], px * px);
                atomicAdd(s_m2[s_idx * 6 + 1], px * py);
                atomicAdd(s_m2[s_idx * 6 + 2], px * pz);
                atomicAdd(s_m2[s_idx * 6 + 3], py * py);
                atomicAdd(s_m2[s_idx * 6 + 4], py * pz);
                atomicAdd(s_m2[s_idx * 6 + 5], pz * pz);

                stored = true;
                break;
            }
            s_idx = (s_idx + 1) % SHARED_TABLE_SIZE;
        }

        if (!stored) {
            add_to_global(key, px, py, pz, px * px, px * py, px * pz, py * py, py * pz, pz * pz, 1);
        }
    }
    barrier();

    for (uint i = l_idx; i < SHARED_TABLE_SIZE; i += l_size) {
        uint key = s_keys[i];
        if (key != EMPTY_KEY) {
            float sx = s_centroids[i * 3 + 0];
            float sy = s_centroids[i * 3 + 1];
            float sz = s_centroids[i * 3 + 2];
            int sc = s_counts[i];

            float m_xx = s_m2[i * 6 + 0];
            float m_xy = s_m2[i * 6 + 1];
            float m_xz = s_m2[i * 6 + 2];
            float m_yy = s_m2[i * 6 + 3];
            float m_yz = s_m2[i * 6 + 4];
            float m_zz = s_m2[i * 6 + 5];
            
            add_to_global(key, sx, sy, sz, m_xx, m_xy, m_xz, m_yy, m_yz, m_zz, sc);
        }
    }
}