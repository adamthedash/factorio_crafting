pub mod items;
pub mod recipes;

use crate::recipes::RECIPES;
use clap::Parser;
use std::collections::HashMap;

use crate::items::Item;

#[derive(Parser)]
#[command(name = "recipe-analyzer")]
#[command(about = "Analyze crafting recipes and their dependencies")]
struct Args {
    /// Item name to analyze (if not provided, analyzes all items)
    #[arg(short, long)]
    item: Option<String>,
}

#[derive(Debug)]
pub struct RequirementNode {
    pub item: Item,
    pub crafters_needed: f32,
    pub dependencies: Vec<RequirementNode>,
}

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
    fn get_required_crafters(&self) -> Vec<(Item, f32)> {
        // Get the hierarchical tree first
        let pretty_tree = self.get_requirements_tree();

        // Recursive function to traverse the tree and collect all requirements
        fn collect_requirements(node: &RequirementNode, acc: &mut HashMap<Item, f32>) {
            // Add this node's requirement
            *acc.entry(node.item).or_default() += node.crafters_needed;

            // Recursively collect from all dependencies
            for dep in &node.dependencies {
                collect_requirements(dep, acc);
            }
        }

        // Flatten the tree into a HashMap, aggregating duplicate items
        let mut items = HashMap::new();
        collect_requirements(&pretty_tree, &mut items);

        // Convert HashMap back to Vec
        items.into_iter().collect()
    }

    /// Get requirements tree with preserved hierarchy
    /// Returns a tree structure showing dependencies between crafters
    fn get_requirements_tree(&self) -> RequirementNode {
        let dependencies = self
            .inputs
            .iter()
            .copied()
            .map(|(item, count_per_craft)| {
                // Calculate how many of this input item we consume per second
                let count_per_second = self.crafts_per_second() * count_per_craft as f32;

                // Find the recipe that produces this input item
                let recipe = RECIPES
                    .iter()
                    .find(|r| r.out.0 == item)
                    .expect("Recipe not found");

                // Calculate how many crafters we need for this dependency
                let crafters_needed = count_per_second / recipe.items_per_second();

                // Recursively get the tree for this dependency
                let mut node = recipe.get_requirements_tree();
                node.crafters_needed = crafters_needed;
                node
            })
            .collect();

        RequirementNode {
            item: self.out.0,
            crafters_needed: 1.0,
            dependencies,
        }
    }
}

fn print_tree(node: &RequirementNode, indent: usize, is_last: bool, prefix: String) {
    // Print the current node with appropriate tree characters
    let connector = if indent == 0 {
        ""
    } else if is_last {
        "└─ "
    } else {
        "├─ "
    };

    println!(
        "{}{}{:.3}x {:?}",
        prefix, connector, node.crafters_needed, node.item
    );

    // Prepare prefix for children
    let child_prefix = if indent == 0 {
        String::new()
    } else {
        format!("{}{}   ", prefix, if is_last { " " } else { "│" })
    };

    // Print all dependencies
    for (i, dep) in node.dependencies.iter().enumerate() {
        let is_last_child = i == node.dependencies.len() - 1;
        print_tree(dep, indent + 1, is_last_child, child_prefix.clone());
    }
}

fn print_recipe_details(recipe: &Recipe) {
    println!("Item: {:?}", recipe.out.0);

    println!("Raw materials:");
    recipe.get_raw_mats().iter().for_each(|(item, amount)| {
        println!("   {amount:.3}x\t{item:?}");
    });
    println!();

    println!("Crafting tree:");
    let pretty_tree = recipe.get_requirements_tree();
    print_tree(&pretty_tree, 0, true, String::new());
    println!();

    let tree = recipe.get_required_crafters();
    println!("Total Crafters:");
    tree.iter().for_each(|(item, ratio)| {
        println!("   {ratio:.3}x\t{item:?}");
    });
}

fn main() {
    let args = Args::parse();

    if let Some(item_name) = args.item {
        // Find the specific recipe
        if let Some(recipe) = RECIPES.iter().find(|r| {
            format!("{:?}", r.out.0)
                .to_lowercase()
                .starts_with(item_name.to_lowercase().as_str())
        }) {
            print_recipe_details(recipe);
        } else {
            eprintln!("Item '{}' not found!", item_name);
            eprintln!("Available items:");
            RECIPES.iter().for_each(|r| {
                eprintln!("  {:?}", r.out.0);
            });
            std::process::exit(1);
        }
    } else {
        // Print all recipes
        RECIPES.iter().for_each(|recipe| {
            print_recipe_details(recipe);
            println!("================================================");
        });
    }
}
