use pix::{
    Raster,
    chan::{Ch8, Channel, Srgb, Straight},
    el::{Pix4, Pixel},
    rgb::Rgb,
};
use png_pong::{Decoder, Step};
use std::{
    fs::{self, File},
    io::Write,
};

type Raster8 = Raster<Pix4<Ch8, Rgb, Straight, Srgb>>;

pub fn path_for(name: &str, ext: &str) -> String {
    let path = format!("assets/{}.{}", name, ext);

    path
}

pub fn load_raster(name: &str) -> Raster8 {
    let path = path_for(name, "png");
    let data: &[u8] = &fs::read(path).expect("Failed to open PNG");
    let decoder = Decoder::new(data).expect("Not PNG").into_steps();
    let Step { raster, .. } = decoder
        .last()
        .expect("No frames in PNG")
        .expect("PNG parsing error");

    let rgba8 = match raster {
        png_pong::PngRaster::Rgba8(raster) => Some(raster),
        _ => None,
    }
    .expect("Not Rgba8 PNG");

    rgba8
}

pub fn alpha_to_565(alpha: f32) -> u16 {
    let value = (255. * alpha) as u8;
    rgb565::Rgb565::from_rgb888_components(value, value, value).to_rgb565()
}

pub fn create_raw(raster: Raster8) -> Vec<u8> {
    let width = raster.width();
    let height = raster.height();

    let mut buffer = Vec::<u8>::new();

    for x in 0..width {
        for y in 0..height {
            let pixel = raster.pixel(x.try_into().unwrap(), y.try_into().unwrap());
            let alpha = pixel.alpha().to_f32();
            let value = alpha_to_565(alpha);
            let [a, b] = value.to_le_bytes();
            buffer.push(a);
            buffer.push(b);
        }
    }

    buffer
}

pub fn write_raw(name: &str, buffer: &Vec<u8>) -> () {
    let mut file = File::create(path_for(name, "raw")).expect("Should be able to create file");
    file.write_all(buffer).expect("Raw write failed");
}

pub fn process(name: &str) -> () {
    let raster = load_raster(name);
    let buffer = create_raw(raster);
    write_raw(name, &buffer);
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    process("tabler-icon-sun");
}
