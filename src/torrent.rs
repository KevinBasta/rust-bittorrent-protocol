
    use std::net::SocketAddr;
    use tokio::net::TcpStream;
    use std::collections::{HashMap, HashSet};
    use crate::peer::{PeerState, PeerConnection};
    use crate::protocol::connect;

    pub struct Torrent {
        trackers: Vec<String>,
        pieces: HashSet<i32>,
        remaining_pieces: HashSet<i32>,
        peers: HashMap<SocketAddr, PeerConnection>,
        dead_peers: HashSet<SocketAddr>,
        active_peers: HashSet<SocketAddr>
    }

    impl Torrent {
        pub fn new(path: String) -> Self {
            // initalize data structures
            Self{ 
                trackers: Vec::new(),
                pieces: HashSet::new(), 
                remaining_pieces: HashSet::new(), 
                peers: HashMap::new(),
                dead_peers: HashSet::new(),
                active_peers: HashSet::new()
            }

            // load torrent data from metainfo file
        }

        fn tracker_bootstrap() -> bool {
            // bootstrap initial peers via tracker 

            // fills in peers

            return false;
        }

        fn dht_bootstrap() -> bool {
            // bootstrap initial peers via distributed hash table (DHT)

            // fills in peers

            return false;
        }

        pub async fn runtime() {
            let dummy: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 12345));
            let out = connect(dummy).await;
            match out {
                Ok(s) => {println!("Connected")},
                Err(e) => {println!("not connected")}
            }
        }



    }
