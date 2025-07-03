
pub fn w3_problem_one() {

    let mut one_to_ten: Vec<i32> = vec![];

    let mut i: i32 = 1; 
    while i <= 10{
        one_to_ten.push(i);

        i+=1;
    }

    println!("Problem One");
    println!("{:?}", one_to_ten);
    println!("========");
}