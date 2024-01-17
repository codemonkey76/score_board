use scoreboard_lib::Game;
use crate::egui_multiwin_dynamic::multi_window::NewWindowRequest;
use crate::ui::color_scheme::ColorScheme;
use crate::ui::font_config::FontConfig;
use crate::ui::grid_config::GridConfig;
use crate::ui::score_grid::ScoreGrid;

#[derive(Default)]
pub struct AppCommon {
    pub show_score_window: bool,
    pub show_debug_grid: bool,
    pub show_grid_config: bool,
    pub display_grid: ScoreGrid,
    pub scoring_grid: ScoreGrid,
    pub first_run: bool,
    pub color_scheme: ColorScheme,
    pub grid_config: GridConfig,
    pub font_config: FontConfig,
    pub bjj_match: Option<Game>
}

impl AppCommon {
    pub fn new() -> Self {
        Self {
            first_run: true,
            ..Default::default()
        }
    }

    pub fn process_event(&mut self, _event: egui_multiwin::NoEvent) -> Vec<NewWindowRequest> {
        Vec::new()
    }
}