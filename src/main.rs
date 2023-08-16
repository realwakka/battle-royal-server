use std::{sync::Arc, sync::Mutex, collections::HashMap};

use warp::{Filter, ws::WebSocket};
use futures_util::{FutureExt, StreamExt};
use warp::ws::{Ws, Message};
use serde::{Deserialize, Serialize};

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

async fn session(mut ws: WebSocket) {

    let mut x = 0;
    let mut y = 0;

    while let Some(message) = ws.next().await {
	match message {
	    Ok(message) => {
		println!("{:?}", message);
		let Ok(str) = message.to_str() else {
		    continue;
		};
		let Ok(request) = serde_json::from_str::<MoveRequest>(&str) else {
		    continue;
		};
		match request.direction {
		    Direction::Left => { x += 100; }
		    Direction::Right => { x -= 100; }
		    Direction::Up => { y -= 100; }
		    Direction::Down => { y += 100; }
		}
	    },
	    Err(e) => {

	    }
	}
    }
}

#[tokio::main]
async fn main() {
    let port : u16 = std::env::var("PORT").expect("no port env").parse().expect("failed to parse port");

    // let front = warp::path::end().and(warp::fs::dir("frontend/dist"));
    let front = warp::fs::dir("frontend/dist");

    // let Arc::new(Mutex::new(HashMap::new()));    

    let websocket = warp::path("socket")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(session)
	});

    let routes = front.or(websocket);

    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
}
