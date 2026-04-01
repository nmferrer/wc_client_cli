use std::io;
use regex::Regex;
use serde::{ Deserialize, Serialize };
#[derive(Debug, Deserialize, Serialize, Clone)]
enum ForecastType {
    Forecast,
    ForecastHourly,
    ForecastGridData
}
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Input {
    city: String,
    state: String,
    forecast: ForecastType,
}


fn parse_buffer(buf: &String) -> Result<crate::Input, String> {
    //Expected format: {command city, state -optional_flags}
        //On is_match:  Pack captured values into JSON struct
        //Else:         Fail and show invalid input
    //Pattern: command city, state //-flags dropped for now
    
    let re = Regex::new(r"^([a-zA-Z]+)\s([a-zA-Z\s]+),\s([a-zA-Z]{2,})").unwrap();
    assert!(re.is_match(buf));//verify: Does it match?
    let Some((_full,[command, city, state])) = 
        re.captures(buf).map(|caps| caps.extract()) else { return Err("Regex failed".to_string()) };
    //TODO: Gracefully handle panic! 
    let mut f_type = ForecastType::Forecast;
    match command { //OK. Extend to other commands.
        "forecast"          => f_type = ForecastType::Forecast,
        "forecastHourly"    => f_type = ForecastType::ForecastHourly,
        "forecastGridData"  => f_type = ForecastType::ForecastGridData,
        _ => println!("Invalid Command."),
    } //Invalid: Raise error. Regex handles formatting, shouldn't need to handle too much logic at client.
    
    //OKOK: Current Task: Serialize/Deserialize Input struct between client/server 
    Ok(Input{city: city.to_string(), state:state.to_string(), forecast:f_type})
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //While localhost:8080 is live, read terminal until newline char.
    //Parse string, create valid REST object adhering to API, send to server.
    //Output response at terminal.
   
    //TODO: GRACEFULLY HANDLE ERRORS AND BAD INPUT
    let addr = "127.0.0.1:8080";
    let addr_url = format!("http://{}/forecast", addr);
    //TODO: Hacky solution. In practice, reqwest accesses URL
    //TODO: CONSIDER different endpoints for different functions
    let client = reqwest::Client::builder()
        .build()
        .expect("client built");
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(_) => {
            let req_body = parse_buffer(&buf).unwrap();
            let res = client.get(addr_url)
                .json(&req_body)
                .send()
                .await?;
            let data = res.json::<Vec<String>>().await.expect("Response is vector of Strings.");
            for row in data {
                println!("{}", row);
            }
        }
        Err(error) => println!("error: {error}"),
    }
    Ok(())
}
