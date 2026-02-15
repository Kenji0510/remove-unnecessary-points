# 02/15/2026
## RTX 4080
```bash
=== Vulkan Device Information ===
Device Name: NVIDIA GeForce RTX 4080
Device Type: DiscreteGpu
Vulkan context initialized successfully.

=== Parameters ===
Input PCD path: data/input/transformed-combined-frame-125.pcd
Loaded points: 79662
Voxel size: 0.1
MIN_Z: -0.5
MAX_Z: 1.5
MIN_Z_RANGE: 0.7
MAX_Z_RANGE: 1.75
K_NEIGHBORS: 20
PLANARITY_THRESHOLD: 0.6
LINEARITY_THRESHOLD: 0.5
SCATTERING_THRESHOLD: 0.2
====================
Compute shader execution time: 369.214µs
Number of output points: 12729
GPU voxelization: 12729 points
Compute shader execution time: 2.534482ms
=== Processing Result ===
After processed points: 2542
Processing time: 44.66ms
Saved removed unnecessary points pcd to data/output/2d-xy/convert-2d-pts-by-covariance/removed-by-shape-feats_voxel-0.1_NUM-125.pcd
Saved voxelized pcd with shape features to data/output/2d-xy/convert-2d-pts-by-covariance/original-by-shape-feats_voxel-0.1_NUM-125.pcd
```

```bash
=== Vulkan Device Information ===
Device Name: NVIDIA GeForce RTX 4080
Device Type: DiscreteGpu
Vulkan context initialized successfully.

=== Parameters ===
Input PCD path: data/input/transformed-combined-frame-125.pcd
Loaded points: 79662
Voxel size: 0.05
MIN_Z: -0.5
MAX_Z: 1.5
MIN_Z_RANGE: 0.7
MAX_Z_RANGE: 1.75
K_NEIGHBORS: 20
PLANARITY_THRESHOLD: 0.6
LINEARITY_THRESHOLD: 0.5
SCATTERING_THRESHOLD: 0.2
====================
Compute shader execution time: 359.686µs
Number of output points: 31434
GPU voxelization: 31434 points
Compute shader execution time: 6.392825ms
=== Processing Result ===
After processed points: 4734
Processing time: 55.37ms
Saved removed unnecessary points pcd to data/output/2d-xy/convert-2d-pts-by-covariance/removed-by-shape-feats_voxel-0.05_NUM-125.pcd
Saved voxelized pcd with shape features to data/output/2d-xy/convert-2d-pts-by-covariance/original-by-shape-feats_voxel-0.05_NUM-125.pcd
```

## Radeon 780m
```bash
=== Parameters ===
Input PCD path: data/input/transformed-combined-frame-125.pcd
Loaded points: 79662
Voxel size: 0.05
MIN_Z: -0.5
MAX_Z: 1.5
MIN_Z_RANGE: 0.7
MAX_Z_RANGE: 1.75
K_NEIGHBORS: 20
PLANARITY_THRESHOLD: 0.6
LINEARITY_THRESHOLD: 0.5
SCATTERING_THRESHOLD: 0.2
====================
Compute shader execution time: 1.411314ms
Number of output points: 31434
=== Processing Result ===
After processed points: 4704
Processing time: 22.92ms
```

## AGX Orin 64GB
```bash
=== Vulkan Device Information ===
Device Name: NVIDIA Tegra Orin (nvgpu)
Device Type: IntegratedGpu
Vulkan context initialized successfully.

=== Parameters ===
Input PCD path: data/input/transformed-combined-frame-125.pcd
Loaded points: 79662
Voxel size: 0.1
MIN_Z: -0.5
MAX_Z: 1.5
MIN_Z_RANGE: 0.7
MAX_Z_RANGE: 1.75
K_NEIGHBORS: 20
PLANARITY_THRESHOLD: 0.6
LINEARITY_THRESHOLD: 0.5
SCATTERING_THRESHOLD: 0.2
====================
Compute shader execution time: 1.634053ms
Number of output points: 12729
GPU voxelization: 12729 points
Compute shader execution time: 42.464761ms
=== Processing Result ===
After processed points: 2542
Processing time: 106.12ms
Saved removed unnecessary points pcd to data/output/2d-xy/convert-2d-pts-by-covariance/removed-by-shape-feats_voxel-0.1_NUM-125.pcd
Saved voxelized pcd with shape features to data/output/2d-xy/convert-2d-pts-by-covariance/original-by-shape-feats_voxel-0.1_NUM-125.pcd
```

```bash
=== Vulkan Device Information ===
Device Name: NVIDIA Tegra Orin (nvgpu)
Device Type: IntegratedGpu
Vulkan context initialized successfully.

=== Parameters ===
Input PCD path: data/input/transformed-combined-frame-125.pcd
Loaded points: 79662
Voxel size: 0.05
MIN_Z: -0.5
MAX_Z: 1.5
MIN_Z_RANGE: 0.7
MAX_Z_RANGE: 1.75
K_NEIGHBORS: 20
PLANARITY_THRESHOLD: 0.6
LINEARITY_THRESHOLD: 0.5
SCATTERING_THRESHOLD: 0.2
====================
Compute shader execution time: 1.62663ms
Number of output points: 31434
GPU voxelization: 31434 points
Compute shader execution time: 97.102388ms
=== Processing Result ===
After processed points: 4734
Processing time: 175.70ms
Saved removed unnecessary points pcd to data/output/2d-xy/convert-2d-pts-by-covariance/removed-by-shape-feats_voxel-0.05_NUM-125.pcd
Saved voxelized pcd with shape features to data/output/2d-xy/convert-2d-pts-by-covariance/original-by-shape-feats_voxel-0.05_NUM-125.pcd
```