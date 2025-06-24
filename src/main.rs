use clap::Parser;
use serde::{Deserialize, Serialize};

mod terrain;
mod plate_tectonics;
mod climate;
mod biomes;
mod rivers;
mod output;

use terrain::TerrainGenerator;

#[derive(Parser)]
#[command(name = "terrain-generator")]
#[command(about = "Generate realistic terrain for fictional worlds")]
struct Args {
    #[arg(short, long, default_value = "512")]
    width: u32,
    
    #[arg(short = 'H', long, default_value = "512")]
    height: u32,
    
    #[arg(short = 'p', long, default_value = "30.0")]
    water_percentage: f32,
    
    #[arg(short, long, default_value = "terrain")]
    output: String,
    
    #[arg(long, default_value = "42")]
    seed: u64,
    
    #[arg(long, default_value = "false")]
    json: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainCell {
    pub elevation: f32,
    pub temperature: f32,
    pub rainfall: f32,
    pub plate_id: usize,
    pub is_water: bool,
    pub biome: BiomeType,
    pub has_river: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BiomeType {
    Ocean,
    Desert,
    Grassland,
    Forest,
    Tundra,
    Mountain,
    River,
    Beach,
    Rainforest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TectonicPlate {
    pub id: usize,
    pub center: (f32, f32),
    pub velocity: (f32, f32),
    pub age: f32,
    pub plate_type: PlateType,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PlateType {
    Oceanic,
    Continental,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerrainData {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Vec<TerrainCell>>,
    pub plates: Vec<TectonicPlate>,
    pub generation_params: GenerationParams,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationParams {
    pub water_percentage: f32,
    pub seed: u64,
    pub plate_count: usize,
}

fn main() {
    let args = Args::parse();
    
    let mut generator = TerrainGenerator::new(
        args.width,
        args.height,
        args.water_percentage,
        args.seed,
    );
    
    println!("Generating terrain...");
    let terrain_data = generator.generate();
    
    println!("Exporting PNG image...");
    output::export_png(&terrain_data, &format!("{}.png", args.output))
        .expect("Failed to export PNG");
    
    if args.json {
        println!("Exporting JSON data...");
        output::export_json(&terrain_data, &format!("{}.json", args.output))
            .expect("Failed to export JSON");
    }
    
    println!("Terrain generation complete!");
}