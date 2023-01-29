use crate::game::player::Team;

use crate::game::board::Board;
use crate::game::board::BOARD_SIZE;

use super::ChessPiece;


#[derive(Debug,Default)]
pub struct Tower 
{
    _owned_by : Team ,
    pub _position : (i32,i32) ,
    _allowed_moves : Vec<(i32,i32)>,
    _unique_id : u32 ,
    _alive : bool ,
}

// impl ChessPiece for Tower
// {
//     fn new( position :&(i32,i32), owner : &Team ) -> Tower
//     {
//         let mut res = Tower {
//             _position : *position,
//             _unique_id : 0 ,
//             _alive : true ,
//             _owned_by : *owner ,
//             _allowed_moves : Vec::with_capacity(2 * BOARD_SIZE),
//         };
//         return res;
//     }

//     fn is_alive(&self) -> bool
//     {
//         self._alive
//     }

//     fn team(&self) -> Team
//     {
//         self._owned_by
//     }

//     fn position(&self) -> (i32,i32)
//     {
//         self._position
//     }

//     fn generate_moves(&self) -> Vec<(i32,i32)>
//     {
//         let y_move : i32 ;
//         match self._owned_by {
//             Team::White =>  y_move = 1 ,
//             Team::Black =>  y_move = -1 ,
//         }
//         return vec![(self._position.0-1, self._position.1+y_move), (self._position.0+1, self._position.1+y_move)]
//     }

//     fn check_moves<T: ChessPiece>(& mut self, board : &Board<T>) -> Vec<(i32,i32)>
//     {
//         self._allowed_moves.clear();
//         self._allowed_moves.reserve(3);

//         let forward_move = (self._position.0, self._position.1+1);
        
//         let eat_moves = self.generate_moves();
//         // check forward
//         // if let board.is_occupied(move) = None 
//         // {
//         //     self._allowed_moves.push(forward_move)
//         // }

//         // let i = 0 ;
//         // while let board.is_occupied(eat_moves.get(i).uwnwrap()) = Some(piece)
//         // {
//         //     if 
//         //     {
//         //         self._allowed_moves.push(eat_move.get(i));
//         //     }
//         // }
//         // //check diagonals
//         return self._allowed_moves;
//     }

// }