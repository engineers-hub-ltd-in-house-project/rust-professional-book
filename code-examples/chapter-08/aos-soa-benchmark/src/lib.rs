// AoS (Array of Structs) 表現
#[derive(Clone, Copy)]
pub struct PointAoS {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// SoA (Struct of Arrays) 表現
pub struct PointsSoA {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Vec<f64>,
}

impl PointsSoA {
    pub fn new(size: usize) -> Self {
        Self {
            x: vec![0.0; size],
            y: vec![0.0; size],
            z: vec![0.0; size],
        }
    }
}

// AoSのx座標を合計する関数
pub fn sum_x_aos(points: &[PointAoS]) -> f64 {
    points.iter().map(|p| p.x).sum()
}

// SoAのx座標を合計する関数
pub fn sum_x_soa(points: &PointsSoA) -> f64 {
    points.x.iter().sum()
}
