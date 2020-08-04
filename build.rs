use serde_yaml::Value;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    manifest().ok();
    icons();
}

fn manifest() -> std::io::Result<()> {
    let mut file = File::open("manifest.yaml")?;
    let mut content = String::new();
    file.read_to_string(&mut content).ok();

    let data: Value = serde_yaml::from_str(&content).unwrap();

    let json = serde_json::to_string(&data).unwrap();

    let mut file = File::create("build/manifest.json")?;
    file.write_all(&json.as_bytes())?;
    Ok(())
}

fn icons() {
    to_png(32);
    to_png(48);
    to_png(96);
    invert_png(32);
}

fn to_png(size: u32) {
    let opt = usvg::Options::default();
    let rtree = usvg::Tree::from_file("icon.svg", &opt).unwrap();
    let img = resvg::render(&rtree, usvg::FitTo::Width(size), None).unwrap();
    img.save_png(format!("build/icons/remite-{}.png", size))
        .unwrap();
}

fn invert_png(size: u32) {
        let mut original = image::open(format!("build/icons/remite-{}.png", size)).unwrap();
        original.invert();
        original.save(format!("build/icons/remite-{}-light.png", size)).unwrap();
}
