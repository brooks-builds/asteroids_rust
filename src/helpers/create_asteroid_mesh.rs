use ggez::graphics::{DrawMode, Mesh, MeshBuilder, WHITE};
use ggez::{Context, GameResult};
use rand::random;

const RANDOM_OFFSET: f32 = 100.0;

pub fn create_asteroid_mesh(context: &mut Context, radius: f32) -> GameResult<Mesh> {
    let first_point = [
        0.0,
        -radius + random::<f32>() * RANDOM_OFFSET - RANDOM_OFFSET / 2.0,
    ];
    let points = [
        // top
        first_point,
        // top right
        [
            radius / 2.0 + random::<f32>() * RANDOM_OFFSET - RANDOM_OFFSET / 2.0,
            -radius / 2.0 + random::<f32>() * RANDOM_OFFSET - RANDOM_OFFSET / 2.0,
        ],
        // right
        [
            radius + random::<f32>() * RANDOM_OFFSET - RANDOM_OFFSET / 2.0,
            0.0,
        ],
        // bottom right
        [
            radius / 2.0 + random::<f32>() * RANDOM_OFFSET - RANDOM_OFFSET / 2.0,
            radius / 2.0 + random::<f32>() * RANDOM_OFFSET - RANDOM_OFFSET / 2.0,
        ],
        // bottom
        [
            0.0,
            radius + random::<f32>() * RANDOM_OFFSET - RANDOM_OFFSET / 2.0,
        ],
        // bottom left
        [
            -radius / 2.0 + random::<f32>() * RANDOM_OFFSET - RANDOM_OFFSET / 2.0,
            radius / 2.0 + random::<f32>() * RANDOM_OFFSET - RANDOM_OFFSET / 2.0,
        ],
        // left
        [
            -radius + random::<f32>() * RANDOM_OFFSET - RANDOM_OFFSET / 2.0,
            0.0,
        ],
        // top left
        [
            -radius / 2.0 + random::<f32>() * RANDOM_OFFSET - RANDOM_OFFSET / 2.0,
            -radius / 2.0 + random::<f32>() * RANDOM_OFFSET - RANDOM_OFFSET / 2.0,
        ],
        // top
        first_point,
    ];
    MeshBuilder::new()
        .polyline(DrawMode::stroke(3.0), &points, WHITE)?
        .build(context)
}
