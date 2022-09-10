// you can replace match with this
#[derive(Debug, Clone, Copy)]

enum Position {
    Manager,
    Supervisor,
    Worker,
}
#[derive(Debug, Clone, Copy)]
struct Employee{
    position: Position,
    work_hours: i64,
}

fn print_employee(emp: Employee){
    println!("{:?}", emp);
}

fn main(){
    let me = Employee {
        position: Position::Worker,
        work_hours: 40
    };
   /* match me.position {
        Position::Manager => println!("Manager"),
        Position::Supervisor => println!("Supervisor"),
        Position::Worker => println!("Worker"),
    }*/
    println!("{:?}", me.position);
    // using derive on struct
    println!("{:?}", me);

    print_employee(me);
    print_employee(me);
}