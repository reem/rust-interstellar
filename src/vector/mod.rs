#[deriving(Show, Clone, PartialEq, PartialOrd)]
pub struct Vector {
  pub x: f64,
  pub y: f64
}

impl Add<Vector, Vector> for Vector {
  fn add(&self, other: &Vector) -> Vector {
    return Vector {
      x: self.x + other.x,
      y: self.y + other.y
    }
  }
}
