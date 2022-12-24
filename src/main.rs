// /src/main.rs


use rocket::fs::{relative, FileServer}; // <--
use rocket_dyn_templates::Template; // <--

#[macro_use]
extern crate rocket;
mod routes;
mod lightning;


#[get("/hola/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hola, tienes {} años y te llamas {}!", age, name)
}



#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/public", FileServer::from(relative!("static"))) // <-- Seteamos un directorio para contenido estático
        .mount(
        	"/", 
		        routes![routes::index, 
		        routes::create_invoice,
		        routes::lookup_invoice
                ],
                )
        .attach(Template::fairing()) // <--
}



