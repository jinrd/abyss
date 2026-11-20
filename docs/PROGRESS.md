# ABYSS 학습 진도

## 현재 상태 (세션 인수인계)

- **현재 Phase**: Phase 0 (Rust 개발 환경 준비)
- **모드**: LEARN MODE
- **진행 중**: Phase 0 - Step 2 (함수)

### 확정된 컨벤션

- 프로젝트는 `abyss/` 루트에서 `cargo init --bin`으로 생성 (edition 2024)
- 능력치(HP, ATK, DEF) 타입: `i32` — 피해 공식 중간값이 음수가 될 수 있어서. 사망 판정은 `hp <= 0`.
- 변수/함수 이름: snake_case
- 컴파일 경고·clippy 경고 0개 유지, `cargo fmt` 적용

### 다음 과제

- 반복되는 상태 출력 코드를 함수로 분리 (함수 인자, 타입 명시)
- 이후: 구조체(struct)로 플레이어 데이터 묶기
- Phase 0 첫 Git 커밋 (아직 커밋 없음)

## 환경

- rustc 1.92.0 / cargo 1.92.0 / rustup 1.28.2, clippy·rustfmt 설치됨, git 2.52.0

## 완료한 기능

- 플레이어 이름, HP, ATK를 변수로 선언하고 출력

## 학습한 Rust 개념

- Cargo 빌드 흐름, crate root, `fn main` 진입점 → `principles/phase-0-variables.md`
- `let` / `let mut`, 타입 명시와 추론, `i32` vs `u32`

## 아직 어려운 개념

- (없음 — 확인되는 대로 기록)

## 컴파일 오류와 해결

- cannot assign twice to immutable variable → `let mut`

## 테스트 결과

- `cargo run`: 정상 출력, 경고 0
- `cargo clippy`: 경고 0
- `cargo fmt --check`: 빈 줄 2곳 차이 (미적용)
