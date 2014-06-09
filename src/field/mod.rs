use super::vector::Vector;
use super::particle::Particle;
use super::draw::{Drawable, Color};

pub trait Field {
  fn affect<P: Particle>(&self, &mut P) -> ();
}
