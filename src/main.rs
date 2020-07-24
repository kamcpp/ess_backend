mod domain;
mod models;

use std::sync::{Arc, Mutex};

use tide::{Request, Response, Result};

use domain::Employee;

use models::{HelloRequest, HelloResponse, EmployeeModel};

struct ServiceState {
}

impl ServiceState {
    fn new() -> Self {
        Self {
        }
    }
}

type SharedSyncState = Arc<Mutex<ServiceState>>;

async fn handle_add_employee(mut req: Request<SharedSyncState>) -> Result<Response> {
    /* let employee_model: EmployeeModel = req.body_json().await?;
    let mut state = req.state().lock().unwrap();
    let mut found = false;
    state.employees.iter().for_each(|employee| {
        if employee.username == employee_model.username ||
            employee.employee_nr == employee_model.employee_nr ||
            employee.office_email == employee_model.office_email {
            found = true;
        }
    });
    if found {
        return Ok(Response::builder(409).body("".to_string()).build());
    }
    state.id_counter +=1 ;
    let new_id = state.id_counter;
    state.employees.push(Employee {
        id: new_id,
        employee_nr: employee_model.employee_nr,
        first_name: employee_model.first_name,
        second_name: employee_model.second_name,
        username: employee_model.username,
        office_email: employee_model.office_email,
        mobile: employee_model.mobile,
    });*/
    Ok(Response::builder(200).body("ok".to_string()).build())
}

async fn handle_get_all_employees(req: Request<SharedSyncState>) -> Result<String> {
    let state = req.state().lock().unwrap();
    let to_return: Vec<EmployeeModel> = Vec::new(); /*state.employees.iter().map(|employee| EmployeeModel {
        id: Some(employee.id),
        employee_nr: employee.employee_nr.clone(),
        first_name: employee.first_name.clone(),
        second_name: employee.second_name.clone(),
        username: employee.username.clone(),
        office_email: employee.office_email.clone(),
        mobile: employee.mobile.clone(),
    }).collect();*/
    Ok(serde_json::to_string(&to_return)?)
}

async fn handle_hello(mut req: Request<SharedSyncState>) -> Result<String> {
    let hello_req: HelloRequest = req.body_json().await?;
    let hello_resp = HelloResponse { greeting: format!("Hello, {}!", hello_req.name), };
    Ok(serde_json::to_string(&hello_resp)?)
}

#[async_std::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    let state = Arc::new(Mutex::new(ServiceState::new()));
    let mut app = tide::with_state(state);
    app.at("/api/hello").post(handle_hello);
    app.at("/api/employee").post(handle_add_employee);
    app.at("/api/employee/all").get(handle_get_all_employees);
    app.listen("0.0.0.0:9090").await?;
    Ok(())
}
