# Ternary Shield — Containment and Shielding on Ternary Grids

**Ternary Shield** implements shielding, containment, and breach analysis on 2D ternary grids. It identifies fully-surrounded regions (shielded cells), detects breach points (weak borders), reinforces protective barriers (cellular automaton growth), and measures shield permeability — the ratio of boundary edges to total shield area.

## Why It Matters

Spatial protection is fundamental in security, defense, and system reliability. On a ternary grid where values represent {enemy (-1), neutral (0), friendly (+1)}, shielding analysis reveals which friendly territories are secure, which are vulnerable, and where reinforcements should go. This maps directly to fleet security: which GPU nodes are surrounded by healthy peers (shielded), which are exposed to failure propagation (breach points), and where adding nodes would most improve resilience.

## How It Works

### Shield Detection

`shield(grid, width, protect)` finds cells matching the `protect` value that are completely surrounded by same-valued neighbors. A cell is shielded if all 4 orthogonal neighbors share its value. Interior cells of large homogeneous regions are shielded; boundary cells are not. O(N) for N cells.

### Breach Detection

`breach(grid, width, protect)` identifies weak points: cells matching `protect` that have 1-2 non-protect neighbors. These are the most vulnerable shield cells — small breaches that could expand. Cells with 3+ non-protect neighbors are considered breached (not just breached-able). O(N).

### Reinforcement

`reinforce(grid, width, protect)` is a cellular automaton step: for each non-protect cell, if the majority of neighbors are protect, flip it to protect. This grows the protected region, filling gaps. Returns count of cells reinforced. O(N) per step. Multiple iterations create progressive expansion.

### Permeability

`permeability(grid, width, protect)` measures how "leaky" the shield is:

```
P = boundary_edges / (2 × protect_cells × avg_neighbors)
```

Where `boundary_edges` are edges between protect and non-protect cells. P = 0 means perfect shield (no leaks); P = 1 means no shield (all boundary). O(N) computation.

### Neighbor Computation

Uses 4-connected (von Neumann) neighborhood. Edge and corner cells have fewer neighbors. The `get_neighbors()` function handles boundary conditions.

## Quick Start

```rust
use ternary_shield::{shield, breach, reinforce, permeability};

let mut grid: Vec<i8> = vec![
    0,  0,  0,  0,  0,
    0,  1,  1,  1,  0,
    0,  1,  1,  1,  0,
    0,  1,  1,  1,  0,
    0,  0,  0,  0,  0,
];
let width = 5;
let protect: i8 = 1;

// Detect shielded cells
let shields = shield(&grid, width, protect);
// Center cell (2,2) is fully surrounded → shields[12] = 1

// Find weak points
let breaches = breach(&grid, width, protect);

// Measure permeability
let p = permeability(&grid, width, protect);
println!("Shield permeability: {:.2}", p);
```

```bash
cargo add ternary-shield
```

## API

| Type / Function | Description |
|---|---|
| `shield(grid, width, protect) → Vec<i8>` | Detect fully-surrounded cells |
| `breach(grid, width, protect) → Vec<(usize, usize)>` | Find weak border cells |
| `reinforce(grid, width, protect) → usize` | CA growth step (O(N)) |
| `permeability(grid, width, protect) → f64` | Shield leakiness metric |

## Architecture Notes

Shielding models fleet resilience in **SuperInstance**. Healthy GPU nodes (+1) form protective clusters; failed nodes (-1) are threats that breach the shield. The γ + η = C conservation maps to the permeability metric: γ = shielded region strength, η = boundary leakage. A well-shielded fleet has low permeability (high γ, low η). See [Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

- Wolfram, Stephen. *A New Kind of Science*, Wolfram Media, 2002 — cellular automata.
| Gardner, Martin. "Mathematical Games," *Scientific American*, 223(4), 1970 — Game of Life.
| Gruau, Frédéric. "Neural Network Synthesis Using Cellular Encoding," *IWGM*, 1994.

## License

MIT
