pub struct GridConfig {
    pub bottom: f32,
    pub score: f32
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            bottom: 0.3,
            score: 0.17
        }
    }
}