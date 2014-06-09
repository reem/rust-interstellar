// extern crate graphics;
// extern crate piston;

use super::vector::Vector;
// use graphics::*;
// use piston::{Game, Gl, RenderArgs};

pub struct Color(pub uint, pub uint, pub uint, pub uint);

pub trait Drawable {
  fn drawpos(&self) -> Vector;
  fn color(&self) -> Color;
  fn size(&self) -> uint;
}
