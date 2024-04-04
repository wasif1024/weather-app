use prettytable::{row,Table};
use serde::Deserialize;
pub(crate) mod helper;
#[derive(Debug, Deserialize)]
pub struct WeatherBox{
    pub coord:CoordStruct,
    pub weather:Vec<WeatherStruct>,
    pub base:String,
    pub main:MainStruct,
    pub visibility:i64,
    pub wind:WindStruct,
    pub clouds:CloudStruct,
    pub dt:i64,
    pub sys:SysStruct,
    pub timezone:i64,
pub id:i64,
pub name:String,
pub cod:i64
}
impl WeatherBox {
    pub fn display(&self) {
        let mut table = Table::new();
        table.add_row(row!["Temperature", "Temperature Max", "Wind Speed","Pressure"]);
        table.add_row(row![self.main.temp, self.main.temp_max,self.wind.speed,self.main.pressure]);
        table.printstd();
    }
}
#[derive(Debug, Deserialize)]
pub struct CoordStruct{
    pub lon:f32,
    pub lat:f32,
}
#[derive(Debug, Deserialize)]
pub struct WeatherStruct{
    id:i32,
    main:String,
    description:String,
    icon:String
}
#[derive(Debug, Deserialize)]
pub struct MainStruct{
    temp:f32,
    feels_like:f32,
    temp_min:f32,
    temp_max:f32,
    pressure:f32,
    humidity:f32,
    sea_level:f32,
    grnd_level:f32
}
#[derive(Debug, Deserialize)]
pub struct WindStruct{
    pub speed:f32,
    pub deg:f32,
    pub gust:f32,
}
#[derive(Debug, Deserialize)]
pub struct CloudStruct{
    pub all:i64,
}
#[derive(Debug, Deserialize)]
pub struct SysStruct{
    pub country:String,
    pub sunrise:i64,
    pub sunset:i64
}