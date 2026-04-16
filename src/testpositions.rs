use core::str::FromStr;

#[allow(dead_code)]
use super::*;
use position::Position;
use piece::Piece;
use board::Board;


#[test]
fn test_default(){
    let fen = "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - - 1";
    let board: Board = 
        Board::from_str(fen).unwrap();
    assert_eq!(Board::default(), board);
    assert_eq!(board.get_turn_color(), Color::Red);
}
#[test]
fn test_check(){
    let fen = "3rkr3/9/9/9/9/9/9/9/3KA4/9 w - - - 1";
    let board: Board = 
        Board::from_str(fen).unwrap();
    assert_eq!(board.get_turn_color(), Color::Red);
    assert!(board.is_in_check(RED));
}
#[test]
fn test_mate(){
    let fen = "1R7/4kc3/9/9/9/9/9/9/4A4/3KC4 w - - - 1";
    let board: Board = 
        Board::from_str(fen).unwrap();
    assert!(board.is_checkmate());
}
#[test]
fn test_cannon(){
    let fen = "4k4/4a4/9/4h4/9/9/4C4/9/9/4K4 w - - - 1";
    let board: Board = 
        Board::from_str(fen).unwrap();
    assert!(board.is_legal_move(Move::Piece(Position::new(3, 4), Position::new(5, 4)), RED));
    assert!(!board.is_legal_move(Move::Piece(Position::new(3, 4), Position::new(6, 4)), RED));
    assert!(board.is_threatened(Position::new(8, 4),BLACK));

}
#[test]
fn test_pawn_past_river(){
    let fen = "1R7/4kc3/9/9/9/9/9/9/4A4/3KC4 w - - - 1";
    let board: Board = 
        Board::from_str(fen).unwrap();
}
#[test]
fn test_pawn(){
    let fen = "1R7/4kc3/9/9/9/9/9/9/4A4/3KC4 w - - - 1";
    let board: Board = 
        Board::from_str(fen).unwrap();
}
#[test]
fn test_horse(){
    let fen = "1R7/4kc3/9/9/9/9/9/9/4A4/3KC4 w - - - 1";
    let board: Board = 
        Board::from_str(fen).unwrap();
}
#[test]
fn test_elephant(){
    let fen = "1R7/4kc3/9/9/9/9/9/9/4A4/3KC4 w - - - 1";
    let board: Board = 
        Board::from_str(fen).unwrap();
}
#[test]
fn test_illegal_move(){
    let fen = "1R7/4kc3/9/9/9/9/9/9/4A4/3KC4 w - - - 1";
    let board: Board = 
        Board::from_str(fen).unwrap();
}
#[test]
fn test_out_of_bounds_move(){
let fen = "1R7/4kc3/9/9/9/9/9/9/4A4/3KC4 w - - - 1";
    let board: Board = 
        Board::from_str(fen).unwrap();
}