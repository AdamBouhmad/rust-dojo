pub fn w3_problem_six(){

    let mut vector: Vec<i32> = vec![];

    for i in 1..=10{
        vector.push(i);
    }

    for i in vector {
        if i % 2 == 0 {
            println!("{i}");
        }
    }    
    
}