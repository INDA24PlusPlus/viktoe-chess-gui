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

    let local_turn_is_white = true;

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

        if game.whites_turn == local_turn_is_white && is_mouse_button_pressed(MouseButton::Left) {
            handle_input(&mut game, square_size, &mut current_index, &mut selecting, &mut is_promote);
        } else {

        }

        let moves = game.get_moves_list(current_index);

        selecting = !moves.is_empty();

        for square in squares {
            display_square(
                &game,
                &moves,
                square,
                square_size,
                ceris,
                green,
                &king,
                &queen,
                &bishop,
                &knight,
                &rook,
                &pawn,
            );
        }

        if is_promote {
            let color = if game.whites_turn { ceris } else { green };
            display_pawn_promotion(square_size, color, &queen, &bishop, &knight, &rook);
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

fn handle_input(game: &mut ChessBoard, square_size: f32, current_index: &mut usize, selecting: &mut bool, is_promote: &mut bool) {
    let (x, y) = mouse_position();
    let x = x / square_size;
    let y = y / square_size;
    let x = x.floor() as usize;
    let y = y.floor() as usize;


    *current_index = if *selecting {
        *is_promote = handle_move(game, *current_index, x + y * 8);

        while *is_promote {
            let (x, y) = mouse_position();
            let x = x / square_size;
            let y = y / square_size;
            let x = x.floor() as usize;
            let y = y.floor() as usize;

            if y as f32 > screen_height() / 2.0 - square_size / 2.0
            && (y as f32) < screen_height() / 2.0 + square_size / 2.0
            && (x as f32) > screen_width() / 2.0 - square_size * 2.0 {

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
                    game.handle_promotion(*current_index, x + y * 8, piece)
                        .unwrap();
                }

                *is_promote = false;
            }
        }

        usize::MAX
    } else {
        x + y * 8
    }
}

fn handle_move(game: &mut ChessBoard, from: usize, to: usize) -> bool {
    !game.move_piece(from, to).unwrap_or(true)
}

fn display_pawn_promotion(
    square_size: f32,
    color: Color,
    queen: &Texture2D,
    bishop: &Texture2D,
    knight: &Texture2D,
    rook: &Texture2D,
) {
    let piece_params = DrawTextureParams {
        dest_size: Some(vec2(square_size, square_size)),
        ..Default::default()
    };

    let height_start = screen_height() / 2.0 - square_size / 2.0;
    let width_start = screen_width() / 2.0 - 2.0 * square_size;
    draw_rectangle(
        width_start,
        height_start,
        4.0 * square_size,
        square_size,
        BLACK,
    );

    draw_texture_ex(
        queen,
        width_start,
        height_start,
        color,
        piece_params.clone(),
    );
    draw_texture_ex(
        bishop,
        width_start + square_size,
        height_start,
        color,
        piece_params.clone(),
    );
    draw_texture_ex(
        knight,
        width_start + square_size * 2.0,
        height_start,
        color,
        piece_params.clone(),
    );
    draw_texture_ex(
        rook,
        width_start + square_size * 3.0,
        height_start,
        color,
        piece_params.clone(),
    );
}

fn display_square(
    game: &ChessBoard,
    moves: &Vec<usize>,
    square: Square,
    square_size: f32,
    ceris: Color,
    green: Color,
    king: &Texture2D,
    queen: &Texture2D,
    bishop: &Texture2D,
    knight: &Texture2D,
    rook: &Texture2D,
    pawn: &Texture2D,
) {
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
}
