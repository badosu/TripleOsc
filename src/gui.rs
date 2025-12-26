use nih_plug::{editor::Editor, params::Param, prelude::AsyncExecutor};
use nih_plug_egui::{
  create_egui_editor, egui::Vec2, resizable_window::ResizableWindow, widgets,
};

use crate::TripleOsc;

const MIN_SIZE_X: f32 = 800.0;
const MIN_SIZE_Y: f32 = 600.0;

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
  egui_state: std::sync::Arc<nih_plug_egui::EguiState>,
) -> impl Fn(
  &nih_plug_egui::egui::Context,
  &nih_plug::prelude::ParamSetter<'_>,
  &mut (),
) {
  move |egui_ctx, setter, _state| {
    ResizableWindow::new("res-wind")
      .min_size(Vec2::new(MIN_SIZE_X, MIN_SIZE_Y))
      .show(egui_ctx, egui_state.as_ref(), |ui| {
        ui.horizontal(|ui| {
          ui.vertical(|ui| {
            ui.heading("Global");

            let global_params =
              [&params.gain, &params.amp_attack_ms, &params.amp_release_ms];
            for param in global_params {
              ui.label(param.name());
              ui.add(widgets::ParamSlider::for_param(param, setter));
            }
          });

          ui.vertical(|ui| {
            ui.heading("Osc 1");
            ui.label(format!("Waveform: {}", params.osc1_wave));

            let osc_params = [&params.osc1_gain, &params.osc1_detune];
            for param in osc_params {
              ui.label(param.name());
              ui.add(widgets::ParamSlider::for_param(param, setter));
            }
          });

          ui.vertical(|ui| {
            ui.heading("Osc 2");
            ui.label(format!("Waveform: {}", params.osc2_wave));

            let osc_params = [&params.osc2_gain, &params.osc2_detune];
            for param in osc_params {
              ui.label(param.name());
              ui.add(widgets::ParamSlider::for_param(param, setter));
            }
          });

          ui.vertical(|ui| {
            ui.heading("Osc 3");
            ui.label(format!("Waveform: {}", params.osc3_wave));

            let osc_params = [&params.osc3_gain, &params.osc3_detune];
            for param in osc_params {
              ui.label(param.name());
              ui.add(widgets::ParamSlider::for_param(param, setter));
            }
          });
        });
      });
  }
}
