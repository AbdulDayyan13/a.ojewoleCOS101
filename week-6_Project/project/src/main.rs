use std::io::{self, BufRead}; // Added BufRead

fn main(){
    // 1. Defined the 'menu' array with char key, f64 price, and name.
    let menu: [(char, f64, &str); 5] = [
        ('P', 3200.00, "Poundo Yam & Edinkaiko Soup"),
        ('F', 3000.00, "Fried Rice & Chicken"),
        ('A', 2500.00, "Amala & Ewedu Soup"),
        ('E', 2000.00, "Eba & Egusi Soup"),
        ('W', 2500.00, "White Rice & Stew"),
    ];
    
    println!("Hello! Welcome to Afro Continental. Here are our specials for today:");
    // Print specials from the menu
    for &(_, price, name) in &menu {
        println!("{} - N{:.2}", name, price);
    }

    let mut total: f64 = 0.0;

    println!("\n Menu");
    for &(key, price, name) in &menu {
        println!("{}: {} (N{:.2})", key, name, price);
    }
    println!("\n");
    println!("Enter orders (e.g., 'P 2'). Press Enter on empty line to finish.");
    let stdin = io::stdin();
    let mut reader = stdin.lock(); 
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break, 
            Ok(_) => {
                if line.trim().is_empty() {
                    break; 
                }
            }
            Err(_) => {
                println!("Error reading input.");
                break;
            }
        }
        let line = line.trim().to_uppercase();
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            println!("Invalid format: {}. Try 'A 1'.", line);
            continue;
        }
        let food_type = parts[0].chars().next().unwrap_or(' ');
        let quantity: u32 = match parts[1].parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Invalid quantity: {}.", parts[1]);
                continue;
            }
        };
        if let Some((_, price, name)) = menu.iter().find(|(k, _, _)| *k == food_type) {
            let cost = price * quantity as f64;
            total += cost;
            println!("Added {} x {} (N{:.2})", name, quantity, cost);
        } else {
            println!("Invalid Food Type: {}.", food_type);
        }
    }
    
    let mut final_charge = total;
    let mut discount = 0.0;
    if final_charge > 10000.0 {
        discount = total * 0.05;
        final_charge = total - discount;
    }
    println!("\n Summary ");
    println!("Subtotal: N{:.2}", total);
    println!("Discount: -N{:.2}", discount); 
    println!("");
    println!("Total Charge: N{:.2}", final_charge);
}