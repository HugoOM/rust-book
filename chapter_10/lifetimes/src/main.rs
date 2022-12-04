use std::fmt::Display;

fn main() {
    // let string1 = String::from("abcd");
    // let string2 = "xyz";

    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {}", result);

    let string1 = String::from("long string is long");
    let mut result;

    {
        // let string2 = String::from("xyz");
        //* Static Lifetime is implicit for String literals
        let string2 = "xyz";
        result = longest(string1.as_str(), string2);
    }
    println!("The longest string is {}", result);

    let (a, b) = test_multi_return();
    println!("{} {}", a, b);

    let string3 = "some test string";

    result =
        longest_with_an_announcement(string1.as_str(), string3, "Today is someone's birthday!");
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Uses a tuple and destructuring for multi-return & assignation
fn test_multi_return() -> (i32, i32) {
    (5, 10)
}

fn longest_with_an_announcement<'a>(x: &'a str, y: &'a str, ann: impl Display) -> &'a str {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
