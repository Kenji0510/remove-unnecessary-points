use core::f64;
use std::{
    collections::HashMap,
    f32::{INFINITY, NEG_INFINITY},
    num::{NonZero, NonZeroUsize},
    os::unix::process,
    sync::Arc,
};

use anyhow::{Context, Result};
use log::{debug, info};
use nalgebra::{Matrix3, SymmetricEigen, Vector3};
use ndarray::Array2;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use remove_unnecessary_points::{
    clustering::extract_human_clusters,
    convert_2d_xy::{CellStats, project_to_xy_grid},
    gpu_clustering::ClusteringGpuContext,
    gpu_covariance::CovarianceGpuContext,
    gpu_voxel::VoxelGpuContext,
    init_gpu::VulkanContext,
    load_pcds::load_filenames,
    oprate_pcd::{
        PointXYZ, PointXYZCovs, PointXYZWithShapeFeat, load_pcd_xyz, load_pcd_xyzrgb, save_pcd,
        save_pcd_with_covs, save_pcd_with_shape_feats, save_xyz_pcd,
    },
    plot::plot_xy_grid_heatmap,
    voxelize::voxel_downsample_array2,
};

const K_NEIGHBORS: usize = 20;
const VOXEL_SIZE: f32 = 0.05;
const PLANARITY_THRESHOLD: f64 = 0.85;
const LINEARITY_THRESHOLD: f64 = 0.85;
const SCATTERING_THRESHOLD: f64 = 0.06;
const NORMAL_Z_THRESHOLD: f64 = 0.85;
const MIN_CLUSTER_POINTS: usize = 200;
const PCD_PATH: &str = "data/input/20260210/box/transformed-combined-frame-180.pcd";
const PCD_DIR_PATH: &str = "/home/kenji/workspace/rust/r2r-subscriber-for-avia/data/output/transformed_data/avia/avias-20260210-04-carryboard-and-box";
const SAVE_DATA_PATH: &str = "data/output/branch-20260301-Unified-each-process";
const MIN_Z: f32 = -0.5;
const MAX_Z: f32 = 1.5;
const MIN_Z_RANGE: f32 = 0.7;
const MAX_Z_RANGE: f32 = 1.75;
const NUMBERING: usize = 180;

const DEBUG_ITERATIONS: usize = 9;

struct ProcessTime {
    voxelization_time: std::time::Duration,
    covariance_time: std::time::Duration,
    clustering_time: std::time::Duration,
}

struct ProcessTimeResults {
    raw_processing_time: ProcessTime,
    downsampled_processing_time: ProcessTime,
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    // <!--- Load PCD paths --->
    let pcd_dir = load_filenames(PCD_DIR_PATH)?;
    info!(
        "Found {} PCD files in directory {}",
        pcd_dir.len(),
        PCD_DIR_PATH
    );
    // return Ok(());
    // <!--- Load PCD paths --->

    // <!--- Initialize GPU contexts --->
    let vulkan_context = VulkanContext::new().context("Failed to initialize Vulkan context")?;
    let mut gpu_voxel_ctx = VoxelGpuContext::new(vulkan_context.clone())
        .context("Failed to create GPU voxel context")?;
    let mut gpu_covariance_ctx = CovarianceGpuContext::new(vulkan_context.clone())
        .context("Failed to create GPU covariance context")?;
    let mut gpu_clustering_ctx = ClusteringGpuContext::new(vulkan_context.clone())
        .context("Failed to create GPU clustering context")?;
    // <!--- Initialize GPU contexts --->

    debug!("=== Parameters ===");
    debug!("Input PCD path: {}", PCD_PATH);
    debug!("Voxel size: {}", VOXEL_SIZE);
    debug!("MIN_Z: {}", MIN_Z);
    debug!("MAX_Z: {}", MAX_Z);
    debug!("MIN_Z_RANGE: {}", MIN_Z_RANGE);
    debug!("MAX_Z_RANGE: {}", MAX_Z_RANGE);
    debug!("K_NEIGHBORS: {}", K_NEIGHBORS);
    debug!("PLANARITY_THRESHOLD: {}", PLANARITY_THRESHOLD);
    debug!("LINEARITY_THRESHOLD: {}", LINEARITY_THRESHOLD);
    debug!("SCATTERING_THRESHOLD: {}", SCATTERING_THRESHOLD);
    debug!("NORMAL_Z_THRESHOLD: {}", NORMAL_Z_THRESHOLD);
    debug!("DEBUG_ITERATIONS: {}", DEBUG_ITERATIONS);
    debug!("====================");

    for (i, pcd_path) in pcd_dir.iter().enumerate() {
        debug!(
            "=== Processing PCD {}/{}: {} ===",
            i + 1,
            pcd_dir.len(),
            pcd_path.display()
        );
        // <!--- Load PCD --->
        let pcd = load_pcd_xyz(pcd_path.to_str().context("Invalid PCD path")?)
            .context("Failed to load the pcd")?;
        // <!--- Load PCD --->

        // <!--- Convert to 2D XY grid --->
        // Remove unnecessary points by z-range before converting to 2D XY grid
        let processed_grid = project_to_xy_grid(&pcd, VOXEL_SIZE, NEG_INFINITY, INFINITY);
        // <!--- Convert to 2D XY grid --->

        let processed_pcd = grid_to_pcd(&processed_grid);
        let pts_vec = pcd_to_vecf32(&processed_pcd);

        // <!--- Downsampled pcd by GPU Voxelization --->
        let downsampled_pts = gpu_voxel_ctx.voxelization(&pts_vec, pts_vec.len(), VOXEL_SIZE)?;
        // <!--- Downsampled pcd by GPU Voxelization --->

        // <!--- Compute covariances --->
        // Compute covariances on GPU using the same downsampled points
        let pts_covs = gpu_covariance_ctx.compute_covariances(
            &gpu_voxel_ctx,
            &downsampled_pts,
            downsampled_pts.len(),
            false,
        )?;
        // <!--- Compute covariances --->

        // <!--- Compute shape features --->
        let shape_feats = compute_shape_features_02(&pts_covs);
        // <!--- Compute shape features --->

        let pcd_with_shape_feats = convert_to_pcd_from_vec(&downsampled_pts, &shape_feats);

        // <!--- Remove unnecessary points by shape features --->
        let removed_pcd = remove_unnecessary_points_by_shape_feats(&pcd_with_shape_feats)?;
        // <!--- Remove unnecessary points by shape features --->

        let pts_vec = pcd_shapefeat_to_vecf32(&removed_pcd);
        // <!--- Downsampled pcd by GPU Voxelization --->
        let downsampled_pts = gpu_voxel_ctx.voxelization(&pts_vec, pts_vec.len(), VOXEL_SIZE)?;
        // <!--- Downsampled pcd by GPU Voxelization --->

        // <!--- Compute clustering on GPU using the same downsampled points --->
        let cluster_ids = gpu_clustering_ctx.clustering(
            &gpu_voxel_ctx,
            &downsampled_pts,
            downsampled_pts.len(),
            VOXEL_SIZE,
        )?;
        // <!--- Compute clustering on GPU using the same downsampled points --->

        // <!--- Save removed unnecessary points pcd --->
        // let save_removed_pcd_path = format!(
        //     "{}/removed_unnecessary_pcd/removed_num-{}_voxel-{}.pcd",
        //     SAVE_DATA_PATH, i, VOXEL_SIZE
        // );
        // save_pcd_with_shape_feats(&pcd_with_shape_feats, &save_removed_pcd_path)?;
        // info!(
        //     "Saved removed unnecessary points pcd to {}",
        //     save_removed_pcd_path
        // );
        // <!--- Save removed unnecessary points pcd --->

        // <!--- Save clustering results --->
        let clustering_results_save_path = format!(
            "{}/clustering/clustering_num-{}_voxel-{}.pcd",
            SAVE_DATA_PATH, i, VOXEL_SIZE
        );
        save_colored_clusters_pcd(
            &clustering_results_save_path,
            &downsampled_pts,
            &cluster_ids,
        )?;
        info!("Saved colored clusters to {}", clustering_results_save_path);

        let human_cluster_pairs =
            extract_human_clusters(&downsampled_pts, &cluster_ids, MIN_CLUSTER_POINTS);

        for (j, (_, human_cluster, count)) in human_cluster_pairs.iter().enumerate() {
            if j > 3 {
                break;
            }

            let human_cluster_save_path = format!(
                "{}/human_clusters/human-cluster-{}_num-{}_voxel-{}.pcd",
                SAVE_DATA_PATH, j, i, VOXEL_SIZE
            );
            let human_cluster_pcd = vecf32_to_pcd(human_cluster);
            save_xyz_pcd(&human_cluster_pcd, &human_cluster_save_path)?;
            info!("Saved human cluster {} to {}", j, human_cluster_save_path);
        }
        // <!--- Save clustering results --->
    }

    Ok(())
}

fn id_to_rgb_float(id: u32) -> f32 {
    let h = ((id as f32) * 137.508) % 360.0;
    let s = 0.85; // 彩度
    let v = 0.95; // 明度

    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let r_u8 = ((r + m) * 255.0) as u32;
    let g_u8 = ((g + m) * 255.0) as u32;
    let b_u8 = ((b + m) * 255.0) as u32;

    let rgb_u32 = (r_u8 << 16) | (g_u8 << 8) | b_u8;
    f32::from_bits(rgb_u32)
}

pub fn save_colored_clusters_pcd(
    path: &str,
    pts: &[[f32; 3]],
    cluster_ids: &[u32],
) -> anyhow::Result<()> {
    use std::io::Write;
    let mut file = std::fs::File::create(path)?;

    writeln!(file, "# .PCD v0.7 - Point Cloud Data file format")?;
    writeln!(file, "VERSION 0.7")?;
    writeln!(file, "FIELDS x y z rgb")?;
    writeln!(file, "SIZE 4 4 4 4")?;
    writeln!(file, "TYPE F F F F")?;
    writeln!(file, "COUNT 1 1 1 1")?;
    writeln!(file, "WIDTH {}", pts.len())?;
    writeln!(file, "HEIGHT 1")?;
    writeln!(file, "VIEWPOINT 0 0 0 1 0 0 0")?;
    writeln!(file, "POINTS {}", pts.len())?;
    writeln!(file, "DATA ascii")?;

    for i in 0..pts.len() {
        let rgb_f32 = id_to_rgb_float(cluster_ids[i]);
        writeln!(
            file,
            "{} {} {} {}",
            pts[i][0], pts[i][1], pts[i][2], rgb_f32
        )?;
    }

    Ok(())
}

fn pcd_shapefeat_to_vecf32(points: &[PointXYZWithShapeFeat]) -> Vec<[f32; 3]> {
    points
        .iter()
        .map(|p| [p.x as f32, p.y as f32, p.z as f32])
        .collect()
}

fn pcd_to_vecf32(points: &[PointXYZ]) -> Vec<[f32; 3]> {
    points
        .iter()
        .map(|p| [p.x as f32, p.y as f32, p.z as f32])
        .collect()
}

fn vecf32_to_pcd(points: &[[f32; 3]]) -> Vec<PointXYZ> {
    points
        .iter()
        .map(|p| PointXYZ {
            x: p[0],
            y: p[1],
            z: p[2],
        })
        .collect()
}

fn remove_unnecessary_points_by_shape_feats(
    pcd_with_shape_feats: &Vec<PointXYZWithShapeFeat>,
) -> Result<Vec<PointXYZWithShapeFeat>> {
    let removed_pts: Vec<PointXYZWithShapeFeat> = pcd_with_shape_feats
        .iter()
        .filter(|p| {
            let is_horizontal = p.normal_z.abs() > NORMAL_Z_THRESHOLD;
            let is_planar = (p.planarity as f64) > PLANARITY_THRESHOLD;
            // let is_linear = (p.linearity as f64) > LINEARITY_THRESHOLD;
            let is_scattering = (p.scattering as f64) > SCATTERING_THRESHOLD;

            if is_horizontal {
                return false;
            }
            if is_planar {
                return false;
            }
            if is_scattering {
                return true;
            }

            false
        })
        .cloned()
        .collect();

    Ok(removed_pts)
}

fn grid_to_pcd(grid: &HashMap<(i32, i32), CellStats>) -> Vec<PointXYZ> {
    let mut points: Vec<PointXYZ> = Vec::new();

    for (&(ix, iy), cell) in grid.iter() {
        if let Some(pts) = &cell.pts {
            for p in pts {
                points.push(PointXYZ {
                    x: p.x,
                    y: p.y,
                    z: p.z,
                });
            }
        }
    }

    points
}

fn remove_unnecessary_points(
    converted_2d_grid: &HashMap<(i32, i32), CellStats>,
    remove_min_z_range: f32,
    remove_max_z_range: f32,
    remove_min_z: f32,
    remove_max_z: f32,
) -> Result<HashMap<(i32, i32), CellStats>> {
    let mut processed_grid: HashMap<(i32, i32), CellStats> = HashMap::new();

    for (&(ix, iy), cell) in converted_2d_grid.iter() {
        // if cell.z_range() < remove_min_z_range || cell.z_range() > remove_max_z_range {
        //     continue;
        // }
        if let Some(pts) = &cell.pts {
            for p in pts {
                if p.z < remove_min_z || p.z > remove_max_z {
                    continue;
                }
                // if cell.z_range() < remove_min_z_range || cell.z_range() > remove_max_z_range {
                if cell.z_range() > remove_max_z_range {
                    continue;
                }

                processed_grid.insert((ix, iy), cell.clone());
            }
        }
    }

    Ok(processed_grid)
}

// fn remove_unnecessary_points(
//     pcd_with_shape_feats: &Vec<PointXYZWithShapeFeat>,
// ) -> Result<Vec<PointXYZWithShapeFeat>> {
//     let removed_pts: Vec<PointXYZWithShapeFeat> = pcd_with_shape_feats
//         .iter()
//         .filter(|p| {
//             !((p.planarity as f64) > PLANARITY_THRESHOLD
//                 || (p.scattering as f64) < SCATTERING_THRESHOLD)
//         })
//         .cloned()
//         .collect();

//     Ok(removed_pts)
// }

fn convert_to_pcd_from_vec(
    pts_vec: &Vec<[f32; 3]>,
    shape_feats: &Vec<ShapeFeat>,
) -> Vec<PointXYZWithShapeFeat> {
    let n_points = pts_vec.len();
    let mut pcd: Vec<PointXYZWithShapeFeat> = Vec::with_capacity(n_points);
    for i in 0..n_points {
        let p = &pts_vec[i];
        let shape_feat = &shape_feats[i];
        let point = PointXYZWithShapeFeat {
            x: p[0] as f32,
            y: p[1] as f32,
            z: p[2] as f32,
            linearity: shape_feat.linearity,
            planarity: shape_feat.planarity,
            scattering: shape_feat.scattering,
            l1: shape_feat.l1,
            l2: shape_feat.l2,
            l3: shape_feat.l3,
            normal_z: shape_feat.normal_z,
        };
        pcd.push(point);
    }

    pcd
}

fn point_xyz_to_array2(points: &[PointXYZ]) -> Array2<f32> {
    let n = points.len();
    let mut arr = Array2::<f32>::zeros((n, 3));
    for (i, p) in points.iter().enumerate() {
        arr[[i, 0]] = p.x as f32;
        arr[[i, 1]] = p.y as f32;
        arr[[i, 2]] = p.z as f32;
    }

    arr
}

#[derive(Debug, Clone, Copy)]
pub struct ShapeFeat {
    pub linearity: f64,
    pub planarity: f64,
    pub scattering: f64,
    pub l1: f64,
    pub l2: f64,
    pub l3: f64,
    pub normal_z: f64,
}

fn sort_desc3(mut a: f64, mut b: f64, mut c: f64) -> (f64, f64, f64) {
    // sort descending
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    if a < c {
        std::mem::swap(&mut a, &mut c);
    }
    if b < c {
        std::mem::swap(&mut b, &mut c);
    }
    (a, b, c)
}

pub fn compute_shape_features(
    pts: &Array2<f32>,
    kdtree: &kiddo::ImmutableKdTree<f32, 3>,
    k_neighbors: usize,
) -> Result<Vec<ShapeFeat>> {
    let n_points = pts.nrows();
    let k_neighbors = NonZeroUsize::new(k_neighbors).context("Invalid number of neighbors")?;

    let shape_feats = (0..n_points)
        .into_par_iter()
        .map(|i| {
            let query = [pts[[i, 0]], pts[[i, 1]], pts[[i, 2]]];

            let neighbors = kdtree.nearest_n::<kiddo::SquaredEuclidean>(&query, k_neighbors);
            if neighbors.len() < 5 {
                return ShapeFeat {
                    linearity: 0.0,
                    planarity: 0.0,
                    scattering: 1.0,
                    l1: 0.0,
                    l2: 0.0,
                    l3: 0.0,
                    normal_z: 0.0,
                };
            }

            // mean
            let mut mean = Vector3::<f64>::zeros();
            for n in &neighbors {
                let idx = n.item as usize;
                mean.x += pts[[idx, 0]] as f64;
                mean.y += pts[[idx, 1]] as f64;
                mean.z += pts[[idx, 2]] as f64;
            }
            mean /= neighbors.len() as f64;

            // covariance
            let mut cov = Matrix3::<f64>::zeros();
            for n in &neighbors {
                let idx = n.item as usize;
                let p = Vector3::new(
                    pts[[idx, 0]] as f64,
                    pts[[idx, 1]] as f64,
                    pts[[idx, 2]] as f64,
                );
                let d = p - mean;
                cov += d * d.transpose();
            }
            cov /= neighbors.len() as f64;

            // eigen (raw)
            let eigen = nalgebra::linalg::SymmetricEigen::new(cov);
            let (l1, l2, l3) = sort_desc3(
                eigen.eigenvalues[0],
                eigen.eigenvalues[1],
                eigen.eigenvalues[2],
            );

            let eps = 1e-12;
            let denom = (l1.abs()).max(eps);

            let linearity = ((l1 - l2) / denom).clamp(0.0, 1.0);
            let planarity = ((l2 - l3) / denom).clamp(0.0, 1.0);
            let scattering = (l3 / denom).clamp(0.0, 1.0);

            ShapeFeat {
                linearity,
                planarity,
                scattering,
                l1,
                l2,
                l3,
                normal_z: eigen.eigenvectors.column(2).z,
            }
        })
        .collect();

    Ok(shape_feats)
}

pub fn compute_shape_features_02(covs: &Vec<[f32; 9]>) -> Vec<ShapeFeat> {
    let num_points = covs.len();
    let mut shape_feats: Vec<ShapeFeat> = Vec::with_capacity(num_points);

    for cov in covs {
        let cov_matrix = Matrix3::new(
            cov[0] as f64,
            cov[1] as f64,
            cov[2] as f64,
            cov[3] as f64,
            cov[4] as f64,
            cov[5] as f64,
            cov[6] as f64,
            cov[7] as f64,
            cov[8] as f64,
        );

        let eigen = nalgebra::linalg::SymmetricEigen::new(cov_matrix);
        let (l1, l2, l3) = sort_desc3(
            eigen.eigenvalues[0],
            eigen.eigenvalues[1],
            eigen.eigenvalues[2],
        );

        let eps = 1e-12;
        let denom = (l1.abs()).max(eps);

        let linearity = ((l1 - l2) / denom).clamp(0.0, 1.0);
        let planarity = ((l2 - l3) / denom).clamp(0.0, 1.0);
        let scattering = (l3 / denom).clamp(0.0, 1.0);

        shape_feats.push(ShapeFeat {
            linearity,
            planarity,
            scattering,
            l1,
            l2,
            l3,
            normal_z: eigen.eigenvectors.column(2).z,
        });
    }

    shape_feats
}
