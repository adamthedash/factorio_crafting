pub mod items;
pub mod recipes;

use crate::recipes::RECIPES;
use std::collections::HashMap;

use crate::items::Item;

pub struct Recipe {
    out: (Item, usize),
    inputs: Vec<(Item, usize)>,
    craft_time: f32,
}

impl Recipe {
    #[inline]
    fn crafts_per_second(&self) -> f32 {
        1. / self.craft_time
    }

    #[inline]
    fn items_per_second(&self) -> f32 {
        self.out.1 as f32 * self.crafts_per_second()
    }

    #[inline]
    fn is_raw(&self) -> bool {
        self.inputs.is_empty()
    }

    // Raw mats per item
    fn get_raw_mats(&self) -> Vec<(Item, f32)> {
        // Create a HashMap to accumulate raw material counts
        let items: HashMap<_, f32> = self
            .inputs
            .iter()
            .copied()
            // For each input item, recursively find its raw materials
            .flat_map(|(item, count)| {
                // Find the recipe that produces this input item
                let recipe = RECIPES
                    .iter()
                    .find(|r| r.out.0 == item)
                    .expect("Recipe not found");

                // If this item is already raw, return it as-is
                if recipe.is_raw() {
                    vec![(item, 1.)]
                } else {
                    // Otherwise, recursively get the raw materials for this item
                    recipe.get_raw_mats()
                }
                .into_iter()
                // Scale up by how much of the sub-item we need for this recipe
                .map(|(item, sub_count)| (item, sub_count * count as f32))
                .collect::<Vec<_>>()
            })
            // Combine all raw materials, summing up duplicate items
            .fold(HashMap::new(), |mut acc, (item, count)| {
                *acc.entry(item).or_default() += count;

                acc
            });

        items
            .into_iter()
            // Normalize the counts per output item (divide by how many items this recipe produces)
            .map(|(item, count)| (item, count / self.out.1 as f32))
            .collect()
    }

    /// Get combined crafting recipes to make this item
    /// Returned value is how many crafts of the sub-components per one of these to keep up
    fn get_requirements_tree(&self) -> Vec<(Item, f32)> {
        // I consume Xi number of items per craft
        // I craft Y crafts per second
        // I produde Z items per craft
        // To craft at full throttle, I need Y*Xi items per second
        // I produce Y*Z items per second
        // To craft at full throttle, I need (Y*Xi) / (Yi*Zi) crafters for item i

        // Process each input item to calculate crafter requirements
        let items = self
            .inputs
            .iter()
            .copied()
            // For each input item, calculate how many crafters we need
            .flat_map(|(item, count_per_craft)| {
                // Calculate how many of this input item we consume per second (Y*Xi)
                let count_per_second = self.crafts_per_second() * count_per_craft as f32;

                // Find the recipe that produces this input item
                let recipe = RECIPES
                    .iter()
                    .find(|r| r.out.0 == item)
                    .expect("Recipe not found");

                // Calculate how many crafters we need: consumption rate / production rate
                let crafters_needed = count_per_second / recipe.items_per_second();

                // Recursively get the requirements for this input item's recipe
                recipe
                    .get_requirements_tree()
                    .into_iter()
                    // Scale the sub-requirements by how many crafters we need
                    .map(|(item, count)| (item, count * crafters_needed))
                    .collect::<Vec<_>>()
            })
            // Combine all requirements, summing up duplicate items
            .fold(HashMap::new(), |mut acc, (item, count)| {
                *acc.entry(item).or_default() += count;

                acc
            });

        // Return this recipe (1 crafter) plus all sub-requirements
        std::iter::once((self.out.0, 1.)).chain(items).collect()
    }
}

fn main() {
    RECIPES.iter().for_each(|r| {
        println!("Item: {:?}", r.out.0);
        println!("Raw materials: {:?}", r.get_raw_mats());
        let tree = r.get_requirements_tree();
        println!("Crafting ratios:");
        tree.iter().for_each(|(item, ratio)| {
            println!("\t{ratio:.3}x\t{item:?}");
        });
        println!("================================================");
    });
}
