use async_std::task;

fn main() -> Result<(), surf::Exception> {
    femme::start(log::LevelFilter::Info)?;
    task::block_on(async {
        let uri = "https://httpbin.org/get";
        let string: String = surf::get(uri).recv_string().await?;
        println!("msg: {}", string);
        Ok(())
    })
}
