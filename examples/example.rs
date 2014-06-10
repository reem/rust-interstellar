extern crate interstellar;
extern crate piston;
extern crate graphics;

use piston::{Game, AssetStore, GameWindow, GameWindowSDL2, GameWindowSettings};

use interstellar::state::State;
use interstellar::field::Gravity;
use interstellar::emitter::StreamEmitter;
use interstellar::particle::PlainParticle;
use interstellar::vector::Vector;

fn main() {
  let mut window: GameWindowSDL2 = GameWindow::new(
    GameWindowSettings {
      title: "Interstellar".to_string(),
      size: [1400, 900],
      fullscreen: true,
      exit_on_esc: true,
      background_color: [0.0, 0.0, 0.0, 0.0],
    }
  ); 
  let particles: Vec<PlainParticle> = vec!();
  let mut simulation = State {
    emitters: vec!(
      StreamEmitter { 
        pos: Vector { x: 100.0, y: 400.0 },
        direction: 0.0,
        ppf: 10,
        startVel: 5.0,
        spread: 0.2 }
    ),
    fields: vec!(Gravity { pos: Vector { x: 500.0, y: 400.0 }, mass: -500.0 }),
    particles: particles,
    particle_limit: 100000,
    max_pos: Vector { x: 1400.0, y: 900.0 },
    min_pos: Vector { x: 0.0, y: 0.0 }
  };
  simulation.run(&mut window, &mut AssetStore::empty());
}
