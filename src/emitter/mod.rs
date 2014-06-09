use super::vector::Vector;
use super::particle::{Particle, PlainParticle};
use super::draw::{Drawable, Color};

use std::rand::random;

pub trait Emitter<P: Particle> {
  fn emit(&mut self) -> Vec<P>;
}

