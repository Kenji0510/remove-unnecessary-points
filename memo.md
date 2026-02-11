# 02/08/2026
```bash
=== Partameteres ===
Input PCD path: data/input/voxelized-H927-hallway-01.pcd
LOaded points: 42263
Voxel size: 0.1
K neighbors: 20
Planarity threshold: 0.6
Scattering threshold: 0.2
====================
Saved removed unnecessary points pcd to data/output/removed-voxelized-H927-hallway-01.pcd
Saved voxelized pcd with shape features to data/output/voxelized-H927-hallway-01.pcd
=== Computation Results ===
Computation time: 5.31ms
Removed points: 3307
```

# 02/11/2026
## RTX 4080
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
Compute shader execution time: 368.966µs
Number of output points: 31434
=== Processing Result ===
After processed points: 4704
Processing time: 14.72ms
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