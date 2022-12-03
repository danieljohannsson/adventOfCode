use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines();
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut total = 0;
    for a in alphabet.iter() {
        println!("{}", a)
    }
    for line in lines {
        let mut element = '0';
        let (first, last) = line.split_at(line.len()/2);
        let ch1: Vec<char> = last.chars().collect();
        let ch2: Vec<char> = first.chars().collect();
        for n in 0..first.len() {
            for u in 0..first.len() {
                if ch1[n] == ch2[u] {
                    element = ch1[n];
                }
            }   
        }
        total += alphabet.iter().position(|&s| s == element).unwrap_or_default() + 1;
        // println!("{}",alphabet.iter().position(|&s| s == element).unwrap_or_default());
        // println!("{}", element);
        // println!("{}", line);
    }
    println!("{}", total);

}
