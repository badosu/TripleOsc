use core::f32;
use nih_plug::prelude::Enum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
#[non_exhaustive]
pub enum Wave {
  #[id = "saw"]
  #[name = "Sawtooth"]
  Saw,
  #[id = "triangle"]
  #[name = "Triangle"]
  Triangle,
  #[id = "sin"]
  #[name = "Sin"]
  Sin,
  #[id = "pulse"]
  #[name = "Pulse"]
  Pulse,
}

impl Wave {
  pub fn sample(self, phase: f32) -> f32 {
    match self {
      Wave::Saw => phase * 2.0 - 1.0,
      Wave::Triangle => 1.0 - 4.0 * (phase - 0.5).abs(),
      Wave::Sin => (phase * f32::consts::TAU).sin(),
      Wave::Pulse => 2.0 * (phase >= 0.5) as i32 as f32 - 1.0,
    }
  }
}
