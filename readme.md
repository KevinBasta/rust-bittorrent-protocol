# Rust Bittorrent Protocol
A peer-to-peer distributed file sharing program

`This repository is in progress` 

# Definitions
`Tracker`:
- a server that keeps track of one or more `Torrents`
- keeps track of complete and incomplete peer downloads
- recieves HTTP/HTTPS GET requests
- responds with "text/plain" `bencoded` data
- can be scraped to get all `Torrents` the `Tracker` is managing

`Torrent`:
- also known as the `metainfo file`
- a `bencoded` metadata file
- contains information about `pieces`
- can contain pieces belonging to one file or multiple files

`pieces`:
- a file that you want to distribute is split up into pieces
- each piece is usually 512KB


# Specification Info

