use std::rc::Rc;
use std::cell::RefCell;

use super::chess_piece::ChessPiece;
use super::player;
use super::player::Team;

#[derive(Debug)]
pub struct Tile<'a>
{
    _position : (u32,u32),
    _piece_on : Option<&'a Box< dyn ChessPiece >> ,
}

// impl Clone for Tile
// {
//     fn clone(&self) -> Tile { *self }
// }

impl<'a> Tile<'a>
{
    pub fn new(x : &u32 , y : &u32) -> Tile<'a>
    {
        let res = Tile {
            _position : (*x,*y),
            _piece_on : None ,
        };
        return res ;
    }
    pub fn set_coord(& mut self, coord : &(u32,u32)) 
    {
        self._position = *coord ;
    }
}

pub const BOARD_SIZE : usize = 8 ;
pub struct Board<'a>
{
    pub _tiles : Vec<Vec<Tile<'a>>>,
}  

impl<'a>  Board<'a> 
{
    pub fn new() -> Board<'a>
    {
        let mut board = Board
        {
            _tiles : Vec::with_capacity(BOARD_SIZE*2) //vec![vec![Tile::new(&0,&0) ; BOARD_SIZE] ; BOARD_SIZE] ,
        };

        for j in 0..board._tiles.len()
        {
            for i in 0..board._tiles[0].len()
            {
                board._tiles[i][j].set_coord(& (i as u32 ,j as u32));
            }
        }
        return board;
    }

    /// add player pieces on their associated tiles on the board 
    pub fn setup_player_pieces(& mut self , player : &'a player::Player) ->() 
    {
        for piece in &player._pieces
        {   
            let x = (*piece).position().0 as usize ;
            let y = (*piece).position().1 as usize ;
            self._tiles[x][y]._piece_on = Some(piece);
        }
    }

    pub fn is_occupied( &self , position : &(i32,i32)) -> Option<Team>
    {
        let x : usize = position.0 as usize ;
        let y : usize = position.1 as usize ;
        match &self._tiles[x][y]._piece_on 
        {
            Option::Some(ChessPiece) => Option::Some(ChessPiece.team()) ,
            Option::None => None ,
        }
    }
}
