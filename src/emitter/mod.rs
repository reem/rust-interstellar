use super::vector::Vector;
use super::particle::{Particle, PlainParticle};
use super::draw::{Drawable, Color};

use std::rand::random;

pub trait Emitter<P: Particle> {
    fn emit(&mut self) -> Vec<P>;
}

pub struct StreamEmitter {
    pub pos: Vector,
    pub direction: f64,
    pub ppf: uint,
    pub startVel: f64,
    pub spread: f64
}

impl Emitter<PlainParticle> for StreamEmitter {
    fn emit(&mut self) -> Vec<PlainParticle> {
        range(0, self.ppf).map(|_| {
            Particle::from_vectors(
                self.pos,
                Vector::from_angle(
                    self.startVel * (random::<f64>() + 0.3), 
                    self.spread - (random::<f64>() * self.spread * 2.0) + self.direction
                ),
                Vector::new(0.0, 0.0)
            )
        }).collect()
    }
}

impl Drawable for StreamEmitter {
    #[inline]
    fn drawpos(&self) -> Vector { self.pos }
    #[inline]
    fn color(&self) -> Color { Color(0, 255, 255, 0) }
    #[inline]
    fn size(&self) -> uint { 4 }
}
