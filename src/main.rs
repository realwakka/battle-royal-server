use warp::Filter;
use futures_util::{FutureExt, StreamExt};

#[tokio::main]
async fn main() {
    let port : u16 = std::env::var("PORT").expect("no port env").parse().expect("failed to parse port");

    let front = warp::path::end().and(warp::fs::dir("frontend/dist"));

    let websocket = warp::path("socket")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|websocket| {
                // Just echo all messages back...
                let (tx, rx) = websocket.split();
                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        eprintln!("websocket error: {:?}", e);
                    }
                })
            })
        });

    let routes = front.or(websocket);

    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
}
