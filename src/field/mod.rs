use super::vector::Vector;
use super::particle::Particle;
use super::draw::{Drawable, Color};

pub trait Field {
    fn affect<P: Particle>(&self, &mut P);
}

pub struct Gravity {
    pub pos:  Vector,
    pub mass: f64
}

impl Field for Gravity {
    fn affect<P: Particle>(&self, particle: &mut P) -> () {
        let (xDiff, yDiff) = (self.pos - particle.pos()).as_tuple();
        let force =
            self.mass / (xDiff.powf(2f64) + yDiff.powf(2f64)).powf(1.5);

        particle.acc_mut().mut_add(&Vector::new(xDiff, yDiff).scale(force));
    }
}

impl Drawable for Gravity {
    #[inline]
    fn drawpos(&self) -> Vector { self.pos }
    fn color(&self) -> Color {
        if self.mass < 0f64 {
            Color(255, 0, 0, 0)
        } else {
            Color(0, 255, 0, 0)
        }
    }
    #[inline]
    fn size(&self) -> uint { 4 }
}
