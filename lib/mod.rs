
use std::error::Error;
use std::cell::RefCell;
use std::rc::Rc;

use console_engine;

pub mod player;
pub mod board;
pub mod chess_piece ;

use player::*;


pub struct Game<'a>
{
    _engine : console_engine::ConsoleEngine,
    _board : board::Board<'a>,
    _players : Vec<Rc<RefCell<Player>>>,
    _current_player : Rc<RefCell<Player>> ,
}

impl<'a> Game<'a>
{
    pub fn new( white_player : Player ,  black_player : Player , engine : console_engine::ConsoleEngine ) -> Game<'a>
    {   
        let mut res = Game {  
            _engine : engine ,
            _board : board::Board::<'a>::new(),
            _players : vec!( 
                Rc::new(RefCell::new(white_player)),
                Rc::new(RefCell::new(black_player)),
            ),
            _current_player : Rc::new(RefCell::new(Player::new(Team::White))),
        };
        res._current_player = Rc::clone(res._players.get(0).unwrap());
        return res ;
    }    

    fn set_pieces(&'a mut self) -> ()
    {
        let white_player = self._players.get(0).unwrap().borrow();
        let black_player = self._players.get(1).unwrap().borrow();
        self._board.setup_player_pieces( & white_player );
        self._board.setup_player_pieces( & black_player);
    }
    fn turn(&self)
    {
        // make_move();
        // next_player();
        todo!();
    }

    fn make_move(&self) -> ()
    {   
        // let mut confirm = false;
        // while !confirm
        // {
            // select_piece();
            // confirm = confirm_move() ; 
        // }
        // move_piece();
        todo!();
    } 

    ///Technically should iterate over self.players to find the one of given Team but currently ugly if
    /// 
    fn get_current_player(&self) -> Rc<RefCell<Player>> //Result<Player,Error>
    {   
        match self._current_player.borrow()._team
        {
            Team::White => return *self._players.get(1).unwrap(),
            Team::Black => return *self._players.get(0).unwrap(),
        } 
    }
    
    fn get_next_player(&self) -> Rc<RefCell<Player>> //Result<Player,Error>
    {   
        match self._current_player.borrow()._team
        {
            Team::White => return *self._players.get(0).unwrap(),
            Team::Black => return *self._players.get(1).unwrap(),
        } 
    }

    pub fn display_board(&self) -> ()
    {
        // for row in &self._board._tiles
        // {
        //     for tile in row
        //     {
               
        //     }
        // }
        todo!();
    }
}//end imp
