use std::io; 


struct Developer {
    name: String,
    experience_years: u32,
}

fn main() {
    println!(" EY Nigeria Developer Experience Search Tool ");


    let mut candidates: Vec<Developer> = Vec::new(); 
    let mut developer_count: u32 = 1;

    loop {
        println!("\n Entering Details for Developer #{} ", developer_count);
        
       
        println!("Enter the name of Developer #{} (or type 'done' to to get results):", developer_count);
        let mut name_input = String::new();
        io::stdin()
            .read_line(&mut name_input)
            .expect("Failed to read name input");
        
        let name = name_input.trim();
        
       
        if name.to_lowercase() == "done" {
            break; 
        }

        
        println!("Enter the years of programming experience for {}:", name);
        let mut exp_input = String::new();
        io::stdin()
            .read_line(&mut exp_input)
            .expect("Failed to read experience input");

       
        let experience: u32 = match exp_input.trim().parse() {
            Ok(num) => num, // Successfully parsed
            Err(_) => {
                println!("Invalid input. Experience set to 0. Please re-enter this candidate.");
                0
            }
        };

       
        let new_developer = Developer {
            name: String::from(name),
            experience_years: experience,
        };
        candidates.push(new_developer);
        
        developer_count += 1;
    }

    
    if candidates.is_empty() {
        println!("\n No candidates were entered. Program finished.");
        return; 
    }

    let mut highest_experience: u32 = 0;
    let mut top_candidate_name: String = String::new(); 

    for candidate in candidates.iter() {
        if candidate.experience_years > highest_experience {
            highest_experience = candidate.experience_years;
           
            top_candidate_name = candidate.name.clone(); 
        }
    }


    println!("\n");
    println!(" Search Complete! Results for {} Candidates:", candidates.len());
    println!("  Top Developer: {}", top_candidate_name);
    println!("  Experience: {} years", highest_experience);
}