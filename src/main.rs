use std::{cell, time::Duration};

use nannou::prelude::*;

const WIDTH: i32 = 1280;
const HEIGHT: i32 = 720;
const STATES: i32 = 8;
const NEIGHBORHOOD: [[i32 ; 2] ; 4] = [[-1, 4], [2, 1], [-1, 1], [5, 5]];
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

}

fn model(app: &App) -> Model {
    app.new_window()
        .title("Template")
        .size(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .build()
        .unwrap();

    Model {}
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



