use std::collections::HashMap;

fn main() {
    println!("{}", firstNotRepeatingCharacter(String::from("abacbad")));
}

fn firstNotRepeatingCharacter(s: String) -> char {
    let mut seen: HashMap<char, (usize, usize)> = HashMap::new();
    s.char_indices().for_each(|(i, c)| {
        let cnt = seen.entry(c).or_insert((i, 0));
        cnt.1 += 1;
    });

    let singles: Option<&(usize, usize)> = seen
        .values()
        .filter(|(_, c)| *c == 1)
        .min_by_key(|(i, _)| i);

    match singles {
        Some((i, _)) => s.chars().nth(*i).unwrap(),
        None => '_',
    }
}
