use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use crate::types::Pair;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    vertex: Pair,
    distance: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dijkstra(grid: Vec<Vec<i32>>, start: Pair, end: Pair) -> Option<(Vec<Pair>, Vec<Pair>)> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut distances = vec![vec![usize::MAX; cols]; rows];
    let mut visited = vec![vec![false; cols]; rows];
    let mut prev: HashMap<Pair, Pair> = HashMap::new();
    let mut visited_cells = Vec::new(); // Store visited cells in order

    let mut pq = BinaryHeap::new();
    pq.push(Node { vertex: start, distance: 0 });

    while let Some(Node { vertex, distance }) = pq.pop() {
        let (i, j) = vertex;
        if visited[i as usize][j as usize] {
            continue;
        }
        visited[i as usize][j as usize] = true;
        visited_cells.push(vertex); // Add visited cell to the list

        if vertex == end {
            // Reconstruct the path
            let mut path = Vec::new();
            let mut current = end;
            while current != start {
                path.push(current);
                current = prev[&current];
            }
            path.push(start);
            path.reverse();

            return Some((path, visited_cells));
        }

        for (di, dj) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let ni = i + di;
            let nj = j + dj;
            if ni >= 0 && ni < rows as i32 && nj >= 0 && nj < cols as i32 && grid[ni as usize][nj as usize] == 1 {
                let cost = distance + grid[ni as usize][nj as usize] as usize;
                if cost < distances[ni as usize][nj as usize] {
                    distances[ni as usize][nj as usize] = cost;
                    pq.push(Node { vertex: (ni, nj), distance: cost });
                    prev.insert((ni, nj), vertex);
                }
            }
        }
    }

    None
}
