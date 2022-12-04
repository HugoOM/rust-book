fn main() {
    let mut s = String::from("hello");

    s.push_str(" some more string");

    let r2 = &s;
    println!("{}", r2);
    let r1 = &mut s;
    println!("{}", r1);
}
