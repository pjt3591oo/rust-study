# RUST 공부

공부결과: upbit 요청후 응답 결과 출력 프로그램

* run

```bash
$ cargo run
```

* issue

```rs
let socket_upbit: Vec<StockByUpbit> = serde_json::from_str(&raw_text).unwrap();
```

해당 코드를 upbit 모듈로 집어넣 은 후 main에서는 다음과 같이 사용하고 싶다.

```rs
let socket_upbit: Vec<StockByUpbit> = upbit::parse($raw_text).unwrap();
```

parse 함수에서 응답결과인 raw_text를 받은 후 json 파싱하여 새로운 구조체 반환하고 싶은데 생성된 구조체가 함수 종료시 drop에서 메모리 해제 되는 현상 때문에 외부로 반환이 되지 않음.

lifetime을 어떤식으로 줘야할 지 고민이 필요함