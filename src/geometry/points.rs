// implements the geometry point structure

pub struct Point {
  x: f64,
  y: f64,
  z: f64,
}

impl Point {
  pub fn magnitude(self) -> f64 {
    // need to get sqrt() into scope for this to work
    // return sqrt( self.x * self.x + self.y * self.y + self.z * self.z);
    return 1.0;
  }
}