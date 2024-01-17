use egui_multiwin::egui::{Grid, Pos2, Rect};
use crate::ui::grid_config::GridConfig;

pub struct ScoreGrid {
    pub window: Rect,
    pub competitor_one: CompetitorGrid,
    pub competitor_two: CompetitorGrid,
    pub match_info: TimeGrid,
}

impl Default for ScoreGrid {
    fn default() -> Self {
        Self {
            window: Rect::NOTHING,
            competitor_one: CompetitorGrid::default(),
            competitor_two: CompetitorGrid::default(),
            match_info: TimeGrid::default()
        }
    }
}

impl ScoreGrid {
    pub fn calc(rect: Rect, config: &GridConfig) -> Self {
        let (top, bottom) = rect.split_top_bottom_at_fraction(1.0 - config.bottom);
        let (top, middle) = top.split_top_bottom_at_fraction(0.5);

        Self {
            window: rect,
            competitor_one: CompetitorGrid::calc(top, config),
            competitor_two: CompetitorGrid::calc(middle, config),
            match_info: TimeGrid::calc(bottom, config),
        }
    }
}

#[derive(Default)]
pub struct CompetitorGrid {
    pub name_grid: NameGrid,
    pub stalling_grid: StallingGrid,
    pub points_grid: PointsGrid
}

impl CompetitorGrid {
    fn calc(rect: Rect, config: &GridConfig) -> Self {
        let (name, right) = rect.split_left_right_at_fraction(config.name);
        let (stalling, points) = right.split_left_right_at_fraction(config.stalling);

        Self {
            name_grid: NameGrid::calc(name, config),
            stalling_grid: StallingGrid::calc(stalling, config),
            points_grid: PointsGrid::calc(points, config)
        }
    }
}

pub struct StallingGrid {
    pub stalling: Rect
}

impl StallingGrid {
    fn calc(rect: Rect, _config: &GridConfig) -> Self {
        Self {
            stalling: rect
        }
    }
}
impl Default for StallingGrid {
    fn default() -> Self {
        Self {
            stalling: Rect::NOTHING
        }
    }
}

pub struct NameGrid {
    pub top: Rect,
    pub bottom: Rect,
    pub flag: Rect,
    pub name: Rect,
    pub country: Rect,
    pub team: Rect
}

impl NameGrid {
    fn calc(rect: Rect, config: &GridConfig) -> Self {
        let (top, bottom) = rect.split_top_bottom_at_fraction(config.name);
        let (flag, name) = top.split_left_right_at_fraction(config.flag);
        let (country, team) = bottom.split_left_right_at_fraction(config.flag);

        Self {
            top,
            bottom,
            flag,
            name,
            country,
            team
        }
    }
}
impl Default for NameGrid {
    fn default() -> Self {
        Self {
            top: Rect::NOTHING,
            bottom: Rect::NOTHING,
            flag: Rect::NOTHING,
            name: Rect::NOTHING,
            country: Rect::NOTHING,
            team: Rect::NOTHING
        }
    }
}

pub struct PointsGrid {
    pub advantage: Rect,
    pub penalty: Rect,
    pub points: Rect
}

impl PointsGrid {
    fn calc(rect: Rect, config: &GridConfig) -> Self {
        let (left, points) = rect.split_left_right_at_fraction(1.0 - config.points);
        let (advantage, penalty) = left.split_top_bottom_at_fraction(0.5);

        Self {
            advantage,
            penalty,
            points
        }
    }
}


impl Default for PointsGrid {
    fn default() -> Self {
        Self {
            advantage: Rect::NOTHING,
            penalty: Rect::NOTHING,
            points: Rect::NOTHING
        }
    }
}

pub struct TimeGrid {
    pub time: Rect,
    pub match_type: Rect,
    pub match_info: Rect,
    pub bracket_info: Rect
}
impl TimeGrid {
    fn calc(rect: Rect, config: &GridConfig) -> Self {
        let (time, right) = rect.split_left_right_at_fraction(config.time);
        let (top, bracket_info) = right.split_top_bottom_at_fraction(1.0 - config.bracket);
        let (match_type, match_info) = top.split_left_right_at_fraction(config.match_type);

        Self {
            time,
            match_type,
            match_info,
            bracket_info
        }
    }
}
impl Default for TimeGrid {
    fn default() -> Self {
        Self {
            time: Rect::NOTHING,
            match_type: Rect::NOTHING,
            match_info: Rect::NOTHING,
            bracket_info: Rect::NOTHING
        }
    }
}
