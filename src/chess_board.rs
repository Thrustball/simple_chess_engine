#![allow(dead_code)]

use std::fmt;
#[derive(Clone, Debug)]
pub enum Figure {
    Rook, // Bauer
    Bishop, // Läufer
    Knight, // Pferdchen
    Caste, // Turm
    King, // König
    Queen, // Dame
}

impl fmt::Display for Figure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Clone, Debug)]
pub enum Colour {
    Black,
    White,
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct Piece {
    figure: Figure,
    colour: Colour,
}
#[derive(Clone)]
pub struct Field {
    piece: Option<Piece>,
}

impl Field {
    pub fn new(figure: Figure, colour: Colour) -> Field {
        Field {piece: Some(Piece {figure: figure, colour: colour}),}
    }
}

pub struct Board {
    board: Vec<Field>,
}

impl Board {
    pub fn new() -> Board {
        let mut field: Vec<Field> = Vec::new();
        let board_size = 64usize;
        for _ in 0..board_size {
            field.push(Field { piece: None });
        }
        Board { board: field }
    }

    pub fn set_field(&mut self, field: Field, x: usize, y: usize) -> Result<(), String> {

       let index = y * 8 + x; 
       if index > 64 {
           return Err(String::from("Out of field"))
       }

       self.board[index] = field;

       return Ok(())
    }

    pub fn get_field(&self, x: usize, y: usize) -> Field {
        let index = y * 8 + x;
        let f: Field = self.board[index].clone();
        f
    }

    pub fn print_board(&self) {
        for i in 0..self.board.len() {
            let cur_field = self.board[i].clone();
            let cur_piece = cur_field.piece;
            match cur_piece {
                Some(f) => {
                    let p = f.figure
                                .to_string()
                                .chars()
                                .next().unwrap();
                    let c = f.colour
                                .to_string()
                                .chars()
                                .next().unwrap();
                    print!("{p}{c} ");

                } 
                None => {
                    print!("ee ");
                }
            }
            if (i+1) % 8 == 0 {println!("");}
        }
    }

}