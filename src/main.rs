use std::sync::{Mutex,Arc};
use actix_web::{get, post, put, delete, error, web, App, HttpServer, Responder, http::StatusCode};
use serde::{Serialize,Deserialize};
use derive_more::{Display, Error};

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
#[derive(Debug)]
struct Todo {
    user_id : i32,
    user_name : String,
    description : String,
    date : String,
    time : String
}

#[derive(Debug, Display, Error)]
enum TodoError {
    #[display(fmt = "Requested id not found")]
    InternalError,
    #[display(fmt = "Bad request")]
    BadRequestError
}
impl error::ResponseError for TodoError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadRequestError => StatusCode::BAD_REQUEST
       }
    }
}

#[post("/todo")]
async fn create(todo: web::Json<Todo>, counter: web::Data<Arc<Mutex<i32>>>, list: web::Data<Arc<Mutex<Vec<Todo>>>>) -> impl Responder {

    let mut id = counter.lock().unwrap();
    *id = *id + 1;

    println!("in post handler");
    let obj = Todo {
        user_id : *id,
        user_name : todo.user_name.clone(),
        description : todo.description.clone(),
        date : todo.date.clone(),
        time : todo.time.clone(),
    };
    let mut list = list.lock().unwrap();
    list.push(obj);

    let obj = Todo {
        user_id : *id,
        user_name : todo.user_name.clone(),
        description : todo.description.clone(),
        date : todo.date.clone(),
        time : todo.time.clone(),
    };
    web::Json(obj)

}

#[get("/todo/{id}")]
async fn get_todo_by_id(todo_id: web::Path<i32>, list: web::Data<Arc<Mutex<Vec<Todo>>>>) -> Result<impl Responder, TodoError> {

    let id = todo_id.into_inner();
    let list = list.lock().unwrap();
    
    for todo in &*list{
        if todo.user_id == id {
            return Ok(web::Json(todo.clone()))
        }
    } 
   
    return Err(TodoError::InternalError) 
}

#[put("todo/{id}")]
async fn modify_by_id(todo: web::Json<Todo>, todo_id: web::Path<i32>, list: web::Data<Arc<Mutex<Vec<Todo>>>>) -> Result<impl Responder, TodoError> {

    let id = todo_id.into_inner();
    let mut list = list.lock().unwrap();

    for mut t in &mut*list{
        if t.user_id == id {
            t.user_name = todo.user_name.clone();
            t.description = todo.description.clone();
            t.date = todo.date.clone();
            t.time = todo.time.clone();
            return Ok(web::Json(t.clone()))
        }
    } 
    Err(TodoError::BadRequestError) 
}

#[delete("todo/{id}")]
async fn delete_by_id(todo_id: web::Path<i32>, list: web::Data<Arc<Mutex<Vec<Todo>>>>) -> Result<impl Responder, TodoError> {

    let id = todo_id.into_inner();
    let mut list = list.lock().unwrap();

    match list.iter_mut().position(|x| x.user_id == id ) {
        Some(index) => return Ok(web::Json(list.remove(index))),
        None => return Err(TodoError::InternalError),
    };

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = Arc::new(Mutex::new(0));

    let mut v: Vec<Todo> = Vec::new();
    let dummy = Todo {
        user_id : 0,
        user_name : String::from("someusername123"),
        description : String::from("collect checking from all banks."),
        date : String::from("12/12/12"),
        time : String::from("10:10 AM"),
    };
    v.push(dummy);
    let list = Arc::new(Mutex::new(v));

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(counter.clone()))
        .app_data(web::Data::new(list.clone()))
            .service(create)
            .service(get_todo_by_id)
            .service(modify_by_id)
            .service(delete_by_id)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}