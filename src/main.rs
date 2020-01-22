extern crate base64;
extern crate factorio;
extern crate flate2;
extern crate serde_json;

use factorio::Recipe;
use factorio::blueprint_string_to_json;
use factorio::registry::RecipeRegistry;

use std::collections::HashMap;

#[derive(Debug)]
struct Producer {
    name: String,
    output_name: String,
    output_per_sec: f64,
    input_per_sec: HashMap<String, f64>,
}

fn assembly_machine_3(recipe: &Recipe, productivity_modules: i32, speed_modules: i32) -> Producer {
    let crafting_speed =
        1.25 * (1.0 + 0.5 * (speed_modules as f64) - 0.15 * (productivity_modules as f64));
    let items_per_sec = crafting_speed / (recipe.time_sec as f64)
        * (1.0 + 0.1 * (productivity_modules as f64))
        * (recipe.output as f64);
    let mut input_per_sec: HashMap<String, f64> = HashMap::new();
    for (item, amount) in &recipe.input {
        input_per_sec.insert(
            item.clone(),
            crafting_speed / (recipe.time_sec as f64) * (*amount as f64),
        );
    }
    return Producer {
        name: format!("assembly-machine-3({})", recipe.name),
        output_name: recipe.name.clone(),
        output_per_sec: items_per_sec,
        input_per_sec: input_per_sec,
    };
}

fn electric_furnace(recipe: &Recipe, productivity_modules: i32, speed_modules: i32) -> Producer {
    let crafting_speed =
        2.0 * (1.0 + 0.5 * (speed_modules as f64) - 0.15 * (productivity_modules as f64));
    let items_per_sec = crafting_speed / (recipe.time_sec as f64)
        * (1.0 + 0.1 * (productivity_modules as f64))
        * (recipe.output as f64);
    let mut input_per_sec: HashMap<String, f64> = HashMap::new();
    for (item, amount) in &recipe.input {
        input_per_sec.insert(
            item.clone(),
            crafting_speed / (recipe.time_sec as f64) * (*amount as f64),
        );
    }
    return Producer {
        name: format!("electric-furnace({})", recipe.name),
        output_name: recipe.name.clone(),
        output_per_sec: items_per_sec,
        input_per_sec: input_per_sec,
    };
}

fn combine_producers(producers: &[Producer], min_output: f64) {
    let mut demand = HashMap::new();
    demand.insert(
        producers[0].output_name.clone(),
        producers[0].output_per_sec * (min_output / producers[0].output_per_sec).ceil(),
    );
    for ref producer in producers {
        let demand_per_sec = demand[&producer.output_name];
        println!(
            "Will produce {} {} / s",
            demand_per_sec, &producer.output_name
        );
        let producer_count = (demand_per_sec / producer.output_per_sec).ceil();
        println!("Will need {} {:?}", producer_count, producer);
        for (item, amount) in &producer.input_per_sec {
            let x = demand.entry(item.clone()).or_insert(0.0);
            *x += (demand_per_sec / producer.output_per_sec) * amount;
            println!(
                "Demands {} {} / s",
                (demand_per_sec / producer.output_per_sec) * amount,
                item
            );
        }
        demand.remove(&producer.output_name);
    }
    for (item, demand_per_sec) in demand {
        println!("Required {} {} / s", demand_per_sec, item);
    }
}

fn main() {
    let blueprint = "0eNqdVu1qwyAUfZf7245cY/qRVxljJK0UITWiZrSUvPtMs42yWKL3l5jgOeee+6F3aLtBGqu0h/oO6thrB/X7HZw666abvvmbkVCD8vICDHRzmXbyaqx0buNMp7yXFkYGSp/kFWocWfLxIRyxZ9uHddPKzj/B8PGDgdReeSVnSY/N7VMPlzYQ1vhaDAPTu3Cw15OCALYpGdzCgm9VYDgpK4/zXz6J/QfMF8DeNtqZ3vpZ5BKez/B8HbzMUY0ZqsW6uQuCIi6b/SVNm2HKyYKsIpCVK2T94F+wbfMzXaRYtiNE8ZtqzPZsT2crsskO+WWWZBkWGcA/5ZUGjNldh/FMxMA5wXuMWS8SyhVLAlu8rMR6plHQexHzexErcnAUK7fk4ApCbDvq5E+pwT15QOfPGszp/3C9xGhE7GosiF3KE7DzJwCPYW9j2JxcR6s9Ep4pj8dN/fSUYvAlrZvlCMErfqj2Oz6O3wRGL1g=";
    let json_str = blueprint_string_to_json(blueprint).unwrap();

    let _v: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    // println!("{}", v);

    let recipes = RecipeRegistry::from_yaml_file("database_normal.yml");

    let processing_unit = assembly_machine_3(recipes.get("processing-unit"), 4, 12);
    let advanced_circuit = assembly_machine_3(recipes.get("advanced-circuit"), 4, 12);
    let electronic_circuit = assembly_machine_3(recipes.get("electronic-circuit"), 4, 12);
    let copper_cable = assembly_machine_3(recipes.get("copper-cable"), 4, 12);
    let copper_plate = electric_furnace(recipes.get("copper-plate"), 2, 8);
    let iron_plate = electric_furnace(recipes.get("iron-plate"), 2, 8);

    combine_producers(
        &[
            processing_unit,
            advanced_circuit,
            electronic_circuit,
            copper_cable,
            copper_plate,
            iron_plate,
        ],
        13.3,
    );
    println!();

    // Advanced oil processing
    factorio::oil::process();
}
