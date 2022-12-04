fn main() {
    // let favorite_color: Option<&str> = None;
    // let is_tuesday = false;
    // let age: Result<u8, _> = "34".parse();

    // if let Some(color) = favorite_color {
    //     println!("Using your favorite color, {}, asd the background", color);
    // } else if is_tuesday {
    //     println!("Tuesday is green day");
    // } else if let Ok(age) = age {
    //     if age > 30 {
    //         println!("Using purple as the background color");
    //     } else {
    //         println!("Using orange as the background color");
    //     }
    // } else {
    //     println!("Using blue as the background color");
    // }

    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&point: &(i32, i32)) {
    println!("Current Location: ({}, {})", point.0, point.1);
}

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
