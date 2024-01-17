use egui_multiwin_dynamic::multi_window::MultiWindow;

pub mod egui_multiwin_dynamic {
    egui_multiwin::tracked_window!(
        crate::AppCommon,
        egui_multiwin::NoEvent,
        crate::windows::MyWindows
    );
    egui_multiwin::multi_window!(
        crate::AppCommon,
        egui_multiwin::NoEvent,
        crate::windows::MyWindows
    );
}

mod windows;
mod ui;


mod app_common;
pub use app_common::AppCommon;
use scoreboard_lib::get_match;

const SCORE_FONT: &[u8] = include_bytes!("../assets/fonts/BebasNeue-Regular.ttf");

use windows::{
    root::{self},
};

fn main() {
    let mut event_loop = egui_multiwin::winit::event_loop::EventLoopBuilder::with_user_event();

    let event_loop = event_loop.build();

    let mut multi_window: MultiWindow = MultiWindow::new();

    let mut font_data = egui_multiwin::egui::FontData::from_static(SCORE_FONT);
    font_data.tweak.y_offset_factor = 0.08;

    multi_window.add_font("score_font".to_string(), font_data);

    let root_window = root::RootWindow::request();

    let mut ac = AppCommon::new();

    let _e = multi_window.add(root_window, &mut ac, &event_loop);

    multi_window.run(event_loop, ac);
}