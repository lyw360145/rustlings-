#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: Fix the compiler error by adding something to this match statement.
    // 匹配项是引用，推导配置值也是引用
    match &optional_point {
        Some(p) => println!("Coordinates are {},{} &", p.x, p.y),
        _ => panic!("No match!"),
    }
    // 匹配值是结构体的引用
    match optional_point {
        Some(ref p) => println!("Coordinates are {},{} ref", p.x, p.y),
        _ => panic!("No match!"),
    }


    println!("{optional_point:?}"); // Don't change this line.
}
