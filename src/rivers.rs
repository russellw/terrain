use crate::{TerrainCell, BiomeType};

pub struct RiverGenerator {
    width: u32,
    height: u32,
}

impl RiverGenerator {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    
    pub fn generate_rivers(&self, cells: &mut Vec<Vec<TerrainCell>>) {
        let sources = self.find_river_sources(cells);
        
        for source in sources {
            self.trace_river(source.0, source.1, cells);
        }
    }
    
    fn find_river_sources(&self, cells: &[Vec<TerrainCell>]) -> Vec<(usize, usize)> {
        let mut sources = Vec::new();
        
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                let cell = &cells[y][x];
                
                if !cell.is_water && cell.elevation > 0.8 && cell.rainfall > 8.0 {
                    if self.is_local_maximum(x, y, cells) {
                        sources.push((x, y));
                    }
                }
            }
        }
        
        sources
    }
    
    fn is_local_maximum(&self, x: usize, y: usize, cells: &[Vec<TerrainCell>]) -> bool {
        let current_elevation = cells[y][x].elevation;
        
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 { continue; }
                
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                    if cells[ny as usize][nx as usize].elevation > current_elevation {
                        return false;
                    }
                }
            }
        }
        
        true
    }
    
    fn trace_river(&self, start_x: usize, start_y: usize, cells: &mut Vec<Vec<TerrainCell>]) {
        let mut current_x = start_x;
        let mut current_y = start_y;
        let mut visited = std::collections::HashSet::new();
        
        loop {
            if visited.contains(&(current_x, current_y)) {
                break;
            }
            
            visited.insert((current_x, current_y));
            
            if cells[current_y][current_x].is_water {
                break;
            }
            
            cells[current_y][current_x].has_river = true;
            cells[current_y][current_x].biome = BiomeType::River;
            
            if let Some((next_x, next_y)) = self.find_lowest_neighbor(current_x, current_y, cells) {
                current_x = next_x;
                current_y = next_y;
            } else {
                break;
            }
            
            if visited.len() > 1000 {
                break;
            }
        }
    }
    
    fn find_lowest_neighbor(&self, x: usize, y: usize, cells: &[Vec<TerrainCell>]) -> Option<(usize, usize)> {
        let mut lowest_elevation = cells[y][x].elevation;
        let mut lowest_pos = None;
        
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 { continue; }
                
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                    let neighbor_elevation = cells[ny as usize][nx as usize].elevation;
                    
                    if neighbor_elevation < lowest_elevation {
                        lowest_elevation = neighbor_elevation;
                        lowest_pos = Some((nx as usize, ny as usize));
                    }
                }
            }
        }
        
        lowest_pos
    }
}