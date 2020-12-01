use anyhow::{anyhow, Context, Result};
use gpio_cdev::*;

const DEFAULT_BRIGHTNESS: u8 = 7;
const RED_INDEX: usize = 0;
const GREEN_INDEX: usize = 1;
const BLUE_INDEX: usize = 2;
const BRIGHTNESS_INDEX: usize = 3;
const DATA_LINE: u32 = 23;
const CLOCK_LINE: u32 = 24;

#[derive(Debug, Copy, Clone)]
struct Pixel {
    values: [u8; 4],
}

impl Pixel {
    pub fn rgb(&self) -> (u8, u8, u8) {
        (
            self.values[RED_INDEX],
            self.values[GREEN_INDEX],
            self.values[BLUE_INDEX],
        )
    }

    pub fn rgbb(&self) -> (u8, u8, u8, f32) {
        (
            self.values[RED_INDEX],
            self.values[GREEN_INDEX],
            self.values[BLUE_INDEX],
            f32::from(0b0001_1111 & self.values[BRIGHTNESS_INDEX]) / 31.0,
        )
    }

    pub fn set_brightness(&mut self, brightness: f32) {
        self.values[BRIGHTNESS_INDEX] = 0b1110_0000 | ((31.0 * brightness.max(0.0).min(1.0)) as u8);
    }

    pub fn set_rgb(&mut self, red: u8, green: u8, blue: u8) {
        self.values[RED_INDEX] = red;
        self.values[GREEN_INDEX] = green;
        self.values[BLUE_INDEX] = blue;
    }

    pub fn set_rgbb(&mut self, red: u8, green: u8, blue: u8, brightness: f32) {
        self.set_rgb(red, green, blue);
        self.set_brightness(brightness);
    }

    pub(crate) fn data(&self) -> &[u8] {
        &self.values
    }

    pub fn clear(&mut self) {
        self.set_rgb(0, 0, 0);
    }
}

impl Default for Pixel {
    fn default() -> Pixel {
        Pixel {
            values: [0, 0, 0, 0b1110_0000 | DEFAULT_BRIGHTNESS],
        }
    }
}

pub struct Blinkt {
    data_output_handle: LineHandle,
    clock_output_handle: LineHandle,
    clear_on_drop: bool,
    pixels: Vec<Pixel>,
}

impl Blinkt {
    pub fn new() -> Result<Blinkt> {
        let mut chip = Chip::new("/dev/gpiochip0").context("Failed to get gpio chip")?;

        let data_output = chip
            .get_line(DATA_LINE)
            .context("Failed to get data line")?;
        let data_output_handle = data_output
            .request(LineRequestFlags::OUTPUT, 0, "data-output")
            .context("Failed to get data output handle")?;

        let clock_output = chip
            .get_line(CLOCK_LINE)
            .context("Failed to get clock line")?;
        let clock_output_handle = clock_output
            .request(LineRequestFlags::OUTPUT, 0, "clock-output")
            .context("Failed to get clock output handle")?;
        Ok(Blinkt {
            data_output_handle,
            clock_output_handle,
            clear_on_drop: true,
            pixels: vec![Pixel::default(); 8],
        })
    }

    fn write_sof(&self) -> Result<()> {
        self.data_output_handle
            .set_value(0)
            .context("Failed to write start of sof")?;
        for _ in 0..32 {
            self.clock_output_handle
                .set_value(1)
                .context("Failed to write sof")?;

            self.clock_output_handle
                .set_value(0)
                .context("Failed to write sof")?;
        }
        Ok(())
    }

    fn write_eof(&self) -> Result<()> {
        self.data_output_handle
            .set_value(0)
            .context("Failed to write start of eof")?;
        for _ in 0..36 {
            self.clock_output_handle
                .set_value(1)
                .context("Failed to write sof")?;

            self.clock_output_handle
                .set_value(0)
                .context("Failed to write sof")?;
        }
        Ok(())
    }

    fn write_byte(&self, mut byte: u8) -> Result<()> {
        for x in 0..8 {
            self.data_output_handle
                .set_value(byte & 0b1000_0000)
                .with_context(|| format!("Failed to write bit \"{:?}\" of byte", x))?;
            self.clock_output_handle
                .set_value(1)
                .context("Failed to start clock pulse of writing bit")?;

            byte <<= 1;
            self.clock_output_handle
                .set_value(0)
                .context("Failed to end clock pulse of writing bit")?;
        }
        Ok(())
    }

    pub fn show(&self) -> Result<()> {
        self.write_sof()?;
        for pixel in &self.pixels {
            let data = pixel.data();
            // write brightness
            self.write_byte(0b1110_0000 | data[3])
                .context("Failed to write brightness byte")?;
            //write blue
            self.write_byte(data[2])
                .context("Failed to write blue byte")?;
            // write green
            self.write_byte(data[1])
                .context("Failed to write green byte")?;
            // write red
            self.write_byte(data[0])
                .context("Failed to write red byte")?;
        }
        self.write_eof()?;
        Ok(())
    }

    pub fn get_pixel(&mut self, pixel: usize) -> Result<(u8, u8, u8, f32)> {
        if let Some(pixel) = self.pixels.get_mut(pixel) {
            return Ok(pixel.rgbb());
        }
        Err(anyhow!("Failed to get data for pixel \"{:?}\"", pixel))
    }

    pub fn set_pixel(&mut self, pixel: usize, red: u8, green: u8, blue: u8, brightness: f32) {
        if let Some(pixel) = self.pixels.get_mut(pixel) {
            pixel.set_rgbb(red, green, blue, brightness);
        }
    }

    pub fn set_all_pixels(&mut self, red: u8, green: u8, blue: u8, brightness: f32) {
        for pixel in 0..8 {
            self.set_pixel(pixel, red, green, blue, brightness);
        }
    }

    pub fn clear(&mut self) {
        self.set_all_pixels(0, 0, 0, 0.2);
    }
}

impl Drop for Blinkt {
    fn drop(&mut self) {
        if self.clear_on_drop {
            self.clear();
            let _ = self.show();
        }
    }
}
