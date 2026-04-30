fn main() {
    let mut my_vector: Vec<&str> = vec!["one", "two", "three"];
    println!("{:?}", my_vector);
    my_vector.push("four");
    my_vector.push("five");
    println!("{:?}", my_vector);
}
