use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut total = 0;
    let mut v = Vec::new();
    for cals in lines {
        if cals == "" {
            v.push(total);
            total = 0;
        } else {
            total += cals.parse::<i32>().unwrap();
        }
    }
    v.sort();
    v.reverse();
    println!("sum: {}", v[0] + v[1] + v[2]);
}