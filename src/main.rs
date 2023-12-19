use std::string::String;

fn main()
{
    let s1 = String::from("127.0.0.1");
    println!("Hello, world!");

    match Host::new(&s1, true)
    {
        Ok(host1) =>
        {
            println!("Success {}", host1.start_port);

            host1.tcp_scan();
        }
        Err(_) =>
        {
            println!("Fail");
        }
    }
}
