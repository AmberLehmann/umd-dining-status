use serde::Deserialize;


// DiningStatus obtained from .json file
#[derive(Debug, Deserialize)]
struct DiningStatus {
    #[serde(alias = "currentTime")]
    time: String,
    #[serde(alias = "level1")]
    north: u8,
    #[serde(alias = "level2")]
    south: u8,
    #[serde(alias = "level3")]
    yahen: u8,
}

// struct Time {
//     second: u8,
//     minute: u8,
//     hour: u8,
//     day: u8,
//     month: u8,
//     year: u8
// }

// Formats the output for each dining hall
fn busy_status<'a>(level: &u8) -> &'a str { 
    match level {
        1 => "Not Busy",
        2 => "Moderately Busy",
        3 => "Very Busy",
        _ => "Error"
    }
}

fn main() {
    let response = 
        reqwest::blocking::get("https://dsa-ws01.umd.edu/DiningBusyMeter/get.json").unwrap();
    let mut din: DiningStatus = response
        .json::<Vec<DiningStatus>>()
        .expect("REASON")
        .remove(0);
    if let Some(index) = din.time.find('.') {
        din.time.truncate(index);
    }
    println!(
        "{:?}\n251 North:    {}\nSouth Campus: {}\nYahentamitsi: {}",
        din.time,
        busy_status(&din.north),
        busy_status(&din.south),
        busy_status(&din.yahen)
    );
}


