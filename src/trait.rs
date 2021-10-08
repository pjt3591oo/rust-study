use std::fmt::Display;

fn func<T: Display>(x: T) -> T{
  x
}

fn main() {
  func(1);
  func("1");
  // func(vec![1,2,3]); // vec는 Display trait을 상속(?)받지 않았기 때문에 에러발생, rust에선 상속 대신 어떤 워딩을 사용해야하지? ㅋㅋ
}