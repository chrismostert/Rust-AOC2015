use serde_scan::scan;
type Ingredient = (&'static str, isize, isize, isize, isize, isize);

fn get_score(ingredients: &[Ingredient], solution: &[isize]) -> (isize, isize) {
    let (mut cap, mut dur, mut flav, mut tex, mut cals) = (0, 0, 0, 0, 0);
    for (idx, teaspoons) in solution.iter().enumerate() {
        let ingredient = ingredients[idx];
        cap += teaspoons * ingredient.1;
        dur += teaspoons * ingredient.2;
        flav += teaspoons * ingredient.3;
        tex += teaspoons * ingredient.4;
        cals += teaspoons * ingredient.5;
    }
    (cap.max(0) * dur.max(0) * flav.max(0) * tex.max(0), cals)
}

fn get_best_recipe(ingredients: &[Ingredient], teaspoons: usize, calorie_max: isize) -> isize {
    find_max(
        vec![0; ingredients.len()],
        0,
        teaspoons,
        ingredients,
        calorie_max,
    )
}

fn find_max(
    mut solution: Vec<isize>,
    idx: usize,
    teaspoons: usize,
    ingredients: &[Ingredient],
    calorie_max: isize,
) -> isize {
    let (score, calories) = get_score(ingredients, &solution);
    // Over calorie budget, no need to recurse further
    if calories > calorie_max && calorie_max != 0 {
        return 0;
    }
    // Processed last item
    if idx == solution.len() {
        // Valid solution?
        if teaspoons == 0 && (calories == calorie_max || calorie_max == 0) {
            return score;
        }
        return 0;
    }

    (1..=teaspoons)
        .map(|val| {
            solution[idx] = val as isize;
            find_max(
                solution.clone(),
                idx + 1,
                teaspoons - val,
                ingredients,
                calorie_max,
            )
        })
        .max()
        .unwrap_or(0)
}

fn main() {
    let ingredients: Vec<Ingredient> = include_str!("../input.txt")
        .lines()
        .map(|line| -> Ingredient {
            scan!("{}: capacity {}, durability {}, flavor {}, texture {}, calories {}" <- line)
                .unwrap()
        })
        .collect();

    let p1 = get_best_recipe(&ingredients, 100, 0);
    println!("Part 1: {}", p1);

    let p2 = get_best_recipe(&ingredients, 100, 500);
    println!("Part 2: {}", p2);
}
