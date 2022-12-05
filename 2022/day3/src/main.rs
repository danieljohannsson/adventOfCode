use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines();
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut total = 0;
    let mut block: Vec<&str> = Vec::new();
    for line in lines {
        block.push(line);
        if block.len() == 3 {
            let mut element = '0';
            let ch1: Vec<char> = block[0].chars().collect();
            let ch2: Vec<char> = block[1].chars().collect();
            let ch3: Vec<char> = block[2].chars().collect();
            for n in 0..ch1.len() {
                for u in 0..ch2.len() {
                    for i in 0..ch3.len() {
                        if ch1[n] == ch2[u] && ch1[n] == ch3[i] && ch2[u] == ch3[i] {
                            element = ch1[n];
                        }
                    }
                }   
            }
            total += alphabet.iter().position(|&s| s == element).unwrap_or_default() + 1;
            block = Vec::new();
            println!("{}",alphabet.iter().position(|&s| s == element).unwrap_or_default());
            println!("{}", element);
            println!("{}", line);
        }
    }
    println!("{}", total);

}
