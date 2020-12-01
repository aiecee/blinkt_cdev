use anyhow::Result;
use blinkt_cdev::*;
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

pub fn main() -> Result<()> {
    let mut blinkt = Blinkt::new()?;
    let mut brightness: f32 = 0.0;
    let mut brightness_change = 0.1;
    let mut rng = rand::thread_rng();
    for pixel in 0..8 {
        blinkt.set_pixel(pixel, rng.gen(), rng.gen(), rng.gen(), brightness);
        blinkt.show()?;
    }
    for _ in 0..100 {
        if brightness == 1.0 {
            brightness_change = -0.1;
        } else if brightness == 0.0 {
            brightness_change = 0.1;
        }
        brightness += brightness_change;
        println!("{}", brightness);
        for pixel in 0..8 {
            let (red, green, blue, _) = blinkt.get_pixel(pixel)?;
            blinkt.set_pixel(pixel, red, green, blue, brightness)
        }
        blinkt.show()?;
        sleep(Duration::from_millis(250));
    }

    Ok(())
}
