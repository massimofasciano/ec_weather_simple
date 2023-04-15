#[cfg(feature = "quickxml_to_serde")]
fn quickxml_to_json(xml: String) -> String {
    let conf = quickxml_to_serde::Config::new_with_custom_values(true, 
        "", "value", quickxml_to_serde::NullValue::Ignore);
    let v = quickxml_to_serde::xml_string_to_json(xml, &conf).unwrap();
    v.get("siteData").unwrap().get("currentConditions").unwrap().to_string()
}

#[cfg(feature = "simple_xml")]
fn simple_xml_to_json_extract(xml: String) -> String {
    let weather_data = simple_xml::from_string(&xml).unwrap();
    let current_conditions = &weather_data.get_nodes("currentConditions").unwrap()[0];
    format!("{{ {} }}",[("temperature","temperature"),("relativeHumidity","relative_humidity")]
        .map(|(tag,name)| format!("\"{}\": {{ \"value\" : {} }}",name,
            current_conditions.get_nodes(tag).unwrap()[0].content.parse::<f64>().unwrap()
        )).join(","))
}

fn main() {
    let url = "https://dd.weather.gc.ca/citypage_weather/xml/QC/s0000635_e.xml";
    let response = reqwest::blocking::get(url).unwrap();
    if response.status().is_success() {
        let xml = response.text().unwrap();
        #[cfg(feature = "quickxml_to_serde")]
        let json = quickxml_to_json(xml);
        #[cfg(feature = "simple_xml")]
        let json = simple_xml_to_json_extract(xml);
        println!("{json}");
    } else {
        panic!("Failed to fetch weather data for {} with status: {}",url,response.status());
    }
}
