use clap::{App, Arg};
use log::error;
mod weather;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Wasif Weather CLI")
        .arg(Arg::with_name("city")
            .required(true)
            .takes_value(true).display_order(2).multiple(true)
            .index(1))
        .get_matches();
    let city = matches.value_of("city").unwrap();
    match crate::weather::helper::get_weather(city).await {
        Ok(weather) => {
            weather.display();
        }
        Err(e) => {
            error!("{}", format!("Get Weather Error:{}",e.to_string()));
        }
    }

    Ok(())
}
