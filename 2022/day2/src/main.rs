use std::fs;
//Rock: A X 1
//Paper: B Y 2
//Scissor: C Z 3
fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut lines = contents.lines();
    let mut total = 0;
    for line in lines {
        let mut c = line.chars();
        let expected = c.next().unwrap();
        c.next();
        let response = c.next().unwrap();
        c.next();
        match expected {
            'A'=> {match response {
                'X'=> total += 4,
                'Y'=> total += 8,
                'Z'=> total += 3,
                _ => println!("")}},
            'B'=> {match response {
                'X'=> total += 1,
                'Y'=> total += 5,
                'Z'=> total += 9,
                _ => println!("")}},
            'C'=> {match response {
                'X'=> total += 7,
                'Y'=> total += 2,
                'Z'=> total += 6,
                _ => println!("")}},
            _ => println!("")
        }
    }
    println!("total: {}", total);
}
