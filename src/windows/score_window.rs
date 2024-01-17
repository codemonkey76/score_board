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
    first_run: bool,
    window_size: PhysicalSize<u32>
}

impl ScoreWindow {
    pub fn request(_label: String) -> NewWindowRequest {
        let window_size = PhysicalSize { width: 400, height: 200 };
        NewWindowRequest {
            window_state: super::MyWindows::Score(ScoreWindow {
                is_fullscreen: false,
                first_run: true,
                window_size
            }),
            builder: egui_multiwin::winit::window::WindowBuilder::new()
                .with_resizable(false)
                .with_title("Scoreboard")
                .with_inner_size(window_size),

            options: egui_multiwin::tracked_window::TrackedWindowOptions {
                vsync: false,
                shader: None,
            },
            id: egui_multiwin::multi_window::new_id(),
        }
    }

    fn draw_grid(&self, c:&mut AppCommon, ui: &mut Ui) {
        ui.painter().rect_stroke(c.display_grid.competitor_one.name_grid.name, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.display_grid.competitor_one.name_grid.flag, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.display_grid.competitor_one.name_grid.team, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.display_grid.competitor_one.name_grid.country, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.display_grid.competitor_one.stalling_grid.stalling, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.display_grid.competitor_one.points_grid.advantage, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.display_grid.competitor_one.points_grid.penalty, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.display_grid.competitor_one.points_grid.points, Rounding::default(), Stroke::new(2.0, Color32::GREEN));

        ui.painter().rect_stroke(c.display_grid.competitor_two.name_grid.name, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.display_grid.competitor_two.name_grid.flag, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.display_grid.competitor_two.name_grid.team, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.display_grid.competitor_two.name_grid.country, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.display_grid.competitor_two.stalling_grid.stalling, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.display_grid.competitor_two.points_grid.advantage, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.display_grid.competitor_two.points_grid.penalty, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.display_grid.competitor_two.points_grid.points, Rounding::default(), Stroke::new(2.0, Color32::GREEN));

        ui.painter().rect_stroke(c.display_grid.match_info.time, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.display_grid.match_info.match_type, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.display_grid.match_info.match_info, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
        ui.painter().rect_stroke(c.display_grid.match_info.bracket_info, Rounding::default(), Stroke::new(2.0, Color32::GREEN));
    }

    fn on_resize(&mut self, c:&mut AppCommon, rect: Rect) {
        self.first_run = false;
        c.display_grid = ScoreGrid::calc(rect, &c.grid_config);
    }
}

impl HandleInput for ScoreWindow {
    fn set_fullscreen(&mut self, fullscreen: bool) { self.is_fullscreen = fullscreen; }

    fn is_fullscreen(&self) -> bool {
        self.is_fullscreen
    }
}

impl TrackedWindow for ScoreWindow {
    fn can_quit(&mut self, c: &mut AppCommon) -> bool {
        c.show_score_window = false;

        true
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

            if self.first_run {
                self.on_resize(c, ui.available_rect_before_wrap());
            }

            if self.window_size != window.inner_size() {
                self.window_size = window.inner_size();
                self.first_run = true;
            }
        });



        RedrawResponse {
            quit: !c.show_score_window,
            new_windows: Vec::new(),
        }
    }
}