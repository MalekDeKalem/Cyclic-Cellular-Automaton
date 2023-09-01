use std::{cell, time::Duration};

use nannou::prelude::*;

const WIDTH: i32 = 1280;
const HEIGHT: i32 = 720;
const STATES: i32 = 8;
const CELL_SIZE: i32 = 2; 
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
    _frame: i32,
    _cols: Vec<Srgb<u8>>,
    _width: i32,
    _height: i32,
    _grid: Vec<Vec<Vec<i32>>>,
    _threshold: i32,
    _neighbourhood: Vec<Vec<i32>>,    
    _posvec: Vec<Point2>
}

fn model(app: &App) -> Model {
    app.new_window()
        .title("Template")
        .size(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .build()
        .unwrap();
    let frame = 0;
    let cols: Vec<Srgb<u8>> = vec![Srgb::<u8>::new(240, 98, 55),Srgb::<u8>::new(88, 32, 155), Srgb::<u8>::new(240, 35, 144), Srgb::<u8>::new(2, 98, 55), Srgb::<u8>::new(240, 54, 5), Srgb::<u8>::new(43, 120, 33),Srgb::<u8>::new(240, 120, 12)];
    let width = WIDTH / CELL_SIZE;
    let height: i32 = HEIGHT / CELL_SIZE;
    let mut grid = vec![vec![vec![0; height as usize]; width as usize]; 2];
    let mut posvec = vec![pt2(0.0, 0.0) ; (width * height) as usize];
    let mut x = (-WIDTH/2 + CELL_SIZE / 2) as f32;
    let mut y = (HEIGHT/2 - CELL_SIZE / 2) as f32;
    for j in 0..height {
        for i in 0..width {
            grid[0][i as usize][j as usize] = random_range(0, STATES - 1);
            posvec[(j * width + i) as usize] = pt2(x, y);
            x += CELL_SIZE as f32;
        }
        y -= CELL_SIZE as f32;
    }
    let threshold = 4;
    let neighbourhood: Vec<Vec<i32>> = vec![vec![-1, 4], vec![2, 1], vec![-1, 1], vec![5, 5]];
    Model { _frame: frame, _cols: cols, _width: width, _height: height, _grid: grid, _threshold: threshold, _neighbourhood: neighbourhood, _posvec: posvec }
}

fn modulo(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

fn update(app: &App, _model: &mut Model, update: Update) {

}


fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);


    draw.to_frame(app, &frame).unwrap();
}



