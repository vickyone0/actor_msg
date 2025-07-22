use actix::prelude::*;
use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, Responder};



#[derive(Message)]
#[rtype(result = "Result<String, ()>")]
struct UserMessage {
    id: String,
    name: String,
}

// === Actor ===

struct UserActor;

impl Actor for UserActor {
    type Context = Context<Self>;
}

impl Handler<UserMessage> for UserActor {
    type Result = Result<String, ()>;

    fn handle(&mut self, msg: UserMessage, _ctx: &mut Self::Context) -> Self::Result {
        let response = format!("User processed - id: {}, name: {}", msg.id, msg.name);
        Ok(response)
    }
}

// === Web Handlers ===

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}

async fn greet_diff(
    req: HttpRequest,
    user_actor: web::Data<Addr<UserActor>>,
) -> impl Responder {
    let id = req.match_info().get("id").unwrap_or("unknown").to_string();
    let name = req.match_info().get("name").unwrap_or("unknown").to_string();

    match user_actor.send(UserMessage { id, name }).await {
        Ok(Ok(response)) => HttpResponse::Ok().body(response),
        Ok(Err(_)) => HttpResponse::InternalServerError().body("Actor returned an error"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to send message to actor"),
    }
}

// === Main Function ===

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Start the actor
    let user_actor = UserActor.start();

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_actor.clone())) 
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/user/{id}/{name}", web::get().to(greet_diff)) 
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
