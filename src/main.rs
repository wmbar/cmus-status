// External imports
use mpris::{PlayerFinder, Player, Metadata};


fn main() {
    let player: Player = PlayerFinder::new().expect("Error connecting to dbus")
                                    .find_active().expect("Not playing");

    let metadata: Metadata = player.get_metadata()
        .expect("Unable to get player metadata");

    println!("{:#?} - {:#?}", metadata.title(), metadata.artists())
}
