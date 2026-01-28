// function graph

use std::io::Write;

const BLACK_PIXEL: [u8; 3] = [0, 0, 0];
const WHITE_PIXEL: [u8; 3] = [255, 255, 255];

// Row major btw
struct Function {
    row: Vec<[u8; 3]>,
    width: usize,
    height: usize,
}

impl Function {
    fn new(width: usize, height: usize) -> Self {
        let row = vec![BLACK_PIXEL; width * height];

        Self { row, width, height }
    }

    fn draw_axes(&mut self) {
        let thickness = 3;
        let half = thickness / 2;

        let center_x = self.width / 2;
        let center_y = self.height / 2;

        for y in 0..self.height {
            for x in 0..self.width {
                let y_axis = (x as i32 - center_x as i32).abs() <= half as i32;
                let x_axis = (y as i32 - center_y as i32).abs() <= half as i32;

                if y_axis || x_axis {
                    self.point(x as i32, y as i32);
                }
            }
        }
    }

    fn mapping(&self, x0: i32, y0: i32, x1: i32, y1: i32) -> (i32, i32, i32, i32) {
        let center_x = self.width as i32 / 2;
        let center_y = self.height as i32 / 2;

        let x0 = x0 + center_x;
        let y0 = center_y - y0;

        let x1 = x1 + center_x;
        let y1 = center_y - y1;

        (x0, y0, x1, y1)
    }

    fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        let (x0, y0, x1, y1) = self.mapping(x0, y0, x1, y1);
        let dx = x1 - x0;
        let dy = y1 - y0;

        let step = if dx > dy { dx } else { dy };

        if step != 0 {
            let step_x = dx / step;
            let step_y = dy / step;

            for i in 0..=step.abs() {
                self.point(x0 + i * step_x, y0 + i * step_y);
            }
        }
    }

    fn point(&mut self, x: i32, y: i32) {
        let index = (y * self.width as i32 + x) as usize;
        if index < self.width * self.height {
            self.row[index as usize] = WHITE_PIXEL;
        } else {
            println!("index out of bounds: {}", index);
        }
    }
}

fn ppm(function: Function) {
    let file = std::fs::File::create("function.ppm").unwrap();
    let mut writer = std::io::BufWriter::new(file);

    writeln!(writer, "P3").unwrap();
    writeln!(writer, "{} {}", function.width, function.height).unwrap();
    writeln!(writer, "255").unwrap();

    for pixel in &function.row {
        writeln!(writer, "{} {} {}", pixel[0], pixel[1], pixel[2]);
    }
}

pub fn function() {
    let mut function = Function::new(800, 800);
    function.draw_axes();
    function.draw_line(399, 399, -399, -399);

    ppm(function);
}
