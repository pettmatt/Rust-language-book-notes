use std::cmp;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
    Black,
    Yellow
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        let mut num_black = 0;
        let mut num_yellow = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Black => num_black += 1,
                ShirtColor::Yellow => num_yellow += 1,
            }
        }

        let max_color = cmp::max(
            cmp::max(num_red, num_blue),
            cmp::max(num_black, num_yellow)
        );

        // This part is not supposed to work "smartly" so this kind of implementation works just fine
        if max_color == num_red {
            ShirtColor::Red
        } else if max_color == num_blue {
            ShirtColor::Blue
        } else if max_color == num_black {
            ShirtColor::Black
        } else {
            ShirtColor::Yellow
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![
            ShirtColor::Blue, ShirtColor::Blue,
            ShirtColor::Yellow, ShirtColor::Yellow,
            ShirtColor::Red,
            ShirtColor::Black
        ],
    };

    let user_pref1 = Some(ShirtColor::Yellow);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}