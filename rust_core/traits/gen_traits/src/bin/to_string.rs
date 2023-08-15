#![allow(dead_code)]

use std::fmt::Display;

enum Colour {
    RED,
    BLUE,
    GREEN,
}

// Can't impl ToString when Display trait is impl.
// Will lead to conflicting trait impl error.

// impl ToString for Colour {
//     fn to_string(&self) -> String {
//         match self {
//             Colour::RED => String::from("Red"),
//             Colour::BLUE => String::from("Blue"),
//             Colour::GREEN => String::from("Green"),
//         }
//     }
// }

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Colour::RED => write!(f, "Monday"),
            Colour::BLUE => write!(f, "Blue"),
            Colour::GREEN => write!(f, "Green"),
        }
    }
}

fn main() {
    let colour = Colour::RED;
    // Impl Display trait auto impls ToString trait.
    let colour_string = colour.to_string();
    println!("Color value - {:}", colour);
    println!("Color string - {:}", colour_string);
}
