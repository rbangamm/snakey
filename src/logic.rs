// Welcome to
// __________         __    __  .__                               __
// \______   \_____ _/  |__/  |_|  |   ____   ______ ____ _____  |  | __ ____
//  |    |  _/\__  \\   __\   __\  | _/ __ \ /  ___//    \\__  \ |  |/ // __ \
//  |    |   \ / __ \|  |  |  | |  |_\  ___/ \___ \|   |  \/ __ \|    <\  ___/
//  |________/(______/__|  |__| |____/\_____>______>___|__(______/__|__\\_____>
//
// This file can be a nice home for your Battlesnake logic and helper functions.
//
// To get you started we've included code to prevent your Battlesnake from moving backwards.
// For more info see docs.battlesnake.com

use log::info;
use serde_json::{json, Value};

use crate::{Battlesnake, Board, Game};


#[derive(PartialEq)]
pub enum Move {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE
}

impl Move {
    fn value(&self) -> String {
        match *self {
            Move::UP => String::from("up"),
            Move::DOWN => String::from("down"),
            Move::LEFT => String::from("left"),
            Move::RIGHT => String::from("right"),
            Move::NONE => String::from("none")
        }
    }
}

// info is called when you create your Battlesnake on play.battlesnake.com
// and controls your Battlesnake's appearance
// TIP: If you open your Battlesnake URL in a browser you should see this data
pub fn info() -> Value {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "rav", // TODO: Your Battlesnake Username
        "color": "#888888", // TODO: Choose color
        "head": "default", // TODO: Choose head
        "tail": "default", // TODO: Choose tail
    });
}

// TODO: Add checks for body, out of bounds, and opponents

// start is called when your Battlesnake begins a game
pub fn start(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

// end is called when your Battlesnake finishes a game
pub fn end(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

// move is called on every turn and returns your next move
// Valid moves are "up", "down", "left", or "right"
// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move(_game: &Game, turn: &u32, _board: &Board, you: &Battlesnake) -> Value {

    // We've included code to prevent your Battlesnake from moving backwards
    let my_head = &you.body[0]; // Coordinates of your head
    let my_neck = &you.body[1]; // Coordinates of your "neck"
    let movement_dir = match (my_head, my_neck) {
        (my_head, my_neck) if my_neck.x < my_head.x => Move::RIGHT,
        (my_head, my_neck) if my_neck.x > my_head.x => Move::LEFT,
        (my_head, my_neck) if my_neck.y < my_head.y => Move::UP,
        (my_head, my_neck) if my_neck.y > my_head.y => Move::DOWN,
        (_, _) => Move::NONE
    };

    let food = &_board.food;
    let priorities = food.iter().map(|f| {
        let x = f.x;
        let y = f.y;
        if y == my_head.y {
            if (x > my_head.x && movement_dir == Move::RIGHT) || (x < my_head.x && movement_dir == Move::LEFT) {
                return (x, y, 3)
            }
            return (x, y, 2)
        } else if x == my_head.x {
            if (y > my_head.y && movement_dir == Move::UP) || (y < my_head.y && movement_dir == Move::DOWN) {
                return (x, y, 3)
            }
            return (x, y, 2)
        } else {
            return (x, y, 1)
        } 
    });

    const MIN_VAL: i32 = -1000;

    let left_score : i32 = priorities
        .clone()
        .filter(|(x, _, _)| x < &my_head.x)
        .map(|(_, _, score)| if movement_dir != Move::RIGHT {score} else {MIN_VAL})
        .sum();
    let right_score : i32 = priorities
        .clone()
        .filter(|(x, _, _)| x > &my_head.x)
        .map(|(_, _, score)| if movement_dir != Move::LEFT {score} else {MIN_VAL})
        .sum();
    let up_score : i32 = priorities
        .clone()
        .filter(|(_, y, _)| y > &my_head.y)
        .map(|(_, _, score)| if movement_dir != Move::DOWN {score} else {MIN_VAL})
        .sum();
    let down_score : i32 = priorities
        .clone()
        .filter(|(_, y, _)| y < &my_head.y)
        .map(|(_, _, score)| if movement_dir != Move::UP {score} else {MIN_VAL})
        .sum();

    let scores = [(left_score, Move::LEFT), (right_score, Move::RIGHT), (up_score, Move::UP), (down_score, Move::DOWN)];


    // TODO: Step 1 - Prevent your Battlesnake from moving out of bounds
    // let board_width = &board.width;
    // let board_height = &board.height;

    // TODO: Step 2 - Prevent your Battlesnake from colliding with itself
    // let my_body = &you.body;

    // TODO: Step 3 - Prevent your Battlesnake from colliding with other Battlesnakes
    // let opponents = &board.snakes;

    // Are there any safe moves left?
    
    // Choose a random move from the safe ones
    let chosen = match scores.iter()
        .max_by(|(score_1, _), (score_2, _)| score_1.cmp(score_2)) {
            Some((_, m)) => m.value(),
            None => String::from("")
        };

    // TODO: Step 4 - Move towards food instead of random, to regain health and survive longer
    // let food = &board.food;

    info!("MOVE {}: {}", turn, chosen);
    return json!({ "move": chosen });
}
