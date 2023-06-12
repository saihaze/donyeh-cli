use std::io::stdin;

use donyeh::prelude::*;

#[derive(Debug, Clone, Copy)]
struct PlayerDecider {}

impl Decider for PlayerDecider {
    fn make_decision(&self, board: &Board, side: Side) -> Option<Move> {
        let side_str = match side {
            Side::Red => "红方",
            Side::Black => "黑方",
        };
        println!("现在由你走子。你是{}。", side_str);
        loop {
            println!("从：");
            let mut buf = String::new();
            match stdin().read_line(&mut buf) {
                Ok(_) => (),
                Err(_) => continue,
            }
            buf.pop();
            let numbers = buf.split(" ").collect::<Vec<&str>>();
            if numbers.len() < 2 {
                continue;
            }
            let x_from = match numbers[0].parse::<i32>() {
                Ok(value) => value,
                Err(_) => continue,
            };
            let y_from = match numbers[1].parse::<i32>() {
                Ok(value) => value,
                Err(_) => continue,
            };
            println!("到：");
            buf = String::new();
            match stdin().read_line(&mut buf) {
                Ok(_) => (),
                Err(_) => continue,
            }
            buf.pop();
            let numbers = buf.split(" ").collect::<Vec<&str>>();
            if numbers.len() < 2 {
                continue;
            }
            let x_to = match numbers[0].parse::<i32>() {
                Ok(value) => value,
                Err(_) => continue,
            };
            let y_to = match numbers[1].parse::<i32>() {
                Ok(value) => value,
                Err(_) => continue,
            };
            let from = (x_from, y_from);
            let to = (x_to, y_to);
            if x_from < 0 || x_from > 8 || y_from < 0 || y_from > 9 {
                continue;
            }
            let piece_to_be_moved = board.get_piece_at(from);
            let mov = Move::new(from, to, piece_to_be_moved);
            if !board.check_move(&mov) {
                println!("err 2");
                println!("{:?}", &mov);
                continue;
            }
            return Some(mov);
        }
    }
}

fn print_board(board: &Board) {
    for y in (0..10).rev() {
        print!("{}", match y {
            0 => "〇",
            1 => "一",
            2 => "二",
            3 => "三",
            4 => "四",
            5 => "五",
            6 => "六",
            7 => "七",
            8 => "八",
            9 => "九",
            _ => unreachable!(),
        });
        for x in 0..9 {
            let piece = board.get_piece_at((x, y));
            match piece {
                Some(piece) => {
                    let ch = match piece.kind {
                        PieceKind::帥 => "帥",
                        PieceKind::車 => "車",
                        PieceKind::馬 => "馬",
                        PieceKind::炮 => "炮",
                        PieceKind::相 => "相",
                        PieceKind::仕 => "仕",
                        PieceKind::中兵 | PieceKind::濟兵 | PieceKind::庶兵 | PieceKind::底兵 => {
                            "兵"
                        }
                    };
                    if piece.side == Side::Red {
                        print!("{}", ansi_term::Color::Red.paint(ch));
                    } else {
                        print!("{}", ansi_term::Color::Green.paint(ch));
                    }
                }
                None => {
                    print!("　")
                }
            }
        }
        println!("");
    }
    println!("　〇一二三四五六七八九");
    println!(
        "分数：{}",
        SimpleEvaluator::new().evaluate(board, Side::Red)
    );
}

fn main() {
    print_board(&Board::new());
    let mut game = Game::new(
        PlayerDecider {},
        MaxMinDecider::new(SimpleEvaluator::new(), 10000000),
    );
    game.bind_on_move(|board, _| {
        print_board(board);
    });
    game.go(&mut Board::new()).unwrap();
}
