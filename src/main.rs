// External imports
use mpris::{Metadata, Player, PlayerFinder};

fn main() {
    let player: Player = PlayerFinder::new()
        .expect("Error connecting to dbus")
        .find_active()
        .expect("Not playing");

    let metadata: Metadata = player
        .get_metadata()
        .expect("Unable to get player metadata");

    let song_title = match metadata.title() {
        Some(r) => r.to_string(),
        None => "No song found!".to_string(),
    };

    let artist_name = match metadata.artists() {
        Some(r) => r[0].to_string(),
        None => "No artist found!".to_string(),
    };

    println!("{} by {}", song_title, artist_name)
}
