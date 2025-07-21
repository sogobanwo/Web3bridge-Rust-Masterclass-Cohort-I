// use std::io;
// use crate::mods::{Leads, LeadStatus};

// pub fn add_lead(leads: &mut Vec<Leads>, name: String, contact: String, value: f64, status: LeadStatus) {
//     if name.is_empty() || contact.is_empty() || value <= 0.0 {
//         println!("Invalid lead data provided.");
//         return;
//     }
    
//     let id = leads.len() as u32 + 1;
//     let new_lead = Leads { id, name, contact, value, status };
    
//     leads.push(new_lead);
//     println!("Lead added successfully.");
// }

// pub fn display_leads(leads: &Vec<Leads>) {
//     if leads.is_empty() {
//         println!("No leads available.");
//         return;
//     }
    
//     println!("{:<5} {:<20} {:<15} {:<10} {:<10}", "ID", "Name", "Contact", "Value", "Status");
//     println!("{:-<65}", "");
    
//     for lead in leads {
//         println!("{:<5} {:<20} {:<15} ${:<9.2} {:?}", 
//                  lead.id, lead.name, lead.contact, lead.value, lead.status);
//     }
// }

// pub fn get_lead_input() -> Result<(String, String, f64, LeadStatus), String> {
//     let mut name = String::new();
//     let mut contact = String::new();
//     let mut value_str = String::new();
//     let mut status_str = String::new();

//     println!("Enter lead name:");
//     io::stdin().read_line(&mut name).expect("Failed to read line");
    
//     println!("Enter contact information:");
//     io::stdin().read_line(&mut contact).expect("Failed to read line");
    
//     println!("Enter lead value:");
//     io::stdin().read_line(&mut value_str).expect("Failed to read line");
    
//     println!("Enter lead status (New, Contacted, Qualified, Lost, Converted):");
//     io::stdin().read_line(&mut status_str).expect("Failed to read line");

//     let value: f64 = value_str.trim().parse()
//         .map_err(|_| "Invalid value entered.")?;

//     let status = match status_str.trim().to_lowercase().as_str() {
//         "new" => LeadStatus::New,
//         "contacted" => LeadStatus::Contacted,
//         "qualified" => LeadStatus::Qualified,
//         "lost" => LeadStatus::Lost,
//         "converted" => LeadStatus::Converted,
//         _ => return Err("Invalid status entered.".to_string()),
//     };

//     Ok((name.trim().to_string(), contact.trim().to_string(), value, status))
// }