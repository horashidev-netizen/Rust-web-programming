struct Coordinate<T>{
    x:T,
    y:T
}

macro_rules! capitalize {
    ($a: expr) => {
        let mut v: Vec<char> = $a.chars().collect();
        v[0] = v[0].to_uppercase().nth(0).unwrap();
        $a = v.into_iter().collect();
    };
}

fn main() {
    let one = Coordinate{x: 50, y: 50};
    let two = Coordinate{x: 500, y: 500};
    let three = Coordinate{x: 5.6, y: 5.6};
    println!("{}, {}, {}", one.x, two.y, three.y);

    let mut test = String::from("test");
    capitalize!(test);
    println!("{}", test);
}
