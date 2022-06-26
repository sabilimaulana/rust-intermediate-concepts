// pub trait Vehicle {
//     fn forward() -> String;
//     fn backward() -> String;

//     fn turn_ignition() -> String {
//         String::from("vroom vroom")
//     }
// }

// pub struct Car {
//     color: String,
// }

// impl Vehicle for Car {
//     fn forward() -> String {
//         String::from("Driving my car forward")
//     }
//     fn backward() -> String {
//         String::from("Backing my car up...")
//     }
// }

// pub struct Truck {
//     color: String,
// }

// impl Vehicle for Truck {
//     fn forward() -> String {
//         String::from("Driving my truck forward")
//     }
//     fn backward() -> String {
//         String::from("Backing my truck up...")
//     }
// }

// fn main() {
//     println!("{}", Car::turn_ignition());
//     println!("{}", Car::forward());
//     println!("{}", Car::backward());
//     println!("{}", Truck::turn_ignition());
//     println!("{}", Truck::forward());
//     println!("{}", Truck::backward());
// }

pub trait Vehicle {
    fn forward(&self) -> String;
    fn backward(&self) -> String;

    fn turn_ignition() -> String {
        String::from("vroom vroom")
    }
}

pub struct Car {
    color: String,
}

impl Vehicle for Car {
    fn forward(&self) -> String {
        String::from(format!("Driving my {} car forward", self.color))
    }
    fn backward(&self) -> String {
        String::from(format!("Backing my {} car up...", self.color))
    }
}

pub struct Truck {
    color: String,
}

impl Vehicle for Truck {
    fn forward(&self) -> String {
        String::from(format!("Driving my {} truck forward", self.color))
    }
    fn backward(&self) -> String {
        String::from(format!("Backing my {} truck up...", self.color))
    }
}

fn main() {
    println!("{}", Car::turn_ignition());
    let car = Car {
        color: String::from("red"),
    };
    println!("{}", car.forward());
    println!("{}", car.backward());

    println!("{}", Truck::turn_ignition());
    let truck = Truck {
        color: String::from("green"),
    };
    println!("{}", truck.forward());
    println!("{}", truck.backward());
}
