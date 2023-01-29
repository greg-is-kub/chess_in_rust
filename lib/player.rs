// use std::collections::HashMap;
use super::chess_piece::ChessPiece;

// use super::board::BOARD_SIZE;

const PLAYER_PIECE_NUMBER:usize= 16;

#[derive(Debug,Copy,Clone, std::cmp::PartialEq)]
pub enum Team
{
    White,
    Black,
}

impl Default for Team 
{
    fn default() -> Team{
        Team::White
    }
}

#[derive(Debug)]
pub struct Player//<T>
where 
    // T: ChessPiece + ?Sized
{
    pub _team : Team,
    pub _pieces : Vec<Box<dyn ChessPiece>>, //box because ChessPiece doesn't have a know siez at compile time
    pub _points : u32,
}

// impl<T> Player<T>
impl Player
// where 
    // T : ChessPiece
{
    fn set_pieces(& mut self , team : Team ) -> ()
    {   
        // let y_offset : u32 ;

        // match team 
        // {
        //     Team::White => 
        //     {
        //         y_offset = 0;
        //         //create pawns
        //         for i in 0..BOARD_SIZE
        //         {
        //             self._pieces.push(Piece::new(PieceType::Pawn , &(2,i.try_into().unwrap()) , team ));
        //         }
        //         //create king and queen
        //         self._pieces.push(Piece::new(PieceType::Queen, &( y_offset , (BOARD_SIZE-4).try_into().unwrap() ), team ));
        //         self._pieces.push(Piece::new(PieceType::King, &( y_offset , 3 ), team ));
        //     }
        //     Team::Black => 
        //     {
        //         y_offset = (BOARD_SIZE-1).try_into().unwrap();
        //         //create pawns
        //         for i in 0..BOARD_SIZE
        //         {
        //             self._pieces.push(Piece::new(PieceType::Pawn , &((BOARD_SIZE-1).try_into().unwrap() ,i.try_into().unwrap()) , team ));
        //         }
        //         //create king and queen
        //         self._pieces.push(Piece::new(PieceType::Queen, &(y_offset , 3), team ));
        //         self._pieces.push(Piece::new(PieceType::King, &(y_offset , (BOARD_SIZE-4).try_into().unwrap() ), team ));
        //     }
        // }
        // //create pawns
        // for i in 0..BOARD_SIZE
        // {
        //     self._pieces.push(Piece::new(PieceType::Pawn , &(2,i.try_into().unwrap()) , team ));
        // }
        // //create towers
        // self._pieces.push(Piece::new(PieceType::Tower, &(y_offset , 0), team ));
        // self._pieces.push(Piece::new(PieceType::Tower, &(y_offset , (BOARD_SIZE-1).try_into().unwrap() ), team ));
        // //create knights
        // self._pieces.push(Piece::new(PieceType::Knight, &(y_offset , 1), team ));
        // self._pieces.push(Piece::new(PieceType::Knight, &(y_offset , (BOARD_SIZE-2).try_into().unwrap() ), team ));
        // //create bischops
        // self._pieces.push(Piece::new(PieceType::Bischop, &(y_offset , 2), team ));
        // self._pieces.push(Piece::new(PieceType::Bischop, &(y_offset , (BOARD_SIZE-3).try_into().unwrap() ), team ));
    }
   
   
    pub fn new(team : Team) -> Player//<T>
    {
        let mut player = Player//::<T>
        {
            _team : team,
            _pieces : Vec::with_capacity(PLAYER_PIECE_NUMBER),
            _points : 0 ,
        };
        player.set_pieces( team );

        return player;
    }
}