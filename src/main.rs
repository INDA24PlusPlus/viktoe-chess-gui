use std::usize;

use davbjor_chess::{
    self, ChessBoard, GameResult,
    PieceType::{self, *},
};
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

    let mut is_promote = false;

    let mut current_index = 0;
    let mut selecting = false;

    while game.game_result == GameResult::Ongoing {
        clear_background(BLACK);

        let square_size = if screen_width() > screen_height() {
            screen_height() / 8.0
        } else {
            screen_width() / 8.0
        };

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            let x = x / square_size;
            let y = y / square_size;
            let x = x.floor() as usize;
            let y = y.floor() as usize;

            current_index = if selecting {
                is_promote = !game.move_piece(current_index, x + y * 8).unwrap_or(true);
                usize::MAX
            } else if is_promote
                && y as f32 > screen_height() / 2.0 - square_size / 2.0
                && (y as f32) < screen_height() / 2.0 + square_size / 2.0
                && (x as f32) > screen_width() / 2.0 - square_size * 2.0
            {
                let piece = if (x as f32) < screen_width() / 2.0 - square_size {
                    if game.whites_turn {
                        Some(PieceType::WhiteQueen)
                    } else {
                        Some(PieceType::BlackQueen)
                    }
                } else if (x as f32) < screen_width() / 2.0 + square_size {
                    if game.whites_turn {
                        Some(PieceType::WhiteBishop)
                    } else {
                        Some(PieceType::BlackBishop)
                    }
                } else if (x as f32) < screen_width() / 2.0 + square_size * 2.0 {
                    if game.whites_turn {
                        Some(PieceType::WhiteKnight)
                    } else {
                        Some(PieceType::BlackKnight)
                    }
                } else if (x as f32) < screen_width() / 2.0 + square_size * 3.0 {
                    if game.whites_turn {
                        Some(PieceType::WhiteRook)
                    } else {
                        Some(PieceType::BlackRook)
                    }
                } else {
                    None
                };

                if let Some(piece) = piece {
                    game.handle_promotion(current_index, x + y * 8, piece)
                        .unwrap();
                }

                is_promote = false;

                usize::MAX
            } else {
                x + y * 8
            }
        }

        let moves = game.get_moves_list(current_index);

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

            let piece_params = DrawTextureParams {
                dest_size: Some(vec2(square_size, square_size)),
                ..Default::default()
            };

            draw_rectangle(
                square.x as f32 * square_size,
                square.y as f32 * square_size,
                square_size,
                square_size,
                color,
            );

            if let Some(piece) = piece {
                draw_texture_ex(
                    piece.0,
                    square.x as f32 * square_size,
                    square.y as f32 * square_size,
                    piece.1,
                    piece_params.clone(),
                );
            }

            let highlight_color = if color == ceris { green } else { ceris };

            if moves.contains(&square.index) {
                draw_circle(
                    square.x as f32 * square_size + square_size / 2.0,
                    square.y as f32 * square_size + square_size / 2.0,
                    square_size / 5.0,
                    highlight_color,
                );
            }

            if is_promote {
                let height_start = screen_height() / 2.0 - square_size / 2.0;
                let width_start = screen_width() / 2.0 - 2.0 * square_size;
                draw_rectangle(
                    width_start,
                    height_start,
                    4.0 * square_size,
                    square_size,
                    BLACK,
                );
                let color = if game.whites_turn { ceris } else { green };

                draw_texture_ex(
                    &queen,
                    width_start,
                    height_start,
                    color,
                    piece_params.clone(),
                );
                draw_texture_ex(
                    &bishop,
                    width_start,
                    height_start,
                    color,
                    piece_params.clone(),
                );
                draw_texture_ex(
                    &knight,
                    width_start,
                    height_start,
                    color,
                    piece_params.clone(),
                );
                draw_texture_ex(
                    &rook,
                    width_start,
                    height_start,
                    color,
                    piece_params.clone(),
                );
            }
        }

        next_frame().await;
    }

    draw_text(
        "Game Over",
        screen_width() / 2.0,
        screen_height() / 2.0,
        30.0,
        BLACK,
    );
    next_frame().await;
}
