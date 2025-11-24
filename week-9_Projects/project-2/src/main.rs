use std::io::Write; 
fn main() {
   
    let names = ["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matric_numbers = ["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let departments = ["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let levels = ["300", "100", "200", "200", "100"];

  
    let mut file = std::fs::File::create("simple_student_data.txt")
        .expect("Failed to create file");

    let header = "Student Name | Matric. Number | Department | Level\n";
    file.write_all(header.as_bytes()).expect("Write failed for header");
    


    for i in 0..names.len() {

        let mut line = String::new();
        line.push_str(names[i]);
        line.push_str(" | ");
        line.push_str(matric_numbers[i]);
        line.push_str(" | ");
        line.push_str(departments[i]);
        line.push_str(" | ");
        line.push_str(levels[i]);
        line.push_str("\n"); 

        file.write_all(line.as_bytes()).expect("Write failed for student line");

        
        println!("{}", line.trim()); 
    }

   
    println!("\nSUCCESS: All student details written to simple_student_data.txt");
} 