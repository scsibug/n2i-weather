// Receive NATS weather messages, and forward to InfluxDB.
use influxdb::InfluxDbWriteable;
use chrono::{DateTime, TimeZone, Utc};

#[derive(InfluxDbWriteable)]
    struct WeatherReading {
        time: DateTime<Utc>,
        temp: f64,
        feels_like: f64,
        humidity: f64,
        pressure: f64,
        uvi: f64,
        visibility: f64,
        wind_deg: f64,
        wind_speed: f64,
        dew_point: f64,
        clouds: f64,
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
        println!("{}", mainobj.get("temp").unwrap());
        let temp = mainobj.get("temp").unwrap().as_f64().unwrap();
        // feels like temp
        let feels_like = mainobj.get("feels_like").unwrap().as_f64().unwrap();
        // humiditiy
        let humidity = mainobj.get("humidity").unwrap().as_f64().unwrap();
        // pressure
        let pressure = mainobj.get("pressure").unwrap().as_f64().unwrap();
        // wind
        let wind_deg = mainobj.get("wind_deg").unwrap().as_f64().unwrap();
        let wind_speed = mainobj.get("wind_speed").unwrap().as_f64().unwrap();
        // dew point
        let dew_point = mainobj.get("dew_point").unwrap().as_f64().unwrap();
        // visibility
        let visibility = mainobj.get("visibility").unwrap().as_f64().unwrap();
        // uvi
        let uvi = mainobj.get("uvi").unwrap().as_f64().unwrap();
        // clouds
        let clouds = mainobj.get("clouds").unwrap().as_f64().unwrap();
        // parse the data payload
        let dt = Utc.timestamp(mainobj.get("dt").unwrap().as_i64().unwrap(), 0); 
        println!("{}", dt);
        let wr = WeatherReading {
            time: dt,
            temp: temp,
            feels_like: feels_like,
            humidity: humidity,
            pressure: pressure,
            wind_deg: wind_deg,
            wind_speed: wind_speed,
            dew_point: dew_point,
            uvi: uvi,
            visibility: visibility,
            clouds: clouds,
            zipcode: "76034".to_string()
        }; 
        let write_result = client
            .query(&wr.into_query("weather")).await;
        assert!(write_result.is_ok(), "Write result to influxdb was not okay");
        //let vr: Result<serde_json::Value, serde_json::error::Error> = serde_json::from_str(event.data().unwrap());
//        event.deserialize(msgstr)
//        let parsed_event = serde::from_str(msgstr).unwrap();

        // Need to run iter_attributes over the parsed Event
    }

}
