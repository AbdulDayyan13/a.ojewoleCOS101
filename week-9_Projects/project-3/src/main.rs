fn main() {
    
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etiye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    
    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

   
    let mut merged_dataset = Vec::new();

    for i in 0..5 {
        
        let name = commissioners.get(i).unwrap_or(&"N/A");
        let ministry = ministries.get(i).unwrap_or(&"N/A");
        let zone = zones.get(i).unwrap_or(&"N/A");
        
        
        let record = (i + 1, *name, *ministry, *zone);
        
        
        merged_dataset.push(record);
    }


   
    
    println!("\n");
    println!("EFCC CONVICTED MINISTERS - MERGED DATASET");
    println!("\n");
    
   
    println!("{:<4} {:<30} {:<20} {:<15}", "S/N", "NAME OF COMMISSIONER", "MINISTRY", "GEOPOLITICAL ZONE");
    println!("{:-<75}", ""); 

    for record in merged_dataset {
        println!(
            
            "{:<4} {:<30} {:<20} {:<15}",
            record.0, 
            record.1, 
            record.2, 
            record.3  
        );
    }
    
    println!("\n");
}