extern crate simple_server;

use std::env;

use simple_server::Server;

fn get_server_port() -> String {
    env::var("PORT").unwrap_or("7878".to_string())
}

fn main() {
    let app = Server::new(|_request, mut response| {
        let motivation = "hello, world!!!".as_bytes(); //templates::motivation();
        Ok(response.body(motivation)?)
    });
 
    let host = "0.0.0.0";
    let port = get_server_port();
    let address = format!("{}:{}", host, port);

    println!("* Running on http://{}", address);
    app.listen(host, &port);
}
