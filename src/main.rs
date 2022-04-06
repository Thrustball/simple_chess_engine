mod chess_board;
fn main() {
    let mut board = chess_board::Board::new();
    let rook = chess_board::Field::new(
       chess_board::Figure::Rook, 
       chess_board::Colour::White);
    match board.set_field(rook.clone(), 3, 4) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    };
    board.print_board();
}
