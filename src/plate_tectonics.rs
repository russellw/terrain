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
        
        for i in 0..count {
            let center_x = self.rng.gen_range(0.0..self.width as f32);
            let center_y = self.rng.gen_range(0.0..self.height as f32);
            
            let velocity_x = self.rng.gen_range(-2.0..2.0);
            let velocity_y = self.rng.gen_range(-2.0..2.0);
            
            let plate_type = if self.rng.gen_bool(0.3) {
                PlateType::Continental
            } else {
                PlateType::Oceanic
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
                let noise_value = self.noise.get([
                    x as f64 / 100.0,
                    y as f64 / 100.0,
                ]) as f32;
                
                let base_elevation = noise_value * 0.5 + 0.5;
                cells[y as usize][x as usize].elevation += base_elevation;
            }
        }
    }
    
    fn add_mountain_ranges(&self, cells: &mut Vec<Vec<TerrainCell>>, plates: &[TectonicPlate]) {
        for y in 0..self.height {
            for x in 0..self.width {
                let plate_id = cells[y as usize][x as usize].plate_id;
                let plate = &plates[plate_id];
                
                if matches!(plate.plate_type, PlateType::Continental) {
                    let mountain_noise = self.noise.get([
                        x as f64 / 50.0,
                        y as f64 / 50.0,
                        1.0,
                    ]) as f32;
                    
                    if mountain_noise > 0.3 {
                        cells[y as usize][x as usize].elevation += (mountain_noise - 0.3) * 2.0;
                    }
                }
            }
        }
    }
}