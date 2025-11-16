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

# 컴파일 및 실행
```bash
cd hello_world
rustc main.rs -o hello_world
./hello_world
```
