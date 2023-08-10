//! This example demonstrates Bevy's immediate mode drawing API intended for visual debugging.

use std::f32::consts::PI;
use bevy::prelude::*;

#[derive(Component)]
struct Player {
    position: Vec2,
    direction: Vec2,    
}

#[derive(Component)]
struct MyGameCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (system, update_config, update_body))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MyGameCamera));
    // // text
    // commands.spawn(TextBundle::from_section(
    //     "Hold 'Left' or 'Right' to change the line width",
    //     TextStyle {
    //         font: asset_server.load("fonts/FiraMono-Medium.ttf"),
    //         font_size: 24.,
    //         color: Color::WHITE,
    //     },
    // ));

    commands
	.spawn(Player { position: Vec2::ZERO, direction: Vec2::ZERO })
	.insert(TransformBundle::default());
    
}

fn update_body(mut gizmos: Gizmos, mut query: Query<(&mut Player, &Transform)>) {
    let (player, _transform) = query.single_mut();
    gizmos.circle_2d(player.position, 100., Color::PURPLE);
}

fn system(mut gizmos: Gizmos, time: Res<Time>) {
    let sin = time.elapsed_seconds().sin() * 50.;
    gizmos.line_2d(Vec2::Y * -sin, Vec2::splat(-80.), Color::RED);
    gizmos.ray_2d(Vec2::Y * sin, Vec2::splat(80.), Color::GREEN);

    // Triangle
    gizmos.linestrip_gradient_2d([
        (Vec2::Y * 300., Color::BLUE),
        (Vec2::new(-255., -155.), Color::RED),
        (Vec2::new(255., -155.), Color::GREEN),
        (Vec2::Y * 300., Color::BLUE),
    ]);

    gizmos.rect_2d(
        Vec2::ZERO,
        time.elapsed_seconds() / 3.,
        Vec2::splat(300.),
        Color::BLACK,
    );

    gizmos.circle_2d(Vec2::ZERO, 120., Color::BLACK);
    gizmos.circle_2d(Vec2::ZERO, 300., Color::NAVY).segments(64);
    gizmos.arc_2d(Vec2::ZERO, sin / 10., PI / 2., 350., Color::ORANGE_RED);
}

fn update_config(mut config: ResMut<GizmoConfig>, keyboard: Res<Input<KeyCode>>, time: Res<Time>,
		 mut query: Query<&mut Player>, mut q: Query<&mut Transform, With<MyGameCamera>>,) {

    let mut player = query.single_mut();
    if keyboard.pressed(KeyCode::Right) {
	player.position.x += 1.0;
    }
    if keyboard.pressed(KeyCode::Left) {
	player.position.x -= 1.0;
    }

    if keyboard.pressed(KeyCode::Up) {
	player.position.y += 1.0;
    }
    if keyboard.pressed(KeyCode::Down) {
	player.position.y -= 1.0;
    }

    let mut t = q.single_mut();
    t.translation.x = player.position.x;
    t.translation.y = player.position.y;
}
