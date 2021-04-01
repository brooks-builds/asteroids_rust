use std::f32;

use bbecs::data_types::point::Point;
use ggez::graphics::{DrawMode, Mesh, MeshBuilder, WHITE};
use ggez::{Context, GameResult};
use rand::{random, thread_rng, Rng};

pub fn create_asteroid_mesh(context: &mut Context, radius: f32) -> GameResult<Mesh> {
    let mut points = vec![];
    let mut rng = thread_rng();

    let point_count = 6;
    let angle = std::f32::consts::TAU / point_count as f32;
    let random_offset = angle as f32 * 0.2;
    for count in 0..point_count {
        let current_angle = (angle * count as f32) + rng.gen_range(-random_offset..random_offset);
        let mut point = Point::new(current_angle.cos(), current_angle.sin());
        point.multiply_scalar(radius);
        points.push(point.to_array());
    }
    MeshBuilder::new()
        .polygon(DrawMode::stroke(3.0), &points, WHITE)?
        // .circle(DrawMode::stroke(1.0), [0.0, 0.0], radius, 5.0, WHITE)
        .build(context)
}
