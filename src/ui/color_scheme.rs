use egui_multiwin::egui::Color32;

pub struct ColorScheme {
    pub competitor_one_name: Color32,
    pub competitor_one_team: Color32,
    pub competitor_one_country: Color32
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            competitor_one_name: Color32::WHITE,
            competitor_one_team: Color32::WHITE,
            competitor_one_country: Color32::WHITE
        }
    }
}