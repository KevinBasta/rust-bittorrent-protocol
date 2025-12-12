mod client;
mod torrent;
mod peer;
mod protocol;
mod bencoding;

use crate::torrent::{Torrent};
use tokio;

#[tokio::main]
async fn main() {


    let mut tor = Torrent::new(String::from("Hello, world!"));
    Torrent::runtime().await    ;
    
    println!("Hello, world!");
}
