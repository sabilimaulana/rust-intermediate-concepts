// Generics

// use std::any;

// enum Custom<T, U> {
//     EXAMPLE(T),
//     SAMPLE(U),
// }

// // Structs

// struct Rectangle<T> {
//     height: T,
//     width: T,
// }

// struct Cube<T, U, V> {
//     height: T,
//     width: U,
//     length: V,
// }

// fn main() {
//     let rect1 = Rectangle {
//         height: 1,
//         width: 2,
//     };
//     let rect1 = Rectangle {
//         height: 1.2,
//         width: 5.1,
//     };

//     let cube1 = Cube {
//         height: 1,
//         width: 2,
//         length: 3,
//     };
//     let cube1 = Cube {
//         height: 1.2,
//         width: 3,
//         length: 4.5,
//     };
// }

// Functions
fn sum_of_numbers<T: std::ops::Mul<Output = T>>(num1: T, num2: T) -> T {
    num1 * num2
}

fn lookup_datatype<T: std::fmt::Display>(object: T) {
    println!("{} {}", object, std::any::type_name::<T>())
}

fn main() {
    println!("{}", sum_of_numbers(1, 5));
    lookup_datatype(1);
    lookup_datatype(1.72);
    lookup_datatype("object");
}
