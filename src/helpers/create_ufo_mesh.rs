use eyre::Result;
use ggez::graphics::{DrawMode, Mesh, MeshBuilder, WHITE};
use ggez::Context;

pub fn create_ufo_mesh(context: &mut Context, size: f32) -> Result<Mesh> {
    let width = 2.0;
    let color = WHITE;
    let short_size = size * 0.25;
    let medium_size = size * 0.5;
    let tiny_size = size * 0.125;
    let mini_size = size * 0.06;
    let mesh = MeshBuilder::new()
        .polyline(
            DrawMode::stroke(width),
            &[
                [medium_size, 0.0],
                [short_size, -short_size],
                [-short_size, -short_size],
                [-medium_size, 0.0],
                [-short_size, short_size],
                [short_size, short_size],
                [medium_size, 0.0],
                [-medium_size, 0.0],
            ],
            color,
        )?
        .polyline(
            DrawMode::stroke(width),
            &[
                [tiny_size, -short_size],
                [tiny_size, -short_size - mini_size],
                [-tiny_size, -short_size - mini_size],
                [-tiny_size, -short_size],
            ],
            color,
        )?
        .build(context)?;
    Ok(mesh)
}
