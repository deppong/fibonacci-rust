fn main() {
    let (mut f0, mut f1, mut fN) = (0, 1, 0);
    let n = std::env::args().nth(1).expect("Usage: fibonacci <nth iterations of fib>");

    for i in 0..n.parse::<u32>().unwrap() {
        if i == 0 {
            println!("{}", f0);
            continue;
        } else if i == 1 {
            println!("{}", f1);
            continue;
        }
        fN = f0 + f1;
        f0 = f1;
        f1 = fN;
        println!("{}", fN);
    }
}

