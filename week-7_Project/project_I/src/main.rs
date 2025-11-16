use std::io;

fn read_number() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let number: f64 = input.trim().parse().expect("Please enter a valid number");
    number
}

fn area_trapezium() {
    println!("\nArea of Trapezium ");
    println!("Enter height:");
    let height = read_number();
    
    println!("Enter base 1:");
    let base1 = read_number();
    
    println!("Enter base 2:");
    let base2 = read_number();

    let area = (height / 2.0) * (base1 + base2);
    println!("The area of the trapezium is: {}", area);
}

fn area_rhombus() {
    println!("\nArea of Rhombus");
    println!("Enter diagonal 1:");
    let d1 = read_number();
    
    println!("Enter diagonal 2:");
    let d2 = read_number();

    let area = 0.5 * d1 * d2;
    println!("The area of the rhombus is: {}", area);
}

fn area_parallelogram() {
    println!("\n-- Area of Parallelogram --");
    println!("Enter base:");
    let base = read_number();
    
    println!("Enter altitude:");
    let altitude = read_number();

    let area = base * altitude;
    println!("The area of the parallelogram is: {}", area);
}


fn area_cube() {
    println!("\nSurface Area of Cube ");
    println!("Enter length of a side:");
    let side = read_number();

    let area = 6.0 * side.powi(2);
    println!("The surface area of the cube is: {}", area);
}

fn volume_cylinder() {
    println!("\nVolume of Cylinder ");
    println!("Enter radius:");
    let radius = read_number();
    
    println!("Enter height:");
    let height = read_number();

    let volume = std::f64::consts::PI * radius.powi(2) * height;
    println!("The volume of the cylinder is: {}", volume);
}

fn main() {
   loop {
        println!("\nGeometry Calculator");
        println!("1. Area of Trapezium");
        println!("2. Area of Rhombus");
        println!("3. Area of Parallelogram");
        println!("4. Surface Area of Cube");
        println!("5. Volume of Cylinder");
        println!("6. Exit"); 
        println!("Please select an option (1-6):"); 

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: u32 = choice
        .trim()
        .parse()
        .expect("Please enter a number between 1 and 5");

    if choice == 1 {
        area_trapezium();
    } else if choice == 2 {
        area_rhombus();
    } else if choice == 3 {
        area_parallelogram();
    } else if choice == 4 {
        area_cube();
    } else if choice == 5 {
        volume_cylinder();
    } else if choice == 6 {
            println!("Goodbye!");
            break; 
    } else {
        println!("Invalid choice. Please run the program again.");
    }
}
}