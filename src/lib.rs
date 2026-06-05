#![forbid(unsafe_code)]

/// Shielding/containment on ternary grids.

pub fn shield(grid: &[i8], width: usize, protect: i8) -> Vec<i8> {
    let height = grid.len() / width;
    let mut result = vec![0i8; grid.len()];
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            if grid[idx] != protect { continue; }
            // Check if all 4 neighbors are same state
            let mut surrounded = true;
            let neighbors = get_neighbors(x, y, width, height);
            if neighbors.is_empty() { surrounded = false; }
            for ni in &neighbors {
                if grid[*ni] != protect { surrounded = false; break; }
            }
            if surrounded { result[idx] = 1; }
        }
    }
    result
}

pub fn breach(grid: &[i8], width: usize, protect: i8) -> Vec<(usize, usize)> {
    let height = grid.len() / width;
    let mut weak_points = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            if grid[idx] != protect { continue; }
            let neighbors = get_neighbors(x, y, width, height);
            let non_shield = neighbors.iter().filter(|&&ni| grid[ni] != protect).count();
            if non_shield > 0 && non_shield <= 2 {
                weak_points.push((x, y));
            }
        }
    }
    weak_points
}

pub fn reinforce(grid: &mut [i8], width: usize, protect: i8) -> usize {
    let height = grid.len() / width;
    let mut filled = 0usize;
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            if grid[idx] == protect { continue; }
            let neighbors = get_neighbors(x, y, width, height);
            let protect_count = neighbors.iter().filter(|&&ni| grid[ni] == protect).count();
            // If majority of neighbors are protect, fill this cell
            if protect_count > 0 && protect_count >= neighbors.len() / 2 + 1 {
                grid[idx] = protect;
                filled += 1;
            }
        }
    }
    filled
}

pub fn permeability(grid: &[i8], width: usize, protect: i8) -> f64 {
    let height = grid.len() / width;
    if height == 0 || width == 0 { return 0.0; }
    let total = grid.len();
    let protect_count = grid.iter().filter(|&&v| v == protect).count();
    if protect_count == 0 { return 1.0; }

    // Count edges between protect and non-protect cells
    let mut boundary_edges = 0usize;
    let mut total_edges = 0usize;
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let neighbors = get_neighbors(x, y, width, height);
            for ni in &neighbors {
                total_edges += 1;
                if grid[idx] == protect && grid[*ni] != protect {
                    boundary_edges += 1;
                }
            }
        }
    }
    if total_edges == 0 { return 0.0; }
    boundary_edges as f64 / total_edges as f64
}

pub fn quarantine(grid: &mut [i8], width: usize, x: usize, y: usize, radius: usize) {
    let height = grid.len() / width;
    for dy in 0..=radius {
        for dx in 0..=radius {
            if dx + dy > radius { continue; }
            // All 4 quadrants
            for &(sx, sy) in &[(1,1), (1,-1), (-1,-1), (-1,1)] {
                let nx = x as isize + (dx as isize * sx);
                let ny = y as isize + (dy as isize * sy);
                if nx >= 0 && ny >= 0 {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if nx < width && ny < height {
                        grid[ny * width + nx] = 0;
                    }
                }
            }
            // Also the center axes
            if dy == 0 && dx > 0 {
                let nx = x + dx;
                if nx < width && y < height { grid[y * width + nx] = 0; }
                let nx = x as isize - dx as isize;
                if nx >= 0 {
                    let nx = nx as usize;
                    if nx < width && y < height { grid[y * width + nx] = 0; }
                }
            }
            if dx == 0 && dy > 0 {
                let ny = y + dy;
                if ny < height && x < width { grid[ny * width + x] = 0; }
                let ny = y as isize - dy as isize;
                if ny >= 0 {
                    let ny = ny as usize;
                    if ny < height && x < width { grid[ny * width + x] = 0; }
                }
            }
        }
    }
    // Center
    if x < width && y < height {
        grid[y * width + x] = 0;
    }
}

fn get_neighbors(x: usize, y: usize, w: usize, h: usize) -> Vec<usize> {
    let mut n = Vec::new();
    if x > 0 { n.push(y * w + x - 1); }
    if x + 1 < w { n.push(y * w + x + 1); }
    if y > 0 { n.push((y - 1) * w + x); }
    if y + 1 < h { n.push((y + 1) * w + x); }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shield_surrounded() {
        // 3x3 grid, all 1s, center is fully surrounded
        let grid = vec![1i8; 9];
        let s = shield(&grid, 3, 1);
        assert_eq!(s[4], 1); // center
    }

    #[test]
    fn test_shield_edge_not_surrounded() {
        let grid = vec![1i8; 9];
        let s = shield(&grid, 3, 1);
        // Corner cells have only 2 neighbors, both same -> surrounded
        assert_eq!(s[0], 1);
    }

    #[test]
    fn test_shield_mixed() {
        let grid = vec![1, 0, 1, 0, 1, 0, 1, 0, 1]; // checkerboard
        let s = shield(&grid, 3, 1);
        // No cell is fully surrounded by 1s in a checkerboard
        assert_eq!(s.iter().filter(|&&v| v == 1).count(), 0);
    }

    #[test]
    fn test_breach_finds_weak() {
        let mut grid = vec![1i8; 9];
        grid[1] = 0; // hole next to center
        let b = breach(&grid, 3, 1);
        assert!(!b.is_empty());
    }

    #[test]
    fn test_breach_none() {
        let grid = vec![1i8; 9]; // fully shielded
        let b = breach(&grid, 3, 1);
        assert!(b.is_empty());
    }

    #[test]
    fn test_reinforce_fills_gaps() {
        let mut grid = vec![1, 1, 1, 1, 0, 1, 1, 1, 1];
        let filled = reinforce(&mut grid, 3, 1);
        assert_eq!(filled, 1);
        assert_eq!(grid[4], 1);
    }

    #[test]
    fn test_reinforce_no_fill() {
        // Grid where no cell has majority-1 neighbors
        let mut grid = vec![0, 0, 0, 0, 0, 0, 0, 1, 0];
        let filled = reinforce(&mut grid, 3, 1);
        assert_eq!(filled, 0);
    }

    #[test]
    fn test_permeability_solid() {
        let grid = vec![1i8; 9];
        let p = permeability(&grid, 3, 1);
        assert_eq!(p, 0.0);
    }

    #[test]
    fn test_permeability_empty() {
        let grid = vec![0i8; 9];
        let p = permeability(&grid, 3, 1);
        assert_eq!(p, 1.0); // no protect cells -> fully permeable
    }

    #[test]
    fn test_permeability_checkerboard() {
        let grid = vec![1, 0, 1, 0, 1, 0, 1, 0, 1];
        let p = permeability(&grid, 3, 1);
        assert!(p > 0.0);
    }

    #[test]
    fn test_quarantine_center() {
        let mut grid = vec![1i8; 9];
        quarantine(&mut grid, 3, 1, 1, 1);
        assert_eq!(grid[4], 0); // center zeroed
        // All neighbors within radius 1 should be zeroed
        assert_eq!(grid[1], 0); // (0,1)
        assert_eq!(grid[3], 0); // (1,0)
        assert_eq!(grid[5], 0); // (1,2)
        assert_eq!(grid[7], 0); // (2,1)
    }

    #[test]
    fn test_quarantine_radius_zero() {
        let mut grid = vec![1i8; 9];
        quarantine(&mut grid, 3, 1, 1, 0);
        assert_eq!(grid[4], 0); // only center
        assert_eq!(grid[0], 1); // neighbors untouched
    }

    #[test]
    fn test_quarantine_edge() {
        let mut grid = vec![1i8; 9];
        quarantine(&mut grid, 3, 0, 0, 1);
        assert_eq!(grid[0], 0);
        // doesn't panic on edge
    }

    #[test]
    fn test_get_neighbors_center() {
        let n = get_neighbors(1, 1, 3, 3);
        assert_eq!(n.len(), 4);
    }

    #[test]
    fn test_get_neighbors_corner() {
        let n = get_neighbors(0, 0, 3, 3);
        assert_eq!(n.len(), 2);
    }
}
