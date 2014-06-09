use super::vector::Vector;
use super::draw::{Drawable, Color};

pub trait Particle {
  fn pos<'a>(&'a mut self) -> &'a mut Vector;
  fn vel<'a>(&'a mut self) -> &'a mut Vector;
  fn acc<'a>(&'a mut self) -> &'a mut Vector;
  fn fromVectors(p: Vector, v: Vector, a: Vector) -> Self;
  fn move(&mut self) {
    let acc = *self.acc();
    self.vel().mut_add(&acc);
    let vel = *self.vel();
    self.pos().mut_add(&vel);
  }
}

pub struct PlainParticle {
  pub pos: Vector,
  pub vel: Vector,
  pub acc: Vector
}

impl Particle for PlainParticle {
  #[inline]
  fn pos<'a>(&'a mut self) -> &'a mut Vector { return &mut self.pos; }
  #[inline]
  fn vel<'a>(&'a mut self) -> &'a mut Vector { return &mut self.vel; }
  #[inline]
  fn acc<'a>(&'a mut self) -> &'a mut Vector { return &mut self.acc; }
  #[inline]
  fn from_vectors(p: Vector, v: Vector, a: Vector) -> PlainParticle{
    return PlainParticle { pos: p, vel: v, acc: a }
  }
}
