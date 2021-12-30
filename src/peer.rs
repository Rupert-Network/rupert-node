#[derive(Debug, Clone, Copy)]
pub enum PeerType {
    Direct,
    Related,
}

#[derive(Debug, Clone)]
pub struct Peer {
    pub ip: Vec<u8>,
    pub port: u16,
    pub variant: PeerType,
}

impl Peer {
    pub fn new(ip: &[u8], port: u16, variant: PeerType) -> Self {
        Peer {
            ip: ip.to_vec(),
            port,
            variant,
        }
    }

    pub fn url(&self) -> String {
        let ip_string = self.ip
            .iter()
            .enumerate()
            .map(|(i, c)| {
                match i + 1 == self.ip.len() {
                    true => format!("{}", c),
                    false => format!("{}.", c),
                }
            })
            .collect::<String>();

            format!("ws://{}:{}", ip_string, self.port)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_url() {
        let peer = Peer::new(&[192, 168, 1, 1], 3001, PeerType::Direct);
        assert_eq!(peer.url(), "ws://192.168.1.1:3001");
    }
}
