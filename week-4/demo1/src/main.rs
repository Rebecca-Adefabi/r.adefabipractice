//student  Grade calculator

use std::io;

fn main() {

    let mut name = String::new();
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    
    println!("Student Grade Calculator");
    println!("\nEnter your name: ");
    io::stdin().read_line(&mut name).expect("Enter your name");

    println!("Enter test scorees over 20");

    println!("\nEnter first subject score:");
    io::stdin().read_line(&mut input1).expect("Enter your score");
    let score1:f32 = input1.trim().parse().expect("Enter correct value");

    println!("\nEnter second subject score:");
    io::stdin().read_line(&mut input2).expect("Enter your score");
    let score2:f32 = input2.trim().parse().expect("Enter correct value");

    println!("\nEnter third subject score:");
    io::stdin().read_line(&mut input3).expect("Enter your score");
    let score3:f32 = input3.trim().parse().expect("Enter correct value");

    println!("{}", name);

    println!("Average              Grade");

    let ave_score:f32 = ((score1/20.0) + (score2/20.0) + (score3/20.0))/3.0 *100.0;

    if ave_score > 100.0 {println!("enter valid test scores");}
    else if ave_score >= 70.0 {
        println!("\n{:.2}                 A", ave_score);
    }
    else if ave_score >=60.0{
        println!("\n{:.2}                 B", ave_score);
    }
    else if ave_score >=50.0{
        println!("\n{:.2}                 C", ave_score);
    }
    else if ave_score >=60.0{
        println!("\n{:.2}                 D", ave_score);
    }
    else if ave_score >=0.0{
        println!("\n{:.2}                 F", ave_score);
    }
    else{
        println!("\n Enter correct test scores");
    }
    
    

    





   
}
