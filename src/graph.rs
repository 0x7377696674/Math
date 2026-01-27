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

    fn draw_axis(&mut self) {
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

    fn point(&mut self, x: i32, y: i32) {
        let index = y * self.width as i32 + x;
        self.row[index as usize] = WHITE_PIXEL;
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
    function.draw_axis();

    ppm(function);
}
