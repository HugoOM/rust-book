fn main() {
    // Clipped reading input ...

    let mut x1 = 1;
    let mut x2 = 1;
    const LIMIT: u32 = 20;
    println!("0");
    println!("1");
    println!("1");

    for _ in 1..LIMIT {
        let temp = x2;

        x2 = x2 + x1;
        x1 = temp;

        println!("{}", x2);
    }

    // println!("The {}th Fibo number is {}", LIMIT + 2, x2);
}
