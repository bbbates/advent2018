pub fn solve_part_one(input: &String) -> String {
    let letters = react_polymer(input);

    println!("{:?}", letters);

    letters.len().to_string()
}

#[cfg(test)]
mod part_one_tests {
    use super::*;

    #[test]
    fn two_letter_polymer_with_reaction_leaves_nothing() {
        assert_eq!(solve_part_one(&String::from("Aa")), "0");
    }

    #[test]
    fn four_letter_polymer_with_reactions_leaves_nothing() {
        assert_eq!(solve_part_one(&String::from("abBA")), "0");
    }

    #[test]
    fn example_polymer() {
        assert_eq!(solve_part_one(&String::from("dabAcCaCBAcCcaDA")), "10");
    }

    #[test]
    fn four_letter_polymer_with_no_reactions_removes_nothing() {
        assert_eq!(solve_part_one(&String::from("abAB")), "4");
        assert_eq!(solve_part_one(&String::from("aabAAB")), "6");
    }
}

fn is_reaction(first: &char, second: &char) -> bool {
    first != second && first.to_ascii_uppercase() == second.to_ascii_uppercase()
}

#[cfg(test)]
mod is_reaction_tests {
    use super::*;

    #[test]
    fn a_reaction_occurs_between_two_letters_of_different_polarity() {
        assert_eq!(is_reaction(&'a', &'A'), true);
        assert_eq!(is_reaction(&'A', &'a'), true);
        assert_eq!(is_reaction(&'z', &'Z'), true);
    }

    #[test]
    fn a_reaction_does_not_occur_between_two_letters_of_same_polarity() {
        assert_eq!(is_reaction(&'A', &'A'), false);
        assert_eq!(is_reaction(&'Z', &'Z'), false);
    }

    #[test]
    fn a_reaction_does_not_occur_between_two_different_letters() {
        assert_eq!(is_reaction(&'A', &'b'), false);
        assert_eq!(is_reaction(&'a', &'b'), false);
        assert_eq!(is_reaction(&'a', &'B'), false);
    }
}


pub fn solve_part_two(input: &String) -> String {
    let after_initial_reaction = react_polymer(input).iter().map(|c| *c).collect::<String>();
    let units: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    let with_units_removed = units.iter().map(|c| {
        let with_unit_removed =
            after_initial_reaction
                .replace(&c.to_string(), "")
                .replace(&c.to_ascii_uppercase().to_string(), "");
        react_polymer(&with_unit_removed).len()
    });

    with_units_removed.min().unwrap().to_string()
}

#[cfg(test)]
mod part_two_tests {
    use super::*;

    #[test]
    fn two_letter_polymer_with_reaction_leaves_nothing() {
        assert_eq!(solve_part_two(&String::from("dabAcCaCBAcCcaDA")), "4");
    }
}

fn react_polymer(input: &String) -> Vec<char> {
    let mut letters = input.trim().chars().collect::<Vec<char>>();
    let mut ptr = 0;
    loop {
        let first = letters[ptr];
        let second = letters[ptr + 1];

//        println!("{} <=> {}", first, second);

        if is_reaction(&first, &second) {
            letters.remove(ptr);
            letters.remove(ptr);
            if ptr != 0 { ptr -= 1 };
//            println!("REACTION! {} <=> {}", first, second);
        } else {
            ptr += 1;
        }

//        println!("PTR>>{}/{}", ptr, letters.len());

        match letters.get(ptr + 1) {
            Some(_) => {}
            None => break
        }
    }
    letters
}
