use std::collections::HashMap;

use itertools::Itertools;
struct Grid {
    lights: HashMap<(usize, usize), bool>,
    corners: Option<[(usize, usize); 4]>,
}

impl Grid {
    fn new(grid_string: &str, broken_corners: bool) -> Self {
        let corners = if broken_corners {
            let height = grid_string.lines().count();
            let width = grid_string.lines().next().unwrap().chars().count();
            Some([
                (0, 0),
                (width - 1, 0),
                (0, height - 1),
                (height - 1, height - 1),
            ])
        } else {
            None
        };

        let mut lights =
            grid_string
                .lines()
                .enumerate()
                .fold(HashMap::new(), |mut acc, (y, row)| {
                    for (x, c) in row.chars().enumerate() {
                        acc.insert(
                            (x, y),
                            match c {
                                '#' => true,
                                '.' => false,
                                _ => unreachable!(),
                            },
                        );
                    }
                    acc
                });

        if let Some(coords) = corners {
            for tup in coords {
                lights.insert(tup, true);
            }
        }

        Grid { lights, corners }
    }

    fn n_neighs_on(&self, x_check: usize, y_check: usize) -> usize {
        (x_check.saturating_sub(1)..=x_check + 1)
            .cartesian_product(y_check.saturating_sub(1)..=y_check + 1)
            .filter(|&coord| {
                coord != (x_check, y_check) && *self.lights.get(&coord).unwrap_or(&false)
            })
            .count()
    }

    fn step(&mut self) {
        self.lights = self
            .lights
            .iter()
            .map(|(&(x, y), &val)| {
                if let Some(coords) = self.corners {
                    if coords.contains(&(x, y)) {
                        return ((x, y), true);
                    }
                }
                (
                    (x, y),
                    match (val, self.n_neighs_on(x, y)) {
                        (true, 2 | 3) => true,
                        (true, _) => false,
                        (false, 3) => true,
                        (false, _) => false,
                    },
                )
            })
            .collect();
    }

    fn step_n_times(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    fn n_on(&self) -> usize {
        self.lights.values().filter(|&&v| v).count()
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let mut grid = Grid::new(input, false);
    grid.step_n_times(100);
    println!("Part 1: {}", grid.n_on());

    let mut grid_broken = Grid::new(input, true);
    grid_broken.step_n_times(100);
    println!("Part 2: {}", grid_broken.n_on());
}
