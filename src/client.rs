use std::collections::{HashMap};
use std::net::{IpAddr, Ipv4Addr};

// the default trait may need to be modified since
// the start state might not be all false
// #[derive(Default)]
pub struct PeerState {
    // is the peer chocking and or interested in me
    peer_chocking: bool,
    peer_interested: bool,
    // am I chocking and or interested in the peer 
    client_chocking: bool,
    client_interested: bool
}

impl Default for PeerState {
    fn default() -> Self {
        PeerState { peer_chocking: false, peer_interested: false, client_chocking: false, client_interested: false }
    }
}

pub struct Client {
    peers: HashMap<IpAddr, PeerState>,

}

impl Client {
    fn new() -> Self {
        Self{ peers: HashMap::new() }
    }

    fn connect(&mut self, new_peer: IpAddr) {
        self.peers.entry(new_peer).or_insert_with(PeerState::default);
    }
}


pub fn test_client() {
    let mut peer = Client::new();
    peer.connect(IpAddr::V4(Ipv4Addr::new(10, 1, 2, 20)));
}
