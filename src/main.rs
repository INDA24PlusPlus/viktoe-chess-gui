use davbjor_chess::{self, ChessBoard, PieceType::*};
use macroquad::{math, prelude::*};

const SCREEN_HEIGHT: usize = 600;
const SCREEN_WIDTH: usize = 600;

#[derive(Debug, Default, Clone, Copy)]
struct Square {
    index: usize,
    x: usize,
    y: usize,
}

#[macroquad::main("Chess")]
async fn main() {
    let ceris = Color::from_hex(0xE83D84);
    let green = Color::from_hex(0x17c27b);

    let white_king = load_texture("./img/Chess_klt45.svg.png").await.unwrap();
    let black_king = load_texture("./img/Chess_kdt45.svg.png").await.unwrap();
    let white_queen = load_texture("./img/Chess_qlt45.svg.png").await.unwrap();
    let black_queen = load_texture("./img/Chess_qdt45.svg.png").await.unwrap();
    let white_bishop = load_texture("./img/Chess_blt45.svg.png").await.unwrap();
    let black_bishop = load_texture("./img/Chess_bdt45.svg.png").await.unwrap();
    let white_knight = load_texture("./img/Chess_nlt45.svg.png").await.unwrap();
    let black_knight = load_texture("./img/Chess_ndt45.svg.png").await.unwrap();
    let white_rook = load_texture("./img/Chess_rlt45.svg.png").await.unwrap();
    let black_rook = load_texture("./img/Chess_rdt45.svg.png").await.unwrap();
    let white_pawn = load_texture("./img/Chess_plt45.svg.png").await.unwrap();
    let black_pawn = load_texture("./img/Chess_pdt45.svg.png").await.unwrap();

    let mut game = ChessBoard::new();

    let mut squares = [Square::default(); 64];

    for (index, square) in squares.iter_mut().enumerate() {
        square.index = index;
        square.x = index % 8;
        square.y = index / 8;
    }

    let square_size = if screen_width() > screen_height() {
        screen_height() / 8.0
    } else {
        screen_width() / 8.0
    };


    loop {
        clear_background(BLACK);

        for square in squares {
            let color = if (square.index + square.y) % 2 == 0 {
                ceris
            } else {
                green
            };

            let piece = match game.piece_at(square.index) {
                WhiteKing => Some(&white_king),
                BlackKing => Some(&black_king),
                WhiteQueen => Some(&white_queen),
                BlackQueen => Some(&black_queen),
                WhiteBishop => Some(&white_bishop),
                BlackBishop => Some(&black_bishop),
                WhiteKnight => Some(&white_knight),
                BlackKnight => Some(&black_knight),
                WhiteRook => Some(&white_rook),
                BlackRook => Some(&black_rook),
                WhitePawn => Some(&white_pawn),
                BlackPawn => Some(&black_pawn),
                Empty => None,
            };

            let piece_params = DrawTextureParams {dest_size: Some(vec2(square_size, square_size)), ..Default::default()};

            draw_rectangle(square.x as f32 * square_size, square.y as f32 * square_size, square_size, square_size, color);

            if let Some(piece) = piece {
                draw_texture_ex(piece, square.x as f32 * square_size, square.y as f32 * square_size, WHITE, piece_params);
            }
        }

        next_frame().await
    }
}
