use rustgql::{schemas::data::{Schema,create_schema}};
use actix_cors::Cors;
use std::{io,sync::Arc};
use actix_web::{
    get, middleware, route,http,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
/// GraphiQL playground UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}
/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let user = data.execute(&st, &()).await;
    HttpResponse::Ok().json(user)
}
#[actix_web::main]
async fn main() -> io::Result<()>{
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let schema = Arc::new(create_schema());
    log::info!("starting HTTP server on port 8082");
    log::info!("GraphiQL playground: http://localhost:8082/graphiql");
    HttpServer::new(move||{
        let cors = Cors::default()
              .allowed_origin("http://localhost:8082")
              .allowed_origin_fn(|origin, _req_head| {
                  origin.as_bytes().ends_with(b"*")
              })
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);
        App::new()
            .app_data(Data::from(schema.clone()))
            
            .service(graphql)
            .service(graphql_playground)
           // .wrap(Cors::permissive())
            .wrap(cors)
            .wrap(middleware::Logger::default()) 
    })
    .workers(2)
    .bind(("127.0.0.1",8082))?
    .run()
    .await
}