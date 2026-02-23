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