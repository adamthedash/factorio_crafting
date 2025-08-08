use std::sync::LazyLock;

use crate::{Recipe, items::Item};

pub static RECIPES: LazyLock<Vec<Recipe>> = LazyLock::new(|| {
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
            out: (LogiScience, 1),
            inputs: vec![(Inserter, 1), (Belt, 1)],
            craft_time: 6.,
        },
        Recipe {
            out: (AutoScience, 1),
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
            out: (ChemScience, 2),
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
            out: (Stone, 1),
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
        Recipe {
            out: (SolarPanel, 1),
            inputs: vec![(CopperPlate, 5), (Steel, 5), (R1Circuit, 15)],
            craft_time: 10.,
        },
        Recipe {
            out: (ElectricFurnace, 1),
            inputs: vec![(R2Circuit, 5), (Steel, 10), (Brick, 10)],
            craft_time: 5.,
        },
        Recipe {
            out: (Brick, 1),
            inputs: vec![(Stone, 2)],
            craft_time: 3.2,
        },
        Recipe {
            out: (IronStick, 2),
            inputs: vec![(IronPlate, 1)],
            craft_time: 0.5,
        },
        Recipe {
            out: (Rail, 2),
            inputs: vec![(Stone, 1), (Steel, 1), (IronStick, 1)],
            craft_time: 0.5,
        },
        Recipe {
            out: (ProdModule, 1),
            inputs: vec![(R1Circuit, 5), (R2Circuit, 5)],
            craft_time: 15.,
        },
        Recipe {
            out: (ProdScience, 3),
            inputs: vec![(Rail, 30), (ElectricFurnace, 1), (ProdModule, 1)],
            craft_time: 21.,
        },
        Recipe {
            out: (SulfuricAcid, 50),
            inputs: vec![(IronPlate, 1), (Sulfur, 5), (Water, 100)],
            craft_time: 1.,
        },
        Recipe {
            out: (Battery, 1),
            inputs: vec![(IronPlate, 1), (CopperPlate, 1), (SulfuricAcid, 30)],
            craft_time: 4.,
        },
        Recipe {
            out: (Accumulator, 1),
            inputs: vec![(IronPlate, 2), (Battery, 5)],
            craft_time: 10.,
        },
        Recipe {
            out: (Ammo, 1),
            inputs: vec![(IronPlate, 4)],
            craft_time: 1.,
        },
        Recipe {
            out: (PiercingAmmo, 2),
            inputs: vec![(Ammo, 2), (Steel, 1), (CopperPlate, 2)],
            craft_time: 6.,
        },
        Recipe {
            out: (Grenade, 1),
            inputs: vec![(Coal, 10), (IronPlate, 5)],
            craft_time: 8.,
        },
        Recipe {
            out: (Wall, 1),
            inputs: vec![(Brick, 5)],
            craft_time: 0.5,
        },
        Recipe {
            out: (MilScience, 2),
            inputs: vec![(PiercingAmmo, 1), (Grenade, 1), (Wall, 2)],
            craft_time: 10.,
        },
        Recipe {
            out: (Concrete, 10),
            inputs: vec![(Iron, 1), (Brick, 5), (Water, 100)],
            craft_time: 10.,
        },
        Recipe {
            out: (LowDensityStructure, 1),
            inputs: vec![(CopperPlate, 20), (Steel, 2), (Plastic, 5)],
            craft_time: 15.,
        },
        Recipe {
            out: (ElectricEngine, 1),
            inputs: vec![(R1Circuit, 2), (Engine, 1), (Lubricant, 15)],
            craft_time: 10.,
        },
        Recipe {
            out: (FlyingRobotFrame, 1),
            inputs: vec![(R1Circuit, 3), (ElectricEngine, 1), (Battery, 2), (Steel, 1)],
            craft_time: 20.,
        },
        Recipe {
            out: (ProcessingUnit, 1),
            inputs: vec![(R1Circuit, 20), (R2Circuit, 2), (SulfuricAcid, 5) ],
            craft_time: 10.,
        },
        Recipe {
            out: (UtilityScience, 3),
            inputs: vec![(ProcessingUnit, 2), (FlyingRobotFrame, 1), (LowDensityStructure, 3) ],
            craft_time: 21.,
        },
        Recipe {
            out: (Lubricant, 10),
            inputs: vec![(HeavyOil, 10)],
            craft_time: 1.,
        },
        Recipe {
            out: (HeavyOil, 25),
            inputs: vec![(Water, 50), (CrudeOil, 100)],
            craft_time: 5.,
        },
    ]
});
