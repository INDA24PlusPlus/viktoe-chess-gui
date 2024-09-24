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

    let king = load_texture("./img/Chess_klt45.svg.png").await.unwrap();
    let queen = load_texture("./img/Chess_qlt45.svg.png").await.unwrap();
    let bishop = load_texture("./img/Chess_blt45.svg.png").await.unwrap();
    let knight = load_texture("./img/Chess_nlt45.svg.png").await.unwrap();
    let rook = load_texture("./img/Chess_rlt45.svg.png").await.unwrap();
    let pawn = load_texture("./img/Chess_plt45.svg.png").await.unwrap();

    let mut game = ChessBoard::new();

    let mut squares = [Square::default(); 64];

    for (index, square) in squares.iter_mut().enumerate() {
        square.index = index;
        square.x = index % 8;
        square.y = index / 8;
    }



    loop {
        clear_background(BLACK);

        let square_size = if screen_width() > screen_height() {
            screen_height() / 8.0
        } else {
            screen_width() / 8.0
        };

        for square in squares {
            let color = if (square.index + square.y) % 2 == 0 {
                ceris
            } else {
                green
            };

            let piece = match game.piece_at(square.index) {
                WhiteKing => Some((&king, ceris)),
                BlackKing => Some((&king, green)),
                WhiteQueen => Some((&queen, ceris)),
                BlackQueen => Some((&queen, green)),
                WhiteBishop => Some((&bishop, ceris)),
                BlackBishop => Some((&bishop, green)),
                WhiteKnight => Some((&knight, ceris)),
                BlackKnight => Some((&knight, green)),
                WhiteRook => Some((&rook, ceris)),
                BlackRook => Some((&rook, green)),
                WhitePawn => Some((&pawn, ceris)),
                BlackPawn => Some((&pawn, green)),
                Empty => None,
            };

            let piece_params = DrawTextureParams {dest_size: Some(vec2(square_size, square_size)), ..Default::default()};

            draw_rectangle(square.x as f32 * square_size, square.y as f32 * square_size, square_size, square_size, color);

            if let Some(piece) = piece {
                draw_texture_ex(piece.0, square.x as f32 * square_size, square.y as f32 * square_size, piece.1, piece_params);
            }
        }

        next_frame().await
    }
}
