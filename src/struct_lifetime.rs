#[derive(Debug)]
struct Person<'a> {
  age: i8,
  name: &'a str,
}

// lifetime은 컴파일 시점에서 dangling reference를 알려준다.

fn main() {
  let result: Person;
  let name = String::from("mung");
  
  {
    // dangling reference 발생 name을 해당 스코드 밖으로 보내면 된다.
    // Person 구조체는 라이프타임을 가지고 있기 때문에 Person이 초기화된 scope보다 오래된(밖에있는?) scope에 정의된 name을 가질 수 있다.
    // 아래 name을 사용하게 되면 해당 scope가 끝나면, drop이 호출되면서 name은 해제된다. 즉 Person.name은 다른 것을 참조하게 되는 현산을 dangling reference라고 함
    
    // let name = String::from("mung");
    let age = 30;
    result = Person {
      age: age,
      name: &name,
    };
  }

  println!("{:?}", result);
}