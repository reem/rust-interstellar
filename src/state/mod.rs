use super::field::Field;
use super::emitter::Emitter;
use super::particle::Particle;
use super::vector::Vector;

pub struct State <F, P, E> {
  fields: Vec<F>,
  emitters: Vec<E>,
  particles: Vec<P>,
  particle_limit: uint,
  max_pos: Vector,
  min_pos: Vector
}
