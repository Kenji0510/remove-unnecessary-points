use std::collections::HashMap;
use anyhow::Result;

use plotters::{chart::ChartBuilder, prelude::{BitMapBackend, IntoDrawingArea, Rectangle}, style::{BLACK, Color, IntoFont, RGBColor, WHITE}};

use crate::convert_2d_xy::CellStats;


pub fn plot_xy_grid_heatmap<F>(
    grid: &HashMap<(i32, i32), CellStats>,
    voxel_xy: f32,
    out_path: &str,
    title: &str,
    value_fn: F,
) -> Result<()>
where
    F: Fn(&CellStats) -> f64,
{
    if grid.is_empty() {
        return Err(anyhow::anyhow!("grid is empty"));
    }

    // 範囲と値域を計算
    let mut min_ix = i32::MAX;
    let mut max_ix = i32::MIN;
    let mut min_iy = i32::MAX;
    let mut max_iy = i32::MIN;

    let mut vmin = f64::INFINITY;
    let mut vmax = f64::NEG_INFINITY;

    for (&(ix, iy), cell) in grid.iter() {
        min_ix = min_ix.min(ix);
        max_ix = max_ix.max(ix);
        min_iy = min_iy.min(iy);
        max_iy = max_iy.max(iy);

        let v = value_fn(cell);
        vmin = vmin.min(v);
        vmax = vmax.max(v);
    }

    // 値が全部同じの場合に備える
    if (vmax - vmin).abs() < 1e-12 {
        vmax = vmin + 1.0;
    }

    // 画像サイズ（セル数に比例させる。大きすぎる場合は適当に縮小してもOK）
    let nx = (max_ix - min_ix + 1) as u32;
    let ny = (max_iy - min_iy + 1) as u32;

    // 1セルあたりのピクセル（見やすさ優先で調整）
    let px_per_cell: u32 = 6;
    let w = (nx * px_per_cell).max(600);
    let h = (ny * px_per_cell).max(600);

    let root = BitMapBackend::new(out_path, (w, h)).into_drawing_area();
    root.fill(&BLACK)?;

    // 軸は「メートル」にする（セル index -> meter）
    let x_min = min_ix as f64 * voxel_xy as f64;
    let x_max = (max_ix as f64 + 1.0) * voxel_xy as f64;
    let y_min = min_iy as f64 * voxel_xy as f64;
    let y_max = (max_iy as f64 + 1.0) * voxel_xy as f64;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 24).into_font().color(&WHITE))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart
        .configure_mesh()
        .x_desc("X [m]")
        .y_desc("Y [m]")
        .axis_style(&WHITE)
        .label_style(("sans-serif", 14).into_font().color(&WHITE))
        .light_line_style(&RGBColor(60, 60, 60))
        .draw()?;

    // 値 -> 色（簡易カラーマップ）
    // ここでは青(低)→赤(高)にしています。
    let color_of = |v: f64| -> RGBColor {
        let t = ((v - vmin) / (vmax - vmin)).clamp(0.0, 1.0);
        // 青 -> 赤
        let r = (255.0 * t) as u8;
        let g = (255.0 * (1.0 - (2.0 * (t - 0.5).abs())).max(0.0)) as u8; // 中間を少し明るく
        let b = (255.0 * (1.0 - t)) as u8;
        RGBColor(r, g, b)
    };

    // 各セルを塗る
    chart.draw_series(grid.iter().map(|(&(ix, iy), cell)| {
        let x0 = ix as f64 * voxel_xy as f64;
        let y0 = iy as f64 * voxel_xy as f64;
        let x1 = (ix as f64 + 1.0) * voxel_xy as f64;
        let y1 = (iy as f64 + 1.0) * voxel_xy as f64;

        let v = value_fn(cell);
        let c = color_of(v);

        Rectangle::new([(x0, y0), (x1, y1)], c.filled())
    }))?;

    root.present()?;
    Ok(())
}