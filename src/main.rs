extern crate motivations;
extern crate simple_server;

use std::env;
use motivations::MOTIVATIONS;

use simple_server::Server;

/// Look up our server port number in PORT, for compatibility with Heroku.
fn get_server_port() -> String {
    env::var("PORT").unwrap_or("7878".to_string())
}

fn main() {
    let app = Server::new(|_request, mut response| {
        let motivation = MOTIVATIONS[0].as_bytes();
        Ok(response.body(motivation)?)
    });
 
    let host = "0.0.0.0";
    let port = get_server_port();
    let address = format!("{}:{}", host, port);

    println!("* Running on http://{}", address);
    app.listen(host, &port);
}
