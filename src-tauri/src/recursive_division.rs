use rand::prelude::*;

pub fn recursive_division_maze(width: usize, height: usize) -> Vec<Vec<usize>> {
    let mut maze = vec![vec![0; width]; height];
    let mut rng = thread_rng();
    let mut stack = vec![(1, 1)];

    while let Some((x, y)) = stack.pop() {
        maze[y][x] = 1;

        let directions: Vec<(isize, isize)> = vec![
            (-2, 0), // Up
            (0, -2), // Left
            (2, 0),  // Down
            (0, 2),  // Right
        ];

        let mut dir_rng = directions.clone();
        dir_rng.shuffle(&mut rng);

        for (dx, dy) in dir_rng {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 && nx < width as isize && ny < height as isize {
                let nx = nx as usize;
                let ny = ny as usize;

                if maze[ny][nx] == 0 {
                    let (mx, my) = ((x + nx) / 2, (y + ny) / 2);
                    maze[my][mx] = 1;
                    stack.push((nx, ny));
                }
            }
        }
    }

    maze
}