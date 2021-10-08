use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;

// Result<T, E>
/*
  enum Result<T, E> {
    Ok(T),
    Err(E),
  }
*/

// Result는 T 타입을 받는 Ok와 E 타입을 받는 Err을 match로 처리할 수 있다.
fn readFile1() {
  let f: Result<File, Error> = File::open("hello.txt");

  let f = match f {
    Ok(file) => file,
    Err(error) => {
      panic!("There was a problem opening the file: {:?}", error)
    },
  };
}

// unwrap()은 Err 반환시 panic() 발생
// File::open()의 반환값은 File을 받는다.
fn readFile2() {
  let f: File = File::open("hello.txt").unwrap();
}

// expect()은 Err 반환시 커스텀 메시지를 포함한 panic() 발생
// File::open()의 반환값은 File을 받는다.
fn readFile2() {
  let f: File = File::open("hello.txt").expect("Failed to open hello.txt");
}

// 에러 전파
fn readFile_ErrBroadcast() -> Result<String, io::Error> {
  let f = File::open("hello.txt");

  let mut f = match f {
      Ok(file) => file,
      Err(e) => return Err(e),
  };

  let mut s = String::new();

  match f.read_to_string(&mut s) {
      Ok(_) => Ok(s),
      Err(e) => Err(e),
  }
}

// ?는 Ok(T) 반환시 코드가 계속 진행되고 Err(E) 반환시 해당 함수를 호출하는 부분으로 Err(E)를 반환한다.
// ?는 Result<T, E>를 반환하는 함수에서만 사용할 수 있다.
fn read_username_from_file() -> Result<String, io::Error> {
  let mut f = File::open("hello.txt")?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}

// ?는 체이닝 형태로 사용할 수 있다.
fn read_username_from_file() -> Result<String, io::Error> {
  let mut s = String::new();

  File::open("hello.txt")?.read_to_string(&mut s)?;

  Ok(s)
}

fn main() {
  let filepath = String::from("/Users/jeongtaepark/Desktop/rust-owner/src/hello.txt");
  let f1:Result<File, Error> = File::open(&filepath);
  
  let mut f: File = match f1 {
    Ok(file) => file,
    // 에러 형태별로 분기하여 처리
    Err(error) if error.kind() == ErrorKind::NotFound => {
      println!("{:?}", error.kind());
      println!("{:?}", ErrorKind::NotFound);
      panic!("not found file: {:?}", error);
    },

    Err(error) => {
      println!("{:?}", error.kind());
      panic!("error: {:?}", error);
    }
  };

  println!("{:?}", f);

  let mut str = String::new();
  f.read_to_string(&mut str);
  println!("{:?}", &str);

  // // unwrap은 open 결과가 Err이면 panic을 발생시킴
  // let f2: File = File::open(&filepath).unwrap();
  // println!("{:?}", f2);

  // // unwrap은 open 결과가 Err이면 메시지를 전달하여 panic을 발생시킴
  // let f3: File = File::open(&filepath).expect("error!!!!!@!@!@");
  // println!("{:?}", f3);
}
