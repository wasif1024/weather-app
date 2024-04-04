use reqwest::Client;
use log::{info,error};
pub async fn get_weather(city: &str) -> Result<crate::weather::WeatherBox, reqwest::Error> {
    let client = Client::new();
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid=47288600756feb19c46aafe3f7a3d4a0&units=metric", city);
    match client.get(&url).send().await{
        Ok(response)=>{
            match response.json::<crate::weather::WeatherBox>().await{
                Ok(parsed_respons)=>{
                    info!("Got Weather Response");
                    Ok(parsed_respons)
                },Err(err)=>{
                    error!("{}", format!("Get Weather Error:{}",err.to_string()));
                    Err(err)
                }
            }
            
        },Err(err)=>{
            error!("{}", format!("Get Weather Error:{}",err.to_string()));
            Err(err)
        }
    }
    
}