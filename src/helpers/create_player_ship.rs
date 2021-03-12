use ggez::graphics::{Color, Mesh, MeshBuilder};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

pub fn create_player_ship(
    context: &mut Context,
    player_size: f32,
    player_ship_color: Color,
    is_thrusting: bool,
    thruster_color: Color,
) -> GameResult<Mesh> {
    let mut mesh = MeshBuilder::new();

    mesh.triangles(&create_ship(player_size), player_ship_color)?;

    if is_thrusting {
        mesh.triangles(&create_thrusters(player_size), thruster_color)?;
    }

    mesh.build(context)
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
