
mod client {
    use std::collections::{HashMap};
    use std::net::{SocketAddr, Ipv4Addr, TcpStream};
        use crate::peer::{PeerState, PeerConnection};

    pub struct Client {
        peers: HashMap<SocketAddr, PeerState>,
    }

    impl Client {
        fn new() -> Self {
            Self{ peers: HashMap::new() }
        }

        fn connect(&mut self, new_peer: SocketAddr) {
            let mut stream = TcpStream::connect(new_peer);

            // Add peer
            //self.peers.entry(new_peer).or_insert_with(PeerState::default, stream);
        }
    }


    pub fn test_client() {
        // let mut peer = Client::new();
        // peer.connect(IpAddr::V4(Ipv4Addr::new(10, 1, 2, 20)));
    }

}