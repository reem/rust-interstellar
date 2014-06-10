#[deriving(Show, Clone, PartialEq, PartialOrd)]
pub struct Vector {
    pub x: f64,
    pub y: f64
}

impl Vector {
    pub fn mut_add(&mut self, other: &Vector) {
        self.x += other.x;
        self.y += other.y;
    }

    #[inline]
    pub fn fast_add(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }

    pub fn from_angle(magnitude: f64, angle: f64) -> Vector {
        return Vector {
            x: magnitude * angle.cos(),
            y: magnitude * angle.sin()
        }
    }
}

impl Add<Vector, Vector> for Vector {
    fn add(&self, other: &Vector) -> Vector {
        return Vector {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}
