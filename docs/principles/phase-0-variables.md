# Phase 0 - Step 1: Cargo와 변수, 타입

## Cargo 빌드 흐름

- `cargo run` = 컴파일(`Compiling`) → 빌드 완료(`Finished`) → **실행 파일** 실행(`Running`)
- 실행되는 것은 `.rs` 소스가 아니라 `target/debug/abyss` 기계어 바이너리다.
- `dev` 프로필은 최적화 없음 + 디버그 정보 포함. `--release`는 최적화 빌드.

## main.rs vs fn main()

- `src/main.rs`: 파일. 바이너리 crate의 **crate root** (컴파일 시작점).
- `fn main()`: 함수. 프로그램 실행 시 가장 먼저 호출되는 **진입점**.
- `println!`의 `!`는 매크로 표시. prelude에 포함되어 `use` 없이 사용 가능.

## 변수

- `let`은 기본 **불변**. 값을 바꾸려면 `let mut`.
  - 불변이 기본인 이유: 값이 어디서 바뀌는지 추적하기 쉬워지고, 실수로 인한 변경을 컴파일 시점에 막는다.
- 타입 명시: `let hp: i32 = 30;` / 생략 시 컴파일러가 추론하지만 **정적 타입**이다.
- 정수: `i32`(부호 있음), `u32`(부호 없음) 등.

## 컴파일러 경고도 정보다

- `value assigned to ... is never read`: 대입한 값을 아무 데도 쓰지 않음 → 의미 없는 코드.
- 오류뿐 아니라 경고도 0개로 유지하는 습관.

## 부호 없는 정수의 함정

- `u32`에서 `3 - 5` → 디버그 빌드 panic (`attempt to subtract with overflow`), release는 wrap-around.
- 대안: `saturating_sub`, 또는 `i32` 사용 후 `<= 0` 검사.
- ABYSS 결정: 능력치/HP는 `i32` (피해 공식 `attack + random(-1,1) - defense`에 음수 중간값이 있음).

## 겪은 컴파일 오류

| 오류 | 원인 | 해결 |
|---|---|---|
| cannot assign twice to immutable variable | `let` 변수는 불변 | `let mut` |
