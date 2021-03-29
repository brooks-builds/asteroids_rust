use ggez::graphics::{DrawMode, Mesh, MeshBuilder, WHITE};
use ggez::{Context, GameResult};

pub fn create_bullet_mesh(context: &mut Context) -> GameResult<Mesh> {
    MeshBuilder::new()
        .circle(DrawMode::fill(), [0.0, 0.0], 2.0, 0.1, WHITE)
        .build(context)
}
