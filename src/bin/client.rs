extern crate pueue;
extern crate tokio_core;

use pueue::client::client::Client;
use pueue::settings::Settings;

fn main() {
    let settings = Settings::new().unwrap();
    let save_result = settings.save();

    if save_result.is_err() {
        println!("Failed saving config file.");
        println!("{:?}", save_result.err());
    }

    Client::call(&settings);
}
