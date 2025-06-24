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
        
        for y in 1..self.height as usize - 1 {
            for x in 1..self.width as usize - 1 {
                let cell = &cells[y][x];
                
                // Rivers start in mountains with high rainfall
                if !cell.is_water && cell.elevation > 1.0 && cell.rainfall > 6.0 {
                    // Check if this is a good watershed point (high elevation relative to surroundings)
                    let avg_neighbor_elevation = self.get_average_neighbor_elevation(x, y, cells);
                    
                    if cell.elevation > avg_neighbor_elevation + 0.2 {
                        sources.push((x, y));
                    }
                }
            }
        }
        
        sources
    }
    
    fn get_average_neighbor_elevation(&self, x: usize, y: usize, cells: &[Vec<TerrainCell>]) -> f32 {
        let mut total = 0.0;
        let mut count = 0;
        
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 { continue; }
                
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                    total += cells[ny as usize][nx as usize].elevation;
                    count += 1;
                }
            }
        }
        
        total / count as f32
    }
    
    
    fn trace_river(&self, start_x: usize, start_y: usize, cells: &mut Vec<Vec<TerrainCell>>) {
        let mut current_x = start_x;
        let mut current_y = start_y;
        let mut visited = std::collections::HashSet::new();
        let mut flow_volume = 1.0; // Start with small flow
        
        loop {
            if visited.contains(&(current_x, current_y)) {
                break;
            }
            
            visited.insert((current_x, current_y));
            
            if cells[current_y][current_x].is_water {
                break;
            }
            
            // Only mark as river if flow is significant enough
            if flow_volume > 0.5 {
                cells[current_y][current_x].has_river = true;
                cells[current_y][current_x].biome = BiomeType::River;
            }
            
            // Add flow from local rainfall and nearby rivers
            flow_volume += cells[current_y][current_x].rainfall * 0.1;
            flow_volume += self.count_tributary_flow(current_x, current_y, cells) * 0.2;
            
            if let Some((next_x, next_y)) = self.find_best_flow_direction(current_x, current_y, cells, flow_volume) {
                current_x = next_x;
                current_y = next_y;
            } else {
                break;
            }
            
            if visited.len() > 2000 {
                break;
            }
        }
    }
    
    fn count_tributary_flow(&self, x: usize, y: usize, cells: &[Vec<TerrainCell>]) -> f32 {
        let mut flow = 0.0;
        
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 { continue; }
                
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                    let neighbor = &cells[ny as usize][nx as usize];
                    if neighbor.has_river && neighbor.elevation > cells[y][x].elevation {
                        flow += 1.0;
                    }
                }
            }
        }
        
        flow
    }
    
    fn find_best_flow_direction(&self, x: usize, y: usize, cells: &[Vec<TerrainCell>], flow_volume: f32) -> Option<(usize, usize)> {
        let mut best_score = f32::INFINITY;
        let mut best_pos = None;
        let current_elevation = cells[y][x].elevation;
        
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 { continue; }
                
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                    let neighbor_elevation = cells[ny as usize][nx as usize].elevation;
                    
                    if neighbor_elevation < current_elevation {
                        // Calculate flow preference based on elevation drop and some randomness for meandering
                        let elevation_drop = current_elevation - neighbor_elevation;
                        let distance = ((dx * dx + dy * dy) as f32).sqrt(); // Diagonal penalty
                        
                        // Add some random meandering for larger rivers
                        let meander_factor = if flow_volume > 2.0 {
                            use std::collections::hash_map::DefaultHasher;
                            use std::hash::{Hash, Hasher};
                            
                            let mut hasher = DefaultHasher::new();
                            (x, y, nx, ny).hash(&mut hasher);
                            let hash_val = hasher.finish() as f32 / u64::MAX as f32;
                            (hash_val - 0.5) * 0.3 // Random factor between -0.15 and 0.15
                        } else {
                            0.0
                        };
                        
                        let score = distance / (elevation_drop + 0.1) - meander_factor;
                        
                        if score < best_score {
                            best_score = score;
                            best_pos = Some((nx as usize, ny as usize));
                        }
                    }
                }
            }
        }
        
        best_pos
    }
    
}