use st::io;

fn main()
{
    let mut input = string ::new();

    println!("\n Enter Your Age");
    io::stdin().read_line(&mut input).expect("not a valid string");
    let age:f32 = input.trim().parase().expect("not a valid number");
    let yrs:f32 = input.trim().parase().expect("not a valid number");
    
    if yrs >= 20
    {
        ptintln!(you are experienced );
    }  
        else if age >= 40 
        {
            ptintln!(your incentive is 1,560,000 );
        }
        else if age >=30 && age < 40
        {
            println!("your incentive is 1,480,000")
        }
        else if age <28
        {
            println!("your incentive is 1,300,000")
        }
        else if yrs >20
        {
            println!("your incentive is 100,000")
        }
        else
        {
            println!("no incentive")
        }
    

}

    

    

