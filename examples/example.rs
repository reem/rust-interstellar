extern crate interstellar;

use interstellar::{State, Simulation};
use interstellar::field::Gravity;
use interstellar::emitter::StreamEmitter;
use interstellar::vector::Vector;

fn main() {
  let mut simulation = Simulation.fromStart(State {
    emitters: vec![StreamEmitter { pos: Vector { 100, 400 }, }],
    fields: vec![Gravity { pos: Vector { 500, 400 }, -200 }],
    particles: vec![],
  });
  simulation.run();
}
