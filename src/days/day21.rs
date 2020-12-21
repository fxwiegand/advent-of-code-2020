use itertools::Itertools;
use std::collections::HashSet;

pub(crate) fn solve_day21() -> u64 {
    let input_file = include_str!("../resources/day21.txt");
    let sets = input_file
        .lines()
        .map(|n| {
            let tmp = n.split(" (contains ").collect_vec();
            let ingredients: HashSet<_> = tmp[0].split_whitespace().collect();
            let allergens: HashSet<_> = tmp[1].trim_end_matches(')').split(", ").collect();
            (ingredients, allergens)
        })
        .collect_vec();

    let all_ingredients: Vec<_> = input_file
        .lines()
        .map(|n| {
            let tmp = n.split(" (contains ").collect_vec();
            tmp[0].split_whitespace().collect_vec()
        })
        .flatten()
        .collect();

    let all_unique_ingredients: HashSet<_> = all_ingredients.iter().unique().collect();

    let all_allergenes: HashSet<_> = input_file
        .lines()
        .map(|n| {
            let tmp = n.split(" (contains ").collect_vec();
            tmp[1].trim_end_matches(')').split(", ").collect_vec()
        })
        .flatten()
        .unique()
        .collect();

    let mut no_allergenes = Vec::new();

    for ingredient_to_check in all_unique_ingredients.clone() {
        let mut possible_allergenes = all_allergenes.clone();
        for (ingredients, allergens) in &sets {
            if !ingredients.contains(ingredient_to_check) {
                possible_allergenes = possible_allergenes
                    .difference(&allergens)
                    .map(|s| s.to_owned())
                    .collect();
            }
        }
        if possible_allergenes.is_empty() {
            no_allergenes.push(ingredient_to_check);
        }
    }

    let mut res = 0;
    for ingredient in no_allergenes {
        for a in &all_ingredients {
            if ingredient == a {
                res += 1;
            }
        }
    }

    res
}
