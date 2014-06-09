use super::vector::Vector;
use super::particle::Particle;
use super::draw::{Drawable, Color};

pub trait Field {
  fn affect<P: Particle>(&self, &mut P) -> ();
}

pub struct Gravity {
  pub pos:  Vector,
  pub mass: f64 
}

impl Field for Gravity {
  fn affect<P: Particle>(&self, particle: &mut P) -> () {
    let xDiff = self.pos.x - particle.pos().x;
    let yDiff = self.pos.x - particle.pos().y;
    let force = self.mass / (xDiff.powf(2f64) + yDiff.powf(2f64)).powf(1.5);

    particle.acc().fastAdd(xDiff * force, yDiff * force);
  }
}
