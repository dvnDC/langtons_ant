#[derive(Clone, Copy)]
pub enum Rotation {
    N, // No change
    R1, // 60° clockwise
    R2, // 120° clockwise
    U, // 180°
    L2, // 120° counter-clockwise
    L1, // 60° counter-clockwise
}