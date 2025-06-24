use crate::TerrainData;
use image::{ImageBuffer, Rgb, RgbImage};
use std::fs::File;
use std::io::Write;

pub fn export_png(terrain: &TerrainData, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut img: RgbImage = ImageBuffer::new(terrain.width, terrain.height);
    
    for y in 0..terrain.height {
        for x in 0..terrain.width {
            let cell = &terrain.cells[y as usize][x as usize];
            let slope = calculate_slope(terrain, x as usize, y as usize);
            let color = get_realistic_terrain_color(cell, slope);
            img.put_pixel(x, y, color);
        }
    }
    
    img.save(filename)?;
    Ok(())
}

fn calculate_slope(terrain: &TerrainData, x: usize, y: usize) -> f32 {
    let current_elevation = terrain.cells[y][x].elevation;
    let mut max_slope: f32 = 0.0;
    
    for dy in -1i32..=1 {
        for dx in -1i32..=1 {
            if dx == 0 && dy == 0 { continue; }
            
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            
            if nx >= 0 && nx < terrain.width as i32 && ny >= 0 && ny < terrain.height as i32 {
                let neighbor_elevation = terrain.cells[ny as usize][nx as usize].elevation;
                let elevation_diff = (current_elevation - neighbor_elevation).abs();
                let distance = ((dx * dx + dy * dy) as f32).sqrt();
                let slope = elevation_diff / distance;
                max_slope = max_slope.max(slope);
            }
        }
    }
    
    max_slope
}

fn get_realistic_terrain_color(cell: &crate::TerrainCell, slope: f32) -> Rgb<u8> {
    if cell.is_water {
        return get_water_color(cell.elevation);
    }
    
    if cell.has_river {
        return get_river_color(cell.elevation);
    }
    
    // Calculate vegetation density based on rainfall, temperature, and elevation
    let vegetation_density = calculate_vegetation_density(cell);
    
    // Get base terrain color based on elevation and moisture
    let base_color = get_base_terrain_color(cell, vegetation_density);
    
    // Apply elevation shading
    let shaded_color = apply_elevation_shading(base_color, cell.elevation, slope);
    
    shaded_color
}

fn get_water_color(elevation: f32) -> Rgb<u8> {
    let depth_factor = (1.0 - elevation.max(0.0)).min(1.0);
    let blue_intensity = (30 + (depth_factor * 80.0) as u8).min(120);
    let green_component = (15 + (depth_factor * 40.0) as u8).min(60);
    Rgb([0, green_component, blue_intensity])
}

fn get_river_color(elevation: f32) -> Rgb<u8> {
    let flow_factor = (1.0 - elevation * 0.3).max(0.3);
    let blue = (100.0 + flow_factor * 100.0) as u8;
    Rgb([20, 80, blue])
}

fn calculate_vegetation_density(cell: &crate::TerrainCell) -> f32 {
    let temp_factor = if cell.temperature > -5.0 && cell.temperature < 40.0 {
        let optimal_temp = 20.0;
        1.0 - (cell.temperature - optimal_temp).abs() / 30.0
    } else {
        0.0
    }.max(0.0);
    
    let rainfall_factor = (cell.rainfall / 15.0).min(1.0);
    let elevation_factor = (1.0 - (cell.elevation / 3.0)).max(0.0);
    
    (temp_factor * rainfall_factor * elevation_factor).max(0.0).min(1.0)
}

fn get_base_terrain_color(cell: &crate::TerrainCell, vegetation_density: f32) -> Rgb<u8> {
    let elevation = cell.elevation;
    let temperature = cell.temperature;
    let rainfall = cell.rainfall;
    
    // High elevation - rocky/snowy
    if elevation > 2.0 {
        let snow_factor = ((elevation - 2.0) / 1.0).min(1.0);
        let rock_gray = 120;
        let snow_white = 240;
        let gray_value = (rock_gray as f32 + (snow_white - rock_gray) as f32 * snow_factor) as u8;
        return Rgb([gray_value, gray_value, gray_value.saturating_sub(10)]);
    }
    
    // Very cold - tundra/ice
    if temperature < -5.0 {
        let ice_factor = ((-5.0 - temperature) / 20.0).min(1.0);
        let tundra_brown = [160, 140, 120];
        let ice_color = [220, 230, 255];
        return interpolate_color(tundra_brown, ice_color, ice_factor);
    }
    
    // Desert conditions
    if rainfall < 2.0 && temperature > 15.0 {
        let aridity = (1.0 - rainfall / 2.0).min(1.0);
        let dry_grass = [180, 160, 100];
        let sand = [220, 200, 140];
        return interpolate_color(dry_grass, sand, aridity);
    }
    
    // Vegetation-based coloring
    if vegetation_density > 0.1 {
        get_vegetation_color(vegetation_density, temperature, rainfall)
    } else {
        // Bare ground/rock
        let soil_color = if rainfall > 5.0 {
            [140, 120, 90]  // Dark soil
        } else {
            [180, 160, 120] // Light/sandy soil
        };
        Rgb(soil_color)
    }
}

fn get_vegetation_color(density: f32, temperature: f32, rainfall: f32) -> Rgb<u8> {
    // Dense vegetation colors
    let rainforest_green = [20, 80, 20];      // Dark green
    let temperate_forest = [40, 120, 40];     // Medium green  
    let grassland = [80, 140, 60];            // Light green
    let dry_shrub = [120, 140, 80];           // Yellow-green
    let sparse_vegetation = [140, 120, 80];   // Brown-green
    
    // Determine vegetation type based on climate
    let base_color = if rainfall > 12.0 && temperature > 20.0 {
        rainforest_green
    } else if rainfall > 6.0 && temperature > 5.0 {
        temperate_forest
    } else if rainfall > 3.0 {
        grassland
    } else if rainfall > 1.0 {
        dry_shrub
    } else {
        sparse_vegetation
    };
    
    // Mix with brown soil based on vegetation density
    let soil_color = [120, 100, 70];
    interpolate_color(soil_color, base_color, density)
}

fn apply_elevation_shading(base_color: Rgb<u8>, elevation: f32, slope: f32) -> Rgb<u8> {
    // Calculate shading based on elevation (higher = brighter) and slope (steeper = darker)
    let elevation_brightness = (elevation * 0.2).min(0.4); // Subtle elevation effect
    let slope_darkness = (slope * 0.3).min(0.3);           // Slope shadowing
    
    let net_brightness = elevation_brightness - slope_darkness;
    
    let r = (base_color[0] as f32 * (1.0 + net_brightness)).clamp(0.0, 255.0) as u8;
    let g = (base_color[1] as f32 * (1.0 + net_brightness)).clamp(0.0, 255.0) as u8;
    let b = (base_color[2] as f32 * (1.0 + net_brightness)).clamp(0.0, 255.0) as u8;
    
    Rgb([r, g, b])
}

fn interpolate_color(color1: [u8; 3], color2: [u8; 3], factor: f32) -> Rgb<u8> {
    let factor = factor.clamp(0.0, 1.0);
    let r = (color1[0] as f32 + (color2[0] as f32 - color1[0] as f32) * factor) as u8;
    let g = (color1[1] as f32 + (color2[1] as f32 - color1[1] as f32) * factor) as u8;
    let b = (color1[2] as f32 + (color2[2] as f32 - color1[2] as f32) * factor) as u8;
    Rgb([r, g, b])
}

pub fn export_json(terrain: &TerrainData, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json_data = serde_json::to_string_pretty(terrain)?;
    let mut file = File::create(filename)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}