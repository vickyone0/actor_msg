use actix::prelude::*;

// Define the Ping actor
struct PingActor {
    pong_addr: Addr<PongActor>,
}

impl Actor for PingActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Send the first Ping message
        self.pong_addr.do_send(Ping);
    }
}

// Define the Ping message
#[derive(Message)]
#[rtype(result = "()")]
struct Ping;

// Implement the Handler trait for the Ping message
impl Handler<Ping> for PingActor {
    type Result = ();

    fn handle(&mut self, _msg: Ping, _ctx: &mut Self::Context) -> Self::Result {
        println!("Ping received");
        // Send a Pong message back to the Pong actor
        self.pong_addr.do_send(Pong);
    }
}

// Define the Pong actor
struct PongActor {
    ping_addr: Addr<PingActor>,
    count: usize,
}

impl Actor for PongActor {
    type Context = Context<Self>;
}

// Define the Pong message
#[derive(Message)]
#[rtype(result = "()")]
struct Pong;

// Implement the Handler trait for the Pong message
impl Handler<Pong> for PongActor {
    type Result = ();

    fn handle(&mut self, _msg: Pong, ctx: &mut Self::Context) -> Self::Result {
        println!("Pong received");
        self.count += 1;
        if self.count > 3 {
            // Stop the system after 3 pings and pongs
            System::current().stop();
        } else {
            // Send a Ping message back to the Ping actor
            self.ping_addr.do_send(Ping);
        }
    }
}

#[actix_rt::main]
async fn main() -> Result<(), ()> {
    // Start the Pong actor
    let pong_addr = PongActor {
        ping_addr: Addr::default(), // Placeholder, will be updated later
        count: 0,
    }
    .start();

    // Start the Ping actor
    let ping_addr = PingActor {
        pong_addr: pong_addr.clone(),
    }
    .start();

    // Update the Pong actor with the Ping actor's address
    let mut pong = PongActor::create(|ctx| {
        PongActor {
            ping_addr: ping_addr.clone(),
            count: 0,
        }
    });

    Ok(())
}