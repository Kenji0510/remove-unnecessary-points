# RTX4080
```bash
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::init_gpu] === Available Vulkan Devices ===
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::init_gpu] Device 0: NVIDIA GeForce RTX 4080 (DiscreteGpu)
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::init_gpu] Device 1: llvmpipe (LLVM 15.0.7, 256 bits) (Cpu)
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::init_gpu] 
    
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::init_gpu] Selected device: NVIDIA GeForce RTX 4080 (DiscreteGpu)
    
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::init_gpu] === Vulkan Device Information ===
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::init_gpu] Device Name: NVIDIA GeForce RTX 4080
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::init_gpu] Device Type: DiscreteGpu
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::init_gpu] Vulkan context initialized successfully.
    
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] === Parameters ===
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] Input PCD path: data/input/transformed-combined-frame-125.pcd
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] Loaded points: 79662
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] Voxel size: 0.05
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] MIN_Z: -0.5
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] MAX_Z: 1.5
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] MIN_Z_RANGE: 0.7
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] MAX_Z_RANGE: 1.75
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] K_NEIGHBORS: 20
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] PLANARITY_THRESHOLD: 0.6
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] LINEARITY_THRESHOLD: 0.5
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] SCATTERING_THRESHOLD: 0.2
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] NORMAL_Z_THRESHOLD: 0.85
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] DEBUG_ITERATIONS: 9
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] ====================
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Reallocating buffers for 79662 points
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 480.738µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_covariance] Reallocating buffers for 31434 points
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 198.725µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_clustering] Reallocating buffers for 31434 points
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 998.79µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 173.607µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 175.071µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 966.04µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 148.611µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 189.599µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 963.65µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 140.285µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 170.361µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 946.31µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 139.102µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 169.36µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 956.49µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 139.032µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 171.425µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 973.49µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 146.057µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 177.115µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 973.37µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 140.645µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 173.346µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 970.31µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 140.827µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 172.566µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 958.97µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 141.236µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 183.666µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 966.57µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] === Processing Result ===
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] After processed points: 1823
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] Processing time: 105.98ms
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 88.167µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 636.46µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 66.636µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 639.04µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 65.655µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 638.08µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 65.242µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 637.48µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 66.736µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 635.37µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 60.736µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 644.29µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 69.311µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 647.45µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 104.689µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 635.57µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 66.495µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 636.86µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] Saved colored clusters to data/output/clustering_results/downsampled-clustering-results_voxel-0.05_NUM-125.pcd
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] === Average GPU Processing Time (excluding first 3 iterations) ===
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] Raw points voxelization time: 244.11µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] Raw points covariance time: 233.76µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] Raw points clustering time: 1.01ms
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] Downsampled points voxelization time: 115.40µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] Downsampled points clustering time: 674.34µs
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] ====================
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] Saved removed unnecessary points pcd to data/output/2d-xy/convert-2d-pts-by-covariance/removed-by-shape-feats_voxel-0.05_NUM-125.pcd
[2026-02-23T09:19:18Z DEBUG remove_unnecessary_points] Saved voxelized pcd with shape features to data/output/2d-xy/convert-2d-pts-by-covariance/original-by-shape-feats_voxel-0.05_NUM-125.pcd





[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::init_gpu] === Available Vulkan Devices ===
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::init_gpu] Device 0: NVIDIA GeForce RTX 4080 (DiscreteGpu)
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::init_gpu] Device 1: llvmpipe (LLVM 15.0.7, 256 bits) (Cpu)
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::init_gpu] 
    
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::init_gpu] Selected device: NVIDIA GeForce RTX 4080 (DiscreteGpu)
    
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::init_gpu] === Vulkan Device Information ===
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::init_gpu] Device Name: NVIDIA GeForce RTX 4080
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::init_gpu] Device Type: DiscreteGpu
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::init_gpu] Vulkan context initialized successfully.
    
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] === Parameters ===
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] Input PCD path: data/input/transformed-combined-frame-125.pcd
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] Loaded points: 79662
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] Voxel size: 0.1
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] MIN_Z: -0.5
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] MAX_Z: 1.5
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] MIN_Z_RANGE: 0.7
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] MAX_Z_RANGE: 1.75
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] K_NEIGHBORS: 20
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] PLANARITY_THRESHOLD: 0.6
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] LINEARITY_THRESHOLD: 0.5
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] SCATTERING_THRESHOLD: 0.2
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] NORMAL_Z_THRESHOLD: 0.85
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] DEBUG_ITERATIONS: 9
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] ====================
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Reallocating buffers for 79662 points
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 459.308µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_covariance] Reallocating buffers for 12729 points
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 132.671µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_clustering] Reallocating buffers for 12729 points
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 653.73µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 145.304µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 111.662µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 664.84µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 182.104µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 116.47µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 655.46µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 140.075µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 109.847µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 645.96µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 140.876µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 113.755µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 656.04µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 139.415µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 116.78µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 648.13µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 138.853µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 118.254µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 672.89µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 151.646µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 119.555µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 653.37µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 146.037µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 116.05µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 649.48µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 143.472µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 112.983µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 662.67µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] === Processing Result ===
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] After processed points: 1519
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] Processing time: 97.48ms
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 84.3µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 601.80µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 81.824µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 596.67µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 71.415µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 599.40µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 72.236µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 599.54µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 74.079µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 602.65µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 75.463µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 604.82µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 72.537µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 601.49µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 67.478µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 608.95µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 65.733µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 591.87µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] Saved colored clusters to data/output/clustering_results/downsampled-clustering-results_voxel-0.1_NUM-125.pcd
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] === Average GPU Processing Time (excluding first 3 iterations) ===
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] Raw points voxelization time: 267.18µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] Raw points covariance time: 164.76µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] Raw points clustering time: 709.32µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] Downsampled points voxelization time: 124.54µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] Downsampled points clustering time: 641.73µs
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] ====================
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] Saved removed unnecessary points pcd to data/output/2d-xy/convert-2d-pts-by-covariance/removed-by-shape-feats_voxel-0.1_NUM-125.pcd
[2026-02-23T09:19:57Z DEBUG remove_unnecessary_points] Saved voxelized pcd with shape features to data/output/2d-xy/convert-2d-pts-by-covariance/original-by-shape-feats_voxel-0.1_NUM-125.pcd
```

# M4 pro (on Mac)
```bash
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::init_gpu] === Available Vulkan Devices ===
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::init_gpu] Device 0: Apple M4 Pro (IntegratedGpu)
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::init_gpu] 
    
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::init_gpu] Selected device: Apple M4 Pro (IntegratedGpu)
    
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::init_gpu] === Vulkan Device Information ===
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::init_gpu] Device Name: Apple M4 Pro
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::init_gpu] Device Type: IntegratedGpu
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::init_gpu] Vulkan context initialized successfully.
    
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] === Parameters ===
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] Input PCD path: data/input/transformed-combined-frame-125.pcd
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] Loaded points: 79662
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] Voxel size: 0.05
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] MIN_Z: -0.5
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] MAX_Z: 1.5
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] MIN_Z_RANGE: 0.7
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] MAX_Z_RANGE: 1.75
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] K_NEIGHBORS: 20
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] PLANARITY_THRESHOLD: 0.6
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] LINEARITY_THRESHOLD: 0.5
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] SCATTERING_THRESHOLD: 0.2
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] NORMAL_Z_THRESHOLD: 0.85
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] DEBUG_ITERATIONS: 9
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] ====================
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Reallocating buffers for 79662 points
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 7.889834ms
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_covariance] Reallocating buffers for 31434 points
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 1.501167ms
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_clustering] Reallocating buffers for 31434 points
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 8.15ms
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 524.209µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 481.666µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 3.27ms
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 469.416µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 449.333µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 2.83ms
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 434.084µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 485.791µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 2.94ms
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 383.208µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 434.875µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 2.71ms
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 426.916µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 449.417µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 3.07ms
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 398.25µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 436.458µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 2.78ms
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 425.042µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 469.958µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 2.96ms
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 382.459µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 419.25µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 2.69ms
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 515.75µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 448.875µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 2.85ms
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] === Processing Result ===
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] After processed points: 1823
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] Processing time: 66.97ms
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 242.25µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.23ms
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 627.959µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.17ms
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 193.166µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.27ms
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 180.584µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.28ms
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 204.75µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.37ms
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 179.625µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.11ms
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 255.167µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.14ms
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 250.209µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.21ms
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 211.084µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.30ms
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] Saved colored clusters to data/output/clustering_results/downsampled-clustering-results_voxel-0.05_NUM-125.pcd
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] === Average GPU Processing Time (excluding first 3 iterations) ===
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] Raw points voxelization time: 545.35µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] Raw points covariance time: 527.64µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] Raw points clustering time: 2.88ms
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] Downsampled points voxelization time: 258.99µs
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] Downsampled points clustering time: 1.27ms
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] ====================
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] Saved removed unnecessary points pcd to data/output/2d-xy/convert-2d-pts-by-covariance/removed-by-shape-feats_voxel-0.05_NUM-125.pcd
[2026-02-23T09:43:29Z DEBUG remove_unnecessary_points] Saved voxelized pcd with shape features to data/output/2d-xy/convert-2d-pts-by-covariance/original-by-shape-feats_voxel-0.05_NUM-125.pcd





[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::init_gpu] === Available Vulkan Devices ===
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::init_gpu] Device 0: Apple M4 Pro (IntegratedGpu)
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::init_gpu] 
    
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::init_gpu] Selected device: Apple M4 Pro (IntegratedGpu)
    
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::init_gpu] === Vulkan Device Information ===
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::init_gpu] Device Name: Apple M4 Pro
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::init_gpu] Device Type: IntegratedGpu
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::init_gpu] Vulkan context initialized successfully.
    
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] === Parameters ===
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] Input PCD path: data/input/transformed-combined-frame-125.pcd
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] Loaded points: 79662
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] Voxel size: 0.1
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] MIN_Z: -0.5
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] MAX_Z: 1.5
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] MIN_Z_RANGE: 0.7
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] MAX_Z_RANGE: 1.75
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] K_NEIGHBORS: 20
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] PLANARITY_THRESHOLD: 0.6
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] LINEARITY_THRESHOLD: 0.5
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] SCATTERING_THRESHOLD: 0.2
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] NORMAL_Z_THRESHOLD: 0.85
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] DEBUG_ITERATIONS: 9
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] ====================
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Reallocating buffers for 79662 points
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 8.638ms
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_covariance] Reallocating buffers for 12729 points
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 1.019375ms
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_clustering] Reallocating buffers for 12729 points
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 4.03ms
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 833.667µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 555.958µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 3.70ms
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 1.007667ms
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 562.209µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 3.68ms
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 397.333µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 297.625µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.53ms
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 439µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 801.791µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 2.09ms
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 360.75µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 287.292µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.29ms
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 407.291µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 552.875µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.18ms
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 435.333µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 535.5µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.38ms
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 317µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 233.875µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.01ms
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 353.792µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 245.416µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.05ms
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] === Processing Result ===
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] After processed points: 1519
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] Processing time: 47.42ms
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 374.417µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 987.75µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 455.167µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 881.08µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 201.167µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 868.42µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 218.334µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.48ms
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 187.209µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 846.79µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 176.334µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 953.38µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 185.625µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.58ms
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 191.834µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 940.25µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 202.25µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 941.83µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] Saved colored clusters to data/output/clustering_results/downsampled-clustering-results_voxel-0.1_NUM-125.pcd
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] === Average GPU Processing Time (excluding first 3 iterations) ===
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] Raw points voxelization time: 466.83µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] Raw points covariance time: 489.38µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] Raw points clustering time: 1.37ms
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] Downsampled points voxelization time: 235.31µs
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] Downsampled points clustering time: 1.15ms
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] ====================
[2026-02-23T09:42:37Z DEBUG remove_unnecessary_points] Saved removed unnecessary points pcd to data/output/2d-xy/convert-2d-pts-by-covariance/removed-by-shape-feats_voxel-0.1_NUM-125.pcd
[2026-02-23T09:42:38Z DEBUG remove_unnecessary_points] Saved voxelized pcd with shape features to data/output/2d-xy/convert-2d-pts-by-covariance/original-by-shape-feats_voxel-0.1_NUM-125.pcd
```

# M4 pro (on Fedora, via VirtIO)
```bash
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::init_gpu] === Available Vulkan Devices ===
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::init_gpu] Device 0: Virtio-GPU Venus (Apple M4 Pro) (IntegratedGpu)
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::init_gpu] Device 1: llvmpipe (LLVM 20.1.7, 128 bits) (Cpu)
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::init_gpu] 
    
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::init_gpu] Selected device: Virtio-GPU Venus (Apple M4 Pro) (IntegratedGpu)
    
[2026-02-23T09:36:01Z INFO  remove_unnecessary_points::init_gpu] Warning: Device does not support ext_shader_atomic_float extension.
[2026-02-23T09:36:01Z INFO  remove_unnecessary_points::init_gpu] Using fallback implementation with integer atomics.
[2026-02-23T09:36:01Z INFO  remove_unnecessary_points::init_gpu] Warning: Device does not support float32 atomic add feature.
[2026-02-23T09:36:01Z INFO  remove_unnecessary_points::init_gpu] Using fallback implementation with integer atomics.
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::init_gpu] === Vulkan Device Information ===
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::init_gpu] Device Name: Virtio-GPU Venus (Apple M4 Pro)
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::init_gpu] Device Type: IntegratedGpu
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::init_gpu] Vulkan context initialized successfully.
    
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] === Parameters ===
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] Input PCD path: data/input/transformed-combined-frame-125.pcd
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] Loaded points: 79662
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] Voxel size: 0.05
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] MIN_Z: -0.5
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] MAX_Z: 1.5
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] MIN_Z_RANGE: 0.7
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] MAX_Z_RANGE: 1.75
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] K_NEIGHBORS: 20
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] PLANARITY_THRESHOLD: 0.6
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] LINEARITY_THRESHOLD: 0.5
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] SCATTERING_THRESHOLD: 0.2
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] NORMAL_Z_THRESHOLD: 0.85
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] DEBUG_ITERATIONS: 9
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] ====================
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Reallocating buffers for 79662 points
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 17.552011ms
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_covariance] Reallocating buffers for 31434 points
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 943.336µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_clustering] Reallocating buffers for 31434 points
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 8.15ms
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 1.654338ms
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 661.877µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 2.67ms
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 306.251µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 627.377µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 2.02ms
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 644.669µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 648.752µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 2.27ms
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 590.46µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 597.293µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 2.24ms
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 996.378µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 317.585µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.97ms
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 335.959µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 309.834µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 2.27ms
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 296.751µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 311.209µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 2.17ms
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 329.876µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 325.085µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.93ms
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 342.834µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 317.084µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 2.19ms
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] === Processing Result ===
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] After processed points: 1823
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] Processing time: 73.05ms
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 346.543µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.29ms
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 301.585µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 959.67µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 363.834µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 911.29µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 308.126µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 889.80µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 329.001µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 865.21µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 617.793µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.26ms
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 334.334µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 937.54µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 332.001µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 882.25µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 299.667µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 930.38µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] Saved colored clusters to data/output/clustering_results/downsampled-clustering-results_voxel-0.05_NUM-125.pcd
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] === Average GPU Processing Time (excluding first 3 iterations) ===
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] Raw points voxelization time: 593.45µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] Raw points covariance time: 438.93µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] Raw points clustering time: 2.18ms
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] Downsampled points voxelization time: 410.65µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] Downsampled points clustering time: 997.38µs
[2026-02-23T09:36:01Z DEBUG remove_unnecessary_points] ====================





[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::init_gpu] === Available Vulkan Devices ===
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::init_gpu] Device 0: Virtio-GPU Venus (Apple M4 Pro) (IntegratedGpu)
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::init_gpu] Device 1: llvmpipe (LLVM 20.1.7, 128 bits) (Cpu)
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::init_gpu] 
    
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::init_gpu] Selected device: Virtio-GPU Venus (Apple M4 Pro) (IntegratedGpu)
    
[2026-02-23T09:36:42Z INFO  remove_unnecessary_points::init_gpu] Warning: Device does not support ext_shader_atomic_float extension.
[2026-02-23T09:36:42Z INFO  remove_unnecessary_points::init_gpu] Using fallback implementation with integer atomics.
[2026-02-23T09:36:42Z INFO  remove_unnecessary_points::init_gpu] Warning: Device does not support float32 atomic add feature.
[2026-02-23T09:36:42Z INFO  remove_unnecessary_points::init_gpu] Using fallback implementation with integer atomics.
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::init_gpu] === Vulkan Device Information ===
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::init_gpu] Device Name: Virtio-GPU Venus (Apple M4 Pro)
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::init_gpu] Device Type: IntegratedGpu
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::init_gpu] Vulkan context initialized successfully.
    
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] === Parameters ===
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] Input PCD path: data/input/transformed-combined-frame-125.pcd
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] Loaded points: 79662
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] Voxel size: 0.1
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] MIN_Z: -0.5
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] MAX_Z: 1.5
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] MIN_Z_RANGE: 0.7
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] MAX_Z_RANGE: 1.75
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] K_NEIGHBORS: 20
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] PLANARITY_THRESHOLD: 0.6
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] LINEARITY_THRESHOLD: 0.5
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] SCATTERING_THRESHOLD: 0.2
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] NORMAL_Z_THRESHOLD: 0.85
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] DEBUG_ITERATIONS: 9
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] ====================
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Reallocating buffers for 79662 points
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 16.082173ms
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_covariance] Reallocating buffers for 12729 points
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 894.836µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_clustering] Reallocating buffers for 12729 points
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.64ms
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 328.918µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 330.709µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.89ms
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 318.418µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 296.793µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.56ms
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 564.752µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 311.376µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 965.63µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 308.501µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 309.793µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 919.59µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 312.793µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 308.959µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.26ms
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 302.126µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 319.251µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 925.55µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 307.709µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 310.042µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.25ms
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 304.459µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 304.375µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 981.34µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 307.376µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 305.042µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.25ms
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] === Processing Result ===
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] After processed points: 1519
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] Processing time: 45.27ms
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 320.293µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 909.67µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 319.543µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 958.92µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 310.209µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 955.63µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 620.21µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 928.42µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 351.543µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 945.46µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 295.46µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 921.38µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 313.251µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 965.96µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 307.709µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 952.80µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 360.585µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.26ms
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] Saved colored clusters to data/output/clustering_results/downsampled-clustering-results_voxel-0.1_NUM-125.pcd
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] === Average GPU Processing Time (excluding first 3 iterations) ===
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] Raw points voxelization time: 389.31µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] Raw points covariance time: 357.72µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] Raw points clustering time: 1.14ms
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] Downsampled points voxelization time: 415.38µs
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] Downsampled points clustering time: 1.04ms
[2026-02-23T09:36:42Z DEBUG remove_unnecessary_points] ====================
```

# Radeon 780m
```bash
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::init_gpu] === Available Vulkan Devices ===
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::init_gpu] Device 0: AMD Radeon Graphics (RADV GFX1103_R1) (IntegratedGpu)
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::init_gpu] Device 1: llvmpipe (LLVM 15.0.7, 256 bits) (Cpu)
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::init_gpu] 
    
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::init_gpu] Selected device: AMD Radeon Graphics (RADV GFX1103_R1) (IntegratedGpu)
    
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::init_gpu] === Vulkan Device Information ===
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::init_gpu] Device Name: AMD Radeon Graphics (RADV GFX1103_R1)
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::init_gpu] Device Type: IntegratedGpu
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::init_gpu] Vulkan context initialized successfully.
    
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] === Parameters ===
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] Input PCD path: data/input/transformed-combined-frame-125.pcd
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] Loaded points: 79662
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] Voxel size: 0.05
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] MIN_Z: -0.5
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] MAX_Z: 1.5
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] MIN_Z_RANGE: 0.7
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] MAX_Z_RANGE: 1.75
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] K_NEIGHBORS: 20
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] PLANARITY_THRESHOLD: 0.6
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] LINEARITY_THRESHOLD: 0.5
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] SCATTERING_THRESHOLD: 0.2
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] NORMAL_Z_THRESHOLD: 0.85
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] DEBUG_ITERATIONS: 9
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] ====================
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Reallocating buffers for 79662 points
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 8.327819ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_covariance] Reallocating buffers for 31434 points
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 1.494552ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_clustering] Reallocating buffers for 31434 points
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 10.27ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 735.204µs
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 1.393402ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 10.34ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 774.487µs
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 1.459596ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 10.27ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 806.768µs
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 1.396918ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 9.05ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 761.613µs
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 1.363987ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 8.09ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 686.081µs
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 1.340983ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 7.57ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 682.073µs
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 1.326997ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 6.84ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 646.406µs
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 1.334892ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 6.53ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 666.714µs
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 1.315325ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 6.22ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 643.711µs
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 31434
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] GPU voxelization: 31434 points
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 1.317399ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 6.04ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] === Processing Result ===
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] After processed points: 1823
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] Processing time: 164.44ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 155.062µs
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.91ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 89.759µs
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.37ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 108.084µs
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.40ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 93.035µs
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.40ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 88.165µs
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.45ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 92.023µs
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.44ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 102.723µs
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.43ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 87.164µs
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.40ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 98.876µs
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1822
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 1.40ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] Saved colored clusters to data/output/clustering_results/downsampled-clustering-results_voxel-0.05_NUM-125.pcd
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] === Average GPU Processing Time (excluding first 3 iterations) ===
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] Raw points voxelization time: 914.01µs
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] Raw points covariance time: 1.51ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] Raw points clustering time: 6.96ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] Downsampled points voxelization time: 161.95µs
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] Downsampled points clustering time: 1.45ms
[2026-02-23T10:08:48Z DEBUG remove_unnecessary_points] ====================





[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::init_gpu] === Available Vulkan Devices ===
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::init_gpu] Device 0: AMD Radeon Graphics (RADV GFX1103_R1) (IntegratedGpu)
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::init_gpu] Device 1: llvmpipe (LLVM 15.0.7, 256 bits) (Cpu)
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::init_gpu] 
    
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::init_gpu] Selected device: AMD Radeon Graphics (RADV GFX1103_R1) (IntegratedGpu)
    
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::init_gpu] === Vulkan Device Information ===
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::init_gpu] Device Name: AMD Radeon Graphics (RADV GFX1103_R1)
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::init_gpu] Device Type: IntegratedGpu
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::init_gpu] Vulkan context initialized successfully.
    
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points] === Parameters ===
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points] Input PCD path: data/input/transformed-combined-frame-125.pcd
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points] Loaded points: 79662
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points] Voxel size: 0.1
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points] MIN_Z: -0.5
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points] MAX_Z: 1.5
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points] MIN_Z_RANGE: 0.7
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points] MAX_Z_RANGE: 1.75
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points] K_NEIGHBORS: 20
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points] PLANARITY_THRESHOLD: 0.6
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points] LINEARITY_THRESHOLD: 0.5
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points] SCATTERING_THRESHOLD: 0.2
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points] NORMAL_Z_THRESHOLD: 0.85
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points] DEBUG_ITERATIONS: 9
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points] ====================
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_voxel] Reallocating buffers for 79662 points
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 2.948357ms
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_covariance] Reallocating buffers for 12729 points
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 644.102µs
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_clustering] Reallocating buffers for 12729 points
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 4.15ms
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 645.294µs
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 654.481µs
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 4.13ms
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 651.265µs
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 627.11µs
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 4.10ms
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 644.132µs
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 649.793µs
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 4.25ms
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 666.444µs
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 701.139µs
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 4.13ms
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 633.602µs
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T10:09:13Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 639.624µs
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 4.11ms
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 624.896µs
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 639.143µs
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 4.15ms
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 625.046µs
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 639.904µs
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 4.02ms
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 616.31µs
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 629.865µs
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 3.77ms
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 612.081µs
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 12729
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points] GPU voxelization: 12729 points
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_covariance] Compute covariance shader execution time: 640.866µs
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 3.55ms
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points] === Processing Result ===
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points] After processed points: 1519
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points] Processing time: 86.78ms
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 135.975µs
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 2.29ms
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 113.824µs
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 2.15ms
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 129.534µs
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 2.18ms
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 111.951µs
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 2.20ms
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 112.592µs
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 2.16ms
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 115.568µs
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 2.13ms
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 113.133µs
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 2.06ms
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 122.23µs
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 2.02ms
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Compute voxelization shader execution time: 110.448µs
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_voxel] Number of output points: 1518
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points::gpu_clustering] Compute clustering shader execution time: 2.00ms
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points] Saved colored clusters to data/output/clustering_results/downsampled-clustering-results_voxel-0.1_NUM-125.pcd
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points] === Average GPU Processing Time (excluding first 3 iterations) ===
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points] Raw points voxelization time: 820.90µs
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points] Raw points covariance time: 793.99µs
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points] Raw points clustering time: 4.03ms
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points] Downsampled points voxelization time: 188.23µs
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points] Downsampled points clustering time: 2.13ms
[2026-02-23T10:09:14Z DEBUG remove_unnecessary_points] ====================
```