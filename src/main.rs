use std::{collections::HashMap, sync::LazyLock};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Item {
    // Raw Materials
    Copper,
    Iron,
    Coal,
    Water,
    CrudeOil,

    IronPlate,
    CopperPlate,
    Cog,
    CopperWire,
    Inserter,
    Belt,
    Plastic,
    Petroleum,
    Steel,
    Pipe,
    Sulfur,
    Engine,
    R1Circuit,
    R2Circuit,
    R1Science,
    R2Science,
    R3Science,
}
struct Recipe {
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
        let items: HashMap<_, f32> = self
            .inputs
            .iter()
            .copied()
            .flat_map(|(item, count)| {
                let recipe = RECIPES
                    .iter()
                    .find(|r| r.out.0 == item)
                    .expect("Recipe not found");

                if recipe.is_raw() {
                    vec![(item, 1.)]
                } else {
                    recipe.get_raw_mats()
                }
                .into_iter()
                // Scale up by how much of the sub-item we need
                .map(|(item, sub_count)| (item, sub_count * count as f32))
                .collect::<Vec<_>>()
            })
            .fold(HashMap::new(), |mut acc, (item, count)| {
                *acc.entry(item).or_default() += count;

                acc
            });

        items
            .into_iter()
            // Normalise per item
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

        let items = self
            .inputs
            .iter()
            .copied()
            .flat_map(|(item, count_per_craft)| {
                // Y*Xi
                let count_per_second = self.crafts_per_second() * count_per_craft as f32;

                let recipe = RECIPES
                    .iter()
                    .find(|r| r.out.0 == item)
                    .expect("Recipe not found");

                // (Y*Xi) / (Yi*Zi)
                let crafters_needed = count_per_second / recipe.items_per_second();

                recipe
                    // Requirements for the crafter at full throttle
                    .get_requirements_tree()
                    .into_iter()
                    // Scaled by how many we need
                    .map(|(item, count)| (item, count * crafters_needed))
                    .collect::<Vec<_>>()
            })
            .fold(HashMap::new(), |mut acc, (item, count)| {
                *acc.entry(item).or_default() += count;

                acc
            });

        std::iter::once((self.out.0, 1.)).chain(items).collect()
    }
}

static RECIPES: LazyLock<Vec<Recipe>> = LazyLock::new(|| {
    use Item::*;
    vec![
        Recipe {
            out: (Cog, 1),
            inputs: vec![(IronPlate, 2)],
            craft_time: 0.5,
        },
        Recipe {
            out: (CopperWire, 2),
            inputs: vec![(CopperPlate, 1)],
            craft_time: 0.5,
        },
        Recipe {
            out: (R1Circuit, 1),
            inputs: vec![(CopperWire, 3), (IronPlate, 1)],
            craft_time: 0.5,
        },
        Recipe {
            out: (Inserter, 1),
            inputs: vec![(R1Circuit, 1), (IronPlate, 1), (Cog, 1)],
            craft_time: 0.5,
        },
        Recipe {
            out: (Belt, 2),
            inputs: vec![(IronPlate, 1), (Cog, 1)],
            craft_time: 0.5,
        },
        Recipe {
            out: (R2Science, 1),
            inputs: vec![(Inserter, 1), (Belt, 1)],
            craft_time: 6.,
        },
        Recipe {
            out: (R1Science, 1),
            inputs: vec![(CopperPlate, 1), (Cog, 1)],
            craft_time: 5.,
        },
        Recipe {
            out: (Plastic, 2),
            inputs: vec![(Coal, 1), (Petroleum, 20)],
            craft_time: 1.,
        },
        Recipe {
            out: (Petroleum, 45),
            inputs: vec![(CrudeOil, 100)],
            craft_time: 5.,
        },
        Recipe {
            out: (R2Circuit, 1),
            inputs: vec![(Plastic, 2), (CopperWire, 4), (R1Circuit, 2)],
            craft_time: 5.,
        },
        Recipe {
            out: (Sulfur, 2),
            inputs: vec![(Water, 30), (Petroleum, 30)],
            craft_time: 1.,
        },
        Recipe {
            out: (Pipe, 1),
            inputs: vec![(IronPlate, 1)],
            craft_time: 0.5,
        },
        Recipe {
            out: (Steel, 1),
            inputs: vec![(IronPlate, 5)],
            craft_time: 16.,
        },
        Recipe {
            out: (Engine, 1),
            inputs: vec![(Steel, 1), (Cog, 1), (Pipe, 2)],
            craft_time: 10.,
        },
        Recipe {
            out: (R3Science, 1),
            inputs: vec![(Sulfur, 1), (R2Circuit, 3), (Engine, 2)],
            craft_time: 24.,
        },
        Recipe {
            out: (Copper, 1),
            inputs: vec![],
            craft_time: 2.,
        },
        Recipe {
            out: (CopperPlate, 1),
            inputs: vec![(Copper, 1)],
            craft_time: 3.2,
        },
        Recipe {
            out: (Iron, 1),
            inputs: vec![],
            craft_time: 2.,
        },
        Recipe {
            out: (IronPlate, 1),
            inputs: vec![(Iron, 1)],
            craft_time: 3.2,
        },
        Recipe {
            out: (Coal, 1),
            inputs: vec![],
            craft_time: 2.,
        },
        Recipe {
            out: (Water, 1200),
            inputs: vec![],
            craft_time: 1.,
        },
        Recipe {
            out: (CrudeOil, 1),
            inputs: vec![],
            craft_time: 1.,
        },
    ]
});

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
