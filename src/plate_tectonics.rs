use crate::{TerrainCell, TectonicPlate, PlateType};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use noise::{NoiseFn, Perlin};

pub struct PlateSimulator {
    width: u32,
    height: u32,
    rng: StdRng,
    noise: Perlin,
}

impl PlateSimulator {
    pub fn new(width: u32, height: u32, seed: u64) -> Self {
        Self {
            width,
            height,
            rng: StdRng::seed_from_u64(seed),
            noise: Perlin::new(seed as u32),
        }
    }
    
    pub fn simulate(&mut self, cells: &mut Vec<Vec<TerrainCell>>) -> Vec<TectonicPlate> {
        let plate_count = 6 + self.rng.gen_range(0..4);
        let mut plates = self.generate_plates(plate_count);
        
        self.assign_plate_ownership(cells, &plates);
        self.simulate_plate_interactions(cells, &mut plates);
        self.generate_base_elevation(cells);
        self.add_mountain_ranges(cells, &plates);
        
        plates
    }
    
    fn generate_plates(&mut self, count: usize) -> Vec<TectonicPlate> {
        let mut plates = Vec::new();
        
        // Ensure we have some continental plates spread out
        let continental_count = (count as f32 * 0.4).max(2.0) as usize;
        
        for i in 0..count {
            let (center_x, center_y) = if i < continental_count {
                // Spread continental plates more evenly
                let angle = (i as f32 / continental_count as f32) * 2.0 * std::f32::consts::PI;
                let radius = (self.width.min(self.height) as f32 * 0.3) + self.rng.gen_range(-50.0..50.0);
                let cx = (self.width as f32 * 0.5) + radius * angle.cos();
                let cy = (self.height as f32 * 0.5) + radius * angle.sin();
                (cx.clamp(50.0, self.width as f32 - 50.0), 
                 cy.clamp(50.0, self.height as f32 - 50.0))
            } else {
                (self.rng.gen_range(0.0..self.width as f32),
                 self.rng.gen_range(0.0..self.height as f32))
            };
            
            let velocity_x = self.rng.gen_range(-1.5..1.5);
            let velocity_y = self.rng.gen_range(-1.5..1.5);
            
            let plate_type = if i < continental_count {
                PlateType::Continental
            } else {
                if self.rng.gen_bool(0.2) {
                    PlateType::Continental
                } else {
                    PlateType::Oceanic
                }
            };
            
            plates.push(TectonicPlate {
                id: i,
                center: (center_x, center_y),
                velocity: (velocity_x, velocity_y),
                age: self.rng.gen_range(0.0..100.0),
                plate_type,
            });
        }
        
        plates
    }
    
    fn assign_plate_ownership(&self, cells: &mut Vec<Vec<TerrainCell>>, plates: &[TectonicPlate]) {
        for y in 0..self.height {
            for x in 0..self.width {
                let mut closest_plate = 0;
                let mut min_distance = f32::INFINITY;
                
                for plate in plates {
                    let dx = x as f32 - plate.center.0;
                    let dy = y as f32 - plate.center.1;
                    let distance = (dx * dx + dy * dy).sqrt();
                    
                    if distance < min_distance {
                        min_distance = distance;
                        closest_plate = plate.id;
                    }
                }
                
                cells[y as usize][x as usize].plate_id = closest_plate;
            }
        }
    }
    
    fn simulate_plate_interactions(&self, cells: &mut Vec<Vec<TerrainCell>>, plates: &mut [TectonicPlate]) {
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let current_plate = cells[y as usize][x as usize].plate_id;
                
                let neighbors = [
                    cells[(y - 1) as usize][x as usize].plate_id,
                    cells[(y + 1) as usize][x as usize].plate_id,
                    cells[y as usize][(x - 1) as usize].plate_id,
                    cells[y as usize][(x + 1) as usize].plate_id,
                ];
                
                for &neighbor_plate in &neighbors {
                    if neighbor_plate != current_plate {
                        let interaction_strength = self.calculate_interaction_strength(
                            &plates[current_plate], 
                            &plates[neighbor_plate]
                        );
                        
                        cells[y as usize][x as usize].elevation += interaction_strength;
                    }
                }
            }
        }
    }
    
    fn calculate_interaction_strength(&self, plate1: &TectonicPlate, plate2: &TectonicPlate) -> f32 {
        let vel_diff_x = plate1.velocity.0 - plate2.velocity.0;
        let vel_diff_y = plate1.velocity.1 - plate2.velocity.1;
        let relative_velocity = (vel_diff_x * vel_diff_x + vel_diff_y * vel_diff_y).sqrt();
        
        match (plate1.plate_type, plate2.plate_type) {
            (PlateType::Continental, PlateType::Continental) => relative_velocity * 0.8,
            (PlateType::Continental, PlateType::Oceanic) => relative_velocity * 1.2,
            (PlateType::Oceanic, PlateType::Continental) => relative_velocity * 1.2,
            (PlateType::Oceanic, PlateType::Oceanic) => relative_velocity * 0.4,
        }
    }
    
    fn generate_base_elevation(&self, cells: &mut Vec<Vec<TerrainCell>>) {
        for y in 0..self.height {
            for x in 0..self.width {
                // Multi-octave noise for more detailed terrain
                let large_features = self.noise.get([x as f64 / 200.0, y as f64 / 200.0]) as f32;
                let medium_features = self.noise.get([x as f64 / 100.0, y as f64 / 100.0]) as f32 * 0.5;
                let small_features = self.noise.get([x as f64 / 50.0, y as f64 / 50.0]) as f32 * 0.25;
                
                let combined_noise = large_features + medium_features + small_features;
                let base_elevation = (combined_noise * 0.3 + 0.4).max(0.0);
                
                cells[y as usize][x as usize].elevation = base_elevation;
            }
        }
    }
    
    fn add_mountain_ranges(&self, cells: &mut Vec<Vec<TerrainCell>>, plates: &[TectonicPlate]) {
        // First pass: identify plate boundaries and add mountains there
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let current_plate = cells[y as usize][x as usize].plate_id;
                let current_plate_type = plates[current_plate].plate_type;
                
                // Check if we're at a plate boundary
                let neighbors = [
                    cells[(y - 1) as usize][x as usize].plate_id,
                    cells[(y + 1) as usize][x as usize].plate_id,
                    cells[y as usize][(x - 1) as usize].plate_id,
                    cells[y as usize][(x + 1) as usize].plate_id,
                ];
                
                let is_boundary = neighbors.iter().any(|&neighbor_plate| {
                    neighbor_plate != current_plate && 
                    matches!((current_plate_type, plates[neighbor_plate].plate_type),
                        (PlateType::Continental, PlateType::Continental) |
                        (PlateType::Continental, PlateType::Oceanic) |
                        (PlateType::Oceanic, PlateType::Continental))
                });
                
                if is_boundary {
                    // Add mountains at plate boundaries
                    let mountain_strength = self.noise.get([
                        x as f64 / 30.0,
                        y as f64 / 30.0,
                        2.0,
                    ]) as f32;
                    
                    if mountain_strength > 0.1 {
                        let elevation_boost = (mountain_strength - 0.1) * 1.5;
                        cells[y as usize][x as usize].elevation += elevation_boost;
                    }
                }
                
                // Add some mountains within continental plates too
                if matches!(current_plate_type, PlateType::Continental) {
                    let inland_mountain_noise = self.noise.get([
                        x as f64 / 80.0,
                        y as f64 / 80.0,
                        3.0,
                    ]) as f32;
                    
                    if inland_mountain_noise > 0.4 {
                        cells[y as usize][x as usize].elevation += (inland_mountain_noise - 0.4) * 0.8;
                    }
                }
            }
        }
    }
}