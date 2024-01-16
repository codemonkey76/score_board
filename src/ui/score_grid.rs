use egui_multiwin::egui::{Pos2, Rect};
use crate::ui::grid_config::GridConfig;


pub struct ScoreGrid {
    pub window: Rect,

    pub top: Rect,
    pub middle: Rect,
    pub bottom: Rect,

    pub score1: Rect,
    pub score2: Rect,

    no_values: bool,
}
impl Default for ScoreGrid {
    fn default() -> Self {
        Self {
            window: Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(0.0, 0.0)),
            top: Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(0.0, 0.0)),
            middle: Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(0.0, 0.0)),
            bottom: Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(0.0, 0.0)),

            score1: Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(0.0, 0.0)),
            score2: Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(0.0, 0.0)),

            no_values: true
        }
    }
}
impl ScoreGrid {
    pub fn no_values(&self) -> bool {
        self.no_values
    }
    pub fn calc(window: Rect, config: &GridConfig) -> Self {
        let (top, bottom) = window.split_top_bottom_at_fraction(1.0 - config.bottom);
        let (top, middle) = top.split_top_bottom_at_fraction(0.5);
        let (_,score1) = top.split_left_right_at_fraction(1.0 - config.score);
        let (_,score2) = middle.split_left_right_at_fraction(1.0 - config.score);

        ScoreGrid {
            window,
            top,
            middle,
            bottom,
            score1,
            score2,
            no_values: false
        }
    }
}
