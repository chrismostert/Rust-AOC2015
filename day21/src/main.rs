use itertools::iproduct;

type Loadout<'a> = (&'a Item, &'a Item, &'a Item, &'a Item);

struct Item {
    name: &'static str,
    cost: isize,
    damage: isize,
    armor: isize,
}

struct Boss {
    hp: isize,
    damage: isize,
    armor: isize,
}

fn lines_to_items(lines: &[&'static str], can_be_none: bool) -> Vec<Item> {
    let mut res = lines
        .iter()
        .map(|line| {
            let mut parts = line.split_whitespace();
            Item {
                name: parts.next().unwrap(),
                cost: parts.next().unwrap().parse().unwrap(),
                damage: parts.next().unwrap().parse().unwrap(),
                armor: parts.next().unwrap().parse().unwrap(),
            }
        })
        .collect::<Vec<Item>>();

    if can_be_none {
        res.push(Item {
            name: "None",
            cost: 0,
            damage: 0,
            armor: 0,
        })
    }

    res
}

fn check_win(player_hp: usize, loadout: Loadout, boss: &Boss) -> (bool, isize) {
    let damage_p = Ord::max(
        loadout.0.damage + loadout.1.damage + loadout.2.damage + loadout.3.damage - boss.armor,
        1,
    );
    let armor_p = loadout.0.armor + loadout.1.armor + loadout.2.armor + loadout.3.armor;
    let damage_b = Ord::max(boss.damage - armor_p, 1);

    let player_ttk = (boss.hp as f32 / damage_p as f32).ceil();
    let boss_ttk = (player_hp as f32 / damage_b as f32).ceil();

    (
        player_ttk <= boss_ttk,
        loadout.0.cost + loadout.1.cost + loadout.2.cost + loadout.3.cost,
    )
}

fn main() {
    // Puzzle input
    let shop = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    let boss = Boss {
        hp: 103,
        damage: 9,
        armor: 2,
    };

    let weapons = lines_to_items(&shop[1..6], false);
    let armor = lines_to_items(&shop[8..13], true);
    let rings = lines_to_items(&shop[15..], true);

    let battles = iproduct!(&weapons, &armor, &rings, &rings)
        .filter(|(_, _, ring1, ring2)| ring1.name != ring2.name)
        .map(|loadout| check_win(100, loadout, &boss))
        .collect::<Vec<(bool, isize)>>();

    println!(
        "Part 1: {}",
        battles.iter().filter(|(won, _)| *won).min().unwrap().1
    );
    println!(
        "Part 2: {}",
        battles.iter().filter(|(won, _)| !*won).max().unwrap().1
    );
}
