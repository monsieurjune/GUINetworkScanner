mod host;

fn main()
{
    let ip = String::from("127.0.0.1");
    let h1 = host::Host::new(&String::from("full"), &ip);

    match h1 {
        Ok(h11) => {
			println!("Here we go");
            let str = h11.tcp_scan();
			println!("Result : {}", str);
        }
        Err(_) => {}
    }
}