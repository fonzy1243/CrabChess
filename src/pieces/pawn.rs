use crate::board::*;

// --------------------------------------------------
// Pawn Pushes
// --------------------------------------------------

fn w_single_push_targets(w_pawns: u64, empty: u64) -> u64 {
    nort_one(w_pawns) & empty
}

fn w_dbl_push_targets(w_pawns: u64, empty: u64) -> u64 {
    let rank4: u64 = 0x00000000FF000000;
    let single_pushes = w_single_push_targets(w_pawns, empty);
    nort_one(single_pushes) & empty & rank4
}

fn b_single_push_targets(b_pawns: u64, empty: u64) -> u64 {
    sout_one(b_pawns) & empty
}

fn b_dbl_push_targets(b_pawns: u64, empty: u64) -> u64 {
    let rank5: u64 = 0x00000000FF000000;
    let single_pushes = b_single_push_targets(b_pawns, empty);
    sout_one(single_pushes) & empty & rank5
}

fn w_pawns_able_to_push(w_pawns: u64, empty: u64) -> u64 {
    sout_one(empty) & w_pawns
}

fn w_pawns_able_to_dbl_push(w_pawns: u64, empty: u64) -> u64 {
    let rank4: u64 = 0x00000000FF000000;
    let empty_rank3 = sout_one(empty & rank4) & empty;
    w_pawns_able_to_push(w_pawns, empty_rank3)
}

fn single_push_targets(pawns: u64, empty: u64, color: i32) -> u64 {
    rotate_left(pawns, 8 - (color << 4)) & empty
}

// --------------------------------------------------
// Pawn Attacks
// --------------------------------------------------

fn w_pawn_east_attacks(w_pawns: u64) -> u64 {
    no_ea_one(w_pawns)
}

fn w_pawn_west_attacks(w_pawns: u64) -> u64 {
    no_we_one(w_pawns)
}

fn b_pawn_east_attacks(b_pawns: u64) -> u64 {
    so_ea_one(b_pawns)
}

fn b_pawn_west_attacks(b_pawns: u64) -> u64 {
    so_we_one(b_pawns)
}

fn w_pawn_any_attacks(w_pawns: u64) -> u64 {
    w_pawn_east_attacks(w_pawns) | w_pawn_west_attacks(w_pawns)
}

fn w_pawn_dbl_attacks(w_pawns: u64) -> u64 {
    w_pawn_east_attacks(w_pawns) & w_pawn_west_attacks(w_pawns)
}

fn w_pawn_single_attacks(w_pawns: u64) -> u64 {
    w_pawn_east_attacks(w_pawns) ^ w_pawn_west_attacks(w_pawns)
}

fn b_pawn_any_attacks(b_pawns: u64) -> u64 {
    b_pawn_east_attacks(b_pawns) | b_pawn_west_attacks(b_pawns)
}

fn b_pawn_dbl_attacks(b_pawns: u64) -> u64 {
    b_pawn_east_attacks(b_pawns) & b_pawn_west_attacks(b_pawns)
}

fn b_pawn_single_attacks(b_pawns: u64) -> u64 {
    b_pawn_east_attacks(b_pawns) ^ b_pawn_west_attacks(b_pawns)
}

// Captures
fn w_pawns_able_to_capture_east(w_pawns: u64, b_pieces: u64) -> u64 {
    w_pawns & b_pawn_west_attacks(b_pieces)
}

fn w_pawns_able_to_capture_west(w_pawns: u64, b_pieces: u64) -> u64 {
    w_pawns & b_pawn_east_attacks(b_pieces)
}

fn w_pawns_able_to_capture_any(w_pawns: u64, b_pieces: u64) -> u64 {
    w_pawns & b_pawn_any_attacks(b_pieces)
}
