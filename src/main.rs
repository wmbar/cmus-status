// Standard library imports


// External imports
use mpris::PlayerFinder;

fn main() {
    let player = PlayerFinder::new().expect("Error connecting to dbus")
                                    .find_active().expect("Not playing");

    let metadata = player.get_metadata()
        .expect("Unable to get player metadata");

    println!("{:#?}", metadata);

}
