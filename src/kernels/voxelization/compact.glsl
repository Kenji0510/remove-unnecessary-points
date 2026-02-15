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
layout(set = 0, binding = 4) buffer OutputPoints { float out_points[]; };
layout(set = 0, binding = 5) buffer OutputCount  { uint out_count; };
layout(set = 0, binding = 6) buffer TableSecondMoments { float table_m2[]; };
layout(set = 0, binding = 7) buffer OutputCovariances { float out_cov[]; };


void main() {
    uint idx = gl_GlobalInvocationID.x;
    if (idx >= pc.table_size) {
        return;
    }

    uint key = table_keys[idx];
    uint count = table_counts[idx];

    if (key != EMPTY_KEY && count > 0) {
        float inv = 1.0 / float(count);

        // μ
        float mx = table_centroids[3 * idx + 0] * inv;
        float my = table_centroids[3 * idx + 1] * inv;
        float mz = table_centroids[3 * idx + 2] * inv;

        // E[pp^T]
        float exx = table_m2[6 * idx + 0] * inv;
        float exy = table_m2[6 * idx + 1] * inv;
        float exz = table_m2[6 * idx + 2] * inv;
        float eyy = table_m2[6 * idx + 3] * inv;
        float eyz = table_m2[6 * idx + 4] * inv;
        float ezz = table_m2[6 * idx + 5] * inv;

        // Cov = E[pp^T] - μμ^T
        float cxx = exx - mx * mx;
        float cxy = exy - mx * my;
        float cxz = exz - mx * mz;
        float cyy = eyy - my * my;
        float cyz = eyz - my * mz;
        float czz = ezz - mz * mz;

        float eps = 1e-6;
        cxx += eps; cyy += eps; czz += eps;

        uint w_idx = atomicAdd(out_count, 1);

        // centroid
        out_points[3 * w_idx + 0] = mx;
        out_points[3 * w_idx + 1] = my;
        out_points[3 * w_idx + 2] = mz;

        // covariance
        out_cov[6 * w_idx + 0] = cxx;
        out_cov[6 * w_idx + 1] = cxy;
        out_cov[6 * w_idx + 2] = cxz;
        out_cov[6 * w_idx + 3] = cyy;
        out_cov[6 * w_idx + 4] = cyz;
        out_cov[6 * w_idx + 5] = czz;
    }

}