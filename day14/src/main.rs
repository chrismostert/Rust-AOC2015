use serde_scan::scan;
type ReindeerTuple = (&'static str, usize, usize, usize);

#[derive(Debug)]
struct Reindeer {
    speed: usize,
    fly_max: usize,
    fly_cur: usize,
    rest_max: usize,
    rest_cur: usize,
    distance_flown: usize,
    tournament_points: usize,
}

impl Reindeer {
    fn new(fields: &ReindeerTuple) -> Self {
        Reindeer {
            speed: fields.1,
            fly_max: fields.2,
            fly_cur: fields.2,
            rest_max: fields.3,
            rest_cur: fields.3,
            distance_flown: 0,
            tournament_points: 0,
        }
    }

    fn fly(&mut self) {
        if self.fly_cur > 0 {
            self.distance_flown += self.speed;
            self.fly_cur -= 1;
            return;
        }
        self.rest_cur -= 1;
        if self.rest_cur == 0 {
            self.rest_cur = self.rest_max;
            self.fly_cur = self.fly_max;
        }
    }
}

fn main() {
    let mut input: Vec<Reindeer> = include_str!("../input.txt")
        .lines()
        .map(|line| -> ReindeerTuple {
            scan!("{} can fly {} km/s for {} seconds, but then must rest for {} seconds." <- line)
                .unwrap()
        })
        .map(|reindeer_tuple| Reindeer::new(&reindeer_tuple))
        .collect();

    for _ in 0..2503 {
        // Fly each reindeer
        for r in input.iter_mut() {
            r.fly();
        }
        // Maximum score
        let score_max = input.iter().map(|r| r.distance_flown).max().unwrap();

        // Award tournament points based on max score
        for r in input.iter_mut() {
            if r.distance_flown == score_max {
                r.tournament_points += 1;
            }
        }
    }

    let p1 = input.iter().map(|r| r.distance_flown).max().unwrap();
    println!("Part 1: {}", p1);

    let p2 = input.iter().map(|r| r.tournament_points).max().unwrap();
    println!("Part 2: {}", p2);
}
