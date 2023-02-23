use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    v.push(6);

    let last_element = v.get(5);

    match last_element {
        Some(value) => println!("last_element is {}", value),
        None => println!("No value"),
    }

    //loop
    for i in v {
        println!("Looping {}", i);
    }

    let mut map: HashMap<i32, i32> = HashMap::new();

    let array = [
        0, 1, 3, 4, 5, 6, 7, 8, 9, 5, 4, 3, 2, 3, 3, 4, 3, 23, 2, 4, 42, 24, 4, 2, 2, 3, 2, 3,
    ];

    for n in array {
        let count = map.entry(n).or_insert(0);
        *count += 1;
    }

    println!("{:?}", &map);

    for (k, v) in map {
        println!(
            "the number {} is occured {} {}.",
            k,
            v,
            if v == 1 { "time" } else { "times" }
        );
    }
}
