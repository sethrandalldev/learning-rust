use wallpaper;
use std::{thread, time};

fn main() {
    // Returns the wallpaper of the current desktop.
    println!("{:?}", wallpaper::get());
    // Sets the wallpaper for the current desktop from a URL.
    wallpaper::set_from_url("https://source.unsplash.com/collection/136301/2000x2000").unwrap();
    thread::sleep(time::Duration::from_secs(3));
    wallpaper::set_from_path("/Users/sethrandall/Downloads/IMG_5600.HEIC").unwrap();
    wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
    // Returns the wallpaper of the current desktop.
    println!("{:?}", wallpaper::get());
}
