mod host;

fn main()
{
    let ip = String::from("127.0.0.1");
    let h1 = host::Host::new(3, &ip);

    match h1 {
        Ok(h11) => {
            h11.scan();
        }
        Err(_) => {}
    }
}