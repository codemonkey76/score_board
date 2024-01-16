use crate::egui_multiwin_dynamic::{
    multi_window::NewWindowRequest,
    tracked_window::{RedrawResponse, TrackedWindow},
};
use egui_multiwin::egui::menu;
use egui_multiwin::egui_glow::EguiGlow;
use egui_multiwin::winit::dpi::{PhysicalSize, Size};

use crate::AppCommon;
use crate::windows::HandleInput;

use super::score_window::ScoreWindow;

pub struct RootWindow {
    is_fullscreen: bool,
    window_size: PhysicalSize<u32>
}

impl RootWindow {
    pub fn request() -> NewWindowRequest {
        let window_size = PhysicalSize{ width: 800, height: 600};

        NewWindowRequest {

            window_state: super::MyWindows::Root(RootWindow { is_fullscreen: false, window_size }),
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
                            windows_to_create.push(ScoreWindow::request("score window".to_string()));
                        }
                    }
                });
            });
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