#[derive(Debug)]
struct Point<T, F> {
  x: T,
  y: F,
}

fn main() {
  let int = Point {x: 10, y: 10};
  let s = Point{x: "10", y: "20"};
  let s1 = Point{x: 10, y: "10"};

  println!("{:?}", int);
  println!("{:?}", s);
  println!("{:?}", s1);
}