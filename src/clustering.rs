use log::debug;

pub fn extract_human_clusters(
    pts: &[[f32; 3]],
    cluster_ids: &[u32],
    min_points: usize,
) -> Vec<(u32, Vec<[f32; 3]>, usize)> {
    // 1. クラスタIDごとに座標をグループ化
    let mut clusters: std::collections::HashMap<u32, Vec<[f32; 3]>> =
        std::collections::HashMap::new();
    for i in 0..pts.len() {
        let id = cluster_ids[i];
        clusters.entry(id).or_insert_with(Vec::new).push(pts[i]);
    }

    debug!("Found {} total clusters", clusters.len());

    let mut human_cluster_pairs: Vec<(u32, Vec<[f32; 3]>, usize)> = Vec::new();
    let mut max_count = 0;

    // 2. クラスタごとのサイズ判定
    for (_id, cluster_pts) in clusters.into_iter() {
        let count = cluster_pts.len();

        // ノイズ除去: min_points未満の小さな塊は無視 (ボクセルサイズ0.05mなら適度な値)
        if count < min_points {
            continue;
        }

        // Bounding Boxの計算
        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;
        let mut min_z = f32::MAX;
        let mut max_z = f32::MIN;

        for p in &cluster_pts {
            if p[0] < min_x {
                min_x = p[0];
            }
            if p[0] > max_x {
                max_x = p[0];
            }
            if p[1] < min_y {
                min_y = p[1];
            }
            if p[1] > max_y {
                max_y = p[1];
            }
            if p[2] < min_z {
                min_z = p[2];
            }
            if p[2] > max_z {
                max_z = p[2];
            }
        }

        let width_x = max_x - min_x;
        let width_y = max_y - min_y;
        let height_z = max_z - min_z;

        // 人サイズの条件設定
        // 人の高さ(0.8m〜2.0m) かつ 幅・奥行きが広すぎない(1.0m以下)
        let is_human_height = height_z > 0.8 && height_z < 2.0;
        let is_human_width = width_x < 1.5 && width_y < 1.5;

        // 天井の残骸は「平べったい（高さがない）」か「点数が少なすぎる」のでここで弾かれます
        if is_human_height && is_human_width {
            // if count > max_count {
            //     max_count = count;
            //     human_cluster_pairs.push((_id, cluster_pts, count));
            // }
            max_count = count;
            human_cluster_pairs.push((_id, cluster_pts, count));
        }
    }

    debug!("Extracted {} human-like clusters", human_cluster_pairs.len());

    human_cluster_pairs.sort_by_key(|(_, _, count)| std::cmp::Reverse(*count));

    human_cluster_pairs
}
