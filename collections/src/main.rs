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
}
