use core::f64;
use std::{
    collections::HashMap,
    f32::{INFINITY, NEG_INFINITY},
    num::{NonZero, NonZeroUsize},
    sync::Arc,
};

use anyhow::{Context, Result};
use nalgebra::{Matrix3, SymmetricEigen, Vector3};
use ndarray::Array2;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use remove_unnecessary_points::{
    convert_2d_xy::{CellStats, project_to_xy_grid},
    gpu_covariance::CovarianceGpuContext,
    gpu_voxel::VoxelGpuContext,
    gpu_voxel_temp::voxelization,
    init_gpu::VulkanContext,
    oprate_pcd::{
        PointXYZ, PointXYZCovs, PointXYZWithShapeFeat, load_pcd_xyz, load_pcd_xyzrgb, save_pcd,
        save_pcd_with_covs, save_pcd_with_shape_feats, save_xyz_pcd,
    },
    plot::plot_xy_grid_heatmap,
    voxelize::voxel_downsample_array2,
};

const K_NEIGHBORS: usize = 20;
const VOXEL_SIZE: f32 = 0.05;
const PLANARITY_THRESHOLD: f64 = 0.6;
const LINEARITY_THRESHOLD: f64 = 0.5;
const SCATTERING_THRESHOLD: f64 = 0.2;
const PCD_PATH: &str = "data/input/transformed-combined-frame-125.pcd";
const SAVE_ORIGINAL_PCD_PATH: &str = "data/output/voxelized-H927-hallway-01.pcd";
const SAVE_REMOVED_PCD_PATH: &str =
    "data/output/2d-xy/convert-2d-pts-by-covariance/removed-by-shape-feats_voxelized-0.2.pcd";
const MIN_Z: f32 = -0.5;
const MAX_Z: f32 = 1.5;
const MIN_Z_RANGE: f32 = 0.7;
const MAX_Z_RANGE: f32 = 1.75;
const NUMBERING: usize = 125;

fn main() -> Result<()> {
    let vulkan_context = VulkanContext::new().context("Failed to initialize Vulkan context")?;
    let mut gpu_voxel_ctx = VoxelGpuContext::new(vulkan_context.clone())
        .context("Failed to create GPU voxel context")?;
    let mut gpu_covariance_ctx = CovarianceGpuContext::new(vulkan_context.clone())
        .context("Failed to create GPU covariance context")?;

    let pcd = load_pcd_xyz(PCD_PATH).context("Failed to load the pcd")?;
    // let pcd = load_pcd_xyzrgb(PCD_PATH).context("Failed to load the pcd")?;

    println!("=== Parameters ===");
    println!("Input PCD path: {}", PCD_PATH);
    println!("Loaded points: {}", pcd.len());
    println!("Voxel size: {}", VOXEL_SIZE);
    println!("MIN_Z: {}", MIN_Z);
    println!("MAX_Z: {}", MAX_Z);
    println!("MIN_Z_RANGE: {}", MIN_Z_RANGE);
    println!("MAX_Z_RANGE: {}", MAX_Z_RANGE);
    println!("K_NEIGHBORS: {}", K_NEIGHBORS);
    println!("PLANARITY_THRESHOLD: {}", PLANARITY_THRESHOLD);
    println!("LINEARITY_THRESHOLD: {}", LINEARITY_THRESHOLD);
    println!("SCATTERING_THRESHOLD: {}", SCATTERING_THRESHOLD);
    println!("====================");

    let start = std::time::Instant::now();

    // let converted_2d_grid = project_to_xy_grid(&pcd, VOXEL_SIZE, -0.55, 1.5);
    let processed_grid = project_to_xy_grid(&pcd, VOXEL_SIZE, NEG_INFINITY, INFINITY);

    // let save_path = format!(
    //     "data/output/2d-xy/converted-2d-grid_voxel-{}.png",
    //     VOXEL_SIZE
    // );
    // plot_xy_grid_heatmap(
    //     &converted_2d_grid,
    //     VOXEL_SIZE,
    //     &save_path,
    //     "XY Grid: count",
    //     |cell| cell.z_range() as f64,
    // )?;
    // println!("Saved plot to {}", save_path);

    // let processed_grid = remove_unnecessary_points(&converted_2d_grid, 0.3, 1.3)?;
    // let processed_grid =
    //     remove_unnecessary_points(&converted_2d_grid, MIN_Z, MAX_Z, MIN_Z_RANGE, MAX_Z_RANGE)?;

    // let save_path = format!(
    //     "data/output/2d-xy/processed-2d-grid_voxel-{}_NUM-{}.png",
    //     VOXEL_SIZE, NUMBERING
    // );
    // plot_xy_grid_heatmap(
    //     &processed_grid,
    //     VOXEL_SIZE,
    //     &save_path,
    //     "XY Grid: Count",
    //     |cell| cell.count as f64,
    // )?;
    // println!("Saved plot to {}", save_path);

    let processed_pcd = grid_to_pcd(&processed_grid);

    // let save_path = format!(
    //     "data/output/2d-xy/removed_voxel-{}_NUM-{}.pcd",
    //     VOXEL_SIZE, NUMBERING
    // );
    // save_xyz_pcd(&processed_pcd, &save_path).context("Failed to save the processed pcd")?;
    // println!("Saved processed PCD to {}", save_path);

    // Downsample the points
    // let pts_array2 = point_xyz_to_array2(&processed_pcd);
    // let downsampled_pts = voxel_downsample_array2(&pts_array2, VOXEL_SIZE);

    let pts_vec = pcd_to_vecf32(&processed_pcd);
    let downsampled_pts = gpu_voxel_ctx.voxelization(&pts_vec, pts_vec.len(), VOXEL_SIZE)?;
    println!("GPU voxelization: {} points", downsampled_pts.len());

    // let pts_vec: Vec<[f32; 3]> = downsampled_pts
    //     .outer_iter()
    //     .map(|row| [row[0], row[1], row[2]])
    //     .collect();
    // println!("CPU downsampling: {} points", pts_vec.len());

    // Convert to flat f32 vec for GPU
    // let pts_flat: Vec<f32> = pts_vec.iter().flat_map(|p| p.iter().copied()).collect();
    // let num_points = pts_vec.len();

    // Compute covariances on GPU using the same downsampled points
    let pts_covs =
        gpu_covariance_ctx.compute_covariances(&downsampled_pts, downsampled_pts.len())?;
    // let pts_kdtree = kiddo::ImmutableKdTree::new_from_slice(&pts_vec);
    // let shape_feats = compute_shape_features(&downsampled_pts, &pts_kdtree, K_NEIGHBORS);

    let shape_feats = compute_shape_features_02(&pts_covs);

    let pcd_with_shape_feats = convert_to_pcd_from_vec(&downsampled_pts, &shape_feats);

    let removed_pcd = remove_unnecessary_points_by_shape_feats(&pcd_with_shape_feats)?;

    let elapsed = start.elapsed();
    println!("=== Processing Result ===");
    println!("After processed points: {}", removed_pcd.len());
    println!("Processing time: {:.2?}", elapsed);

    let save_removed_pcd_path = format!(
        "data/output/2d-xy/convert-2d-pts-by-covariance/removed-by-shape-feats_voxel-{}_NUM-{}.pcd",
        VOXEL_SIZE, NUMBERING
    );
    save_pcd_with_shape_feats(&removed_pcd, &save_removed_pcd_path)?;
    println!(
        "Saved removed unnecessary points pcd to {}",
        save_removed_pcd_path
    );

    let save_original_pcd_path = format!(
        "data/output/2d-xy/convert-2d-pts-by-covariance/original-by-shape-feats_voxel-{}_NUM-{}.pcd",
        VOXEL_SIZE, NUMBERING
    );
    save_pcd_with_shape_feats(&pcd_with_shape_feats, &save_original_pcd_path)?;
    println!(
        "Saved voxelized pcd with shape features to {}",
        save_original_pcd_path
    );

    Ok(())
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
            !((p.planarity as f64) > PLANARITY_THRESHOLD
                || (p.scattering as f64) < SCATTERING_THRESHOLD)
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
) -> Vec<ShapeFeat> {
    let n_points = pts.nrows();
    let k_neighbors = NonZeroUsize::new(k_neighbors).unwrap();

    (0..n_points)
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
            }
        })
        .collect()
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
        });
    }

    shape_feats
}
