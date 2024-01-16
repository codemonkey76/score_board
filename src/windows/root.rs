use crate::egui_multiwin_dynamic::{
    multi_window::NewWindowRequest,
    tracked_window::{RedrawResponse, TrackedWindow},
};
use egui_multiwin::egui::{DragValue, menu};
use egui_multiwin::egui_glow::EguiGlow;
use egui_multiwin::winit::dpi::{PhysicalSize};

use crate::AppCommon;
use crate::windows::HandleInput;

use super::score_window::ScoreWindow;

pub struct RootWindow {
    is_fullscreen: bool,
    window_size: PhysicalSize<u32>,
    show_grid_config: bool
}

impl RootWindow {
    pub fn request() -> NewWindowRequest {
        let window_size = PhysicalSize{ width: 800, height: 600};

        NewWindowRequest {

            window_state: super::MyWindows::Root(RootWindow { is_fullscreen: false, window_size, show_grid_config: false }),
            builder: egui_multiwin::winit::window::WindowBuilder::new()
                .with_resizable(true)
                .with_inner_size(window_size)
                .with_title("BJJ Scoreboard"),
            options: egui_multiwin::tracked_window::TrackedWindowOptions {
                vsync: false,
                shader: None,
            },
            id: egui_multiwin::multi_window::new_id(),
        }
    }
    pub fn grid_config_ui(&mut self, c: &mut AppCommon, egui: &mut EguiGlow) {
        egui_multiwin::egui::Window::new("Grid Config")
            .open(&mut c.show_grid_config)
            .vscroll(true)
            .min_width(700.0)
            .show(&egui.egui_ctx, |ui| {
                egui_multiwin::egui::Grid::new("Grid Settings")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Bottom");
                        ui.add(
                            DragValue::new(&mut c.grid_config.bottom)
                                .speed(0.01)
                                .clamp_range(0.1..=0.90),
                        );
                        ui.end_row();

                        ui.label("Name");
                        ui.add(
                            DragValue::new(&mut c.grid_config.name)
                                .speed(0.01)
                                .clamp_range(0.1..=0.90),
                        );
                        ui.end_row();

                        ui.label("Stalling");
                        ui.add(
                            DragValue::new(&mut c.grid_config.stalling)
                                .speed(0.01)
                                .clamp_range(0.1..=0.50),
                        );
                        ui.end_row();

                        ui.label("Adv");
                        ui.add(
                            DragValue::new(&mut c.grid_config.adv)
                                .speed(0.01)
                                .clamp_range(0.1..=0.50),
                        );
                        ui.end_row();

                        ui.label("Time");
                        ui.add(
                            DragValue::new(&mut c.grid_config.time)
                                .speed(0.01)
                                .clamp_range(0.05..=0.50),
                        );
                        ui.end_row();

                        ui.label("Bracket");
                        ui.add(
                            DragValue::new(&mut c.grid_config.bracket)
                                .speed(0.01)
                                .clamp_range(0.1..=0.50),
                        );
                        ui.end_row();

                        ui.label("Match Type");
                        ui.add(
                            DragValue::new(&mut c.grid_config.match_type)
                                .speed(0.01)
                                .clamp_range(0.1..=0.90),
                        );
                        ui.end_row();

                        ui.label("Points");
                        ui.add(
                            DragValue::new(&mut c.grid_config.points)
                                .speed(0.01)
                                .clamp_range(0.1..=0.90),
                        );
                        ui.end_row();

                        ui.label("Flag");
                        ui.add(
                            DragValue::new(&mut c.grid_config.flag)
                                .speed(0.01)
                                .clamp_range(0.1..=0.90),
                        );
                        ui.end_row();
                    });
            });
    }

}

impl HandleInput for RootWindow {
    fn set_fullscreen(&mut self, fullscreen: bool) {
        self.is_fullscreen = fullscreen;
    }

    fn is_fullscreen(&self) -> bool {
        self.is_fullscreen
    }
}

impl TrackedWindow for RootWindow {
    fn is_root(&self) -> bool {
        true
    }

    fn set_root(&mut self, _root: bool) {}

    fn redraw(
        &mut self,
        c: &mut AppCommon,
        egui: &mut EguiGlow,
        window: &egui_multiwin::winit::window::Window,
        _clipboard: &mut egui_multiwin::arboard::Clipboard,
    ) -> RedrawResponse {
        let mut quit = false;

        egui.egui_ctx.request_repaint();

        let mut windows_to_create = vec![];

        self.handle_input(c, window, egui);
        self.grid_config_ui(c, egui);

        egui_multiwin::egui::TopBottomPanel::top("menu_bar").show(&egui.egui_ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Exit").clicked() {
                        quit = true;
                    }
                });
                ui.menu_button("Window", |ui| {
                    if ui.selectable_label(c.show_score_window, "Score window").clicked() {
                        c.show_score_window = !c.show_score_window;

                        if c.show_score_window {
                            windows_to_create.push(ScoreWindow::request("Score".to_string()));
                        }
                    }

                    if ui.selectable_label(c.show_grid_config, "Grid Config").clicked() {
                        c.show_grid_config = !c.show_grid_config;
                    }
                });
            });
        });


        egui_multiwin::egui::CentralPanel::default().show(&egui.egui_ctx, |ui| {

        });


        if self.window_size != window.inner_size() {
            self.window_size = window.inner_size();

            println!("Detected resize");
        }


        RedrawResponse {
            quit,
            new_windows: windows_to_create,
        }
    }
}