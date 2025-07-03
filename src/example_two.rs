
pub fn w3_problem_two() {

    let mut new_vector: Vec<i32> = vec![];

    for i in 6..=10{
        new_vector.push(i);
    }

    new_vector.pop();

    println!("Problem Two");
    println!("{:?}", new_vector);
    println!("========");
}