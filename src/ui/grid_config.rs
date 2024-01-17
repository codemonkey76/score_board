pub struct GridConfig {
    pub bottom: f32,
    pub name: f32,
    pub stalling: f32,
    pub adv: f32,
    pub time: f32,
    pub bracket: f32,
    pub match_type: f32,
    pub points: f32,
    pub flag: f32
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            bottom: 0.3,
            name: 0.535,
            stalling: 0.47,
            adv: 0.30,
            time: 0.30,
            bracket: 0.60,
            match_type: 0.14,
            points: 0.67,
            flag: 0.14
        }
    }
}