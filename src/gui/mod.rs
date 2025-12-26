use crate::{
  TripleOsc,
  gui::custom_widgets::{
    combo_box::ParamComboBox,
    knob::{ArcKnob, KnobLayout},
  },
  wave::Wave,
};
use custom_widgets::knob;
use nih_plug::{
  editor::Editor,
  params::{FloatParam, Param},
  prelude::{AsyncExecutor, Enum, ParamSetter},
};
use nih_plug_egui::{
  EguiState, create_egui_editor,
  egui::{self, Color32},
};

mod custom_widgets;

pub const DARK_GREY_UI_COLOR: Color32 = Color32::from_rgb(42, 42, 42);
pub const TEAL_GREEN: Color32 = Color32::from_rgb(61, 178, 166);
pub const YELLOW_MUSTARD: Color32 = Color32::from_rgb(172, 131, 25);
// pub const MEDIUM_GREY_UI_COLOR: Color32 = Color32::from_rgb(52, 52, 52);
// pub const LIGHTER_GREY_UI_COLOR: Color32 = Color32::from_rgb(69, 69, 69);
// pub const A_BACKGROUND_COLOR_TOP: Color32 = Color32::from_rgb(38, 38, 38);
// pub const DARKEST_BOTTOM_UI_COLOR: Color32 = Color32::from_rgb(27, 27, 27);
// pub const DARKER_GREY_UI_COLOR: Color32 = Color32::from_rgb(34, 34, 34);
// pub const FONT_COLOR: Color32 = Color32::from_rgb(248, 248, 248);

const TEXT_SIZE: f32 = 11.0;
const KNOB_SIZE: f32 = 28.0;

const WIDTH: u32 = 920;
const HEIGHT: u32 = 656;

pub(crate) fn editor_state() -> std::sync::Arc<EguiState> {
  EguiState::from_size(WIDTH, HEIGHT)
}

pub(crate) fn make_gui(
  instance: &TripleOsc,
  _async_executor: AsyncExecutor<TripleOsc>,
) -> Option<Box<dyn Editor>> {
  let params = instance.params.clone();
  let egui_state = params.editor_state.clone();

  create_egui_editor(
    instance.params.editor_state.clone(),
    (),
    gui_build,
    gui_update(params, egui_state),
  )
}

fn gui_build(_egui_ctx: &nih_plug_egui::egui::Context, _state: &mut ()) {}

fn gui_update(
  params: std::sync::Arc<crate::TripleOscParams>,
  _egui_state: std::sync::Arc<nih_plug_egui::EguiState>,
) -> impl Fn(&nih_plug_egui::egui::Context, &ParamSetter<'_>, &mut ()) {
  move |egui_ctx, setter, _state| {
    egui::CentralPanel::default().show(egui_ctx, |ui| {
      ui.vertical(|ui| {
        ui.heading("Oscillators");

        let wave_options: Vec<String> =
          Wave::variants().iter().map(|s| s.to_string()).collect();

        ui.horizontal(|ui| {
          ui.colored_label(TEAL_GREEN, "Type");
          let cb1 = ParamComboBox::for_param(
            &params.osc1_wave,
            setter,
            &wave_options,
            "cb1".to_string(),
          );
          ui.add(cb1);

          ui.add(
            add_knob_name(&params.osc1_gain, setter, "Gain".to_string())
              .set_hover_text("Oscillator 1 Gain (in dB)".to_string())
              .set_line_color(TEAL_GREEN),
          );

          ui.add(
            add_knob_name(&params.osc1_detune, setter, "Detune".to_string())
              .set_hover_text("Oscillator 1 fine detune (in cents)".to_string())
              .set_line_color(TEAL_GREEN),
          );
        });

        ui.horizontal(|ui| {
          ui.colored_label(TEAL_GREEN, "Type");
          let cb1 = ParamComboBox::for_param(
            &params.osc2_wave,
            setter,
            &wave_options,
            "cb1".to_string(),
          );
          ui.add(cb1);

          ui.add(
            add_knob_name(&params.osc2_gain, setter, "Gain".to_string())
              .set_hover_text("Oscillator 2 Gain (in dB)".to_string())
              .set_line_color(TEAL_GREEN),
          );

          ui.add(
            add_knob_name(&params.osc2_detune, setter, "Detune".to_string())
              .set_hover_text("Oscillator 2 fine detune (in cents)".to_string())
              .set_line_color(TEAL_GREEN),
          );
        });

        ui.horizontal(|ui| {
          ui.colored_label(TEAL_GREEN, "Type");
          let cb1 = ParamComboBox::for_param(
            &params.osc3_wave,
            setter,
            &wave_options,
            "cb1".to_string(),
          );
          ui.add(cb1);

          ui.add(
            add_knob_name(&params.osc3_gain, setter, "Gain".to_string())
              .set_hover_text("Oscillator 3 Gain (in dB)".to_string())
              .set_line_color(TEAL_GREEN),
          );

          ui.add(
            add_knob_name(&params.osc3_detune, setter, "Detune".to_string())
              .set_hover_text("Oscillator 3 fine detune (in cents)".to_string())
              .set_line_color(TEAL_GREEN),
          );
        });
      });

      ui.vertical(|ui| {
        ui.add(
          add_knob(&params.gain, setter)
            .set_hover_text("Master gain level".to_string())
            .set_line_color(YELLOW_MUSTARD),
        );

        ui.add(
          add_knob(&params.amp_attack_ms, setter)
            .set_hover_text("Attack envelope duration (in ms)".to_string())
            .set_line_color(YELLOW_MUSTARD),
        );

        ui.add(
          add_knob(&params.amp_release_ms, setter)
            .set_hover_text("Release envelope duration (in ms)".to_string())
            .set_line_color(YELLOW_MUSTARD),
        );
      });
    });
  }
}

fn add_knob<'a>(
  param: &'a FloatParam,
  setter: &'a ParamSetter<'a>,
) -> ArcKnob<'a, FloatParam> {
  add_knob_name(param, setter, param.name().to_string())
}

fn add_knob_name<'a>(
  param: &'a FloatParam,
  setter: &'a ParamSetter<'a>,
  name: String,
) -> ArcKnob<'a, FloatParam> {
  ArcKnob::for_param(param, setter, KNOB_SIZE, KnobLayout::Vertical, name)
    .preset_style(knob::KnobStyle::Preset1)
    .set_fill_color(DARK_GREY_UI_COLOR)
    .set_text_size(TEXT_SIZE)
    .use_outline(true)
}
