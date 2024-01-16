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

const COMPUTER_MODERN_FONT: &[u8] = include_bytes!("./cmunbtl.ttf");

use windows::{
    root::{self},
};

fn main() {
    let mut event_loop = egui_multiwin::winit::event_loop::EventLoopBuilder::with_user_event();

    let event_loop = event_loop.build();

    let mut multi_window: MultiWindow = MultiWindow::new();

    multi_window.add_font(
        "computermodern".to_string(),
        egui_multiwin::egui::FontData::from_static(COMPUTER_MODERN_FONT),
    );

    let root_window = root::RootWindow::request();

    let mut ac = AppCommon::new();

    let _e = multi_window.add(root_window, &mut ac, &event_loop);

    multi_window.run(event_loop, ac);
}