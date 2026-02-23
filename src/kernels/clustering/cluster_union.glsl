#version 450

layout(local_size_x = 256, local_size_y = 1, local_size_z = 1) in;

const uint EMPTY_KEY = 0xFFFFFFFF;
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

layout(set = 0, binding = 0) buffer OutPoints { float out_points[]; };
layout(set = 0, binding = 1) buffer TableKeys { uint table_keys[]; };
layout(set = 0, binding = 2) buffer TableVoxelIndices { uint table_voxel_indices[]; };
layout(set = 0, binding = 3) buffer ClusterIDs { uint cluster_ids[]; };
layout(set = 0, binding = 4) buffer SyncFlag { uint changed_flag; };

uint expandBits(uint v) {
    v = (v * 0x00010001u) & 0xFF0000FFu;
    v = (v * 0x00000101u) & 0x0F00F00Fu;
    v = (v * 0x00000011u) & 0xC30C30C3u;
    v = (v * 0x00000005u) & 0x49249249u;
    return v;
}

uint morton3D(uvec3 v) {
    return expandBits(v.x) | (expandBits(v.y) << 1) | (expandBits(v.z) << 2);
}

uint compute_voxel_key_from_grid(int vx, int vy, int vz) {
    vx = clamp(vx, 0, 1023);
    vy = clamp(vy, 0, 1023);
    vz = clamp(vz, 0, 1023);
    
    return morton3D(uvec3(vx, vy, vz));
}

void main() {
    uint g_idx = gl_GlobalInvocationID.x;
    if (g_idx >= pc.num_points) return;

    float px = out_points[g_idx * 3 + 0];
    float py = out_points[g_idx * 3 + 1];
    float pz = out_points[g_idx * 3 + 2];

    float inv_voxel = 1.0 / pc.voxel_size;
    int base_vx = int(floor(px * inv_voxel)) + GRID_OFFSET_X;
    int base_vy = int(floor(py * inv_voxel)) + GRID_OFFSET_Y;
    int base_vz = int(floor(pz * inv_voxel)) + GRID_OFFSET_Z;

    uint my_cluster = cluster_ids[g_idx];
    bool local_changed = false;

    const int SEARCH_RADIUS = 2;

    for(int dz = -SEARCH_RADIUS; dz <= SEARCH_RADIUS; dz++) {
        for(int dy = -SEARCH_RADIUS; dy <= SEARCH_RADIUS; dy++) {
            for(int dx = -SEARCH_RADIUS; dx <= SEARCH_RADIUS; dx++) {
                if (dx == 0 && dy == 0 && dz == 0) continue;

                // float nx = px + float(dx) * pc.voxel_size;
                // float ny = py + float(dy) * pc.voxel_size;
                // float nz = pz + float(dz) * pc.voxel_size;

                int nx = base_vx + dx;
                int ny = base_vy + dy;
                int nz = base_vz + dz;

                // uint neighbor_key = compute_voxel_key(nx, ny, nz, pc.voxel_size);
                uint neighbor_key = compute_voxel_key_from_grid(nx, ny, nz);

                uint h = (neighbor_key * 2654435761u);
                uint idx = h % uint(pc.table_size);

                for(uint i = 0; i < GLOBAL_PROBE; ++i) {
                    uint current_key = table_keys[idx];
                    if (current_key == neighbor_key) {
                        uint neighbor_w_idx = table_voxel_indices[idx];
                        if (neighbor_w_idx != EMPTY_KEY) {
                            uint neighbor_cluster = cluster_ids[neighbor_w_idx];
                            if (neighbor_cluster < my_cluster) {
                                atomicMin(cluster_ids[g_idx], neighbor_cluster);
                                my_cluster = neighbor_cluster;
                                local_changed = true;
                            } else if (my_cluster < neighbor_cluster) {
                                atomicMin(cluster_ids[neighbor_w_idx], my_cluster);
                                local_changed = true;
                            }
                        }
                        break;
                    } else if (current_key == EMPTY_KEY) {
                        break;
                    }
                    idx = (idx + 1) % uint(pc.table_size);
                }
            }
        }
    }

    if (local_changed) {
        atomicExchange(changed_flag, 1);
    }
}