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

layout(set = 0, binding = 1) buffer TableKeys   { uint table_keys[]; };
layout(set = 0, binding = 2) buffer TableCentroids { float table_centroids[]; };
layout(set = 0, binding = 3) buffer TableCounts { int table_counts[]; };
layout(set = 0, binding = 5) buffer OutputCount  { int out_count; };

void main() {
    uint idx = gl_GlobalInvocationID.x;
    if (idx >= pc.table_size) {
        return;
    }

    table_keys[idx] = EMPTY_KEY;
    table_counts[idx] = 0;

    table_centroids[3 * idx + 0] = 0.0;
    table_centroids[3 * idx + 1] = 0.0;
    table_centroids[3 * idx + 2] = 0.0;

    if (idx == 0) {
        out_count = 0;
    }
}