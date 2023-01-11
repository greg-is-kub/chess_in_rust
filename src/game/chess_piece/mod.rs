////////////////////////
// std include
use core::fmt::Debug;

///////////////////////
// my includes
use crate::game::player::Team;
use crate::game::board::Board;

pub mod pawn;
pub mod tower;
pub mod knight;
pub mod bischop;
pub mod queen;
pub mod king;


enum MoveCondition
{
    MustBeEmpty,
    MustBeOccupied,
    MustNotBeAttacked,
}


pub trait ChessPiece 
{
    //////////////////////////////////////////
    // constructor
    //////////////////////////////////////////
    fn new(position : &(i32 , i32), owner : &Team ) -> Self where Self: Sized;
    
    //////////////////////////////////////////
    // getters
    //////////////////////////////////////////
    fn is_alive(& self) -> bool;

    fn team(&self) -> Team ;

    fn position(&self) -> (i32,i32);
    
    //////////////////////////////////////////
    // methods
    //////////////////////////////////////////
    /// generate all possible moves that can be done by a given piece
    fn generate_moves(&self) -> Vec<(i32,i32)>;
    
    // generates self._allowed moves with the move list given by generate_moves 
    fn check_moves(& mut self , board: &Board ) -> &Vec<(i32,i32)>;

    fn can_eat(&self, other_team : &Team ) -> bool
    {
        self.team() != *other_team
    }
}


impl Debug for dyn ChessPiece {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "oui")
    }
}