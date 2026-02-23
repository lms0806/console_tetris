use bracket_lib::prelude::*;

// 전체 콘솔 크기 (BTermBuilder와 동일하게 맞춤)
const SCREEN_WIDTH: i32 = 120;
const SCREEN_HEIGHT: i32 = 60;

// 한 블록을 콘솔 문자 2x2 크기로 크게 그리기
const CELL_W: i32 = 2;
const CELL_H: i32 = 2;

// 테트리스 논리 보드 크기
const BOARD_WIDTH: i32 = 10;
const BOARD_HEIGHT: i32 = 20;

// 화면 중앙에 보드 배치
const BOARD_PIXEL_WIDTH: i32 = BOARD_WIDTH * CELL_W;
const BOARD_PIXEL_HEIGHT: i32 = BOARD_HEIGHT * CELL_H;
const BOARD_X: i32 = (SCREEN_WIDTH - BOARD_PIXEL_WIDTH) / 2;
const BOARD_Y: i32 = (SCREEN_HEIGHT - BOARD_PIXEL_HEIGHT) / 2;

type Board = [Option<RGB>; BOARD_WIDTH as usize * BOARD_HEIGHT as usize];

fn cell_to_screen(x: i32, y: i32) -> (i32, i32) {
    (BOARD_X + x * CELL_W, BOARD_Y + y * CELL_H)
}

#[derive(Copy, Clone)]
enum Tetromino {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

#[derive(Copy, Clone)]
struct Piece {
    kind: Tetromino,
    rotation: i32,
    x: i32,
    y: i32,
    color: RGB,
}

struct State {
    board: Board,
    current: Piece,
    next: Piece,
    rng: RandomNumberGenerator,
    speed: i32,
    frame: i32,
    score: i32,
    game_over: bool,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let current = Self::random_piece(&mut rng);
        let next = Self::random_piece(&mut rng);

        Self {
            board: [None; BOARD_WIDTH as usize * BOARD_HEIGHT as usize],
            current,
            next,
            rng,
            speed: 15,
            frame: 0,
            score: 0,
            game_over: false,
        }
    }

    fn random_piece(rng: &mut RandomNumberGenerator) -> Piece {
        let kind_index = rng.range(0, 7);
        let kind = match kind_index {
            0 => Tetromino::I,
            1 => Tetromino::O,
            2 => Tetromino::T,
            3 => Tetromino::S,
            4 => Tetromino::Z,
            5 => Tetromino::J,
            _ => Tetromino::L,
        };

        let color = match kind {
            Tetromino::I => RGB::named(CYAN),
            Tetromino::O => RGB::named(YELLOW),
            Tetromino::T => RGB::named(MAGENTA),
            Tetromino::S => RGB::named(GREEN),
            Tetromino::Z => RGB::named(RED),
            Tetromino::J => RGB::named(BLUE),
            Tetromino::L => RGB::named(ORANGE),
        };

        Piece {
            kind,
            rotation: 0,
            x: BOARD_WIDTH / 2,
            y: 0,
            color,
        }
    }

    fn idx(x: i32, y: i32) -> usize {
        (y as usize * BOARD_WIDTH as usize) + x as usize
    }

    fn piece_blocks(piece: &Piece) -> [(i32, i32); 4] {
        // 각 테트로미노 모양을 4x4 그리드 기준으로 정의
        // 회전은 간단한 회전 행렬을 사용
        let base: [(i32, i32); 4] = match piece.kind {
            Tetromino::I => [(-2, 0), (-1, 0), (0, 0), (1, 0)],
            Tetromino::O => [(0, 0), (1, 0), (0, 1), (1, 1)],
            Tetromino::T => [(-1, 0), (0, 0), (1, 0), (0, 1)],
            Tetromino::S => [(0, 0), (1, 0), (-1, 1), (0, 1)],
            Tetromino::Z => [(-1, 0), (0, 0), (0, 1), (1, 1)],
            Tetromino::J => [(-1, 0), (-1, 1), (0, 0), (1, 0)],
            Tetromino::L => [(-1, 0), (0, 0), (1, 0), (1, 1)],
        };

        let rot = ((piece.rotation % 4) + 4) % 4;
        let mut blocks = [(0, 0); 4];

        for (i, (bx, by)) in base.iter().enumerate() {
            let (rx, ry) = match rot {
                0 => (*bx, *by),
                1 => (-*by, *bx),
                2 => (-*bx, -*by),
                _ => (*by, -*bx),
            };
            blocks[i] = (piece.x + rx, piece.y + ry);
        }

        blocks
    }

    fn is_valid_position(&self, piece: &Piece) -> bool {
        for (x, y) in Self::piece_blocks(piece).iter() {
            if *x < 0 || *x >= BOARD_WIDTH || *y >= BOARD_HEIGHT {
                return false;
            }
            if *y >= 0 {
                let idx = Self::idx(*x, *y);
                if self.board[idx].is_some() {
                    return false;
                }
            }
        }
        true
    }

    fn try_move(&mut self, dx: i32, dy: i32) -> bool {
        let mut moved = self.current;
        moved.x += dx;
        moved.y += dy;
        if self.is_valid_position(&moved) {
            self.current = moved;
            true
        } else {
            false
        }
    }

    fn try_rotate(&mut self, dr: i32) {
        let mut rotated = self.current;
        rotated.rotation = (rotated.rotation + dr) % 4;
        if self.is_valid_position(&rotated) {
            self.current = rotated;
        }
    }

    fn lock_piece(&mut self) {
        for (x, y) in Self::piece_blocks(&self.current).iter() {
            if *y < 0 {
                self.game_over = true;
                return;
            }
            let idx = Self::idx(*x, *y);
            self.board[idx] = Some(self.current.color);
        }
        self.clear_lines();
        self.current = self.next;
        self.next = Self::random_piece(&mut self.rng);
        if !self.is_valid_position(&self.current) {
            self.game_over = true;
        }
    }

    fn clear_lines(&mut self) {
        let mut cleared = 0;

        for y in (0..BOARD_HEIGHT).rev() {
            let mut full = true;
            for x in 0..BOARD_WIDTH {
                if self.board[Self::idx(x, y)].is_none() {
                    full = false;
                    break;
                }
            }

            if full {
                cleared += 1;
                // 위에서 한 줄씩 내려오기
                for yy in (1..=y).rev() {
                    for x in 0..BOARD_WIDTH {
                        let from = Self::idx(x, yy - 1);
                        let to = Self::idx(x, yy);
                        self.board[to] = self.board[from];
                    }
                }
                // 맨 윗줄 비우기
                for x in 0..BOARD_WIDTH {
                    self.board[Self::idx(x, 0)] = None;
                }
            }
        }

        if cleared > 0 {
            self.score += match cleared {
                1 => 100,
                2 => 300,
                3 => 500,
                _ => 800,
            };

            if self.speed > 5 {
                self.speed -= 1;
            }
        }
    }

    fn hard_drop(&mut self) {
        while self.try_move(0, 1) {}
        self.lock_piece();
    }

    fn process_input(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Left => {
                    if !self.game_over {
                        self.try_move(-1, 0);
                    }
                }
                VirtualKeyCode::Right => {
                    if !self.game_over {
                        self.try_move(1, 0);
                    }
                }
                VirtualKeyCode::Down => {
                    if !self.game_over {
                        self.try_move(0, 1);
                    }
                }
                VirtualKeyCode::Up => {
                    if !self.game_over {
                        self.try_rotate(1);
                    }
                }
                VirtualKeyCode::Space => {
                    if !self.game_over {
                        self.hard_drop();
                    }
                }
                VirtualKeyCode::R => {
                    if self.game_over {
                        *self = State::new();
                    }
                }
                VirtualKeyCode::Escape => ctx.quit(),
                _ => {}
            }
        }
    }

    fn draw_board(&self, ctx: &mut BTerm) {
        // 배경 클리어
        ctx.cls();

        // 보드 테두리 (확대된 블록 크기에 맞게 그리기)
        let fg = RGB::named(WHITE);
        let left_x = BOARD_X - 1;
        let right_x = BOARD_X + BOARD_PIXEL_WIDTH;
        let bottom_y = BOARD_Y + BOARD_PIXEL_HEIGHT;

        for y in BOARD_Y - 1..=bottom_y {
            ctx.set(left_x, y, fg, RGB::named(BLACK), to_cp437('|'));
            ctx.set(right_x, y, fg, RGB::named(BLACK), to_cp437('|'));
        }
        for x in left_x..=right_x {
            ctx.set(x, bottom_y, fg, RGB::named(BLACK), to_cp437('-'));
        }

        // 고정된 블록들 (2x2 문자로 확대)
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if let Some(color) = self.board[Self::idx(x, y)] {
                    let (sx, sy) = cell_to_screen(x, y);
                    for dy in 0..CELL_H {
                        for dx in 0..CELL_W {
                            ctx.set(
                                sx + dx,
                                sy + dy,
                                color,
                                RGB::named(BLACK),
                                to_cp437('#'),
                            );
                        }
                    }
                }
            }
        }

        // 현재 조각 (2x2 문자로 확대)
        if !self.game_over {
            for (x, y) in Self::piece_blocks(&self.current).iter() {
                if *y >= 0 {
                    let (sx, sy) = cell_to_screen(*x, *y);
                    for dy in 0..CELL_H {
                        for dx in 0..CELL_W {
                            ctx.set(
                                sx + dx,
                                sy + dy,
                                self.current.color,
                                RGB::named(BLACK),
                                to_cp437('#'),
                            );
                        }
                    }
                }
            }
        }

        // 다음 조각 표시 (오른쪽 여백에 작게 표시)
        let info_x = right_x + 4;
        let info_y = BOARD_Y;
        ctx.print(info_x, info_y, "다음 블록:");
        let preview_x = info_x + 2;
        let preview_y = info_y + 2;
        let preview_piece = Piece {
            x: preview_x,
            y: preview_y,
            ..self.next
        };
        for (x, y) in Self::piece_blocks(&preview_piece).iter() {
            ctx.set(
                *x,
                *y,
                self.next.color,
                RGB::named(BLACK),
                to_cp437('#'),
            );
        }

        // 점수 및 안내 (오른쪽 영역에 배치)
        let mut text_y = preview_y + 8;
        ctx.print(info_x, text_y, &format!("점수: {}", self.score));
        text_y += 2;
        ctx.print(info_x, text_y, "조작:");
        text_y += 1;
        ctx.print(info_x, text_y, "← → : 좌우 이동");
        text_y += 1;
        ctx.print(info_x, text_y, "↑   : 회전");
        text_y += 1;
        ctx.print(info_x, text_y, "↓   : 빠르게 내리기");
        text_y += 1;
        ctx.print(info_x, text_y, "Space: 하드 드롭");
        text_y += 1;
        ctx.print(info_x, text_y, "Esc : 종료");

        if self.game_over {
            let msg = "GAME OVER - R 키로 재시작";
            let x = BOARD_X + 1;
            let y = BOARD_Y + BOARD_HEIGHT / 2;
            ctx.print_color(
                x,
                y,
                RGB::named(RED),
                RGB::named(BLACK),
                msg,
            );
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.process_input(ctx);

        if !self.game_over {
            self.frame += 1;
            if self.frame % self.speed == 0 {
                if !self.try_move(0, 1) {
                    self.lock_piece();
                }
            }
        }

        self.draw_board(ctx);
    }
}

fn main() -> BError {
    // 창 크기를 상단 상수와 동일하게 120x60으로 설정
    let context = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)?
        .with_title("Console Tetris (bracket-lib)")
        .build()?;

    let gs = State::new();
    main_loop(context, gs)
}
