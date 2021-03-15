use ggez::graphics::{DrawMode, Mesh, MeshBuilder, WHITE};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

pub fn create_debris_mesh(context: &mut Context) -> GameResult<Mesh> {
    MeshBuilder::new()
        .circle(DrawMode::fill(), Point2 { x: 0.0, y: 0.0 }, 2.0, 0.1, WHITE)
        .build(context)
}
