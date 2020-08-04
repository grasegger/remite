use std::fs::File;
use std::io::prelude::*;
use serde_yaml::Value;

fn main() {
    //println!("cargo:rerun-if-changed=manifest.yml");
    manifest().ok();   
}

fn manifest () -> std::io::Result<()>{
    let mut file = File::open("manifest.yaml")?;
    let mut content = String::new();
    
    file.read_to_string(&mut content).ok();

    let data : Value = serde_yaml::from_str(&content).unwrap();

    let json = serde_json::to_string(&data).unwrap();    

    let mut file = File::create("build/manifest.json")?;
    file.write_all(&json.as_bytes())?;
    Ok(())
}
