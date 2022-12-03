use std::fs;
//Rock: A X 1 lose 
//Paper: B Y 2 draw
//Scissor: C Z 3 win
fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines();
    let mut total = 0;
    for line in lines {
        let mut c = line.chars();
        let expected = c.next().unwrap();
        c.next();
        let response = c.next().unwrap();
        c.next();
        match expected {
            'A'=> {match response {
                'X'=> total += 3,
                'Y'=> total += 4,
                'Z'=> total += 8,
                _ => println!("")}},
            'B'=> {match response {
                'X'=> total += 1,
                'Y'=> total += 5,
                'Z'=> total += 9,
                _ => println!("")}},
            'C'=> {match response {
                'X'=> total += 2,
                'Y'=> total += 6,
                'Z'=> total += 7,
                _ => println!("")}},
            _ => println!("")
        }
    }
    println!("total: {}", total);
}
