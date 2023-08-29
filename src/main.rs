use nannou::prelude::*;

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 750;

/*
The first generation starts out with random states in each of the cells.
In each subsequent generation,
if a cell has a neighboring cell whose value is the successor of the cell's value, 
the cell is "consumed" and takes on the succeeding value
*/


fn main() {
    nannou::app(model).update(update).run();
}

struct Cell {
    _state: u8,
    _pos: Point2,
    _size: u8,
}


struct Model {
    _cells: Vec<Cell>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .title("Template")
        .size(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .build()
        .unwrap();


    // Init the cells randomly at first
    let mut _cells: Vec<Cell> = Vec::new();
    let size: i32 = 10;

    for j in (-HEIGHT / 2 + size/2..=HEIGHT / 2 - size/2).rev().step_by(size as usize) {
        for i in (-WIDTH/2+size as i32/2..WIDTH/2+size as i32/2).step_by(size as usize) {
            let rand_num = random_range(0, 255) as u8;
            _cells.push(Cell { _state: (rand_num), _pos: {pt2(i as f32,j as f32)}, _size: size as u8});
        }
    }


    Model { _cells }
}

fn modulo(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

fn update(app: &App, _model: &mut Model, update: Update) {
}

fn neighbor(_model: &mut Model, index: i32) {
    let cell_size = _model._cells[0]._size as f32;
    let width = WIDTH / cell_size as i32;
    for j in 0..HEIGHT / cell_size as i32 {
        for i in 0..WIDTH / cell_size as i32 {
            let current_cell = &_model._cells[(j * width + i) as usize];
            let left_neighbor_cell = &_model._cells[(j * width + ((i - 1) % width)) as usize];
            let right_neighbor_cell = &_model._cells[(j * width + ((i + 1) % width)) as usize];
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let mut index = 0;
    let width = WIDTH / 10 as i32;
    let height = HEIGHT / 10 as i32;
    let test_index = modulo(0 - 1, height) * width + modulo(0 - 1, width);
    for cell in &model._cells {
        if index == test_index {
            draw.rect()
            .wh(vec2(cell._size as f32, cell._size as f32))
            .x_y(cell._pos.x, cell._pos.y)
            .color(Srgb::<u8>::new(0, 255, 0));
        } else {
            draw.rect()
            .wh(vec2(cell._size as f32, cell._size as f32))
            .x_y(cell._pos.x, cell._pos.y)
            .color(Srgb::<u8>::new(cell._state, 0, 0));
        }
        index += 1;
    }

    println!("{}", test_index);

    draw.to_frame(app, &frame).unwrap();
}



#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn test_indexing1() {
        assert_eq!(
            modulo(0 - 1, 75) * 100 + modulo(0 - 1, 100),
            7499
        )
    }
}