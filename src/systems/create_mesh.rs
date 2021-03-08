use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{World, WorldMethods};
use ggez::graphics::{DrawMode, MeshBuilder, WHITE};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

use crate::names::Names;

pub fn create_mesh(context: &mut Context, world: &mut World) -> GameResult {
    let wrapped_locations = world
        .query_one(Names::Location)
        .expect("querying for locations")
        .borrow();
    let player_size: &f32 = world.get_resource(Names::PlayerSize).unwrap();

    let mesh = &mut MeshBuilder::default();

    wrapped_locations
        .cast()
        .unwrap()
        .iter()
        .enumerate()
        .for_each(|(index, location): (usize, &Point)| {
            let player_triangles = [
                Point2 {
                    x: location.x + player_size / 2.0,
                    y: location.y,
                },
                Point2 {
                    x: location.x - player_size / 2.0,
                    y: location.y + player_size / 2.0,
                },
                Point2 {
                    x: location.x - player_size / 2.0,
                    y: location.y - player_size / 2.0,
                },
            ];
            mesh.triangles(&player_triangles, WHITE)
                .expect("creating player triangle");
        });

    drop(wrapped_locations);

    let mesh = mesh.build(context)?;
    world.add_resource(Names::Mesh, mesh);
    Ok(())
}
