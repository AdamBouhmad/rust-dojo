pub fn w3_problem_five(){
    let mut vector: Vec<i32> = vec![];

    let array = [5,3,9,1,7];

    for i in array {
        vector.push(i);
    };

    vector.sort();

    println!("Problem Five");
    for i in vector {
        println!("{i}");
    };
    println!("========");


}