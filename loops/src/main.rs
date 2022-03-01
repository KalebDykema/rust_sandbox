fn main() {
    // nested_loop();
    // returning_loop();
    // while_loop();
    for_loop();
}

// fn nested_loop() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {}", count);
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {}", remaining);
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }   

//     println!("End count = {}", count)
// }

// fn returning_loop() {
//     let mut counter = 0;

//     // This holds a returned value
//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {}", result);
// }

// fn while_loop () {
//     let mut number = 3;

//     while number != 0 {
//         println!("{}!", number);

//         number -= 1;
//     }

//     println!("LIFTOFF!");
// }

fn for_loop () {
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    
    // Using a while to emulate a for loop
    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index += 1;
    // }

    // The better way to do the above
    // for element in a {
    //     println!("the value is: {}", element);
    // }

    // Reverse a range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}