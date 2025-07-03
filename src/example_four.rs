
pub fn w3_problem_four() {
    let mut vector: Vec<i32> = vec![];

    for i in 1..=5{
        vector.push(i);
    }

    println!("Problem Four");
    for i in vector{
        println!("{}", i * 3);
    }
    println!("========");
}