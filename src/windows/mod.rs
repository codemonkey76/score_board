use egui_multiwin::enum_dispatch::enum_dispatch;

use crate::egui_multiwin_dynamic::tracked_window::{RedrawResponse, TrackedWindow};
use egui_multiwin::egui_glow::EguiGlow;
use std::sync::Arc;
use egui_multiwin::winit::window::Fullscreen;
use crate::AppCommon;

pub mod score_window;
pub mod root;

#[enum_dispatch(TrackedWindow)]
pub enum MyWindows {
    Root(root::RootWindow),
    Score(score_window::ScoreWindow),
}

pub trait HandleInput {
    fn handle_input(&mut self, _c: &mut AppCommon, window: &egui_multiwin::winit::window::Window, egui: &mut EguiGlow) {
        if egui.egui_ctx.input(|i| i.modifiers.alt && i.key_pressed(egui_multiwin::egui::Key::Enter)) {
            self.toggle_fullscreen(window);
        }
    }

    fn toggle_fullscreen(&mut self, window: &egui_multiwin::winit::window::Window) {
        if self.is_fullscreen() {
            window.set_fullscreen(None);
        } else {
            window.set_fullscreen(Some(Fullscreen::Borderless(None)));
        }

        self.set_fullscreen(!self.is_fullscreen());
    }

    fn set_fullscreen(&mut self, fullscreen: bool);
    fn is_fullscreen(&self) -> bool;
}