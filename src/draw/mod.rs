use graphics::*; // Shadows Field
use piston::*;

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
            let (drawX, drawY) = drawpos.as_tuple();
            c.ellipse(
                drawX,
                drawY,
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

    fn run<W: GameWindow>(
        &mut self,
        game_window: &mut W,
        asset_store: &mut AssetStore
    ) {
        let mut game_iter = GameIterator::new(
            game_window,
            &GameIteratorSettings {
                updates_per_second: 30,
                max_frames_per_second: 30
            });

        self.load(asset_store);

        loop {
            match game_iter.next() {
                None => { break }
                Some(e) => match e {
                    Render(args) => self.render(
                        &Context::abs(
                            args.width as f64,
                            args.height as f64
                        ),
                        args
                    ),
                    Update(args) => self.update(args),
                    KeyPress(args) => self.key_press(args),
                    KeyRelease(args) => self.key_release(args),
                    MousePress(args) => self.mouse_press(args),
                    MouseRelease(args) => self.mouse_release(args),
                    MouseMove(args) => self.mouse_move(args),
                    MouseRelativeMove(args) => self.mouse_relative_move(args),

                }
            }
        }
    }
}
