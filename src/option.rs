/*
  Option은 다음과 같이 구현되어 있다.

  enum Option<T> {
    Some(T),
    None,
  }

  Option은 패턴 매칭을 이용하여 Some과 None을 적절히 처리할 수 있다.

  Some(1)과 1은 다른 값이다.
  Some(1) + 1은 에러발생
*/
fn main() {
  let sss1 : Option<i32>= Some(123);
  let sss2 = Some("123");
  let sss3 = Some(&sss1);
  let sss4 = None;

  match sss1 {
    Some(x) => println!("{}", x),
    None => println!("None"),
  }
  match sss2 {
    Some(x) => println!("{}", x),
    None => println!("None"),
  }

  match sss3 {
    Some(x) => println!("{}", x),
    None => println!("None"),
  }

  match sss4 {
    Some(x) => println!("{}", x),
    None => println!("None"),
  }

  Some(1) + 1;
}