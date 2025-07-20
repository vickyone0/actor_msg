use actix::prelude::*;

// Define a message
#[derive(Message)]
#[rtype(result = "Result<String, ()>")]
struct MyMessage {
    num1: i32,
    num2: i32,
}

// Define an actor
struct MyActor;

// Implement the Actor trait
impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Actor started");
    }
}

// Handle MyMessage in MyActor
impl Handler<MyMessage> for MyActor {
    type Result = Result<String, ()>;

    fn handle(&mut self, msg: MyMessage, _ctx: &mut Self::Context) -> Self::Result {
        println!("Received message with num1: {}, num2: {}", msg.num1, msg.num2);
        Ok(format!("Sum is: {}", msg.num1 + msg.num2))
    }
}

// #[actix_rt::main]
// async fn main() {
//     // Start the actor
//     let addr = MyActor.start();

//     // Send a message
//     let result = addr.send(MyMessage { num1: 5, num2: 10 }).await;

//     match result {
//         Ok(Ok(response)) => println!("Response from actor: {}", response),
//         Ok(Err(_)) => println!("Failed to get a response from the actor"),
//         Err(_) => println!("Actor is not running"),
//     }

//     // No need to stop the system manually
// }
