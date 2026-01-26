// Percentage

pub fn percentage(number: i32, percentage: i32) {
    assert!(
        percentage >= 0 && percentage <= 100,
        "Percentage must be between 0 and 100"
    );

    let result = number as f32 * (percentage as f32 / 100.0);
    println!("{}% of {} is {}", percentage, number, result);
}
