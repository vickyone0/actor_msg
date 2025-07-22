use actix::prelude::*;


//Define the actor
struct CounterActor {
    count: usize,
}

impl Actor for CounterActor {

    type Context = Context<Self>;   
}


//Define the message to increment the counter
#[derive(Message)]
#[rtype(result = "usize")]
struct Increment;


//implement the handler for the Increment message
impl  Handler<Increment> for CounterActor {
    type Result = usize;

    fn handle(&mut self, _msg: Increment, _ctx: &mut Self::Context) -> Self::Result {
        self.count += 1;
        self.count
    }
}

//Define the message to decrement the counter
#[derive(Message)]
#[rtype(result = "usize")]
struct Decrement;

//implement the handler for the Decrement message
impl Handler<Decrement> for CounterActor {
    type Result = usize;
    fn handle(&mut self, _msg: Decrement, _ctx: &mut Self::Context) -> Self::Result {
        if self.count > 0 {
            self.count -= 1;
        }
        self.count
    }
}

//Define the message to get the current count
#[derive(Message)]
#[rtype(result = "usize")]
struct GetCount;

//implement the handler for the GetCount message
impl Handler<GetCount> for CounterActor {
    type Result = usize;

    fn handle(&mut self, _msg: GetCount, _ctx: &mut Self::Context) -> Self::Result {
        self.count
    }
}

#[actix_rt::main]
async fn main() -> Result<(),()> {

    //Start the actor
    let addr = CounterActor { count: 0 }.start();

    //Send Increment messages
    addr.send(Increment).await.unwrap();
    addr.send(Increment).await.unwrap();


    //Send GetCount message
    let count = addr.send(GetCount).await.unwrap();
    println!("Current count: {}", count);

    //Send Decrement message
    let count = addr.send(Decrement).await.unwrap();
    println!("Count after decrement: {}", count);

    Ok(())
}