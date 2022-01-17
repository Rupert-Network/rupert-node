use jsonrpc_core::IoHandler;
use futures_util::{FutureExt, SinkExt, StreamExt};
use warp::ws::{Message, WebSocket};


/*
pub async fn ws_rpc_handler(
    ws: warp::ws::Ws,
    handler: IoHandler,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(ws.on_upgrade(move |socket| async {
        let (mut client_ws_sender, mut client_ws_reciever) = socket.split();

        while let Some(res) = client_ws_reciever.next().await {
            match res {
                Ok(message) => {
                    let rpc_response = &handler.handle_request(message.to_str().unwrap());
                    match client_ws_sender.send(Message::text("Ree")).await {
                        Err(e) => println!("Error sending to client: {}", e),
                        _ => {}
                    }
                },
                Err(e) => {
                    let error_message = Message::text(format!("{:?}", e));
                    match client_ws_sender.send(error_message).await {
                        Err(e) => println!("Error sending to client: {}", e),
                        _ => {}
                    }
                }
            }
        }
    }))
}
*/
