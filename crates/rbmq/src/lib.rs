
use amiquip::{Connection, Exchange, Publish, Result as AmqpResult,Error as AmqpError};

pub fn send_msg(payload: &str) -> Result<(),Box<dyn std::error::Error>> {
    
    let mut connection = Connection::insecure_open("amqp://admin:bitnami1@localhost:5672")?;
    let channel = connection.open_channel(None)?;

    let exchange = Exchange::direct(&channel);

    let _ = match exchange.publish(Publish::new(payload.as_bytes(), "rt")) {
        Ok(_) => println!("Message sent"),
        Err(err) => println!("Error sending message: {}", err),
    };

    let _ = connection.close();

    Ok(())
}


#[derive(Debug, Default, serde_derive::Deserialize, PartialEq, Eq)]
pub struct RbmqConfig {
    msconn: String,
    msqueue: String,
}