pub fn w3_problem_three() {

    let mut vector: Vec<i32> = vec![];

    for i in 1..=10{
        vector.push(i);
    }

    println!("Problem Three");

    if let Some(element) = vector.get(4){
        println!("{element}");
    }
    else {
        println!("Index 4 is out of bounds.");
    }
    println!("========");
}