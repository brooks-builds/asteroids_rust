use bbecs::data_types::point::Point;

#[derive(Debug, Clone, Copy)]
pub struct AsteroidData {
    pub size: f32,
    pub speed: f32,
    pub location: Point,
}

impl AsteroidData {
    pub fn new(size: f32, speed: f32, location: Point) -> Self {
        Self {
            size,
            speed,
            location,
        }
    }

    pub fn update_for_destroyed(&mut self) {
        self.size /= 2.0;
        self.speed *= 1.1;
    }
}
