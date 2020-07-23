use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<i32>().unwrap();
    let hor = n * 2 - 1;
    let mid = hor / 2;

    for i in 1..=n {
        let spaceCount = n - i;
        let rightStart = spaceCount + (i * 2 - 1);
        for v in 1..=hor {
            if v <= spaceCount {
                print!("{}", ' ');
            } else {
                if rightStart >= v {
                    print!("{}", '*');
                }
            }
        }
        println!();
    }
}
