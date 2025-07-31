#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Item {
    // Raw Materials
    Copper,
    Iron,
    Coal,
    Water,
    CrudeOil,
    Stone,

    // Furnace
    IronPlate,
    CopperPlate,
    Steel,
    Brick,

    // Oil stuff
    Sulfur,
    Petroleum,
    Plastic,
    SulfuricAcid,

    // Components
    Cog,
    CopperWire,
    Inserter,
    Belt,
    Pipe,
    Engine,
    R1Circuit,
    R2Circuit,
    IronStick,
    Battery,

    // Military
    Ammo,
    PiercingAmmo,
    Grenade,
    Wall,
    Concrete,

    // Usable stuff
    SolarPanel,
    Rail,
    ElectricFurnace,
    ProdModule,
    Accumulator,

    // Science
    AutoScience,
    LogiScience,
    ChemScience,
    ProdScience,
    MilScience,
}
