# 설치 가이드
1. Rustup 설치 스크립트 실행  
   ```bash
   curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
   ```
2. 현재 셸에서 PATH 적용  
   ```bash
   . "$HOME/.cargo/env"
   ```
3. 영구 적용을 위해 `~/.bashrc` 또는 `~/.profile` 끝에 `source "$HOME/.cargo/env"`를 직접 추가한 뒤 `source ~/.bashrc`로 반영
4. 컴파일 도구 확인  
   ```bash
   rustc --version
   ```
5. 필수 빌드 패키지 설치(미설치 시)  
   ```bash
   sudo apt install build-essential
   ```

# 프로젝트 구조
- 각 연습용 프로젝트는 `프로젝트명/` 디렉터리 하나와 그 안의 `main.rs` 파일로만 구성합니다.
- 예시:
  ```
  hello_world/
  └── main.rs
  ```
- 빌드 결과물은 각 프로젝트 내부의 `target/` 폴더(자동으로 `.gitignore`)에 모읍니다.

# rustc로 컴파일 & 실행
```bash
cd hello_world
mkdir -p target
rustc main.rs -o target/hello_world   # 필요 시 -O, --edition 등 추가
./target/hello_world
```

# Cargo를 쓰고 싶다면
```bash
cargo new hello_cargo
cargo build
./target/debug/hello_cargo
# or
cargo run

#check
cargo check
```
