fn get_largest<T: std::cmp::PartialOrd>(t: &Vec<T>) -> &T {
    let mut largest = &t[0];

    for item in t {
        if largest < &item {
            largest = &item
        }
    }

    largest
}

fn main() {
    let list = vec![1, 3, 2, 4, 5, 6, 7, 3, 3, 6, 7, 3];

    let largest = get_largest(&list);

    println!("Largest is {}", largest);
}
