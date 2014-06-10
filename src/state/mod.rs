use super::field::Field;
use super::emitter::Emitter;
use super::particle::Particle;
use super::vector::Vector;

pub struct State <F, P, E> {
    pub fields: Vec<F>,
    pub emitters: Vec<E>,
    pub particles: Vec<P>,
    pub particle_limit: uint,
    pub max_pos: Vector,
    pub min_pos: Vector
}

impl<F: Field, P: Particle, E: Emitter<P>> State<F, P, E> {
    pub fn step(&mut self) {
        if self.particles.len() < self.particle_limit {
            for emitter in self.emitters.mut_iter() {
                self.particles.extend(emitter.emit().move_iter());
            }
        }

        for particle in self.particles.mut_iter() {
            particle.move();
        }

        for particle in self.particles.mut_iter() {
            *particle.acc_mut() = Vector { x: 0f64, y: 0f64 };
            for field in self.fields.iter() {
                field.affect(particle);
            }
        }

        let min = self.min_pos;
        let max = self.max_pos;
        self.particles.retain(|ref p| { p.pos() < max && p.pos() > min });
    }
}
