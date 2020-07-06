
pub const TAU: f64 = 6.283185307179586476925286766559005768394338798750211641949;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * (TAU / 360.0)
}
