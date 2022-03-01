fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    let x = plus_one(5);

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);

    print_labeled_measurements(5, 'h');
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is {}{}", value, unit_label);
}

// A return function
fn plus_one(x: i32) -> i32 {
    x + 1
}