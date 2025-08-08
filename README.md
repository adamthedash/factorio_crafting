# Factorio Recipe Analyzer

A command-line tool for analyzing Factorio crafting recipes and their dependencies. This tool helps you calculate raw material requirements and crafter ratios needed to sustain production chains.

## Usage

### Analyze All Items
```bash
cargo run
```

### Analyze a Specific Item
```bash
cargo run -- --item steel
cargo run -- -i copper
```

### Scale for Multiple Crafters
```bash
# Plan for 5 steel crafters
cargo run -- --item steel --count 5

# Plan for 2.5 copper crafters (fractional values supported)
cargo run -- -i copper -n 2.5
```

## Command Line Options

- `-i, --item <ITEM>`: Analyze a specific item (partial matching supported)
- `-n, --count <COUNT>`: Number of crafters to build (default: 1, supports decimals)
- `-h, --help`: Show help information


## Example Output

```
Item: R1Circuit (4x crafters)
Raw materials:
   6.000x       Copper
   4.000x       Iron

Crafting tree:
4.000x R1Circuit
├─ 6.000x CopperWire
│   └─ 25.600x CopperPlate
│       └─ 2.500x Copper
└─ 25.600x IronPlate
    └─ 2.500x Iron

Total Crafters:
   25.600x      IronPlate
   2.500x       Iron
   6.000x       CopperWire
   4.000x       R1Circuit
   25.600x      CopperPlate
   2.500x       Copper
```

## What It Shows

- **Raw Materials**: The fundamental resources needed
- **Crafting Tree**: Visual hierarchy showing which items depend on which others
- **Total Crafters**: How many crafting machines you need for each recipe type
