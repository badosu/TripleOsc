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
    (((phase + 0.75).floor() - phase) * 4.0 - 1.0).abs() - 1.0
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
      Wave::Pulse => Self::pulse(phase) + pulse_blamp(phase, phase_delta),
      Wave::Triangle => {
        Self::triangle(phase) + triangle_blamp(phase, phase_delta)
      }
    }
  }
}

#[inline]
fn pulse_blamp(phase: f32, phase_delta: f32) -> f32 {
  polyblep(phase, phase_delta) - polyblep((phase + 0.5) % 1.0, phase_delta)
}

/// PolyBLEP by Tale
/// (slightly modified, transpiled to rust)
///
/// See:
/// http://www.kvraudio.com/forum/viewtopic.php?t=375517
/// https://www.martin-finke.de/articles/audio-plugins-018-polyblep-oscillator/
#[inline]
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

const FOUR_THIRDS: f32 = 4.0 / 3.0;

/// blamp By Martin Finke (transpiled to rust)
///
/// PolyBLEP for a triangular waveform of the form:
/// (((phase + 0.75).floor() - phase) * 4.0 - 1.0).abs() - 1.0
///
#[inline]
fn triangle_blamp(phase: f32, phase_delta: f32) -> f32 {
  // AFTER BOTTOM
  if phase >= 0.75 && phase <= 0.75 + phase_delta {
    let p = (phase - 0.75) / phase_delta - 1.0;
    return -(p * p * p * FOUR_THIRDS * phase_delta);
  }

  // BEFORE BOTTOM
  if phase >= 0.75 - phase_delta && phase <= 0.75 {
    let p = (phase - 0.75) / phase_delta + 1.0;
    return p * p * p * FOUR_THIRDS * phase_delta;
  }

  // AFTER PEAK
  if phase >= 0.25 && phase <= 0.25 + phase_delta {
    let p = (phase - 0.25) / phase_delta - 1.0;
    return p * p * p * FOUR_THIRDS * phase_delta;
  }

  // BEFORE PEAK
  if phase >= 0.25 - phase_delta && phase <= 0.25 {
    let p = (phase - 0.25) / phase_delta + 1.0;
    return -(p * p * p * FOUR_THIRDS * phase_delta);
  }

  0.0
}
