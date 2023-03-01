#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Blue,
    Red,
}

struct Store {
    inventory: Vec<ShirtColor>,
}

impl Store {
    fn most_stocked(&self) -> ShirtColor {
        let mut red_count = 0;
        let mut blue_count = 0;

        for color in &self.inventory {
            match color {
                ShirtColor::Blue => blue_count += 1,
                ShirtColor::Red => red_count += 1,
            }
        }

        if red_count < blue_count {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }

    fn giveaway(&self, &user_preference: &Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
}

fn main() {
    let store = Store {
        inventory: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red],
    };

    let user_one_preference = Some(ShirtColor::Red);

    let giveaway_1 = store.giveaway(&user_one_preference);
    println!(
        "User one with preference {:?} got {:?} shirt",
        user_one_preference, giveaway_1
    );

    let user_two_preference = None;

    let giveaway_2 = store.giveaway(&user_two_preference);
    println!(
        "User one with preference {:?} got {:?} shirt",
        user_two_preference, giveaway_2
    );
}
