use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{World, WorldMethods};
use ggez::graphics::{Color, MeshBuilder, WHITE};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

use crate::names::Names;

pub fn create_mesh(context: &mut Context, world: &mut World) -> GameResult {
    let wrapped_locations = world
        .query_one(Names::Location)
        .expect("querying for locations")
        .borrow();

    let is_thrusting: &bool = world.get_resource(Names::Thrusting).unwrap();
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

            if *is_thrusting {
                let thruster_color: &Color = world.get_resource(Names::ThrusterColor).unwrap();
                let engine_triangles = [
                    Point2 {
                        x: location.x - player_size / 2.0,
                        y: location.y + player_size / 4.0,
                    },
                    Point2 {
                        x: location.x - player_size,
                        y: location.y,
                    },
                    Point2 {
                        x: location.x - player_size / 2.0,
                        y: location.y - player_size / 4.0,
                    },
                ];
                mesh.triangles(&engine_triangles, *thruster_color)
                    .expect("creating engine triangle");
            }
        });

    drop(wrapped_locations);

    let mesh = mesh.build(context)?;
    world.add_resource(Names::Mesh, mesh);
    Ok(())
}
