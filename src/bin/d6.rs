use std::collections::HashSet;
use std::io::Read;

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_to_string(&mut buffer)
        .expect("Read fail");

    let input: Vec<char> = buffer.chars().collect();

    let window_size = 4;
    for (offset, window) in input[..].windows(window_size).enumerate() {
        let mut s: HashSet<char> = HashSet::new();
        for c in window.iter() {
            s.insert(*c);
        }

        if s.len() == window_size {
            println!("{}", offset + window_size);
            break;
        }
    }
}
