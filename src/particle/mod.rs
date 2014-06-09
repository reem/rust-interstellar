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
