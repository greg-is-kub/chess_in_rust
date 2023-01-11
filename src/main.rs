// use log::* ;

// use console_engine::pixel;
// use console_engine::Color;
// use console_engine::KeyCode;

mod game;
use game::{*};
use game::chess_piece::ChessPiece;


fn main() {

    let white_player = game::player::Player::new(player::Team::White);
    let black_player = game::player::Player::new(player::Team::Black);
    // println!("created player");
    // println!("{:#?}", white_player );
    let mut engine = console_engine::ConsoleEngine::init(20, 10, 3).unwrap();
    let mut board = game::Game::new( white_player , black_player , engine);
}


// fn main() {
//     // initializes a screen of 20x10 characters with a target of 3 frame per second
//     // coordinates will range from [0,0] to [19,9]
//     let mut engine = console_engine::ConsoleEngine::init(20, 10, 3).unwrap();
//     let value = 14;
//     let board_start : i32 = 15;
//     let board_end :i32 = board_start + (game::board::BOARD_SIZE as i32);
//     // main loop, be aware that you'll have to break it because ctrl+C is captured
//     loop {
//         engine.wait_frame(); // wait for next frame + capture inputs
//         engine.clear_screen(); // reset the screen
//         engine.rect(board_start ,board_start, board_end, board_end , pixel::pxl('░') );
//         // engine.line(0, 0, 19, 9, pixel::pxl('█')); // draw a line of '#' from [0,0] to [19,9]
//         engine.print(0, 4, format!("Result: {}", value).as_str()); // prints some value at [0,4]

//         engine.set_pxl(4, 0, pixel::pxl_fg('O', Color::Cyan)); // write a majestic cyan 'O' at [4,0]

//         if engine.is_key_pressed(KeyCode::Char('q')) { // if the user presses 'q' :
//             break; // exits app
//         }

//         engine.draw(); // draw the screen
//     }
// }