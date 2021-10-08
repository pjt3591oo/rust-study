pub trait Summarizable {
  fn summary(&self) -> String; // 구현체에서 만드시 정의해줘야 함

}
pub trait OtherTrait {
  fn hoho(&self) {
    println!("hoho");
  }
}

struct Person {
  name: String,
  age: u8,
}

impl Summarizable for Person {
  fn summary(&self) -> String {
    format!("{name} is {age} years old", name=self.name, age=self.age)
  }
}

impl OtherTrait for Person {}

// 전달받은 타입은 Summarizable과 OtherTrait으로 구현된 타입이어야 함
fn o<T: Summarizable + OtherTrait>(x: &T){
  println!("{}", x.summary());
  x.hoho();
}

fn main() {
  let p = Person {
    name: String::from("John"),
    age: 42,
  };
  o(&p);
}