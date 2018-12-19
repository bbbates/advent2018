
fn count_splits(lengths: &Vec<usize>, num: i8) -> usize {
    println!("{:?}", lengths);
    lengths.iter().filter(|i| **i == num as usize).count()
}


fn count_repeats(input: &String) -> (i8, i8) {
    let mut chars: Vec<char> = input.chars().collect();
    chars.sort();

    let windowed: Vec<&[char]> = chars.windows(2).collect();
    println!("{:?}", &windowed);
    let split: Vec<usize> = windowed[..]
        .split(|chs| chs[0] == chs[1])
        .inspect(|v| println!(">>>>{:?}", v))
        .map(|chs| chs.len()).collect();

    let twos = count_splits(&split, 2);
    let threes = count_splits(&split, 3);

    (twos as i8, threes as i8)
}


#[cfg(test)]
mod count_repeats_test {
    use super::*;

    #[test]
    fn all_unique_letters_should_give_zero_result() {
        assert_eq!(count_repeats(&String::from("abcdefg")), (0, 0))
    }

    #[test]
    fn single_letter_repeated_twice_returns_single_result() {
        assert_eq!(count_repeats(&String::from("abbcde")), (1, 0))
    }
}

// abbcde => [a b] [b b] [b c] [c d] [d e]
//        =>
