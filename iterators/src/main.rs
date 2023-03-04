fn main() {
    let v1 = vec![1, 2, 3, 4, 5, 6, 6, 7, 8, 9, 10];

    for item in v1.iter() {
        println!("Item is {}", item);
    }

    let mapped_v1: Vec<i32> = v1.iter().map(|x| x * 2).collect();

    println!("Mapped v1 is {:?}", mapped_v1);
}
