use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, usize};

use chess_networking::{Ack, Move, PromotionPiece, Start};
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
    let mut args = std::env::args();

    clear_background(BLACK);
    next_frame().await;

    let (mut game, local_turn_is_white, mut socket) = if let Some(arg) = args.nth(1) {
        if arg.eq("-c") {
            let ip = args.next().expect("Expected ip and port as second argument");
            println!("connecting to {:?}", ip);
            let mut socket = TcpStream::connect(ip).expect("Expected to connect to server");

            println!("connected");

            println!("setting up");
            let start: Vec<u8> = Start {
                is_white: true,
                name: None,
                fen: None,
                time: None,
                inc: None,
            }.try_into().expect("Expected to be able to serialize start struct");

            socket.write_all(&start).expect("Expected to send start struct");

            println!("setup sent");

            let mut buf = [0; 128];
            let amount = socket.read(&mut buf).expect("Expected to be able to read start respones");

            let start_res: Start = buf[0..amount].try_into().expect("Recived wrong struct");

            let game = if let Some(fen) = start_res.fen {
                let mut game = ChessBoard::new();
                game.load(fen);
                game
            } else {
                ChessBoard::new()
            };

            (game, !start_res.is_white, socket)
        } else {
            panic!("failed to connect");
        }
    } else {
        println!("listening");
        let listener = TcpListener::bind("127.0.0.1:8080").expect("Expected to be able to bind port");

        let (mut socket, start) = if let Ok((mut socket, _addr)) = listener.accept() {
            println!("connection recived");
            let mut buf = [0; 128];
            let amount = socket.read(&mut buf).expect("Expected to be able to read start message");

            (socket, Start::try_from(&buf[0..amount]).expect("Recived wrong struct"))
        } else {
            panic!("Failed to establish connection");
        };

        println!("setting up");

        let start_res: Vec<u8> = Start {
            is_white: !start.is_white,
            name: None,
            fen: None,
            time: None,
            inc: None
        }.try_into().expect("Expected to be able to serialize start struct");

        socket.write_all(&start_res).unwrap();

        (ChessBoard::new(), !start.is_white, socket)
    };

    println!("setup complete");

    let ceris = Color::from_hex(0xE83D84);
    let green = Color::from_hex(0x17c27b);

    let king = load_texture("./img/Chess_klt45.svg.png").await.expect("Expected to load texture");
    let queen = load_texture("./img/Chess_qlt45.svg.png").await.expect("Expected to load texture");
    let bishop = load_texture("./img/Chess_blt45.svg.png").await.expect("Expected to load texture");
    let knight = load_texture("./img/Chess_nlt45.svg.png").await.expect("Expected to load texture");
    let rook = load_texture("./img/Chess_rlt45.svg.png").await.expect("Expected to load texture");
    let pawn = load_texture("./img/Chess_plt45.svg.png").await.expect("Expected to load texture");

    let mut squares = [Square::default(); 64];

    for (index, square) in squares.iter_mut().enumerate() {
        square.index = index;
        square.x = index % 8;
        square.y = index / 8;
    }

    let mut is_promote = None;

    let mut current_index: Option<usize> = None;

    while game.game_result == GameResult::Ongoing {
        clear_background(BLACK);

        let square_size = if screen_width() > screen_height() {
            screen_height() / 8.0
        } else {
            screen_width() / 8.0
        };

        if game.whites_turn == local_turn_is_white {
            if is_mouse_button_pressed(MouseButton::Left) {
                handle_input(&mut game, square_size, &mut current_index, &mut is_promote, &mut socket);
            }
        } else {
            let mut buf = [0; 128];
            let amount = socket.read(&mut buf).expect("Expected to be able to read move");

            let performed_move: Move = buf[0..amount].try_into().expect("Recived wrong struct");
            let promote = performed_move.promotion;
            let from = performed_move.from.0 + performed_move.from.1 * 8;
            let to = performed_move.to.0 + performed_move.to.1 * 8;

            let mut temp_game = game.clone();
            if let Some(piece) = promote {
                let piece = match piece {
                    PromotionPiece::Queen => if game.whites_turn { PieceType::WhiteQueen } else { PieceType::BlackQueen },
                    PromotionPiece::Bishop => if game.whites_turn { PieceType::WhiteBishop } else { PieceType::BlackBishop },
                    PromotionPiece::Knight => if game.whites_turn { PieceType::WhiteKnight } else { PieceType::BlackKnight },
                    PromotionPiece::Rook => if game.whites_turn { PieceType::WhiteRook } else { PieceType::BlackRook },
                };

                if temp_game.handle_promotion(from.into(), to.into(), piece).is_ok() {
                    game.handle_promotion(from.into(), to.into(), piece).expect("Expected to be able to performe same move");

                    let ack: Vec<u8> = Ack {
                        ok: true,
                        end_state: None
                    }.try_into().expect("Expected to be able to serialize struct");

                    socket.write_all(&ack).expect("Expected to be able to send ack");
                } else {
                    let ack: Vec<u8> = Ack {
                        ok: false,
                        end_state: None
                    }.try_into().expect("Expected to be able to serialize struct");

                    socket.write_all(&ack).expect("Expected to be able to send ack");
                }
            } else {
                if temp_game.move_piece(from.into(), to.into()).is_ok() {
                    game.move_piece(from.into(), to.into()).expect("Expected to be able to performe same move");

                    let ack: Vec<u8> = Ack {
                        ok: true,
                        end_state: None
                    }.try_into().expect("Expected to be able to serialize struct");

                    socket.write_all(&ack).expect("Expected to be able to send ack");
                } else {
                    let ack: Vec<u8> = Ack {
                        ok: false,
                        end_state: None
                    }.try_into().expect("Expected to be able to serialize struct");

                    socket.write_all(&ack).expect("Expected to be able to send ack");
                }
            }
        }

        let moves = if let Some(index) = current_index {
            game.get_moves_list(index)
        } else {
            Vec::new()
        };

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

        if is_promote.is_some() {
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
    loop {}
}

fn select(square_size: f32) -> (usize, usize) {
    let (x, y) = mouse_position();
    let x = (x / square_size).floor() as usize;
    let y = (y / square_size).floor() as usize;

    println!("{x}, {y}");

    (x, y)
}

fn net_move(from: (u8, u8), to: (u8, u8), promotion: Option<PieceType>, socket: &mut TcpStream) -> bool {
    let promotion = if let Some(piece) = promotion {
        match piece {
            WhiteQueen | BlackQueen => Some(PromotionPiece::Queen),
            WhiteBishop | BlackBishop => Some(PromotionPiece::Bishop),
            WhiteKnight | BlackKnight => Some(PromotionPiece::Knight),
            WhiteRook | BlackRook => Some(PromotionPiece::Rook),
            _ => None,
        }
    } else {
        None
    };

    println!("Netmove: {promotion:?}");

    let performed_move: Vec<u8> = Move {
        from,
        to,
        promotion,
        forfeit: false,
        offer_draw: false,
    }.try_into().expect("Expected to be able to serialize struct");

    socket.write_all(&performed_move).expect("Expected to be able to send move");

    let mut buf = [0; 1024];
    let amount = socket.read(&mut buf).expect("Expected to be able to recive ack");

    let ack = Ack::try_from(&buf[0..amount]).expect("Recived wrong struct");
    ack.ok
}

fn handle_input(game: &mut ChessBoard, square_size: f32, current: &mut Option<usize>, is_promote: &mut Option<(u8, u8)>, socket: &mut TcpStream) {
    let (x, y) = select(square_size);
    let index = x + y * 8;

    if let Some(current_index) = current {
        let current_x = *current_index % 8;
        let current_y = *current_index / 8;

        if let Some(promote) = is_promote {
            let (x, y) = select(square_size);
            println!("promote: {x}, {y}");

            let top = 3.5;
            let bottom = 4.5;
            let left = 2.0;
            let queen = 3.0;
            let bishop = 4.0;
            let knight = 5.0;
            let rook = 6.0;

            if y as f32 > top
            && (y as f32) < bottom
            && (x as f32) > left {

                let piece = if (x as f32) < queen {
                    if game.whites_turn {
                        Some(PieceType::WhiteQueen)
                    } else {
                        Some(PieceType::BlackQueen)
                    }
                } else if (x as f32) < bishop {
                    if game.whites_turn {
                        Some(PieceType::WhiteBishop)
                    } else {
                        Some(PieceType::BlackBishop)
                    }
                } else if (x as f32) < knight {
                    if game.whites_turn {
                        Some(PieceType::WhiteKnight)
                    } else {
                        Some(PieceType::BlackKnight)
                    }
                } else if (x as f32) < rook {
                    if game.whites_turn {
                        Some(PieceType::WhiteRook)
                    } else {
                        Some(PieceType::BlackRook)
                    }
                } else {
                    None
                };

                if let Some(piece) = piece {
                    let mut temp_game = (*game).clone();
                    println!("{:?}", piece);
                    if temp_game.handle_promotion(*current_index, (promote.0 + promote.1 * 8).into(), piece).unwrap() {
                        let from = (current_x as u8, current_y as u8);
                        let to = *promote;

                        if net_move(from, to, Some(piece), socket) {
                            game.handle_promotion(*current_index, (promote.0 + promote.1 * 8).into(), piece)
                                .unwrap();

                            *is_promote = None;
                            *current = None;
                        }
                    }
                    *is_promote = None;
                    *current = None;
                }
            }
        } else {
            let mut temp_game = (*game).clone();
            match temp_game.move_piece(*current_index, index) {
                Ok(true) => {
                    let from = (current_x as u8, current_y as u8);
                    let to = (x as u8, y as u8);
                    if net_move(from, to, None, socket) {
                        game.move_piece(*current_index, index).unwrap();
                    }
                    *is_promote = None;
                    *current = None;
                },
                Ok(false) => *is_promote = Some((x.try_into().unwrap(), y.try_into().unwrap())),
                Err(_) => {
                    *current = None;
                    return
                }
            }
        } 
    } else {
        *current = Some(x + y * 8);
    }
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
