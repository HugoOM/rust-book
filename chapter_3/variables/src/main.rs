fn main() {
    let x = 0;
    println!("The value of x is: {}", x);
    let mut x = x + 1;

    x += 0xff;
    println!("The value of x is: {}", x);

    let tup = (500, 6.4, 't', "Test");

    println!("Value of the tup: {:?}", tup);

    // Tuple destructuring
    let (_, y, _z, _) = tup;
    println!(
        "The value of the second content element is {} -- destructured",
        y
    );
    // Specific tuple element (0-based index)
    println!(
        "The value of the second content element is {} -- index-accessed",
        tup.1
    );

    // let arr = [1, 2, 3, 4, 5];
}
