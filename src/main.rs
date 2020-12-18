// Receive NATS weather messages, and forward to InfluxDB.
use influxdb::InfluxDbWriteable;
use chrono::{DateTime, Utc};

#[derive(InfluxDbWriteable)]
    struct WeatherReading {
        time: DateTime<Utc>,
        temp: f64,
        feels_like: f64,
        humidity: f64,
        #[tag] zipcode: String,
    }


#[async_std::main]
async fn main() {
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
    // Connect to influxdb
    println!("Connecting to InfluxDB");
    let client = influxdb::Client::new("http://ektar.wellorder.net:8086", "iot");
    for msg in sub.messages() {
        println!("Received Message!");
        println!("This message subject is: {}", msg.subject);
        let utf8res = std::str::from_utf8(&msg.data);
        let msgstr = match utf8res {
            Ok(s) => s,
            Err(e) => { std::process::exit(1) }
        };
        println!("Message is: {}", msgstr);
        // Build a JSON deserializer for the message
        let event : cloudevents::event::Event = serde_json::from_str(msgstr).unwrap();
        println!("{}", event);
        let payload = match event.data().unwrap() {
            cloudevents::Data::Json(v) => v,
            _ => { 
                println!("Did not match JSON payload");
                std::process::exit(1);
            }
        }; 
        println!("{}", payload);
        // extract fields from payload
        let mainobj = match payload {
            serde_json::value::Value::Object(m) => m,
            _ => {
                println!("Expected a top-level object");
                std::process::exit(1);
            }
        };
        // extract temp from mainobj
        println!("{}", mainobj.get("temp").unwrap())
        // parse the data payload
        //
        //let vr: Result<serde_json::Value, serde_json::error::Error> = serde_json::from_str(event.data().unwrap());
//        event.deserialize(msgstr)
//        let parsed_event = serde::from_str(msgstr).unwrap();

        // Need to run iter_attributes over the parsed Event
    }

}
