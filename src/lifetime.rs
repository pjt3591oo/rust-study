fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
  x
}

// lifetime 명시에 따라 에러 지점이 달라짐
// 함수의 lifetime 사용은 3가지 생략 규칙이 존재

// 규칙1. 참조자(&)인 각각의 파라미터는 고유한 라이프타임 파라미터를 가짐
//     fn foo(x: &i32) -> &str {}             ||    fn foo<'a>(x: &'a i32) -> &'a str {}              규칙2의 적용으로 반환 타입도 라이프타임을 가짐
//     fn foo(x: &i32, y: &i32) -> &str {}    ||    fn foo<'a, 'b>(x: &'a i32, y: &'b i32) -> &str {} 규칙2의 적용으로 반환 타입은 라이프타임을 가지지 않는다

// 규칙2. 딱 하나의 라이프타임 파라미터만 있다면(하나의 참조자만 가진다면), 그 라이프타임이 모든 출력 라이프타임 파라미터들에 대입되는 것이 가능함
//     fn foo(x: &i32) -> &str {}             ||    fn foo<'a>(x: &'a i32) -> &'a str {} 

// 규칙3. 만인 여려개의 라이프타임 파라미터가 있는데, 메소드라서 그중 하나가 &self 혹은 &mut self 라면, self의 라이프타임이 모든 출력 라이프타임 파라미터에 대입 => 더욱 멋진 메소드를 만들 수 있다고 한다.
//       => 메소드의 경우 self의 라이프타임을 모든 출력 파라미터에 대입
//
//        impl<'a> ImportantExcerpt<'a> { // 규칙1의 적용으로 &self와 반환인 i32의 라이프타임 생략가능
//          fn level(&self) -> i32 {
//            3
//          }
//        }
//
//        impl<'a> ImportantExcerpt<'a> {
//           fn announce_and_return_part(&self, announcement: &str) -> &str { // 규칙3의 적용으로 파라미터가 다수 존재하지만 &self가 있으므로 &self의 라이프타임으로 반환 타입의 라이프타임은 &self와 동일
//               println!("Attention please: {}", announcement);
//               self.part
//           }
//        }       
//        
// 정적 라이프타임
//    모든 스트링 리터럴은 static 라이프타임을 가짐 => 항상 이용가능
//    let s: &'static str = "I have a static lifetime.";


fn main() {
  let string1 = String::from("abcd");
  let result;

  {
    let string2 = String::from("hello world");
  
    // result의 라이프라임이 참조자 string1 또는 string2보다 길면 함수 혹은 호출 지점에서 에러발생
    result = longest(string1.as_str(), string2.as_str()); // &string == string.as_str()
  }
  println!("The longest string is {}", result);
}