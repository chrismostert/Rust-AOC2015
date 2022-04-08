use regex::Regex;

struct Command {
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
    ctype: CommandType,
}

enum CommandType {
    On,
    Off,
    Toggle,
}

struct Grid {
    lights: Vec<Vec<usize>>,
}

impl Grid {
    fn new(size: usize) -> Self {
        Grid {
            lights: vec![vec![0; size]; size],
        }
    }

    fn execute(&mut self, command: &Command, part2: bool) {
        for x in (command.x0)..=(command.x1) {
            for y in (command.y0)..=(command.y1) {
                if !part2 {
                    match command.ctype {
                        CommandType::On => self.lights[x][y] = 1,
                        CommandType::Off => self.lights[x][y] = 0,
                        CommandType::Toggle => {
                            self.lights[x][y] = if self.lights[x][y] == 0 { 1 } else { 0 }
                        }
                    }
                } else {
                    match command.ctype {
                        CommandType::On => self.lights[x][y] += 1,
                        CommandType::Off => self.lights[x][y] = self.lights[x][y].saturating_sub(1),
                        CommandType::Toggle => self.lights[x][y] += 2,
                    }
                }
            }
        }
    }

    fn n_on(&self, part2: bool) -> usize {
        if !part2 {
            return self.lights.iter().flatten().filter(|&&x| x == 1).count()
        }
        self.lights.iter().flatten().sum()
    }
}

fn parse_command(command: &str, reg: &Regex) -> Command {
    let groups = reg.captures(command).unwrap();
    let get_coord = |i| -> usize { groups.get(i).unwrap().as_str().parse().unwrap() };
    let (x0, y0, x1, y1): (usize, usize, usize, usize) =
        (get_coord(2), get_coord(3), get_coord(4), get_coord(5));

    Command {
        x0,
        y0,
        x1,
        y1,
        ctype: match groups.get(1).unwrap().as_str() {
            "turn on" => CommandType::On,
            "turn off" => CommandType::Off,
            "toggle" => CommandType::Toggle,
            _ => unreachable!(),
        },
    }
}

fn calc_lights(input: &[Command], gridsize: usize, part2: bool) -> usize {
    input
        .iter()
        .fold(Grid::new(gridsize), |mut acc, c| {
            acc.execute(c, part2);
            acc
        })
        .n_on(part2)
}

fn main() {
    let reg = Regex::new(r"(.+) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let input: Vec<Command> = include_str!("../input.txt")
        .lines()
        .map(|command| parse_command(command, &reg))
        .collect();

    println!("Part 1: {}", calc_lights(&input, 1000, false));
    println!("Part 2: {}", calc_lights(&input, 1000, true));
}
