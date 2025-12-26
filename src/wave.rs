use core::f32;
use nih_plug::prelude::Enum;
use std::f32::consts::TAU;

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
  fn saw(phase: f32) -> f32 {
    phase * 2.0 - 1.0
  }

  fn triangle(phase: f32) -> f32 {
    if phase < 0.25 {
      // 0 -> 1
      phase * 4.0
    } else if phase < 0.75 {
      // 1 -> -1
      2.0 - phase * 4.0
    } else {
      // -1 -> 0
      phase * 4.0 - 4.0
    }
  }

  fn pulse(phase: f32) -> f32 {
    1.0 - 2.0 * (phase >= 0.5) as i32 as f32
  }

  // phase -> 0..1
  // output -> -1..1
  pub fn polyblep_sample(self, phase: f32, phase_delta: f32) -> f32 {
    match self {
      Wave::Sin => (phase * TAU).sin(),
      Wave::Saw => Self::saw(phase) - polyblep(phase, phase_delta),
      Wave::Pulse => {
        Self::pulse(phase) + polyblep(phase, phase_delta)
          - polyblep((phase + 0.5) % 1.0, phase_delta)
      }
      Wave::Triangle => {
        // FIXME: Introduce polyblep
        Self::triangle(phase)
      }
    }
  }
}

/// PolyBLEP by Tale
/// (slightly modified, transpiled to rust)
///
/// See:
/// http://www.kvraudio.com/forum/viewtopic.php?t=375517
/// https://www.martin-finke.de/articles/audio-plugins-018-polyblep-oscillator/
fn polyblep(phase: f32, phase_delta: f32) -> f32 {
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
