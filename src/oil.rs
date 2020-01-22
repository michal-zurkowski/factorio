use std::collections::HashMap;

#[derive(Debug)]
pub struct Processor {
    name: String,
    input_per_sec: HashMap<String, f64>,
    output_per_sec: HashMap<String, f64>,
}

pub fn apply_modules(
    processor: &mut Processor,
    productivity_modules: i32,
    speed_modules: i32,
    crafting_speed: f64,
) {
    let crafting_speed = crafting_speed
        * (1.0 + 0.5 * (speed_modules as f64) - 0.15 * (productivity_modules as f64));
    let productivity_multiplier = 1.0 + 0.1 * (productivity_modules as f64);
    for (_item, mut per_sec) in &mut processor.input_per_sec {
        *per_sec *= crafting_speed;
    }
    for (_item, mut per_sec) in &mut processor.output_per_sec {
        *per_sec *= crafting_speed * productivity_multiplier;
    }
}

pub fn advanced_oil_processing(productivity_modules: i32, speed_modules: i32) -> Processor {
    let mut processor = Processor {
        name: String::from("oil-refinery(advanced-oil-processing)"), // I am not sure about the name in parentheses
        input_per_sec: [
            (String::from("crude-oil"), 100.0 / 5.0),
            (String::from("water"), 50.0 / 5.0),
        ].iter()
            .cloned()
            .collect(),
        output_per_sec: [
            (String::from("heavy-oil"), 10.0 / 5.0),
            (String::from("light-oil"), 45.0 / 5.0),
            (String::from("petroleum-gas"), 55.0 / 5.0),
        ].iter()
            .cloned()
            .collect(),
    };
    apply_modules(&mut processor, productivity_modules, speed_modules, 1.0);
    processor
}

pub fn heavy_oil_cracking(productivity_modules: i32, speed_modules: i32) -> Processor {
    let mut processor = Processor {
        name: String::from("chemical-plant(heavy-oil-cracking)"), // I am not sure about the name in parentheses
        input_per_sec: [
            (String::from("heavy-oil"), 40.0 / 3.0),
            (String::from("water"), 30.0 / 3.0),
        ].iter()
            .cloned()
            .collect(),
        output_per_sec: [(String::from("light-oil"), 30.0 / 3.0)]
            .iter()
            .cloned()
            .collect(),
    };
    apply_modules(&mut processor, productivity_modules, speed_modules, 1.25);
    processor
}

pub fn light_oil_cracking(productivity_modules: i32, speed_modules: i32) -> Processor {
    let mut processor = Processor {
        name: String::from("chemical-plant(light-oil-cracking)"), // I am not sure about the name in parentheses
        input_per_sec: [
            (String::from("light-oil"), 30.0 / 3.0),
            (String::from("water"), 30.0 / 3.0),
        ].iter()
            .cloned()
            .collect(),
        output_per_sec: [(String::from("petroleum-gas"), 20.0 / 3.0)]
            .iter()
            .cloned()
            .collect(),
    };
    apply_modules(&mut processor, productivity_modules, speed_modules, 1.25);
    processor
}

pub fn plastic_bar(productivity_modules: i32, speed_modules: i32) -> Processor {
    let mut processor = Processor {
        name: String::from("chemical-plant(plastic-bar)"), // I am not sure about the name in parentheses
        input_per_sec: [
            (String::from("coal"), 1.0 / 1.0),
            (String::from("petroleum-gas"), 20.0 / 1.0),
        ].iter()
            .cloned()
            .collect(),
        output_per_sec: [(String::from("plastic-bar"), 2.0 / 1.0)]
            .iter()
            .cloned()
            .collect(),
    };
    apply_modules(&mut processor, productivity_modules, speed_modules, 1.25);
    processor
}

pub fn process() {
    // TODO: this method is all hardcoded and should be made more generic.

    let advanced = advanced_oil_processing(3, 12);
    let heavy = heavy_oil_cracking(3, 12);
    let light = light_oil_cracking(3, 12);
    // TODO: Not sure if we can actually have so many speed modules here, probably not, have to change
    let plastic = plastic_bar(3, 12);

    // This is the number of oil refinaries we want to build (change this number).
    let oil_refinery_count = 15.0;
    let oil_per_sec = advanced.input_per_sec["crude-oil"] * oil_refinery_count;

    println!(
        "Assuming that we want to build {} oil refineries, which will process {} oil / s",
        oil_refinery_count, oil_per_sec
    );

    let heavy_per_sec = advanced.output_per_sec["heavy-oil"] * oil_refinery_count;
    let mut light_per_sec = advanced.output_per_sec["light-oil"] * oil_refinery_count;
    let mut petroleum_per_sec = advanced.output_per_sec["petroleum-gas"] * oil_refinery_count;

    let heavy_count = heavy_per_sec / heavy.input_per_sec["heavy-oil"];
    println!(
        "Need to build {} chemical plants with heavy oil cracking.",
        heavy_count
    );
    light_per_sec += heavy_count * heavy.output_per_sec["light-oil"];

    let light_count = light_per_sec / light.input_per_sec["light-oil"];
    println!(
        "Need to build {} chemical plants with light oil cracking.",
        light_count
    );
    petroleum_per_sec += light_count * light.output_per_sec["petroleum-gas"];

    let plastic_count = petroleum_per_sec / plastic.input_per_sec["petroleum-gas"];
    println!(
        "Need to build {} chemical plants with plastic.",
        plastic_count
    );
    println!(
        "Will require {} coal / s",
        plastic_count * plastic.input_per_sec["coal"]
    );
    println!(
        "Will produce {} plastic / s",
        plastic_count * plastic.output_per_sec["plastic-bar"]
    );
}
