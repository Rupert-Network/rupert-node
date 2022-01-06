//New Endpoint
//Gather and return list of peers
//Each peer listing

mod core;
mod peer;
mod protocol;
mod serve;

use warp::Filter;
//use crate::peer::Peer;
//Need alt


#[tokio::main]
 async fn main() {
                //GET Endpoints
     let list_peers =  warp::path("l_peers")
         .and(warp::get())
         //.and(do_auth())
         //security check filter
         .map(|| "list of peers - path success")
         .and_then(get_peer_list);
        }


     //let peer_listing = warp::path("p_list")
     //.and(warp::get())
     //.map(|| "each peer listing - path success");

     let get_EP = list_peers;
     //.or(peer_listing);


    async fn get_peer_list(_user_ctx: UserCtx, id: i64)
    -> Result<Json, warp::Rejection> {
        //TODO: Get and return peer list
        //There should be a handler function w a
        //hard coded list of peers from peer.rs


    }

     let routes = get_ep; //or(post_endpoints);
     println!("API Running");
     warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
     //running local host till further notice
 }
