use serde_yaml::Value;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use wasm_pack::command::build::*;
use std::process::Command;
use std::fs;

fn main() {
    npm_dependencies();
    milligram();
    packages();
    manifest().ok();
    icons();
}

fn milligram () {
    fs::copy("node_modules/milligram/dist/milligram.min.css", "build/lib/milligram.css").unwrap();
}

fn packages() {
    package("background");
    package("options");
}

fn npm_dependencies() {
    Command::new("sh")
            .arg("-c")
            .arg("npm i")
            .output()
            .expect("failed to execute process");
}

fn package(package_name: &str) {
    let options = BuildOptions {
        disable_dts: true,
        target: Target::NoModules,
        out_dir: format!("../../build/{}", package_name),
        out_name: Some(package_name.to_string()),
        path: Some(PathBuf::from(format!("packages/remite-{}", package_name))),
        dev: cfg!(debug_assertions),
        release: !cfg!(debug_assertions),
        ..Default::default()
    };

    let mut build = Build::try_from_opts(options).unwrap();
    build.run().expect("Was not able to build.");
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
    original
        .save(format!("build/icons/remite-{}-light.png", size))
        .unwrap();
}
