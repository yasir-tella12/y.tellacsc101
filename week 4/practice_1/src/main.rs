fn main() {
    let name = "Aisha Lawal";
    let uni:&str = "Pan Atlantic University";
    let addr:&str ="km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    println!("Name: {}", name);
    println!("University: {}, ,\nAdress: {}",uni,addr);
 


    let department:&'static str = "computer science";
    let school:&'static str = "school of science and technology";
    println!("department: {}, ,\school: {}",department,school);