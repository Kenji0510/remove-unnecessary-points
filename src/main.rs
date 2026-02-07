use core::f64;
use std::num::{NonZero, NonZeroUsize};

use anyhow::{Context, Result};
use nalgebra::{Matrix3, SymmetricEigen, Vector3};
use ndarray::Array2;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use remove_unnecessary_points::{
    oprate_pcd::{PointXYZ, PointXYZCovs, PointXYZWithShapeFeat, load_pcd_xyz, load_pcd_xyzrgb, save_pcd_with_covs, save_pcd_with_shape_feats},
    voxelize::voxel_downsample_array2,
};

const K_NEIGHBORS: usize = 20;
const VOXEL_SIZE: f32 = 0.01;
const PCD_PATH: &str = "data/input/with-person-H927-on-hallway.pcd";
const SAVE_PCD_PATH: &str = "data/output/voxelized-01_with-person-H927-on-hallway.pcd";

fn main() -> Result<()> {
    // let pcd = load_pcd_xyz(PCD_PATH).context("Failed to load the pcd")?;
    let pcd = load_pcd_xyzrgb(PCD_PATH).context("Failed to load the pcd")?;

    let pts = point_xyz_to_array2(&pcd);

    let voxelized_pts = voxel_downsample_array2(&pts, VOXEL_SIZE);

    let pts_vec: Vec<[f32; 3]> = voxelized_pts
        .outer_iter()
        .map(|row| [row[0], row[1], row[2]])
        .collect();

    let pts_kdtree = kiddo::ImmutableKdTree::new_from_slice(&pts_vec);

    // let pts_covs = compute_covariances(&voxelized_pts, &pts_kdtree);
    let shape_feats = compute_shape_features(&voxelized_pts, &pts_kdtree, K_NEIGHBORS);

    // let pcd_with_covs = convert_to_pcd_from_vec(&pts_vec, &pts_covs);
    let pcd_with_shape_feats = convert_to_pcd_from_vec(&pts_vec, &shape_feats);
    // save_pcd_with_covs(&pcd_with_covs, SAVE_PCD_PATH)?;
    save_pcd_with_shape_feats(&pcd_with_shape_feats, SAVE_PCD_PATH)?;

    Ok(())
}

// fn convert_to_pcd_from_vec(
//     pts_vec: &Vec<[f32; 3]>,
//     pts_covs: &Vec<Matrix3<f64>>,
// ) -> Vec<PointXYZCovs> {
//     let n_points = pts_vec.len();
//     let mut pcd: Vec<PointXYZCovs> = Vec::with_capacity(n_points);
//     for i in 0..n_points {
//         let p = &pts_vec[i];
//         let cov = &pts_covs[i];
//         let point = PointXYZCovs {
//             x: p[0] as f32,
//             y: p[1] as f32,
//             z: p[2] as f32,
//             cov11: cov[(0, 0)] as f32,
//             cov12: cov[(0, 1)] as f32,
//             cov13: cov[(0, 2)] as f32,
//             cov21: cov[(1, 0)] as f32,
//             cov22: cov[(1, 1)] as f32,
//             cov23: cov[(1, 2)] as f32,
//             cov31: cov[(2, 0)] as f32,
//             cov32: cov[(2, 1)] as f32,
//             cov33: cov[(2, 2)] as f32,
//         };
//         pcd.push(point);
//     }

//     pcd
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
    if a < b { std::mem::swap(&mut a, &mut b); }
    if a < c { std::mem::swap(&mut a, &mut c); }
    if b < c { std::mem::swap(&mut b, &mut c); }
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
                    l1: 0.0, l2: 0.0, l3: 0.0,
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

            ShapeFeat { linearity, planarity, scattering, l1, l2, l3 }
        })
        .collect()
}

fn compute_covariances(
    pts: &Array2<f32>,
    kdtree: &kiddo::ImmutableKdTree<f32, 3>,
) -> Vec<Matrix3<f64>> {
    let n_points = pts.nrows();
    let k_neighbors = NonZero::new(K_NEIGHBORS).unwrap();

    (0..n_points)
        .into_par_iter()
        .map(|i| {
            let qx = pts[[i, 0]];
            let qy = pts[[i, 1]];
            let qz = pts[[i, 2]];
            let query = [qx, qy, qz];

            let neighbors = kdtree.nearest_n::<kiddo::SquaredEuclidean>(&query, k_neighbors);

            if neighbors.len() < 5 {
                return Matrix3::identity();
            }

            let mut mean = Vector3::zeros();
            for n in &neighbors {
                let idx = n.item as usize;
                mean += Vector3::new(
                    pts[[idx, 0]] as f64,
                    pts[[idx, 1]] as f64,
                    pts[[idx, 2]] as f64,
                );
            }
            mean /= neighbors.len() as f64;

            let mut cov = Matrix3::zeros();
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

            let eigen = SymmetricEigen::new(cov);
            let rot = eigen.eigenvectors;
            let mut vals = eigen.eigenvalues;

            let max_val = vals.max();
            if max_val > 0.0 {
                vals /= max_val;
            }

            let min_eigenvalue = 1e-6;
            for i in 0..vals.len() {
                if vals[i] < min_eigenvalue {
                    vals[i] = min_eigenvalue;
                }
            }

            let regularized_cov = rot * Matrix3::from_diagonal(&vals) * rot.transpose();

            regularized_cov
        })
        .collect()
}
