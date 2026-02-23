#version 450

layout(local_size_x = 256, local_size_y = 1, local_size_z = 1) in;

layout(push_constant) uniform PushConstants {
    int num_points;
    int _pad1;
    int _pad2;
    int _pad3;
} pc;

layout(set = 0, binding = 0) buffer ClusterIDs { uint cluster_ids[]; };

void main() {
    uint g_idx = gl_GlobalInvocationID.x;
    if (g_idx >= pc.num_points) return;

    uint id = cluster_ids[g_idx];
    
    while (id != cluster_ids[id]) {
        id = cluster_ids[id];
    }
    
    cluster_ids[g_idx] = id;
}