extern crate env_logger;
#[macro_use]
extern crate log;

extern crate simple_server;

use simple_server::Server;

fn main() {
    env_logger::init();

    let host = "0.0.0.0";
    let port = "7878";

    let server = Server::new(|request, mut response| {
        info!("Request received. {} {}", request.method(), request.uri());
        println!("request recieved");
         
        use std::process::Command;
        let output = Command::new("service")
                     .arg("--status-all")
                     //.arg("status")
                     .output()
                     .expect("failed to execute process");
        
        //println!("{:?}", output.stderr);
        Ok(response.body((String::from_utf8_lossy(&output.stdout)).as_bytes().to_vec())?)
        //Ok(response.body("Hello".as_bytes().to_vec())?)
    });

    server.listen(host, port);
}