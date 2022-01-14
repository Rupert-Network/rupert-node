use super::peer::*;
use jsonrpc_core::*;
use jsonrpc_derive::rpc;

#[rpc]
pub trait Rpc {
    #[rpc(name = "protocol_version")]
    fn protocol_version(&self) -> Result<String>;

    #[rpc(name = "list_peers")]
    fn list_peers(&self, max: u32) -> Result<Vec<Peer>>;
}

pub struct Protocol;

impl Rpc for Protocol {
    fn protocol_version(&self) -> Result<String> {
        Ok("v0.1a".to_string())
    }

    fn list_peers(&self, max: u32) -> Result<Vec<Peer>> {
        let peers = vec![
            Peer::new(&[192, 168, 1, 1], 3001, PeerType::Direct),
            Peer::new(&[192, 168, 2, 2], 3002, PeerType::Related),
            Peer::new(&[192, 168, 3, 3], 3003, PeerType::Direct),
            Peer::new(&[192, 168, 4, 4], 3004, PeerType::Related),
            Peer::new(&[192, 168, 5, 5], 3005, PeerType::Direct),
        ]
        .iter()
        .take(max as usize)
        .cloned()
        .collect::<Vec<Peer>>();

        Ok(peers)
    }
}

#[cfg(test)]
mod test {
    use jsonrpc_core::{futures, IoHandler};
    use jsonrpc_core_client::transports::local;
    // gen_client wont be detected by lsp as it is built by #[rpc] macro
    use super::{gen_client, *};

    #[test]
    fn protocol_new_test() {
        let mut io = IoHandler::new();
        io.extend_with(Protocol.to_delegate());

        let (client, server) = local::connect::<gen_client::Client, _, _>(io);

        let fut = async move {
            let res = client.list_peers(2).await.unwrap();
            assert_eq!(
                res,
                vec![
                    Peer::new(&[192, 168, 1, 1], 3001, PeerType::Direct),
                    Peer::new(&[192, 168, 2, 2], 3002, PeerType::Related),
                ]
            );
        };

        futures::executor::block_on(async move { futures::join!(server, fut) })
            .0
            .unwrap();

        assert!(true);
    }
} /* test */
