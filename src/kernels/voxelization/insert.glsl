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


shared uint s_keys[SHARED_TABLE_SIZE];
// shared float s_centroids[SHARED_TABLE_SIZE * 3];
shared uint s_centroids[SHARED_TABLE_SIZE * 3];
shared int s_counts[SHARED_TABLE_SIZE];


void atomicAddSharedFloat(uint index, float val) {
    uint assumed;
    uint old_val = s_centroids[index];
    do {
        assumed = old_val;
        // 現在のuint値をfloatに戻して加算し、再度uintにキャスト
        float new_float = uintBitsToFloat(assumed) + val;
        uint new_val = floatBitsToUint(new_float);
        
        // assumedと一致していればnew_valに置き換え、そうでなければold_valを更新してリトライ
        old_val = atomicCompSwap(s_centroids[index], assumed, new_val);
    } while (assumed != old_val);
}

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

void add_to_global(uint key, float px, float py, float pz, int count) {
    uint h = (key * 2654435761u);
    uint idx = h % uint(pc.table_size);

    for (uint i = 0; i < GLOBAL_PROBE; ++i) {
        uint old_key = atomicCompSwap(table_keys[idx], EMPTY_KEY, key);

        if (old_key == EMPTY_KEY || old_key == key) {
            atomicAdd(table_centroids[3 * idx + 0], px);
            atomicAdd(table_centroids[3 * idx + 1], py);
            atomicAdd(table_centroids[3 * idx + 2], pz);
            atomicAdd(table_counts[idx], count);
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
        // s_centroids[i * 3 + 0] = 0.0;
        // s_centroids[i * 3 + 1] = 0.0;
        // s_centroids[i * 3 + 2] = 0.0;
        s_centroids[i * 3 + 0] = 0u;
        s_centroids[i * 3 + 1] = 0u;
        s_centroids[i * 3 + 2] = 0u;
        s_counts[i] = 0;
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
                // atomicAdd(s_centroids[s_idx * 3 + 0], px);
                // atomicAdd(s_centroids[s_idx * 3 + 1], py);
                // atomicAdd(s_centroids[s_idx * 3 + 2], pz);
                atomicAddSharedFloat(s_idx * 3 + 0, px);
                atomicAddSharedFloat(s_idx * 3 + 1, py);
                atomicAddSharedFloat(s_idx * 3 + 2, pz);
                atomicAdd(s_counts[s_idx], 1);
                stored = true;
                break;
            }
            s_idx = (s_idx + 1) % SHARED_TABLE_SIZE;
        }

        if (!stored) {
            add_to_global(key, px, py, pz, 1);
        }
    }
    barrier();

    for (uint i = l_idx; i < SHARED_TABLE_SIZE; i += l_size) {
        uint key = s_keys[i];
        if (key != EMPTY_KEY) {
            // float sx = s_centroids[i * 3 + 0];
            // float sy = s_centroids[i * 3 + 1];
            // float sz = s_centroids[i * 3 + 2];
            float sx = uintBitsToFloat(s_centroids[i * 3 + 0]);
            float sy = uintBitsToFloat(s_centroids[i * 3 + 1]);
            float sz = uintBitsToFloat(s_centroids[i * 3 + 2]);
            int sc = s_counts[i];
            add_to_global(key, sx, sy, sz, sc);
        }
    }
}