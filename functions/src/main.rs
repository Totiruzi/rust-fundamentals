fn main() {
    println!("Hello, world!");
    another_function();
    print_labeled_measurement(7, 'b');
}

 fn another_function() {
    println!("Yelling from another function, Hello!");
 }

 fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement value is {value}{unit_label}");
 }