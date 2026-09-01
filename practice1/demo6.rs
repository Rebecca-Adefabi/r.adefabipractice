fn main(){
	println!("========================================");
	println!("Student grade calculator");
	println!("========================================");
	println!("Course code     C.A score     Exam score");

	let course1 = "MTH 101";
	let c_a1 = 22;
	let exam1 = 57;
	let total1 = c_a1 + exam1;
    println!("{}         {}            {}", course1,c_a1, exam1);

    let course2 = "STA 111";
	let c_a2 = 32;
	let exam2 = 56;
	let total2 = c_a2 + exam2;
    println!("{}         {}            {}", course2,c_a2, exam2);

    let course3 = "SEN 192";
	let c_a3 = 30;
	let exam3 = 45;
	let total3 = c_a3 + exam3;
    println!("{}         {}            {}", course3,c_a3, exam3);
    println!("========================================");

    let total = total1 + total2 + total3;

    println!("Total:          {}", total);

    let average = total/3;

    println!("Average:        {}", average);



}