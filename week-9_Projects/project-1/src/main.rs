use std::io::Write; 

fn main() {
   
    let lager_data = "Lager:\n  33 Export\n  Desperados\n  Goldberg\n  Gulder\n  Heineken\n  Star\n";
    let stout_data = "Stout:\n  Legend\n  Turbo King\n  Williams\n";
    let non_alcoholic_data = "Non-Alcoholic:\n  Maltina\n  Amstel Malta\n  Malta Gold\n  Fayrouz\n";

    let mut file = std::fs::File::create("drinks_categories_full.txt")
        .expect("Failed to create file");

  
    file.write_all(" NIGERIAN BREWERIES PLC PORTFOLIO \n\n".as_bytes())
        .expect("Write failed for header");

    file.write_all(lager_data.as_bytes())
        .expect("Write failed for Lager section");

    
    file.write_all("\n\n".as_bytes())
        .expect("Write failed for separator");

  
    file.write_all(stout_data.as_bytes())
        .expect("Write failed for Stout section");
    
  
    file.write_all("\n".as_bytes())
        .expect("Write failed for separator");

  
    file.write_all(non_alcoholic_data.as_bytes())
        .expect("Write failed for Non-Alcoholic section");

   
    println!("\nSuccessfully wrote the complete drinks portfolio to drinks_categories_full.txt");
}