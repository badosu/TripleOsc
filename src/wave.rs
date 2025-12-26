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
  // phase -> 0..1
  // output -> -1..1
  pub fn sample(self, phase: f32, phase_delta: f32) -> f32 {
    match self {
      Wave::Saw => phase * 2.0 - 1.0 - poly_blep(phase, phase_delta),
      Wave::Triangle => 1.0 - 4.0 * (phase - 0.5).abs(),
      Wave::Sin => (phase * f32::consts::TAU).sin(),
      Wave::Pulse => 2.0 * (phase >= 0.5) as i32 as f32 - 1.0,
    }
  }
}

/// PolyBLEP by Tale
/// (slightly modified, transpiled to rust)
///
/// See:
/// http://www.kvraudio.com/forum/viewtopic.php?t=375517
/// https://www.martin-finke.de/articles/audio-plugins-018-polyblep-oscillator/
fn poly_blep(phase: f32, phase_delta: f32) -> f32 {
  // 0 <= phase < 1
  if phase < phase_delta {
    let t = phase / phase_delta;
    t + t - t * t - 1.0
  }
  // -1 < phase < 0
  else if phase > 1.0 - phase_delta {
    let t = (phase - 1.0) / phase_delta;
    t * t + t + t + 1.0
  }
  // 0 otherwise
  else {
    0.0
  }
}
