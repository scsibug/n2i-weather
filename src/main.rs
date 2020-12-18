// Receive NATS weather messages, and forward to InfluxDB.

fn main() {
    println!("Connecting to NATS");
    let ncres = nats::connect("nats.wellorder.net");
    let nc = match ncres {
        Ok(conn) => conn,
        Err(e) => {
            println!("Could not connect, bailing");
            std::process::exit(1);
        }
    };
    println!("Subscribing to iot.weather topic");
    let subres = nc.subscribe("iot.weather");
    let sub = match subres {
        Ok(s) => s,
        Err(e) => {
            println!("Could not get subscription, bailing");
            std::process::exit(1);
        }
    };
    for msg in sub.messages() {
        println!("Received Message!");
        println!("This message subject is: {}", msg.subject);
        let utf8res = std::str::from_utf8(&msg.data);
        let msgstr = match utf8res {
            Ok(s) => s,
            Err(e) => { std::process::exit(1) }
        };
        println!("Message is: {}", msgstr)
//        let event: CloudEvent = cloudevents::from_str(data).unwrap();
    }

}
