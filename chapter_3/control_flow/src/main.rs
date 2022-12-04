fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Fucked up ternary model:
    println!("Some ternary return value {}", if true { 1337 } else { 33 });

    let loop_result = loop {
        println!("Again!");
        break 5;
    };
    println!("Result returned from loop is: {}", loop_result);

    let arr = ["Hugo", "Katia", "Bamboo", "Croquette"];
    for el in arr.iter() {
        println!("{}", el);
    }
}
