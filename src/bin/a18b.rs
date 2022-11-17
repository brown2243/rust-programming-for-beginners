// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum Roles {
    Line,
    Kitchen,
    Assembly,
}
enum Building {
    Maintenance,
    Marketing,
    Managers,
}
struct Employees {
    role: Roles,
    place: Building,
}

fn check_role(employees: &Employees) -> Result<(), String> {
    return match employees.role {
        Roles::Assembly => Ok(()),
        _ => Err("Failed".to_owned()),
    };
}
fn check_wrap(employees: &Employees) -> Result<(), String> {
    let ans = check_role(&employees)?;
    println!("{:#?}", ans);
    Ok(())
}

fn main() {
    let brown = Employees {
        role: Roles::Assembly,
        place: Building::Managers,
    };

    let ans = check_wrap(&brown);
    println!("{:#?}", ans);
}
