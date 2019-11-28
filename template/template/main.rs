use {{snakecase info.title}}::server::*;

#[derive(Clone)]
struct Server;
impl {{camelcase info.title}} for Server {
    type Error = std::io::Error;
}

fn main() -> std::io::Result<()> {
    let server = Server{};
    run(server)
}