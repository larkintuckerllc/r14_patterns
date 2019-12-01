fn main() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let (x, y, z) = (1, 2, 3);

    let point = (3, 5);
    print_coordinates(&point);

    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;

    foo(3, 4);

    let numbers = (2, 4, 8, 16, 32);
    let (first, _, third, _, fifth) = numbers;

    let origin = Point { x: 0, y: 0 };
    let Point { x, .. } = origin;
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

struct Point {
    x: i32,
    y: i32,
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
