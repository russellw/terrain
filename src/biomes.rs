use crate::{TerrainCell, BiomeType};

pub struct BiomeAssigner;

impl BiomeAssigner {
    pub fn new() -> Self {
        Self
    }
    
    pub fn assign_biomes(&self, cells: &mut Vec<Vec<TerrainCell>>) {
        for row in cells.iter_mut() {
            for cell in row.iter_mut() {
                if cell.is_water {
                    cell.biome = BiomeType::Ocean;
                } else {
                    cell.biome = self.determine_biome(cell);
                }
            }
        }
        
        self.add_beaches(cells);
    }
    
    fn determine_biome(&self, cell: &TerrainCell) -> BiomeType {
        let temp = cell.temperature;
        let rainfall = cell.rainfall;
        let elevation = cell.elevation;
        
        if elevation > 1.5 {
            return BiomeType::Mountain;
        }
        
        if temp < -5.0 {
            return BiomeType::Tundra;
        }
        
        if rainfall < 2.0 {
            if temp > 20.0 {
                BiomeType::Desert
            } else {
                BiomeType::Grassland
            }
        } else if rainfall > 10.0 && temp > 20.0 {
            BiomeType::Rainforest
        } else if rainfall > 5.0 {
            BiomeType::Forest
        } else {
            BiomeType::Grassland
        }
    }
    
    fn add_beaches(&self, cells: &mut Vec<Vec<TerrainCell>>) {
        let height = cells.len();
        let width = cells[0].len();
        
        for y in 0..height {
            for x in 0..width {
                if !cells[y][x].is_water && cells[y][x].elevation < 0.3 {
                    if self.is_adjacent_to_water(x, y, cells) {
                        cells[y][x].biome = BiomeType::Beach;
                    }
                }
            }
        }
    }
    
    fn is_adjacent_to_water(&self, x: usize, y: usize, cells: &[Vec<TerrainCell>]) -> bool {
        let height = cells.len();
        let width = cells[0].len();
        
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 { continue; }
                
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                    if cells[ny as usize][nx as usize].is_water {
                        return true;
                    }
                }
            }
        }
        
        false
    }
}