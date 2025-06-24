use crate::{TerrainData, TerrainCell, BiomeType, GenerationParams};
use crate::plate_tectonics::PlateSimulator;
use crate::climate::ClimateSimulator;
use crate::biomes::BiomeAssigner;
use crate::rivers::RiverGenerator;

pub struct TerrainGenerator {
    width: u32,
    height: u32,
    water_percentage: f32,
    seed: u64,
}

impl TerrainGenerator {
    pub fn new(width: u32, height: u32, water_percentage: f32, seed: u64) -> Self {
        Self {
            width,
            height,
            water_percentage,
            seed,
        }
    }
    
    pub fn generate(&mut self) -> TerrainData {
        let mut cells = vec![vec![TerrainCell {
            elevation: 0.0,
            temperature: 15.0,
            rainfall: 0.0,
            plate_id: 0,
            is_water: false,
            biome: BiomeType::Grassland,
            has_river: false,
        }; self.width as usize]; self.height as usize];
        
        let mut plate_sim = PlateSimulator::new(self.width, self.height, self.seed);
        let plates = plate_sim.simulate(&mut cells);
        
        let mut climate_sim = ClimateSimulator::new(self.width, self.height);
        climate_sim.simulate(&mut cells);
        
        self.assign_water_bodies(&mut cells);
        
        let mut biome_assigner = BiomeAssigner::new();
        biome_assigner.assign_biomes(&mut cells);
        
        let mut river_gen = RiverGenerator::new(self.width, self.height);
        river_gen.generate_rivers(&mut cells);
        
        TerrainData {
            width: self.width,
            height: self.height,
            cells,
            plates,
            generation_params: GenerationParams {
                water_percentage: self.water_percentage,
                seed: self.seed,
                plate_count: plates.len(),
            },
        }
    }
    
    fn assign_water_bodies(&self, cells: &mut Vec<Vec<TerrainCell>>) {
        let mut elevations: Vec<f32> = Vec::new();
        
        for row in cells.iter() {
            for cell in row.iter() {
                elevations.push(cell.elevation);
            }
        }
        
        elevations.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let water_threshold_index = (elevations.len() as f32 * self.water_percentage / 100.0) as usize;
        let water_threshold = elevations[water_threshold_index.min(elevations.len() - 1)];
        
        for row in cells.iter_mut() {
            for cell in row.iter_mut() {
                if cell.elevation <= water_threshold {
                    cell.is_water = true;
                    cell.biome = BiomeType::Ocean;
                }
            }
        }
    }
}