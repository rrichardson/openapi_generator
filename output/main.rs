use vizyr_api::server::*;

#[derive(Clone)]
struct Server;
impl VizyrApi for Server {
    type Error = std::io::Error;
}

fn main() -> std::io::Result<()> {
    let server = Server {};
    run(server)
}
