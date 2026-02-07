use anyhow::{Context, Result};
use pcd_rs::{PcdDeserialize, PcdSerialize, Reader};

#[derive(Debug, Clone, PcdDeserialize, PcdSerialize)]
pub struct PointXYZ {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, PcdDeserialize, PcdSerialize)]
pub struct PointXYZCovs {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub cov11: f32,
    pub cov12: f32,
    pub cov13: f32,
    pub cov21: f32,
    pub cov22: f32,
    pub cov23: f32,
    pub cov31: f32,
    pub cov32: f32,
    pub cov33: f32,
}

#[derive(Debug, Clone, PcdDeserialize, PcdSerialize)]
pub struct PointXYZWithShapeFeat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub linearity: f64,
    pub planarity: f64,
    pub scattering: f64,
    pub l1: f64,
    pub l2: f64,
    pub l3: f64,
}

#[derive(Debug, Clone, PcdDeserialize, PcdSerialize)]
pub struct PointXYZNormal {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub normal_x: f32,
    pub normal_y: f32,
    pub normal_z: f32,
}

#[derive(Debug, Clone, PcdDeserialize, PcdSerialize)]
pub struct PointXYZRGB {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rgb: f32,
}

pub fn load_pcd_xyz(file_path: &str) -> Result<Vec<PointXYZ>> {
    let reader = match Reader::open(file_path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to open PCD file: {}", e);
            return Err(anyhow::anyhow!("Failed to open PCD file: {}", e));
        }
    };

    let points: Vec<PointXYZ> = match reader.collect() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to read PCD data: {}", e);
            return Err(anyhow::anyhow!("Failed to read PCD data: {}", e));
        }
    };

    Ok(points)
}

pub fn load_pcd_xyzrgb(file_path: &str) -> Result<Vec<PointXYZ>> {
    let reader = match Reader::open(file_path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to open PCD file: {}", e);
            return Err(anyhow::anyhow!("Failed to open PCD file: {}", e));
        }
    };

    let points_rgb: Vec<PointXYZRGB> = match reader.collect() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to read PCD data: {}", e);
            return Err(anyhow::anyhow!("Failed to read PCD data: {}", e));
        }
    };

    let points = points_rgb
        .into_iter()
        .map(|p| PointXYZ {
            x: p.x,
            y: p.y,
            z: p.z,
        })
        .collect();

    Ok(points)
}


pub fn save_pcd(points: &[PointXYZNormal], file_path: &str) -> Result<()> {
    let mut writer = pcd_rs::WriterInit {
        width: 1,
        height: points.len() as u64,
        viewpoint: Default::default(),
        data_kind: pcd_rs::DataKind::Ascii,
        schema: None,
    }
    .create(file_path)?;

    for point in points {
        writer.push(point)?;
    }
    writer.finish()?;

    Ok(())
}

pub fn save_pcd_with_covs(points: &[PointXYZCovs], file_path: &str) -> Result<()> {
    let mut writer = pcd_rs::WriterInit {
        width: 1,
        height: points.len() as u64,
        viewpoint: Default::default(),
        data_kind: pcd_rs::DataKind::Ascii,
        schema: None,
    }
    .create(file_path)?;

    for point in points {
        writer.push(point)?;
    }
    writer.finish()?;

    Ok(())
}

pub fn save_pcd_with_shape_feats(points: &[PointXYZWithShapeFeat], file_path: &str) -> Result<()> {
    let mut writer = pcd_rs::WriterInit {
        width: 1,
        height: points.len() as u64,
        viewpoint: Default::default(),
        data_kind: pcd_rs::DataKind::Ascii,
        schema: None,
    }
    .create(file_path)?;

    for point in points {
        writer.push(point)?;
    }
    writer.finish()?;

    Ok(())
}
