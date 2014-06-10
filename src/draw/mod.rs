use graphics::*; // Shadows Field
use piston::{Game, Gl, RenderArgs, UpdateArgs};

use super::vector::Vector;
use super::state::State;
use super::particle::Particle;
use super::field::Field;
use super::emitter::Emitter;

pub struct Color(pub uint, pub uint, pub uint, pub uint);

pub trait Drawable {
  fn drawpos(&self) -> Vector;
  fn color(&self) -> Color;
  fn size(&self) -> uint;
}

fn draw<D: Drawable>(c: &Context, gl: &mut Gl, d: &D) {
  let drawpos = d.drawpos();
  match d.color() {
    Color(r, g, b, _) => {
      c.ellipse(
        drawpos.x, 
        drawpos.y, 
        d.size() as f64, 
        d.size() as f64)
      .rgb(r as f32, g as f32, b as f32) 
      .fill(gl)
    }
  }
}

impl<F: Drawable + Field, 
     P: Drawable + Particle, 
     E: Drawable + Emitter<P>> Game for State<F, P, E> {
  fn render(&self, c: &Context, args: RenderArgs) {
    for particle in self.particles.iter() {
      draw(c, args.gl, particle);
    }
    for emitter in self.emitters.iter() {
      draw(c, args.gl, emitter);
    }
    for field in self.fields.iter() {
      draw(c, args.gl, field);
    }
  }
  fn update(&mut self, _args: UpdateArgs) {
    self.step();
  }
}
