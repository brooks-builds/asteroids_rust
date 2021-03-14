use ggez::graphics::{DrawMode, Mesh, MeshBuilder, WHITE};
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use rand::random;

const RANDOM_OFFSET: f32 = 100.0;

pub fn create_asteroid_mesh(context: &mut Context, radius: f32) -> GameResult<Mesh> {
    let first_point = Point2 {
        x: 0.0,
        y: -radius + random::<f32>() * RANDOM_OFFSET - RANDOM_OFFSET / 2.0,
    };
    let points = [
        // top
        first_point,
        // top right
        Point2 {
            x: radius / 2.0 + random::<f32>() * RANDOM_OFFSET - RANDOM_OFFSET / 2.0,
            y: -radius / 2.0 + random::<f32>() * RANDOM_OFFSET - RANDOM_OFFSET / 2.0,
        },
        // right
        Point2 {
            x: radius + random::<f32>() * RANDOM_OFFSET - RANDOM_OFFSET / 2.0,
            y: 0.0,
        },
        // bottom right
        Point2 {
            x: radius / 2.0 + random::<f32>() * RANDOM_OFFSET - RANDOM_OFFSET / 2.0,
            y: radius / 2.0 + random::<f32>() * RANDOM_OFFSET - RANDOM_OFFSET / 2.0,
        },
        // bottom
        Point2 {
            x: 0.0,
            y: radius + random::<f32>() * RANDOM_OFFSET - RANDOM_OFFSET / 2.0,
        },
        // bottom left
        Point2 {
            x: -radius / 2.0 + random::<f32>() * RANDOM_OFFSET - RANDOM_OFFSET / 2.0,
            y: radius / 2.0 + random::<f32>() * RANDOM_OFFSET - RANDOM_OFFSET / 2.0,
        },
        // left
        Point2 {
            x: -radius + random::<f32>() * RANDOM_OFFSET - RANDOM_OFFSET / 2.0,
            y: 0.0,
        },
        // top left
        Point2 {
            x: -radius / 2.0 + random::<f32>() * RANDOM_OFFSET - RANDOM_OFFSET / 2.0,
            y: -radius / 2.0 + random::<f32>() * RANDOM_OFFSET - RANDOM_OFFSET / 2.0,
        },
        // top
        first_point,
    ];
    MeshBuilder::new()
        .polyline(DrawMode::stroke(3.0), &points, WHITE)?
        .build(context)
}
