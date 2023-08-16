//! This example demonstrates Bevy's immediate mode drawing API intended for visual debugging.

use std::f32::consts::PI;
use bevy::prelude::*;
use gloo_net::websocket::futures::WebSocket;
// use tungstenite::stream::MaybeTlsStream;
// use tungstenite::connect;
// use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use bevy::tasks::{TaskPool, Task};
use futures::channel::mpsc::{Sender, Receiver, TryRecvError, TrySendError};
use futures::StreamExt;
use gloo_net::websocket::Message;
use std::net::SocketAddr;
use futures::SinkExt;
use serde::{Deserialize, Serialize};

#[derive(Resource)]
pub struct Network {
    send_channel: Sender<Message>,
    position: Arc<Mutex<Vec2>>
}

#[derive(Serialize, Deserialize)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Serialize, Deserialize)]
struct MoveRequest {
    direction: Direction
}

impl Network {
    pub fn new() -> Self {
	let mut ws = WebSocket::open("ws://localhost:8080/socket").unwrap();
	let (mut ws_write, mut ws_read) = ws.split();
	
	let (send, mut recv) = futures::channel::mpsc::channel::<Message>(100);
	let res = wasm_bindgen_futures::spawn_local(async move {
	    while let Some(msg) = recv.next().await {
		info!("{:?}", msg);		
		let _ = ws_write.send(msg.clone()).await;
	    }
	});

	wasm_bindgen_futures::spawn_local(async move {
	    while let Some(msg) = ws_read.next().await {
		info!("from ws read {:?}", msg);		
	    }
	});	
	
        Self {
	    send_channel: send,
	    position: Arc::new(Mutex::new(Vec2::ZERO))
        }
    }

    pub fn try_send(&mut self, message: Message) -> Result<(), TrySendError<Message>> {
	self.send_channel.try_send(message)
    }

    pub fn parse_socket_addr(addr: &str) -> SocketAddr {
        addr.parse().unwrap()
    }
}

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

async fn ws_loop(ws: WebSocket, mut recv_channel: Receiver<Message>) {

    while let Some(msg) = recv_channel.next().await {
        info!("1. {:?}", msg);
    }

    // while Some(message) => recv_channel
    // loop {
    // 	futures::select! {
    // 	    message = recv_channel.next() => {}
    // 	};
    // }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MyGameCamera));
    commands
	.spawn(Player { position: Vec2::ZERO, direction: Vec2::ZERO })
	.insert(TransformBundle::default());
    
    commands.insert_resource(Network::new());
}

fn update_body(mut gizmos: Gizmos, mut query: Query<(&mut Player, &Transform)>, network: Res<Network>) {

    // let pos = *network.into_inner().position.lock().unwrap();
    let (mut player, _transform) = query.single_mut();
    // gizmos.circle_2d(pos, 100., Color::PURPLE);

    player.position = *network.into_inner().position.lock().unwrap();
    
}

fn system(mut gizmos: Gizmos, mut query: Query<(&mut Player, &Transform)>, time: Res<Time>) {
    let (player, _transform) = query.single_mut();    
    gizmos.circle_2d(player.position, 100., Color::PURPLE);
    
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
		 mut query: Query<&mut Player>, mut q: Query<&mut Transform, With<MyGameCamera>>,
		 mut network: ResMut<Network>,
) {
    let mut player = query.single_mut();
    if keyboard.just_pressed(KeyCode::Right) {
	player.position.x += 100.0;

	let _ = network.try_send(Message::Text(String::from("asdf")));
    }
    if keyboard.just_pressed(KeyCode::Left) {
	player.position.x -= 100.0;
    }

    if keyboard.just_pressed(KeyCode::Up) {
	player.position.y += 100.0;
    }
    if keyboard.just_pressed(KeyCode::Down) {
	player.position.y -= 100.0;
	let result = network.try_send(Message::Text(String::from("test")));
	info!("{:?}", result);
    }

    let mut t = q.single_mut();
    t.translation.x = player.position.x;
    t.translation.y = player.position.y;
}
