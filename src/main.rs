use donyeh::prelude::*;

fn print_board(board: &Board) {
    for y in (0..10).rev() {
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
    println!("分数：{}", SimpleEvaluator::new().evaluate(board, Side::Red));
}

fn main() {
    print_board(&Board::new());
    let mut game = Game::new(
        MaxMinDecider::new(SimpleEvaluator::new(), 10000000),
        RandomDecider::new()
    );
    game.bind_on_move(|board, _| {
        print_board(board);
    });
    game.go(&mut Board::new()).unwrap();
}
