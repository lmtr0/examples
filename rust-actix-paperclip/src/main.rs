use actix_web::{App, HttpServer, Error};
use paperclip::actix::{
    // extension trait for actix_web::App and proc-macro attributes
    OpenApiExt, Apiv2Schema, api_v2_operation,
    // If you prefer the macro syntax for defining routes, import the paperclip macros
    // get, post, put, delete
    // use this instead of actix_web::web
    web::{self, Json},
};
use serde::{Serialize, Deserialize};

// Mark containers (body, query, parameter, etc.) like so...
#[derive(Serialize, Deserialize, Apiv2Schema)]
struct Pet {
    // You can optionaly add description for an individual property with the following line
    /// Name of the pet
    name: String,
    id: Option<i64>,
}

// Mark operations like so...
#[api_v2_operation]
// Add the next line if you want to use the macro syntax
// #[post("/pets")]
async fn echo_pet(body: Json<Pet>) -> Result<Json<Pet>, Error> {
    Ok(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        // Record services and routes from this line.
        .wrap_api()
        // Add routes like you normally do...
        .service(
            web::resource("/pets")
                .route(web::post().to(echo_pet))
        )
        .with_json_spec_v3_at("/api/spec.json")

        // ... or if you wish to build the spec by yourself...

        // .with_raw_json_spec(|app, spec| {
        //     app.route("/api/spec", web::get().to(move || {
        //         let spec = spec.clone();
        //         async move {
        //             paperclip::actix::HttpResponseWrapper(actix_web::HttpResponse::Ok().json(&spec))
        //         }
        //     }))
        // })

        // IMPORTANT: Build the app!
        .build()
    ).bind("127.0.0.1:8080")?
    .run().await
}