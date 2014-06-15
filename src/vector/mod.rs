use std::simd::f64x2;

#[deriving(Show)]
pub struct Vector {
    inner: f64x2
}

impl Vector {
    #[inline]
    pub fn mut_add(&mut self, other: &Vector) {
        self.inner += other.inner;
    }

    #[inline]
    pub fn fast_add(&mut self, dx: f64, dy: f64) {
        self.inner += f64x2(dx, dy);
    }

    #[inline]
    pub fn from_angle(magnitude: f64, angle: f64) -> Vector {
        Vector::new(magnitude * angle.cos(), magnitude * angle.sin())
    }

    #[inline]
    pub fn new(x: f64, y: f64) -> Vector {
        Vector { inner: f64x2(x, y) }
    }

    pub fn as_tuple(&self) -> (f64, f64) {
        let f64x2(x, y) = self.inner;
        (x, y)
    }

    pub fn scale(&mut self, scalar: f64) -> Vector {
        self.inner *= f64x2(scalar, scalar);
        *self
    }
}

impl Add<Vector, Vector> for Vector {
    fn add(&self, other: &Vector) -> Vector {
        return Vector {
            inner: self.inner + other.inner
        }
    }
}

impl Sub<Vector, Vector> for Vector {
    fn sub(&self, other: &Vector) -> Vector {
        return Vector {
            inner: self.inner - other.inner
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        let f64x2(x1, y1) = self.inner;
        let f64x2(x2, y2) = other.inner;
        x1 == x2 && y1 == y2
    }
}

impl PartialOrd for Vector {
    fn lt(&self, other: &Vector) -> bool {
        let f64x2(x1, y1) = self.inner;
        let f64x2(x2, y2) = other.inner;
        x1 < x2 && y1 < y2
    }
}

impl Clone for Vector {
    fn clone(&self) -> Vector { *self }
}
