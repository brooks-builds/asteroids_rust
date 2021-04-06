use eyre::Result;
use ggez::graphics::{DrawMode, Mesh, MeshBuilder, WHITE};
use ggez::Context;

pub fn create_lives_left_mesh(context: &mut Context, lives_left: u32) -> Result<Mesh> {
    let mut mesh = MeshBuilder::new();
    let size = 15.0;

    for count in 0..lives_left {
        let offset = size * count as f32;
        let points = [
            [size / 2.0 + offset, 0.0],
            [0.0 + offset, size],
            [size + offset, size],
        ];
        mesh.polygon(DrawMode::fill(), &points, WHITE)?;
    }

    Ok(mesh.build(context)?)
}
