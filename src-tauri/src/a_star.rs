use std::collections::{BinaryHeap, HashSet, HashMap};
use std::cmp::Ordering;
use crate::types::Pair;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    position: Pair,
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn heuristic(current: Pair, goal: Pair) -> usize {
    let dx = (goal.0 as isize - current.0 as isize).abs() as usize;
    let dy = (goal.1 as isize - current.1 as isize).abs() as usize;
    dx + dy
}

pub fn astar(grid: Vec<Vec<i32>>, start: Pair, end: Pair) -> Option<(Vec<Pair>, Vec<Pair>)> {
    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();
    let mut visited_cells = Vec::new();
    let mut path = Vec::new();

    g_score.insert(start, 0);
    f_score.insert(start, heuristic(start, end));

    open_set.push(Node { position: start, cost: heuristic(start, end) });

    while let Some(Node { position, cost: _ }) = open_set.pop() {
        if position == end {
            // Reconstruct path
            let mut current = position;
            while let Some(&prev) = came_from.get(&current) {
                path.push(current);
                current = prev;
            }
            path.push(start);
            path.reverse(); // Sort the path
            return Some((path, visited_cells));
        }

        closed_set.insert(position);

        for (dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let neighbor = ((position.0 as isize + dx) as i32, (position.1 as isize + dy) as i32);
            if neighbor.0 >= 0 && neighbor.0 < grid.len() as i32 && neighbor.1 >= 0 && neighbor.1 < grid[0].len() as i32 && grid[neighbor.0 as usize][neighbor.1 as usize] == 1 && !closed_set.contains(&neighbor) {
                let tentative_g_score = g_score.get(&position).unwrap_or(&usize::MAX) + 1;
                if tentative_g_score < *g_score.entry(neighbor).or_insert(usize::MAX) {
                    came_from.insert(neighbor, position);
                    g_score.insert(neighbor, tentative_g_score);
                    f_score.insert(neighbor, tentative_g_score + heuristic(neighbor, end));
                    open_set.push(Node { position: neighbor, cost: tentative_g_score + heuristic(neighbor, end) });
                    visited_cells.push(neighbor);
                }
            }
        }
    }

    None
}
