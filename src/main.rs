use std::usize;

use davbjor_chess::{self, ChessBoard, PieceType::*};
use macroquad::prelude::*;

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


    let mut current_index = 0;
    let mut selecting = false;

    loop {
        clear_background(BLACK);

        let square_size = if screen_width() > screen_height() {
            screen_height() / 8.0
        } else {
            screen_width() / 8.0
        };

        let current_temp_index = if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            let x = x / square_size;
            let y = y / square_size;
            let x = x.floor() as usize;
            let y = y.floor() as usize;

            x + y * 8
        } else {
            current_index
        };

        let moves = if selecting {
            game.move_piece(current_index, current_temp_index);
            Vec::new()
        } else {
            
            game.get_moves_list(current_index)
        };


        selecting = !moves.is_empty();

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

            let highlight_color = if color == ceris {
                green
            } else {
                ceris
            };

            if moves.contains(&square.index) {
                draw_circle(square.x as f32 * square_size + square_size / 2.0, square.y as f32 * square_size + square_size / 2.0, square_size / 5.0, highlight_color);
            }
        }


        next_frame().await
    }
}
