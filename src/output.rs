use crate::{TerrainData, BiomeType};
use image::{ImageBuffer, Rgb, RgbImage};
use std::fs::File;
use std::io::Write;

pub fn export_png(terrain: &TerrainData, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut img: RgbImage = ImageBuffer::new(terrain.width, terrain.height);
    
    for y in 0..terrain.height {
        for x in 0..terrain.width {
            let cell = &terrain.cells[y as usize][x as usize];
            let color = get_biome_color(&cell.biome, cell.elevation);
            img.put_pixel(x, y, color);
        }
    }
    
    img.save(filename)?;
    Ok(())
}

fn get_biome_color(biome: &BiomeType, elevation: f32) -> Rgb<u8> {
    match biome {
        BiomeType::Ocean => {
            let depth_factor = (1.0 - elevation.max(0.0)).min(1.0);
            let blue_intensity = (20 + (depth_factor * 100.0) as u8).min(120);
            Rgb([0, 50, blue_intensity])
        },
        BiomeType::Desert => Rgb([194, 178, 128]),
        BiomeType::Grassland => Rgb([124, 252, 0]),
        BiomeType::Forest => Rgb([34, 139, 34]),
        BiomeType::Tundra => Rgb([176, 196, 222]),
        BiomeType::Mountain => {
            let height_factor = elevation.min(2.0) / 2.0;
            let gray_value = (128 + height_factor * 100.0) as u8;
            Rgb([gray_value, gray_value, gray_value])
        },
        BiomeType::River => Rgb([0, 191, 255]),
        BiomeType::Beach => Rgb([238, 203, 173]),
        BiomeType::Rainforest => Rgb([0, 100, 0]),
    }
}

pub fn export_json(terrain: &TerrainData, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json_data = serde_json::to_string_pretty(terrain)?;
    let mut file = File::create(filename)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}