use super::super::peer::*;
use futures_util::{TryFutureExt, SinkExt, StreamExt};
use jsonrpc_core::IoHandler;
use tokio::sync::mpsc;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{
    ws::{Message, WebSocket},
    Filter,
};

pub fn rpc(
    rpc_handler: IoHandler,
) -> impl Filter<Extract = (IoHandler,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || rpc_handler.clone())
}

pub fn ws_rpc(
    rpc_handler: IoHandler,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("ws_rpc")
        .and(warp::ws())
        .and(rpc(rpc_handler))
        .and_then(ws_rpc_handler)
}

pub async fn ws_rpc_handler(
    ws: warp::ws::Ws,
    handler: IoHandler,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(ws.on_upgrade(move |socket| async move {
        let rpc_handler = handler.clone();

        let (mut client_ws_sender, mut client_ws_reciever) = socket.split();

        while let Some(res) = client_ws_reciever.next().await {
            match res {
                Ok(message) => {
                    let rpc_response = rpc_handler.handle_request(message.to_str().unwrap()).await.unwrap_or(String::from("Couldn't process rpc request"));

                    client_ws_sender.send(Message::text(rpc_response)).unwrap_or_else(|e| {
                        println!("Error sending to client: {}", e);
                    }).await;
                },
                Err(e) => {
                    let error_message = Message::text(format!("{:?}", e));
                    client_ws_sender.send(error_message).unwrap_or_else(|e| {
                        println!("Error sending to client: {}", e);
                    }).await;
                }
            }
        }
    }))
}

pub fn list_peers() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("l_peers")
        .and(warp::get())
        .and_then(get_peer_list)
}

//get peer list Handler Fn
pub async fn get_peer_list() -> Result<impl warp::Reply, std::convert::Infallible> {
    let fake_list = vec![
        Peer::new(&[192, 168, 1, 1], 3001, PeerType::Direct),
        Peer::new(&[192, 168, 2, 2], 3002, PeerType::Related),
        Peer::new(&[192, 168, 3, 3], 3003, PeerType::Direct),
        Peer::new(&[192, 168, 4, 4], 3004, PeerType::Related),
        Peer::new(&[192, 168, 5, 5], 3005, PeerType::Direct),
    ];

    Ok(warp::reply::json(&fake_list))
}

#[cfg(test)]
mod test {
    use super::{
        super::super::peer::*,
        list_peers,
        ws_rpc,
    };
    use jsonrpc_core::{IoHandler, Result};
    use jsonrpc_derive::rpc;
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn test_get_peer_list() {
        let res = warp::test::request()
            .path("/l_peers")
            .method("GET")
            .reply(&list_peers())
            .await;
        //Shouldnt be an error, so, it expencts the OK case and retruns whats inside of the OK//

        let res_json = serde_json::to_string(&vec![
            Peer::new(&[192, 168, 1, 1], 3001, PeerType::Direct),
            Peer::new(&[192, 168, 2, 2], 3002, PeerType::Related),
            Peer::new(&[192, 168, 3, 3], 3003, PeerType::Direct),
            Peer::new(&[192, 168, 4, 4], 3004, PeerType::Related),
            Peer::new(&[192, 168, 5, 5], 3005, PeerType::Direct),
        ])
        .unwrap();

        let res_body = res.body();

        assert!(!res_body.is_empty());

        if let Ok(res_string) = String::from_utf8(res_body.to_vec()) {
            assert_eq!(res_json, res_string);
        } else {
            assert!(false);
        }
    }

    #[rpc]
    pub trait SampleRpc {
        #[rpc(name = "hello_world")]
        fn hello_world(&self) -> Result<String>;
    }

    pub struct SampleProtocol;

    impl SampleRpc for SampleProtocol {
        fn hello_world(&self) -> Result<String> {
            Ok("Hello World!".to_string())
        }
    }

    #[tokio::test]
    async fn test_rpc_ws() {
        let mut rpc_handler = IoHandler::new();
        rpc_handler.extend_with(SampleProtocol.to_delegate());

        let mut res = warp::test::ws()
            .path("/ws_rpc")
            .handshake(ws_rpc(rpc_handler))
            .await
            .expect("handshake");

        // TODO: test that rpc endpoints are accessible

        let _send = &res.send_text(r#"{"jsonrpc": "2.0", "method": "hello_world", "id": 1}"#).await;
        let rec = res.recv().await.unwrap();
        let rec_string = rec.to_str().unwrap();

        assert!(rec_string.contains("Hello World!"));
    }
}
