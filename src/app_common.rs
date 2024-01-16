use crate::egui_multiwin_dynamic::multi_window::NewWindowRequest;
use crate::ui::color_scheme::ColorScheme;
use crate::ui::font_config::FontConfig;
use crate::ui::grid_config::GridConfig;
use crate::ui::score_grid::ScoreGrid;

#[derive(Default)]
pub struct AppCommon {
    pub show_score_window: bool,
    pub show_debug_grid: bool,
    pub score_grid: ScoreGrid,
    _color_scheme: ColorScheme,
    pub grid_config: GridConfig,
    _font_config: FontConfig,
}

impl AppCommon {
    pub fn new() -> Self {
        AppCommon::default()
    }

    pub fn process_event(&mut self, _event: egui_multiwin::NoEvent) -> Vec<NewWindowRequest> {
        Vec::new()
    }
}