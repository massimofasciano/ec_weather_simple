fn main() {
    let mut url = "https://dd.weather.gc.ca/citypage_weather/xml/QC/s0000635_e.xml";
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() > 1 {
        url = &args[1];
    }
    let response = reqwest::blocking::get(url).unwrap();
    if response.status().is_success() {
        println!("{}",{
            let conf = quickxml_to_serde::Config::new_with_custom_values(true, 
                "", "value", quickxml_to_serde::NullValue::Ignore);
            quickxml_to_serde::xml_string_to_json(response.text().unwrap(), &conf).unwrap()
                .get("siteData").unwrap()
                .get("currentConditions").unwrap()
        });
    } else {
        println!("{{\"error\":\"Failed to fetch weather data for {} with status: {}\"}}",url,response.status());
        std::process::exit(1);
    }
}
