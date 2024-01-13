fn main() {
    let x: i32 = 5;
    another_function(x);
    print_labeled_measurement(x, 'h');
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}