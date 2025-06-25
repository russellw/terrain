use crate::{TerrainCell, BiomeType};

pub struct BiomeAssigner;

impl BiomeAssigner {
    pub fn new() -> Self {
        Self
    }
    
    pub fn assign_biomes(&self, cells: &mut Vec<Vec<TerrainCell>>) {
        // First pass: basic biome assignment
        for row in cells.iter_mut() {
            for cell in row.iter_mut() {
                if cell.is_water {
                    cell.biome = BiomeType::Ocean;
                } else {
                    cell.biome = self.determine_biome(cell);
                }
            }
        }
        
        // Second pass: smooth transitions and add special features
        self.smooth_biome_transitions(cells);
        self.add_beaches(cells);
        self.enhance_coastal_features(cells);
    }
    
    fn determine_biome(&self, cell: &TerrainCell) -> BiomeType {
        let temp = cell.temperature;
        let rainfall = cell.rainfall;
        let elevation = cell.elevation;
        
        // More nuanced elevation-based biomes
        if elevation > 2.0 {
            return BiomeType::Mountain;
        }
        
        if elevation > 1.5 && temp < 5.0 {
            return BiomeType::Tundra;
        }
        
        if temp < -5.0 {
            return BiomeType::Tundra;
        }
        
        // Improved biome logic with better transitions
        if rainfall < 1.5 {
            if temp > 25.0 {
                BiomeType::Desert
            } else if temp > 10.0 {
                BiomeType::Grassland
            } else {
                BiomeType::Tundra
            }
        } else if rainfall > 12.0 && temp > 22.0 {
            BiomeType::Rainforest
        } else if rainfall > 6.0 && temp > 5.0 {
            BiomeType::Forest
        } else if rainfall > 3.0 {
            BiomeType::Grassland
        } else {
            if temp > 15.0 {
                BiomeType::Grassland
            } else {
                BiomeType::Tundra
            }
        }
    }
    
    fn smooth_biome_transitions(&self, cells: &mut Vec<Vec<TerrainCell>>) {
        let height = cells.len();
        let width = cells[0].len();
        let mut new_biomes = vec![vec![BiomeType::Ocean; width]; height];
        
        // Copy current biomes
        for y in 0..height {
            for x in 0..width {
                new_biomes[y][x] = cells[y][x].biome;
            }
        }
        
        // Smooth non-water biomes (but preserve rivers)
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                if !cells[y][x].is_water && !cells[y][x].has_river {
                    let neighbors = self.get_neighbor_biomes(x, y, cells);
                    let current_biome = cells[y][x].biome;
                    
                    // If surrounded by different biomes, consider transition
                    let different_neighbors = neighbors.iter()
                        .filter(|&&biome| biome != current_biome && biome != BiomeType::Ocean)
                        .count();
                    
                    if different_neighbors >= 4 {
                        // Find most common non-ocean neighbor biome
                        if let Some(common_biome) = self.most_common_biome(&neighbors) {
                            if common_biome != BiomeType::Ocean {
                                new_biomes[y][x] = common_biome;
                            }
                        }
                    }
                }
            }
        }
        
        // Apply smoothed biomes (but preserve rivers)
        for y in 0..height {
            for x in 0..width {
                if !cells[y][x].is_water && !cells[y][x].has_river {
                    cells[y][x].biome = new_biomes[y][x];
                }
            }
        }
    }
    
    fn get_neighbor_biomes(&self, x: usize, y: usize, cells: &[Vec<TerrainCell>]) -> Vec<BiomeType> {
        let mut neighbors = Vec::new();
        
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 { continue; }
                
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if nx >= 0 && nx < cells[0].len() as i32 && ny >= 0 && ny < cells.len() as i32 {
                    neighbors.push(cells[ny as usize][nx as usize].biome);
                }
            }
        }
        
        neighbors
    }
    
    fn most_common_biome(&self, biomes: &[BiomeType]) -> Option<BiomeType> {
        use std::collections::HashMap;
        
        let mut counts = HashMap::new();
        for &biome in biomes {
            *counts.entry(biome).or_insert(0) += 1;
        }
        
        counts.into_iter()
            .filter(|(biome, _)| *biome != BiomeType::Ocean)
            .max_by_key(|(_, count)| *count)
            .map(|(biome, _)| biome)
    }
    
    fn enhance_coastal_features(&self, cells: &mut Vec<Vec<TerrainCell>>) {
        let height = cells.len();
        let width = cells[0].len();
        
        for y in 0..height {
            for x in 0..width {
                if !cells[y][x].is_water && cells[y][x].elevation < 0.4 {
                    if self.is_adjacent_to_water(x, y, cells) {
                        // Create more diverse coastal biomes
                        let temp = cells[y][x].temperature;
                        let rainfall = cells[y][x].rainfall;
                        
                        if temp > 20.0 && rainfall < 3.0 {
                            cells[y][x].biome = BiomeType::Beach;
                        } else if temp > 15.0 && rainfall > 8.0 {
                            // Coastal forest/swamp
                            cells[y][x].biome = BiomeType::Forest;
                        } else {
                            cells[y][x].biome = BiomeType::Beach;
                        }
                    }
                }
            }
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