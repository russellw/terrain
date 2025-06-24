use crate::TerrainCell;

pub struct ClimateSimulator {
    width: u32,
    height: u32,
}

impl ClimateSimulator {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    
    pub fn simulate(&self, cells: &mut Vec<Vec<TerrainCell>>) {
        self.calculate_temperature(cells);
        self.simulate_prevailing_winds(cells);
        self.calculate_rainfall(cells);
        self.apply_rain_shadows(cells);
    }
    
    fn calculate_temperature(&self, cells: &mut Vec<Vec<TerrainCell>>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let latitude_factor = (y as f32 / self.height as f32 - 0.5).abs();
                let elevation = cells[y as usize][x as usize].elevation;
                
                let base_temp = 30.0 - latitude_factor * 40.0;
                let elevation_cooling = elevation * 6.5;
                
                cells[y as usize][x as usize].temperature = (base_temp - elevation_cooling).max(-20.0);
            }
        }
    }
    
    fn simulate_prevailing_winds(&self, cells: &mut Vec<Vec<TerrainCell>>) {
        for y in 0..self.height {
            let latitude = y as f32 / self.height as f32;
            
            let wind_direction = if latitude < 0.3 {
                1
            } else if latitude < 0.6 {
                -1
            } else {
                1
            };
            
            for x in 0..self.width {
                let moisture = self.calculate_atmospheric_moisture(x, y, cells);
                
                if wind_direction > 0 && x < self.width - 1 {
                    self.transfer_moisture(x, y, x + 1, y, moisture * 0.1, cells);
                } else if wind_direction < 0 && x > 0 {
                    self.transfer_moisture(x, y, x - 1, y, moisture * 0.1, cells);
                }
            }
        }
    }
    
    fn calculate_atmospheric_moisture(&self, x: u32, y: u32, cells: &[Vec<TerrainCell>]) -> f32 {
        let cell = &cells[y as usize][x as usize];
        
        if cell.is_water {
            let temp_factor = (cell.temperature + 20.0) / 50.0;
            temp_factor.clamp(0.1, 1.0) * 10.0
        } else {
            cell.rainfall * 0.1
        }
    }
    
    fn transfer_moisture(&self, _from_x: u32, _from_y: u32, to_x: u32, to_y: u32, 
                        amount: f32, cells: &mut Vec<Vec<TerrainCell>>) {
        if to_x < self.width && to_y < self.height {
            cells[to_y as usize][to_x as usize].rainfall += amount;
        }
    }
    
    fn calculate_rainfall(&self, cells: &mut Vec<Vec<TerrainCell>>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let convection_rainfall = self.calculate_convection_rainfall(x, y, cells);
                let cell = &mut cells[y as usize][x as usize];
                
                if !cell.is_water {
                    let elevation_factor = (1.0 - cell.elevation.min(1.0)).max(0.0);
                    let temperature_factor = if cell.temperature > 0.0 && cell.temperature < 35.0 {
                        1.0 - (cell.temperature - 17.5).abs() / 17.5
                    } else {
                        0.1
                    };
                    
                    cell.rainfall += elevation_factor * temperature_factor * 5.0 + convection_rainfall;
                    cell.rainfall = cell.rainfall.min(20.0);
                }
            }
        }
    }
    
    fn calculate_convection_rainfall(&self, x: u32, y: u32, cells: &[Vec<TerrainCell>]) -> f32 {
        let cell = &cells[y as usize][x as usize];
        
        if cell.temperature > 25.0 {
            let heat_factor = (cell.temperature - 25.0) / 10.0;
            let nearby_water = self.count_nearby_water(x, y, cells) as f32 / 8.0;
            
            heat_factor * nearby_water * 3.0
        } else {
            0.0
        }
    }
    
    fn count_nearby_water(&self, x: u32, y: u32, cells: &[Vec<TerrainCell>]) -> usize {
        let mut count = 0;
        
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 { continue; }
                
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                    if cells[ny as usize][nx as usize].is_water {
                        count += 1;
                    }
                }
            }
        }
        
        count
    }
    
    fn apply_rain_shadows(&self, cells: &mut Vec<Vec<TerrainCell>>) {
        for y in 0..self.height {
            for x in 1..self.width {
                let current_elevation = cells[y as usize][x as usize].elevation;
                let prev_elevation = cells[y as usize][(x - 1) as usize].elevation;
                
                if current_elevation > prev_elevation + 0.3 {
                    let shadow_strength = (current_elevation - prev_elevation) * 0.5;
                    
                    for shadow_x in (x + 1)..self.width.min(x + 5) {
                        let distance_factor = 1.0 / (shadow_x - x) as f32;
                        let reduction = shadow_strength * distance_factor;
                        
                        cells[y as usize][shadow_x as usize].rainfall = 
                            (cells[y as usize][shadow_x as usize].rainfall - reduction).max(0.0);
                    }
                }
            }
        }
    }
}