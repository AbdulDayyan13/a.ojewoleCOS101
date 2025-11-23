use std::io;


fn ask_to_continue() -> bool {
    println!("Do you want to check another staff member? (Type 'yes' or 'no')");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read continuation input");
    
   
    input.trim().to_lowercase() != "no"
}


fn main() {
    let staff_details = vec![
      
        ("Office Administrator", 0, 2, "APS 1-2", "Intern"),
        ("Academic", 0, 2, "APS 1-2", "— (No specific entry)"),
        ("Lawyer", 0, 2, "APS 1-2", "Paralegal"),
        ("Teacher", 0, 2, "APS 1-2", "Placement"),

        ("Office Administrator", 3, 4, "APS 3-5", "Administrator"),
        ("Academic", 3, 4, "APS 3-5", "Research Assistant"),
        ("Lawyer", 3, 4, "APS 3-5", "Junior Associate"),
        ("Teacher", 3, 4, "APS 3-5", "Classroom Teacher"),

      
        ("Office Administrator", 5, 8, "APS 5-8", "Senior Administrator"),
        ("Academic", 5, 8, "APS 5-8", "PhD Candidate"),
        ("Lawyer", 5, 8, "APS 5-8", "Associate"),
        ("Teacher", 5, 8, "APS 5-8", "Snr Teacher"),

      
        ("Office Administrator", 9, 10, "EL 1 8-10", "Office Manager"),
        ("Academic", 9, 10, "EL 1 8-10", "Post-Doc Researcher"),
        ("Lawyer", 9, 10, "EL 1 8-10", "Senior Associate 1–2"),
        ("Teacher", 9, 10, "EL 1 8-10", "Leading Teacher"),

       
        ("Office Administrator", 11, 12, "EL 2 10-13", "Director"),
        ("Academic", 11, 12, "EL 2 10-13", "Senior Lecturer"),
        ("Lawyer", 11, 12, "EL 2 10-13", "Senior Associate 3–4"),
        ("Teacher", 11, 12, "EL 2 10-13", "Deputy Principal"),

       
        ("Office Administrator", 13, 999, "SES", "CEO"),
        ("Academic", 13, 999, "SES", "Dean"),
        ("Lawyer", 13, 999, "SES", "Partner"),
        ("Teacher", 13, 999, "SES", "Principal"),
    ];

  
    loop {
        
        println!(" Public Service APS Level Checker ");
    
        
       
        println!("Enter the staff's profession (e.g., Lawyer):");

        let mut profession_input = String::new();
        io::stdin()
            .read_line(&mut profession_input)
            .expect("Failed to read profession input");

        let profession = profession_input.trim();
        
       

        println!("Enter the staff's years of work experience (e.g., 6):");

        let mut experience_input = String::new();
        io::stdin()
            .read_line(&mut experience_input)
            .expect("Failed to read experience input");

        
        let experience: i32 = experience_input.trim().parse().unwrap_or(0);

       
        check_and_print_details(&staff_details, profession, experience);
        
      
        if !ask_to_continue() {
            println!("\n Thank you for using the Staff Level Checker. Goodbye!");
            break; 
        }
    }
}



fn get_staff_details<'a>(
    data: &'a [(&str, i32, i32, &str, &str)],
    profession: &str,
    years_exp: i32
) -> Option<(&'a str, &'a str)> {
    
    for &(pro, min_exp, max_exp, level, role) in data.iter() {
        
        if pro.to_lowercase() == profession.to_lowercase() && years_exp >= min_exp && years_exp <= max_exp {
            return Some((level, role));
        }
    }
    return None;
}

fn check_and_print_details(data: &[(&str, i32, i32, &str, &str)], profession: &str, experience: i32) {
    let result = get_staff_details(data, profession, experience);

    match result {
        Some((level, role)) => {
            println!(
                "\n VALIDATION SUCCESSFUL!\n Profession Category: {}\n Experience: {} years\n\n  Determined Staff Level: {}\n Specific Job Role: {}",
                profession, experience, level, role
            );
        }
        None => {
            println!(
                "\n VALIDATION FAILED!**\n Profession Category: {}\n Experience: {} years\n\n Error: Could not find a matching Staff Level and Role. Check spelling or experience range.",
                profession, experience
            );
        }
    }
}