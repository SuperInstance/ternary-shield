# ternary-shield

**Containment on ternary grids. Find the armor, find the weak points, test the walls.**

A shield is a cell completely surrounded by cells of the same state — an island of stability that nothing can penetrate from the outside. A breach is a cell of the protected state adjacent to a different state — the weak point where an attack could get through.

This crate computes both. Given a ternary grid and a protected state, it identifies shielded regions (safe), breached regions (vulnerable), and provides structural analysis of the containment boundary.

## What's Inside

- **`shield(grid, width, protect)`** — find cells that are fully surrounded by the protected state (marked 1 in output)
- **`breach(grid, width, protect)`** — find vulnerable cells: protected cells with non-protected neighbors
- **`containment_score(grid, width, protect)`** — fraction of protected cells that are fully shielded (0-1)
- **`wall(grid, width)`** — find boundary cells: cells with at least one different-valued neighbor
- **`isolate(grid, width, value)`** — remove non-value cells, leaving only islands of the target value

## Quick Example

```rust
use ternary_shield::*;

// A solid block of +1 cells with one exposed corner
let grid = vec![
    0, 0, 0, 0, 0,
    0, 1, 1, 1, 0,
    0, 1, 1, 1, 0,
    0, 1, 1, 0, 0,  // bottom-right corner is exposed
    0, 0, 0, 0, 0,
];

let shields = shield(&grid, 5, 1); // fully surrounded +1 cells
let breaches = breach(&grid, 5, 1); // exposed +1 cells
let score = containment_score(&grid, 5, 1);

println!("Shielded: {} cells", shields.iter().filter(|&&v| v == 1).count());
println!("Breaches: {} cells", breaches.len());
println!("Containment: {:.0}%", score * 100.0);
```

## The Deeper Truth

**Containment is a topological property.** A fully surrounded cell is topologically *inside* the protected region — it can't be reached from the outside without crossing the boundary. A breached cell is on the *frontier* — it's where the boundary is incomplete. The containment score measures what fraction of the protected population is safe vs. exposed.

This is the spatial analog of trust in agent systems: shielded agents are in the high-trust core, breached agents are on the periphery where they interact with outsiders. The goal is to maximize the shielded fraction while maintaining the breached frontier (you need *some* contact with the outside).

**Use cases:**
- **Territory analysis** — which regions are defensible vs. exposed?
- **Network security** — identify isolated vs. border nodes
- **Cell biology** — membrane integrity of cell clusters
- **Game AI** — defensive positioning and vulnerability assessment
- **Image segmentation** — boundary detection on ternary images

## See Also

- **ternary-morph** — morphological operations (dilation grows shields, erosion creates breaches)
- **ternary-irradiate** — radiation that tests shielding effectiveness
- **ternary-percolation** — can an attacker percolate through the shield?
- **ternary-morph** → `reconstruct` — grow a shield from a seed

## Install

```bash
cargo add ternary-shield
```

## License

MIT
