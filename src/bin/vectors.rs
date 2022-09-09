fn main(){
    struct Test {
        score: i32
    }



    let my_numbers = vec![1, 2, 3];
    

    let mut my_numbers = Vec::new();
    my_numbers.push(1);
    my_numbers.push(2);
    my_numbers.push(3);
    my_numbers.pop();
    my_numbers.len();

    let two = my_numbers[1];

    
    let my_number = vec![4,5,6];

    println!("{:?}", my_numbers);

    for num in my_number{
        println!("{:?}", num);
    }

    let my_scores = vec![
        Test { score: 90},
        Test { score: 88},
        Test { score: 77},
        Test { score: 93},
    ];

    for test in my_scores {
        println!("score = {:?}", test.score);
    }
}