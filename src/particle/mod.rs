use super::vector::Vector;
use super::draw::{Drawable, Color};

pub trait Particle {
    fn pos_mut<'a>(&'a mut self) -> &'a mut Vector;
    fn vel_mut<'a>(&'a mut self) -> &'a mut Vector;
    fn acc_mut<'a>(&'a mut self) -> &'a mut Vector;
    fn pos(& self) -> Vector;
    fn vel(& self) -> Vector;
    fn acc(& self) -> Vector;
    fn from_vectors(p: Vector, v: Vector, a: Vector) -> Self;
    fn move(&mut self) {
        let acc = self.acc();
        self.vel_mut().mut_add(&acc);
        let vel = self.vel();
        self.pos_mut().mut_add(&vel);
    }
}

pub struct PlainParticle {
    pub pos: Vector,
    pub vel: Vector,
    pub acc: Vector
}

impl Particle for PlainParticle {
    #[inline]
    fn pos_mut<'a>(&'a mut self) -> &'a mut Vector { &mut self.pos }
    #[inline]
    fn vel_mut<'a>(&'a mut self) -> &'a mut Vector { &mut self.vel }
    #[inline]
    fn acc_mut<'a>(&'a mut self) -> &'a mut Vector { &mut self.acc }
    #[inline]
    fn pos(&self) -> Vector { self.pos }
    #[inline]
    fn vel(&self) -> Vector { self.vel }
    #[inline]
    fn acc(&self) -> Vector { self.acc }
    #[inline]
    fn from_vectors(p: Vector, v: Vector, a: Vector) -> PlainParticle {
        PlainParticle { pos: p, vel: v, acc: a }
    }
}

impl Drawable for PlainParticle {
    #[inline]
    fn drawpos(&self) -> Vector { self.pos }
    #[inline]
    fn color(&self) -> Color { Color(255, 255, 255, 0) } 
    #[inline]
    fn size(&self) -> uint { 1 }
}
