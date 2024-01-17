pub struct FontConfig {
    pub scale: f32,
    pub competitor: f32,
    pub team: f32,
    pub country: f32
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            scale: 1.0,
            competitor: 18.0,
            team: 18.0,
            country: 18.0
        }
    }
}