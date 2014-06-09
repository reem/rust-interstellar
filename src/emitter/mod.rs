use super::vector::Vector;
use super::particle::{Particle, PlainParticle};
use super::draw::{Drawable, Color};

use std::rand::random;

pub trait Emitter<P: Particle> {
  fn emit(&mut self) -> Vec<P>;
}

pub struct StreamEmitter {
  pos: Vector,
  direction: f64,
  ppf: uint,
  startVel: f64,
  spread: f64
}

impl Emitter<PlainParticle> for StreamEmitter {
  fn emit(&mut self) -> Vec<PlainParticle> {
    range(0, self.ppf).map(|_| {
      Particle::from_vectors(
        self.pos,
        Vector::from_angle(
          self.startVel * random::<f64>(), 
          self.spread - (random::<f64>() * self.spread * 2f64) + self.direction
        ),
        Vector { x: 0f64, y: 0f64}
      )
    }).collect()
  }
}

