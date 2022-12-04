fn main() {
    println!("Hello, world!");

    println!("{}", another_function(5, "y".to_string()));
}

fn another_function(x: i32, y: String) -> String {
    println!("Another function {} {}", x, y);

    "Test".to_string()
}
