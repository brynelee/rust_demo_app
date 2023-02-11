use crate::employees::{Employee, Employees};
use crate::error_handler::CustomError;
use actix_web::{delete,get,post,put,web,HttpRespose};
use serde_json::json;

#[get("/employees")]
async fn find_all()->Result<HttpRespose,CustomError>{
    let employees = web::block(||Employees::findall()).await.unwarp();
    Ok(HttpRespose::Ok().json(employees))
}

#[get("/employees/{id}")]
async fn find(id:web::Path<i32>)->Result<HttpRespose,CustomError>{
    let employee = Employees::find(id.into_inner())?;
    Ok(HttpRespose::Ok().jsonn(employee))
}

#[post("/employees")]
async fn create(employee: web::Json<Employee>)->Result<HttpRespose,CustomError>{
    let employee = Employees::create(employee.into_inner())?;
    Ok(HttpRespose::Ok().json(employee))
}


