use nih_plug::{
  formatters, params::FloatParam, prelude::FloatRange, prelude::SmoothingStyle,
  util,
};

use crate::ModulationId;

pub fn detune_multiplier(cents: f32) -> f32 {
  2.0_f32.powf(cents / 1200.0)
}

pub fn new_gain_param(name: &str, poly_mod_id: ModulationId) -> FloatParam {
  FloatParam::new(
    name,
    util::db_to_gain(-12.0),
    // Because we're representing gain as decibels the range is already logarithmic
    FloatRange::Linear {
      min: util::db_to_gain(-36.0),
      max: util::db_to_gain(0.0),
    },
  )
  // This enables polyphonic mdoulation for this parameter by representing all related
  // events with this ID. After enabling this, the plugin **must** start sending
  // `VoiceTerminated` events to the host whenever a voice has ended.
  .with_poly_modulation_id(poly_mod_id.into())
  .with_smoother(SmoothingStyle::Logarithmic(5.0))
  .with_unit(" dB")
  .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
  .with_string_to_value(formatters::s2v_f32_gain_to_db())
}

pub fn new_detune_param(name: &str, poly_mod_id: ModulationId) -> FloatParam {
  FloatParam::new(
    name,
    0.0,
    // Because we're representing gain as decibels the range is already logarithmic
    FloatRange::Linear {
      min: -50.0,
      max: 50.0,
    },
  )
  // This enables polyphonic modulation for this parameter by representing all related
  // events with this ID. After enabling this, the plugin **must** start sending
  // `VoiceTerminated` events to the host whenever a voice has ended.
  .with_poly_modulation_id(poly_mod_id.into())
  .with_smoother(SmoothingStyle::Logarithmic(5.0))
  .with_step_size(1.0)
  .with_unit(" cents")
}

pub fn new_envelope_param(name: &str, default: f32) -> FloatParam {
  FloatParam::new(
    name,
    default,
    FloatRange::Skewed {
      min: 0.0,
      max: 2000.0,
      factor: FloatRange::skew_factor(-1.0),
    },
  )
  // These parameters are global (and they cannot be changed once the voice has started).
  // They also don't need any smoothing themselves because they affect smoothing
  // coefficients.
  .with_step_size(0.1)
  .with_unit(" ms")
}
