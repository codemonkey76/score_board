use egui_multiwin::egui::{Color32, Rect, Rounding, Stroke, Ui};
use crate::egui_multiwin_dynamic::{
    multi_window::NewWindowRequest,
    tracked_window::{RedrawResponse, TrackedWindow},
};

use egui_multiwin::egui_glow::EguiGlow;
use egui_multiwin::winit::dpi::PhysicalSize;

use crate::AppCommon;
use crate::ui::score_grid::ScoreGrid;
use crate::windows::HandleInput;

pub struct ScoreWindow {
    is_fullscreen: bool,
    window_size: PhysicalSize<u32>
}

impl ScoreWindow {
    pub fn request(label: String) -> NewWindowRequest {
        let window_size = PhysicalSize { width: 400, height: 200 };
        NewWindowRequest {
            window_state: super::MyWindows::Score(ScoreWindow {
                is_fullscreen: false,
                window_size
            }),
            builder: egui_multiwin::winit::window::WindowBuilder::new()
                .with_resizable(false)
                .with_inner_size(window_size),

            options: egui_multiwin::tracked_window::TrackedWindowOptions {
                vsync: false,
                shader: None,
            },
            id: egui_multiwin::multi_window::new_id(),
        }
    }

    fn draw_grid(&self, c:&mut AppCommon, ui: &mut Ui) {
        ui.painter().rect_stroke(c.score_grid.top, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.score_grid.middle, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.score_grid.bottom, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.score_grid.score1, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.score_grid.score2, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
    }

    fn on_resize(&self, c:&mut AppCommon, rect: Rect) {
        c.score_grid = ScoreGrid::calc(rect, &c.grid_config);
    }
}

impl HandleInput for ScoreWindow {
    fn set_fullscreen(&mut self, fullscreen: bool) {
        self.is_fullscreen = fullscreen;
    }

    fn is_fullscreen(&self) -> bool {
        self.is_fullscreen
    }
}

impl TrackedWindow for ScoreWindow {
    fn can_quit(&mut self, c: &mut AppCommon) -> bool {
        !c.show_score_window
    }

    fn redraw(
        &mut self,
        c: &mut AppCommon,
        egui: &mut EguiGlow,
        window: &egui_multiwin::winit::window::Window,
        _clipboard: &mut egui_multiwin::arboard::Clipboard,
    ) -> RedrawResponse {

        self.handle_input(c, window, egui);

        egui_multiwin::egui::CentralPanel::default().show(&egui.egui_ctx, |ui| {
            self.draw_grid(c, ui);

            if self.window_size != window.inner_size() || c.score_grid.no_values() {
                self.window_size = window.inner_size();
                self.on_resize(c, ui.clip_rect());
            }
        });

        RedrawResponse {
            quit: !c.show_score_window,
            new_windows: Vec::new(),
        }
    }
}