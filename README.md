# rustStudy

<br>

## 설치

- 윈도우즈

  - VC++ 혹은 Microsoft Visual Studio Build Tools 설치 https://visualstudio.microsoft.com/ko/downloads/?q=build+tools

  - rust 설치 : https://rust-lang.org/tools/install/

  - 설치확인

    - rustc --version

    - cargo --version

- macOS

  - curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

<br>

## 리눅스용 빌드 세팅

- cargo install cross --git https://github.com/cross-rs/cross

- cross build --target x86_64-unknown-linux-musl --release

<br>

## 샘플 생성

- 새 실행형(Binary) 프로젝트 생성
  - cargo new cpu_stress_test
  - cd cpu_stress_test
- 병렬 처리 라이브러리인 rayon 추가
  - cargo add rayon
- 실행 
  - cargo run --release