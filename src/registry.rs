extern crate serde_yaml;

use Recipe;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeRegistry {
    version: String,
    recipes: HashMap<String, Recipe>,
}

impl RecipeRegistry {
    pub fn new() -> RecipeRegistry {
        return RecipeRegistry {
            version: String::new(),
            recipes: HashMap::new(),
        };
    }

    pub fn from_yaml_file(yaml_filename: &str) -> RecipeRegistry {
        // Open database file and read its content.
        let mut yaml_file = File::open(yaml_filename)
            .expect("Database file not found.");

        // Read file to string.
        let mut yaml_str = String::new();
        yaml_file.read_to_string(&mut yaml_str)
            .expect("Something went wrong reading the file.");

        // Parse yaml.
        let registry: RecipeRegistry = serde_yaml::from_str(&yaml_str).unwrap();

        println!("Loaded database version {}", registry.version);
        println!("Loaded {} recipies", registry.recipes.len());
        registry
    }

    pub fn add_recipe(&mut self, recipe: Recipe) {
        let name = recipe.name.clone();
        self.recipes.insert(name, recipe);
    }

    pub fn get(&self, name: &str) -> &Recipe {
        return &self.recipes[name];
    }
}
