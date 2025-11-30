use std::io::Write; 

fn main() {
   
    let students = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", 300),
        ("Adams Aliyu", "ECO10110101", "Economics", 100),
        ("Shania Bolade", "CSC10328828", "Computer", 200),
        ("Adekunle Gold", "EEE11020202", "Electrical", 200),
        ("Blanca Edemoh", "MEE10202001", "Mechanical", 100),
    ];

    println!("\nPAU SMIS");
    
    println!("{:<20} {:<15} {:<15} {:<5}", "Student Name", "Matric. Number", "Department", "Level");
    println!("{:-<60}", "");

    for student in &students {
        println!(
            "{:<20} {:<15} {:<15} {:<5}", 
            student.0, 
            student.1, 
            student.2, 
            student.3  
        );
    }
    
    
    let mut file = match std::fs::File::create("pau_smis_output.csv") {
        Ok(f) => f,
        Err(e) => {
            println!("Error: Could not create the file: {}", e);
            return; 
        }
    };

   
    if let Err(e) = writeln!(file, "PAU SMIS,,,") {
        println!("Error writing title: {}", e);
        return;
    }
    
    if let Err(e) = writeln!(file, "Student Name,Matric. Number,Department,Level") {
        println!("Error writing headers: {}", e);
        return;
    }

   
    for student in &students {
    
        if let Err(e) = writeln!(file, "{},{},{},{}", 
            student.0, student.1, student.2, student.3) {
            println!("Error writing student data: {}", e);
            return;
        }
    }

    println!("\n Success! Data saved to 'pau_smis_output.csv'.");
    println!("This file can be opened directly in a spreadsheet program to match the image format.");
} 