       use tokio::net::TcpStream;

   
    // the default trait may need to be modified since
    // the start state might not be all false
    // #[derive(Default)]
    pub struct PeerState {
        // is the peer chocking and or interested in me
        peer_chocking: bool,
        peer_interested: bool,

        // am I chocking and or interested in the peer 
        am_choking: bool,
        am_interested: bool
    }

    impl Default for PeerState {
        fn default() -> Self {
            PeerState { peer_chocking: false, peer_interested: false, am_choking: false, am_interested: false }
        }
    }

    pub struct PeerConnection {
        state: PeerState,
        stream: TcpStream
    }
