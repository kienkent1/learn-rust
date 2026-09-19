// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// Authentication
// Authorization

struct Employee {
    position: Poisition,
    status: Status,
}

enum Poisition {
    Manager,
    Engineer,
    Intern,
    CEO,
    CTO,
}
enum Status {
    Active,
    Inactive,
}

fn try_access(emp: &Employee) -> Result<(), String> {
    match emp.status {
        Status::Inactive => Err("Access denied: Inactive employee".to_owned()),
        Status::Active => match emp.position {
            Poisition::Manager | Poisition::CEO | Poisition::CTO => Ok(()),
            _ => Err("Invalid position".to_owned()),
        },
    }
}

fn print_access(emp: &Employee) -> Result<(), String> {
    let access = try_access(emp)?; // ? ở đây để khi lỗi thì tự return lại, còn không thì chạy tiếp
    println!("Access granted");
    Ok(())
}

fn main() {
    let manager = Employee {
        position: Poisition::Manager,
        status: Status::Active,
    };
    let intern = Employee {
        position: Poisition::Intern,
        status: Status::Active,
    };

    print_access(&manager);

    print!("=============Intern===========");

    print_access(&intern);
}
