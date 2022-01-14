use super::super::peer::*;
use futures_util::{FutureExt, StreamExt};
use warp::filters::BoxedFilter;
use warp::reply::Json;
use warp::Filter;

pub fn ws_rpc() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Copy {
    warp::path!("ws_rpc")
        .and(warp::ws())
        .and_then(ws_rpc_handler)
}

pub async fn ws_rpc_handler(
    ws: warp::ws::Ws,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(ws.on_upgrade(move |socket| {
        // TODO:
        // - add clients as peers
        // - when data is recieved pass it to rpc handler

        // tx -> send stream, rx -> recieve stream
        let (tx, rx) = socket.split();

        // forwards data sent back to sender
        rx.forward(tx).map(|res| {
            if let Err(e) = res {
                println!("websocket error: {}", e);
            }
        })
    }))
}

// pub fn handle_peer_ws(ws: war) {//for peers that are connected}
pub fn list_peers() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Copy {
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
    use super::super::super::peer::*;
    use super::{list_peers, ws_rpc};

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

    #[tokio::test]
    async fn test_ws_rpc() {
        let mut res = warp::test::ws()
            .path("/ws_rpc")
            .handshake(ws_rpc())
            .await
            .expect("handshake");

        // TODO: test that rpc endpoints are accessible

        let _send = &res.send_text("bing_bong").await;

        let rec = res.recv().await.unwrap();

        let rec_string = rec.to_str().unwrap();

        assert_eq!("bing_bong", rec_string);
    }
}
