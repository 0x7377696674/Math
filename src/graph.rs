// function graph

use std::io::Write;

pub fn function() {
    let width = 800;
    let height = 800;

    let file = std::fs::File::create("function.ppm").unwrap();
    let mut writer = std::io::BufWriter::new(file);

    writeln!(writer, "P3").unwrap();
    writeln!(writer, "{} {}", width, height).unwrap();
    writeln!(writer, "255").unwrap();

    let mut pixels = vec![[255, 255, 255]; height * width];

    let center_x = width / 2;
    let center_y = height / 2;

    let thickness = 5;
    let half_thickness = thickness / 2;

    for y in 0..height {
        for thick in 0..thickness {
            let x = center_x + thick - half_thickness;
            let index = y * width + x;
            pixels[index] = [0, 0, 0];
        }
    }

    for x in 0..width {
        for thick in 0..thickness {
            let y = center_y + thick - half_thickness;
            let index = y * height + x;
            pixels[index] = [0, 0, 0];
        }
    }

    for pixel in &pixels {
        writeln!(writer, "{} {} {}", pixel[0], pixel[1], pixel[2]);
    }
}
