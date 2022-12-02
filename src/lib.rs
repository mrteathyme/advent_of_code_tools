use std::path::Path;
use std::fs;
use chrono::Datelike;

pub async fn puzzle_input(token: String, year:Option<i32> ,day: Option<i32>) -> Result<String, reqwest::Error>{
    let current_date = chrono::Utc::now();
    let year_parsed;
    let day_parsed;

    match year {
    Some(i) => {
        year_parsed = i;
    }
    None => {
        year_parsed = current_date.year();
    }
    }
    match day {
    Some(i) => {
        day_parsed = i;
    }
    None => {
        day_parsed = current_date.year() as i32;
    }
    }

    let rs = Path::new(&format!("{year_parsed}-{day_parsed}.txt")).exists();
    
    if rs == true{
        Ok(fs::read_to_string(format!("{year_parsed}-{day_parsed}.txt")).expect("failed to read file"))
    }
    else{
        let client = reqwest::Client::new();
        let uri = format!("https://adventofcode.com/{}/day/{}/input", year_parsed.to_string(), day_parsed.to_string());
        let res = client.get(uri).header("Cookie", format!("session={token}")).send().await?.text().await?;
        fs::write(format!("{year_parsed}-{day_parsed}.txt"), res.to_string()).expect("Unable to write file");
        Ok(res.to_string())
    }
}
