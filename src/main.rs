mod lukas;

use macroquad::prelude::*;

use std::collections::LinkedList;
use std::process::exit;
use macroquad::rand::{gen_range, rand};

///How many Squares the Playground should have
const SQUARES: i16 = 12;


///Used like Coordinates
/// Tuple of 2 ints having the number of the grid they are on right now
type Point = (i16, i16);

/// The Snake
/// Point head -> coordinates of the head
/// LinkedList<Point> -> List of Points(coords) for the body of the snaked
/// Point dir -> direction of the snake
struct Snake {
    head: Point,
    body: LinkedList<Point>,
    dir: Point,
}

/// Direction Constants

const UP:Point = (0, -1);
const DOWN:Point = (0, 1);
const  RIGHT:Point = (1, 0);
const LEFT:Point = (-1, 0);

/// Use makro and name the window Snake
#[macroquad::main("Snake")]
async fn main() {

    /// Directions of the Snake

    /// Initializes the Snake
    /// Creates a new Snake with the head having 0,0 as its coords and setting the direction of the snake to "right"
    let mut snake = Snake {
        head: (0, 0),
        dir: RIGHT,
        body: LinkedList::new(),
    };

    let apple_texture = load_texture("apple.png").await.unwrap();


    /// Initializing the first Random fruit Location (0 - SQUARES)
    let mut fruit: Point = (gen_range(0, SQUARES), gen_range(0, SQUARES));
    /// Initializing the score
    let mut score = 0;
    /// Initializing the Snake's speed
    let mut speed = 0.3;
    ///Time since start of the game
    let mut last_update = get_time();
    /// Boolean if game is over
    let mut game_over = false;

    loop {
        if !game_over {

            ///Performing the Key
            perform_key(&mut snake, get_last_key_pressed());

            let time_since_last_update = get_time() - last_update;

            /// If the time since we last updated is higher than the speed of the game
            /// Then we need to immediately update
            if time_since_last_update > speed {
                // Updating last_update to be now cause we are updating now
                last_update = get_time();
                /// The direction of Snake has already been set
                /// meaning now we can just make the body follow the head
                ///
                /// This just pushes the body in to the head meaning it moves the body
                /// but not the head. (This only inserts one element and doesn't remove)
                snake.body.push_front(snake.head);

                /// This adds the direction of the Snake's Head
                add_direction(&mut snake);


                /// Check if Snakes Coordinates equal to the fruits coords
                if snake.head == fruit {
                    // Generates a new fruit
                    fruit = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
                    // Adds score
                    score += 1;
                    // Makes the Game faster
                    speed *= 0.9;
                } else {
                    /// If the Snake didn't hit a fruit of course we shorten the length as we made it longer before
                    snake.body.pop_back();
                }

                // This here Checks for collisions to the border or if the Head is on Body
                game_over = check_for_border_collisions(&snake) || snake.body.contains(&snake.head);
            }
        }

        // Updating the GUI

        if !game_over {
            clear_background(LIGHTGRAY);

            let game_size = screen_width().min(screen_height());
            let offset_x = (screen_width() - game_size) / 2. + 10.;
            let offset_y = (screen_height() - game_size) / 2. + 10.;
            let sq_size = (screen_height() - offset_y * 2.) / SQUARES as f32;

            draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);


            /// Draw the Lines for the x
            for i in 1..SQUARES {
                draw_line(
                    offset_x,
                    offset_y + sq_size * i as f32,
                    screen_width() - offset_x,
                    offset_y + sq_size * i as f32,
                    2.,
                    LIGHTGRAY,
                );
            }


            /// Draw The Lines for y
            for i in 1..SQUARES {
                draw_line(
                    offset_x + sq_size * i as f32,
                    offset_y,
                    offset_x + sq_size * i as f32,
                    screen_height() - offset_y,
                    2.,
                    LIGHTGRAY,
                );
            }


            // Draw the head
            draw_rectangle(
                offset_x + snake.head.0 as f32 * sq_size,
                offset_y + snake.head.1 as f32 * sq_size,
                sq_size,
                sq_size,
                DARKGREEN,
            );

            // Draw the Snake Body
            for (x, y) in &snake.body {
                draw_rectangle(
                    offset_x + *x as f32 * sq_size,
                    offset_y + *y as f32 * sq_size,
                    sq_size,
                    sq_size,
                    LIME,
                );
            }

            // Draw the Fruit
            draw_texture_ex(
                &apple_texture,
                offset_x + fruit.0 as f32 * sq_size,
                offset_y + fruit.1 as f32 * sq_size,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(sq_size, sq_size)),
                    ..Default::default()
                },
            );

            draw_text(format!("SCORE: {score}").as_str(), 10., 20., 20., DARKGRAY);
        } else {
            exit(0)
        }
        next_frame().await;
    }
}
fn get_direction(key_code: KeyCode) -> Option<(i16, i16)>{
    match key_code {
        KeyCode::Right => Some(RIGHT),
        KeyCode::Left => Some(LEFT),
        KeyCode::Down => Some(DOWN),
        KeyCode::Up => Some(UP),
        KeyCode::W => Some(UP),
        KeyCode::A => Some(LEFT),
        KeyCode::S => Some(DOWN),
        KeyCode::D => Some(RIGHT),
        _ => None
    }
}
fn get_opposite((x, y): (i16, i16)) -> (i16, i16) {
    return (-x, -y);
}
fn perform_key(snake: &mut Snake, key_code: Option<KeyCode>){
    ///Check if key Code isn't none
    if key_code.is_none() { return; }
    /// Get direction of the key (eg. Left = Left Direction)
    let direction = get_direction(key_code.unwrap());
    /// Check if direction exists (meaning if the key he pressed is a valid one)
    if direction.is_some() {
        /// Check if snake direction doesnt equal the opposite (Snake cant go from left to right)
        if snake.dir != get_opposite(direction.unwrap()) {
            snake.dir = direction.unwrap();
        }
    }
}
fn add_direction(snake: &mut Snake) {
    /// Getting the Snake head
    let mut snake_head = snake.head;
    /// Getting the Snake Direction
    let direction = snake.dir;
    // X Coordinate = The old X + direction X
    snake.head.0 = snake_head.0 + direction.0;
    // Y Coordinate = The old Y + direction Y
    snake.head.1 = snake_head.1 + direction.1;
}

fn check_for_border_collisions(snake: &Snake) -> bool{
    return  snake.head.0 < 0 || snake.head.0 >= SQUARES || snake.head.1 < 0 || snake.head.1 >= SQUARES
}