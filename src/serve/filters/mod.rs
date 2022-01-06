use super::super::peer::*;
use warp::reply::Json;
use warp::Filter;
use warp::filters::BoxedFilter;

// pub fn handle_peer_ws(ws: war) {//for peers that are connected}

pub fn list_peers() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Copy {
    warp::path!("l_peers")
        .and(warp::get())
        .and_then(get_peer_list)
}


//get peer list Handler Fn
pub async fn get_peer_list()
    -> Result<impl warp::Reply, std::convert::Infallible> {
    let fake_list = vec![
        Peer::new(&[192, 168, 1, 1], 3001, PeerType::Direct),
        Peer::new(&[192, 168, 2, 2], 3002, PeerType::Related),
        Peer::new(&[192, 168, 3, 3], 3003, PeerType::Direct),
        Peer::new(&[192, 168, 4, 4], 3004, PeerType::Related),
        Peer::new(&[192, 168, 5, 5], 3005, PeerType::Direct)
    ];

    Ok(warp::reply::json(&fake_list))
}

#[cfg(test)]
mod test {
    use super::list_peers;
    use super::super::super::peer::*;

    #[tokio::test]
    async fn test_get_peer_list() {
        let res = warp::test::request()
            .path("/l_peers")
            .method("GET")
            .reply(&list_peers())
            .await;
            //Shouldnt be an error, so, it expencts the OK case and retruns whats inside of the OK//

            let res_json = serde_json::to_string(
                &vec![
                    Peer::new(&[192, 168, 1, 1], 3001, PeerType::Direct),
                    Peer::new(&[192, 168, 2, 2], 3002, PeerType::Related),
                    Peer::new(&[192, 168, 3, 3], 3003, PeerType::Direct),
                    Peer::new(&[192, 168, 4, 4], 3004, PeerType::Related),
                    Peer::new(&[192, 168, 5, 5], 3005, PeerType::Direct)
                ]
            ).unwrap();
            
            let res_body = res.body();

            assert!(!res_body.is_empty());

            if let Ok(res_string) = String::from_utf8(res_body.to_vec()) {
                assert_eq!(res_json, res_string);
            }
            else {
                assert!(false);
            }
    }
}
