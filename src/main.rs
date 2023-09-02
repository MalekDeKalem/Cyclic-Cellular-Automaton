use std::{cell, time::Duration};

use nannou::prelude::*;

const WIDTH: i32 = 1280;
const HEIGHT: i32 = 720;
const STATES: i32 = 8;
const CELL_SIZE: i32 = 10; 
/*
    The first generation starts out with random states in each of the cells.
    In each subsequent generation,
    if a cell has a neighboring cell whose value is the successor of the cell's value, 
    the cell is "consumed" and takes on the succeeding value
*/


fn main() {
    nannou::app(model).update(update).run();
}


struct Model {
    _cols: Vec<Srgb<u8>>,
    _width: i32,
    _height: i32,
    _grid: Vec<Vec<i32>>,
    _threshold: i32,
    _neighbourhood: Vec<Vec<i32>>,    
    _posvec: Vec<Point2>,
    _rules: Vec<Vec<i32>>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .title("Template")
        .size(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .build()
        .unwrap();
    let cols: Vec<Srgb<u8>> = vec![Srgb::<u8>::new(240, 98, 55),Srgb::<u8>::new(88, 32, 155), Srgb::<u8>::new(240, 35, 144), Srgb::<u8>::new(2, 98, 55), Srgb::<u8>::new(240, 54, 5), Srgb::<u8>::new(43, 120, 33),Srgb::<u8>::new(240, 120, 12)];
    let width = WIDTH / CELL_SIZE;
    let height: i32 = HEIGHT / CELL_SIZE;
    let mut grid = vec![vec![0; height as usize]; width as usize];
    let mut posvec = vec![pt2(0.0, 0.0) ; (width * height) as usize];

    let mut x = (-WIDTH/2 + CELL_SIZE / 2) as f32;
    let mut y = (HEIGHT/2 - CELL_SIZE / 2) as f32;
    for j in 0..height {
        for i in 0..width {
            grid[i as usize][j as usize] = random_range(0, STATES - 1);
            posvec.push(pt2(x, y));
            x += CELL_SIZE as f32;
        }
        x = (-WIDTH/2 + CELL_SIZE / 2) as f32;
        y -= CELL_SIZE as f32;
    }
    let threshold = 4;
    let neighbourhood: Vec<Vec<i32>> = vec![vec![-1, 4], vec![2, 1], vec![-1, 1], vec![5, 5]];
    let rules: Vec<Vec<i32>> = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1], 
        vec![2, 0, 2, 0, 2, 0, 2, 0],  
        vec![0, 2, 0, 2, 0, 2, 0, 2],
        vec![3, 2, 4, 5, 0, 0, 3, 2],
        vec![6, 3, 0, 3, 3, 0, 0, 1],
        vec![7, 6, 2, 4, 0, 4, 2, 3],
        vec![4, 4, 2, 1, 6, 4, 2, 2],
        vec![6, 6, 3, 2, 0, 0, 4, 2],   
    ];
    Model { _cols: cols, _width: width, _height: height, _grid: grid, _threshold: threshold, _neighbourhood: neighbourhood, _posvec: posvec , _rules: rules }
}

fn modulo(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

fn update(app: &App, _model: &mut Model, update: Update) {

    
}


fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);


    for i in 0..model._posvec.len() {
        draw.rect()
            .x_y(model._posvec[i].x, model._posvec[i].y)
            .w_h(CELL_SIZE as f32, CELL_SIZE as f32)
            .color(Srgb::<u8>::new(random_range(0, 255), 0, 0));
    }


    draw.to_frame(app, &frame).unwrap();
}






// # Define constants
// NUM_STATES = 3
// GRID_WIDTH = 50  # Width of the grid
// GRID_HEIGHT = 30  # Height of the grid
// NUM_GENERATIONS = 30  # Number of time steps

// # Initialize the cellular automaton grid
// grid = initialize_grid(GRID_WIDTH, GRID_HEIGHT, NUM_STATES)

// # Define transition rules (you can define your own rules)
// RULES = [
//     [1, 1, 1, 1, 1, 1, 1, 1],  # Rule for state 0 -> state 1
//     [2, 0, 2, 0, 2, 0, 2, 0],  # Rule for state 1 -> state 2
//     [0, 2, 0, 2, 0, 2, 0, 2]   # Rule for state 2 -> state 0
// ]

// # Function to initialize the grid with random states
// function initialize_grid(width, height, num_states):
//     grid = empty_grid(width, height)
//     for x in range(width):
//         for y in range(height):
//             grid[x][y] = random_state(num_states)
//     return grid

// # Function to generate a random state
// function random_state(num_states):
//     return random_integer(0, num_states - 1)

// # Function to create an empty grid
// function empty_grid(width, height):
//     grid = new empty array of size width x height
//     for x in range(width):
//         for y in range(height):
//             grid[x][y] = 0  # Initialize to state 0
//     return grid

// # Function to update the automaton for one generation
// function update_grid(grid, rules):
//     new_grid = empty_grid(GRID_WIDTH, GRID_HEIGHT)
//     for x in range(GRID_WIDTH):
//         for y in range(GRID_HEIGHT):
//             neighbors = get_neighbors(x, y, GRID_WIDTH, GRID_HEIGHT)
//             new_state = rules[grid[x][y]][neighbors]
//             new_grid[x][y] = new_state
//     return new_grid

// # Function to get the states of neighboring cells
// function get_neighbors(x, y, width, height):
//     neighbors = []
//     for dx in [-1, 0, 1]:
//         for dy in [-1, 0, 1]:
//             nx = (x + dx) % width
//             ny = (y + dy) % height
//             neighbors.append(grid[nx][ny])
//     return neighbors

// # Function to print the current state of the automaton
// function print_grid(grid):
//     for y in range(GRID_HEIGHT):
//         for x in range(GRID_WIDTH):
//             print(grid[x][y], end=' ')
//         print()
//     print()

// # Main loop to run the automaton for multiple generations
// for generation in range(NUM_GENERATIONS):
//     print_grid(grid)
//     grid = update_grid(grid, RULES)