use actix_files::{Files};
use actix_web::{web, App, HttpServer, HttpResponse};

use handlebars::Handlebars;
use serde_json::json;


async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse { let data = json!({
        "project_name": "Customers",

    "cats": [
            {
                "name": "Klaus",
                "image_path":
                "/static/image/customer1.jpeg"
            },
            {
                "name": "Antony",
                "image_path":
                "/static/image/customer2.jpeg"
            },
            {
                "name": "Stephan",
                "image_path":
                "/static/image/customer3.jpeg"
            },
            {
                "name": "Antonela",
                "image_path":
                "/static/image/customer4.jpeg"
            },
                    {
                "name": "Silva",
                "image_path":
                "/static/image/customer5.jpeg"
            },
                    {
                "name": "Marta",
                "image_path":
                "/static/image/customer6.jpeg"
            }]

});
    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body) }


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();



    handlebars
        .register_templates_directory(".html", "./static/")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    println!("Listening on port 8080");

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(
                Files::new("/static", "static")
                    .show_files_listing(),
            )
            .route("/", web::get().to(index))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await

}