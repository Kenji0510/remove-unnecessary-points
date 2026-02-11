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

layout(std430, set = 0, binding = 0) buffer InputPoints { float in_points[]; };
layout(std430, set = 0, binding = 1) buffer TableKeys   { uint table_keys[]; };
layout(std430, set = 0, binding = 2) buffer TableCentroids { float table_centroids[]; };
layout(std430, set = 0, binding = 3) buffer TableCounts { int table_counts[]; };
layout(std430, set = 0, binding = 4) buffer OutputPoints { float out_points[]; };
layout(std430, set = 0, binding = 5) buffer OutputCount  { int out_count; };


void main() {
    uint idx = gl_GlobalInvocationID.x;
    if (idx >= pc.table_size) {
        return;
    }

    uint key = table_keys[idx];
    int count = table_counts[idx];

    if (key != EMPTY_KEY && count > 0) {
        float sx = table_centroids[3 * idx + 0];
        float sy = table_centroids[3 * idx + 1];
        float sz = table_centroids[3 * idx + 2];

        if (count > 1) {
            float inv = 1.0 / float(count);
            sx *= inv;
            sy *= inv;
            sz *= inv;
        }

        int w_idx = atomicAdd(out_count, 1);

        out_points[3 * w_idx + 0] = sx;
        out_points[3 * w_idx + 1] = sy;;
        out_points[3 * w_idx + 2] = sz;
    }
}