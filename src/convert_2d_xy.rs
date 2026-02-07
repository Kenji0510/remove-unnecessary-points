use std::collections::HashMap;

use crate::oprate_pcd::PointXYZ;



#[derive(Debug, Default, Clone)]
pub struct CellStats {
    pub count: u32,
    pub z_min: f32,
    pub z_max: f32,
    pub z_sum: f64,
    pub z_sq_sum: f64,
}

impl CellStats {
    fn new(z: f32) -> Self {
        Self {
            count: 1,
            z_min: z,
            z_max: z,
            z_sum: z as f64,
            z_sq_sum: (z as f64) * (z as f64),
        }
    }
    fn add(&mut self, z: f32) {
        self.count += 1;
        self.z_min = self.z_min.min(z);
        self.z_max = self.z_max.max(z);
        self.z_sum += z as f64;
        self.z_sq_sum += (z as f64) * (z as f64);
    }
    pub fn z_mean(&self) -> f32 {
        (self.z_sum / self.count as f64) as f32
    }
    pub fn z_std(&self) -> f32 {
        let n = self.count as f64;
        let mean = self.z_sum / n;
        let var = (self.z_sq_sum / n) - mean * mean;
        var.max(0.0).sqrt() as f32
    }
    pub fn z_range(&self) -> f32 {
        self.z_max - self.z_min
    }
}

pub fn project_to_xy_grid(
    pts: &[PointXYZ],
    voxel_xy: f32,
    z_min_keep: f32,
    z_max_keep: f32,
) -> HashMap<(i32, i32), CellStats> {
    let mut grid: HashMap<(i32, i32), CellStats> = HashMap::new();

    for p in pts {
        if p.z < z_min_keep || p.z > z_max_keep {
            continue;
        }

        let ix = (p.x / voxel_xy).floor() as i32;
        let iy = (p.y / voxel_xy).floor() as i32;

        grid.entry((ix, iy))
            .and_modify(|cell| cell.add(p.z))
            .or_insert_with(|| CellStats::new(p.z));
    }

    grid
}