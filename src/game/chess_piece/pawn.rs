use crate::game::player::Team;
use crate::game::board::Board;

use super::ChessPiece;


#[derive(Debug,Default)]
pub struct Pawn 
{
    _owned_by : Team ,
    pub _position : (i32,i32) ,
    _allowed_moves : Vec<(i32,i32)>,
    _unique_id : u32 ,
    _alive : bool ,
}

impl ChessPiece for Pawn
{
    fn new( position :&(i32,i32), owner : &Team ) -> Pawn
    {
        let mut res = Pawn {
            _position : *position,
            _unique_id : 0 ,
            _alive : true ,
            _owned_by : *owner ,
            _allowed_moves : Vec::with_capacity(3),
        };
        return res;
    }

    fn is_alive(&self) -> bool
    {
        self._alive
    }

    fn team(&self) -> Team
    {
        self._owned_by
    }

    fn position(&self) -> (i32,i32)
    {
        return self._position ;
    }

    fn generate_moves(&self) -> Vec<(i32,i32)>
    {
        let y_move : i32 ;
        match self._owned_by {
            Team::White =>  y_move = 1 ,
            Team::Black =>  y_move = -1 ,
        }
        return vec![(self._position.0-1, self._position.1+y_move), (self._position.0+1, self._position.1+y_move)]
    }

    fn check_moves(& mut self, board : &Board) -> &Vec<(i32,i32)>
    {
        self._allowed_moves.clear();

        let forward_move = (self._position.0, self._position.1+1);
        // check forward tile
        if let None = board.is_occupied(&forward_move) 
        {
            self._allowed_moves.push(forward_move);
        }
        
        //check diagonals
        let eat_moves = self.generate_moves();
        let mut  i = 0 ;
        for eat_move in eat_moves
        {
            let Some(team) = board.is_occupied(&eat_move) else
            {
                continue;
            };
            if self.can_eat(&team)
            {
                self._allowed_moves.push(eat_move);
            }
            i += 1;
        }
        return &self._allowed_moves;
    }

    // let some_piece = board.is_occupied(&forward_move) ;
    // match some_piece {
    //     Some(piece)=>self._allowed_moves.push(forward_move),
    //     None => forward_move = forward_move,
    // } 

    

}
