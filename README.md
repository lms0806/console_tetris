console_tetris
==============

### 개요

이 프로젝트는 [`bracket-lib`](https://github.com/amethyst/bracket-lib?tab=readme-ov-file)를 활용해서 만든 콘솔 테트리스 게임입니다.  
`bracket-lib`의 `bracket-terminal` 모듈을 이용해 CP437 스타일의 터미널 화면 위에 블록을 렌더링하고, 키보드 입력을 처리하여 테트리스 게임을 구현했습니다.

### 빌드 및 실행 방법

#### 1. Rust 설치

먼저 Rust가 설치되어 있어야 합니다. 설치가 안 되어 있다면 아래 페이지를 참고해 `rustup`으로 설치합니다.

- Rust 공식 사이트: <https://www.rust-lang.org/>

#### 2. 의존성

`Cargo.toml`에는 다음과 같이 `bracket-lib`가 의존성으로 포함되어 있습니다.

```toml
[dependencies]
bracket_lib = "0.8.7"
```

따로 추가 설정을 할 필요는 없고, 프로젝트 루트에서 `cargo run`을 실행하면 자동으로 다운로드/빌드됩니다.

#### 3. 실행

프로젝트 루트(이 `README.md`가 있는 디렉터리)에서 아래 명령을 실행하세요.

```bash
cargo run
```

성공적으로 실행되면 120x60 크기의 콘솔 창이 열리고, 중앙에 테트리스 보드가 표시됩니다.

### 조작 방법

- **← / →**: 현재 블록 좌우 이동  
- **↓**: 현재 블록 한 칸 빠르게 내리기  
- **↑**: 현재 블록 회전  
- **Space**: 하드 드롭(바닥까지 한 번에 떨어뜨리기)  
- **R**: 게임 오버 상태에서 게임 재시작  
- **Esc**: 게임 종료  

줄을 완성하면 자동으로 삭제되고, 여러 줄을 동시에 삭제할수록 더 많은 점수를 얻게 됩니다.  
점수가 오를수록 블록이 떨어지는 속도가 점점 빨라집니다.

### 참고

`bracket-lib` 사용법 및 더 자세한 설명은 공식 저장소의 README를 참고하세요.  
- [`bracket-lib` GitHub](https://github.com/amethyst/bracket-lib?tab=readme-ov-file)
