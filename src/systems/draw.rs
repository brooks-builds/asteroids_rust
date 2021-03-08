use bbecs::world::{World, WorldMethods};
use ggez::graphics::{DrawParam, Mesh};
use ggez::{graphics, Context, GameResult};

use crate::names::Names;

pub fn draw_system(context: &mut Context, world: &mut World) -> GameResult {
    let mesh: &Mesh = world
        .get_resource::<Names>(Names::Mesh)
        .expect("trying to extract mesh");
    graphics::draw(context, mesh, DrawParam::new())
}
