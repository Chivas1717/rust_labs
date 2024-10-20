#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, relative};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
mod calculator;
use calculator::{Calculator, CalculatorError};

#[derive(Deserialize)]
struct Operation {
    a: f64,
    b: f64,
    op: String,
}

#[derive(Serialize)]
struct ResultResponse {
    result: String,
}

#[post("/calculate", format = "json", data = "<operation>")]
fn calculate(operation: Json<Operation>) -> Json<ResultResponse> {
    let mut calc = Calculator::new();

    let result = match operation.op.as_str() {
        "+" => calc.add(operation.a, operation.b).to_string(),
        "-" => calc.subtract(operation.a, operation.b).to_string(),
        "*" => calc.multiply(operation.a, operation.b).to_string(),
        "/" => match calc.divide(operation.a, operation.b) {
            Ok(res) => res.to_string(),
            Err(e) => e.to_string(),
        },
        _ => "Некоректна операція.".to_string(),
    };

    Json(ResultResponse { result })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![calculate])
        .mount("/", FileServer::from(relative!("static")))
}
