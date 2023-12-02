use sdk::{
    bitboard::{Bitboard, Direction},
    position::{Color, Piece, Position},
    square::FILE_MASKS,
};

pub const PROTECTED_PASSED_PAWNS_BONUS: i32 = 20;

pub fn bonus_for_protected_passed_pawnes(pos: &Position) -> i32 {
    let white_protected_passed_pawns =
        mask_protected_passed_pawns(pos, Color::White).count() as i32;
    let black_protected_passed_pawns =
        mask_protected_passed_pawns(pos, Color::Black).count() as i32;

    (white_protected_passed_pawns - black_protected_passed_pawns) * PROTECTED_PASSED_PAWNS_BONUS
}

pub fn mask_protected_passed_pawns(pos: &Position, color: Color) -> Bitboard {
    let our_pawns = pos.pieces[color as usize][Piece::Pawn as usize];
    let enemy_pawns = pos.pieces[color.enemy() as usize][Piece::Pawn as usize];

    let dirs = if color == Color::White {
        [Direction::NorthEast, Direction::NorthWest]
    } else {
        [Direction::SouthEast, Direction::SouthWest]
    };

    let protected_pawns = (our_pawns.shift(&dirs[0]) | our_pawns.shift(&dirs[1])) & our_pawns;

    let mut protected_passed_pawns = Bitboard::empty();
    for pawn in protected_pawns {
        let file = pawn.file() as usize;

        let no_enemy_pawns_on_left_file =
            file == 0 || (enemy_pawns & FILE_MASKS[file - 1]).is_empty();
        let no_enemy_pawns_on_right_file =
            file == 7 || (enemy_pawns & FILE_MASKS[file + 1]).is_empty();

        if no_enemy_pawns_on_left_file && no_enemy_pawns_on_right_file {
            protected_passed_pawns |= pawn.bitboard();
        }
    }

    protected_passed_pawns
}
