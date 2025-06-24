#!/usr/bin/env python3

import os
import sys

def check_file_exists(path):
    if os.path.exists(path):
        print(f"✓ {path} exists")
        return True
    else:
        print(f"✗ {path} missing")
        return False

def validate_project():
    files_to_check = [
        "Cargo.toml",
        "src/main.rs", 
        "src/terrain.rs",
        "src/plate_tectonics.rs",
        "src/climate.rs",
        "src/biomes.rs", 
        "src/rivers.rs",
        "src/output.rs"
    ]
    
    all_good = True
    print("Validating terrain generator project structure:")
    
    for file_path in files_to_check:
        if not check_file_exists(file_path):
            all_good = False
    
    if all_good:
        print("\n✓ All required files are present!")
        print("✓ Project structure is complete!")
        
        with open("Cargo.toml", "r") as f:
            cargo_content = f.read()
            if "clap" in cargo_content and "image" in cargo_content:
                print("✓ Dependencies are configured!")
            else:
                print("✗ Some dependencies may be missing")
                
    else:
        print("\n✗ Some files are missing!")
        return False
        
    return True

if __name__ == "__main__":
    validate_project()