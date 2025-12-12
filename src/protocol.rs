
    use std::net::SocketAddr;
    use tokio::net::TcpStream;
    use tokio::time::{timeout, Duration};
    use std::io;

    pub async fn connect(addr: SocketAddr) -> Result<TcpStream, io::Error> {
        let stream = timeout(Duration::from_secs(5), TcpStream::connect(addr)).await;

        match stream {
            Ok(s) => s,
            Err(_) => Err(io::Error::new(io::ErrorKind::TimedOut, "connection timed out")),
        }
    }

