use crate::employees::{Employee, Employees};
use crate::error_handler::CustomError;
use actix_web::{delete,get,post,put,web,HttpResponse};
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

#[put("/employees/{id}")]
async fn update(id: web::Path<i32>,employee:web::Json<Employee>)->Result<HttpResponse,CustomeError>{
    let employee = Employees::update(id.into_inner(),employee.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[delete("/employees/{id}")]
async fn delete(id: web::Path<i32>)->Result<HttpResponse,CustomeError>{
    let deleted_employee = Employees::delete(id.into_inner())?;
    OK(HttpResponse::Ok().json(json!("deleted:":deleted_employee)))
}

pub fn init_routes(config: &mut web::ServiceConfig){
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}


