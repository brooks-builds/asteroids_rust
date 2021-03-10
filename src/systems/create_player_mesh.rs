use bbecs::world::{World, WorldMethods};
use eyre::Result;
use ggez::graphics::{Color, Mesh, MeshBuilder, WHITE};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

use crate::names::Names;

pub fn create_player_mesh_system(context: &mut Context, world: &mut World) -> Result<()> {
    let is_thrusting: &bool = world.get_resource(Names::Thrusting)?;
    let player_size: &f32 = world.get_resource(Names::PlayerSize)?;
    let thruster_color: &Color = world.get_resource(Names::ThrusterColor)?;
    let player_ship_color: &Color = world.get_resource(Names::PlayerShipColor)?;

    let mut mesh = MeshBuilder::new();

    mesh.triangles(&create_ship(*player_size), *player_ship_color)
        .unwrap();

    if *is_thrusting {
        mesh.triangles(&create_thrusters(*player_size), *thruster_color)
            .unwrap();
    }

    world.add_resource(Names::PlayerMesh, mesh.build(context).unwrap());
    Ok(())
}

fn create_ship(player_size: f32) -> [Point2<f32>; 3] {
    [
        Point2 {
            x: player_size / 2.0,
            y: 0.0,
        },
        Point2 {
            x: -player_size / 2.0,
            y: player_size / 3.0,
        },
        Point2 {
            x: -player_size / 2.0,
            y: -player_size / 3.0,
        },
    ]
}

fn create_thrusters(player_size: f32) -> [Point2<f32>; 3] {
    [
        Point2 {
            x: -player_size / 2.0,
            y: player_size / 4.0,
        },
        Point2 {
            x: -player_size,
            y: 0.0,
        },
        Point2 {
            x: -player_size / 2.0,
            y: -player_size / 4.0,
        },
    ]
}
