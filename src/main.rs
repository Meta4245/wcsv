// Copyright 2023 Meta4245
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde::{Deserialize, Serialize};
use std::io::Write;
use std::time::Duration;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};
use ureq::{Agent, AgentBuilder};

#[allow(non_camel_case_types)]
#[derive(Debug, EnumCountMacro, EnumIter)]
enum Countries {
    Islamic_Republic_Of_Afghanistan,
    Albania,
    Algeria,
    Andorra,
    Angola,
    Antigua_And_Barbuda,
    Argentina,
    Armenia,
    Australia,
    Austria,
    Azerbaijan,
    Bahamas,
    Bahrain,
    Bangladesh,
    Barbados,
    Belarus,
    Belgium,
    Belize,
    Benin,
    Bhutan,
    Bolivia,
    Bosnia_And_Herzegovina,
    Botswana,
    Brazil,
    Brunei,
    Bulgaria,
    Burkina_Faso,
    Burundi,
    Cambodia,
    Cameroon,
    Canada,
    Central_African_Republic,
    Chad,
    Chile,
    China,
    Colombia,
    Comoros,
    Republic_Of_The_Congo,
    Costa_Rica,
    Croatia,
    Cuba,
    Cyprus,
    Czech_Republic,
    Denmark,
    Djibouti,
    Dominica,
    Dominican_Republic,
    Ecuador,
    Egypt,
    El_Salvador,
    Equatorial_Guinea,
    Eritrea,
    Estonia,
    Eswatini,
    Ethiopia,
    Fiji,
    Finland,
    France,
    Gabon,
    Gambia,
    Georgia,
    Germany,
    Ghana,
    Greece,
    Grenada,
    Guatemala,
    Guinea,
    Guinea_Bissau,
    Guyana,
    Haiti,
    Honduras,
    Hungary,
    Iceland,
    India,
    Indonesia,
    Iran,
    Iraq,
    Ireland,
    Israel,
    Italy,
    Jamaica,
    Japan,
    Jordan,
    Kazakhstan,
    Kenya,
    Kiribati,
    Korea,
    Kuwait,
    Kyrgyzstan,
    Latvia,
    Lebanon,
    Lesotho,
    Liberia,
    Libya,
    Liechtenstein,
    Lithuania,
    Luxembourg,
    Madagascar,
    Malawi,
    Malaysia,
    Maldives,
    Mali,
    Malta,
    Marshall_Islands,
    Mauritania,
    Mauritius,
    Mexico,
    Micronesia,
    Moldova,
    Monaco,
    Mongolia,
    Montenegro,
    Morocco,
    Mozambique,
    Myanmar,
    Namibia,
    Nauru,
    Nepal,
    Netherlands,
    New_Zealand,
    Nicaragua,
    Niger,
    Nigeria,
    North_Macedonia,
    Norway,
    Oman,
    Pakistan,
    Palau,
    Palestine,
    Panama,
    Papua_New_Guinea,
    Paraguay,
    Peru,
    Philippines,
    Poland,
    Portugal,
    Qatar,
    Romania,
    Russia,
    Rwanda,
    Saint_Kitts_And_Nevis,
    Saint_Lucia,
    Saint_Vincent_and_the_Grenadines,
    Samoa,
    San_Marino,
    Sao_Tome_And_Principe,
    Saudi_Arabia,
    Senegal,
    Serbia,
    Seychelles,
    Sierra_Leone,
    Singapore,
    Slovakia,
    Slovenia,
    Solomon_Islands,
    Somalia,
    South_Africa,
    South_Sudan,
    Spain,
    Sri_Lanka,
    Sudan,
    Suriname,
    Sweden,
    Switzerland,
    Syria,
    Tajikistan,
    Tanzania,
    Thailand,
    Togo,
    Tonga,
    Trinidad_And_Tobago,
    Tunisia,
    Turkey,
    Turkmenistan,
    Tuvalu,
    Uganda,
    Ukraine,
    United_Arab_Emirates,
    United_Kingdom,
    United_States,
    Uruguay,
    Uzbekistan,
    Vanuatu,
    Vatican_City,
    Venezuela,
    Vietnam,
    Yemen,
    Zambia,
    Zimbabwe,
}

#[derive(Serialize, Deserialize, Clone)]
struct CurrencyInfo {
    name: String,
    code: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct CountryInfo {
    name: String,
    capital: String,
    region: String,
    iso2: String,
    gdp: Option<f64>,
    gdp_per_capita: Option<f32>,
    gdp_growth: Option<f32>,
    currency: Option<CurrencyInfo>,
    surface_area: Option<f64>,
    sex_ratio: f32,
    life_expectancy_male: Option<f32>,
    life_expectancy_female: Option<f32>,
    infant_mortality: Option<f32>,
    employment_services: Option<f32>,
    employment_industry: Option<f32>,
    employment_agriculture: Option<f32>,
    unemployment: Option<f32>,
    imports: Option<f64>,
    exports: Option<f64>,
    homicide_rate: Option<f32>,
    population: f64,
    pop_growth: f32,
    pop_density: f32,
    urban_population: f32,
    urban_population_growth: f32,
    internet_users: Option<f32>,
    fertility: Option<f32>,
    refugees: Option<f64>,
    primary_school_enrollment_male: Option<f32>,
    primary_school_enrollment_female: Option<f32>,
    secondary_school_enrollment_male: Option<f32>,
    secondary_school_enrollment_female: Option<f32>,
    post_secondary_enrollment_male: Option<f32>,
    post_secondary_enrollment_female: Option<f32>,
    co2_emissions: Option<f32>,
    forested_area: Option<f32>,
    tourists: Option<f64>,
    threatened_species: f64,
}

fn handle_option_f32(handle: Option<f32>) -> String {
    match handle {
        Some(value) => value.to_string(),
        None => "No Info".to_string(),
    }
}

fn handle_option_f64(handle: Option<f64>) -> String {
    match handle {
        Some(value) => value.to_string(),
        None => "No Info".to_string(),
    }
}

impl std::fmt::Display for Countries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut value = format!("{:?}", self);
        if value == "Guinea_Bissau" {
            value = "Guinea-Bissau".to_string();
        } else {
            value = str::replace(value.as_str(), "_", " ");
        }
        write!(f, "{}", value)
    }
}

fn main() {
    let api_key: String;
    let path = std::path::Path::new("apikey");
    if path.exists() {
        api_key = std::fs::read_to_string(path).expect("failure reading apikey file");
    } else {
        print!("api key file does not exist. enter api key from api-ninjas.com: ");
        std::io::stdout().flush().expect("failure flushing stdout");
        let mut read_api_key = String::new();
        std::io::stdin()
            .read_line(&mut read_api_key)
            .expect("failure reading from stdin");
        api_key = read_api_key;
        std::fs::write(path, &api_key).expect("failure writing to file");
    }
    let agent: Agent = AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();
    let mut countries: Vec<CountryInfo> = Vec::new();
    let mut i: u16 = 1;
    println!("fetching countries");
    for country in Countries::iter() {
        println!("{} [{}/{}]", country.to_string(), i, Countries::COUNT);
        let url = format!(
            "https://api.api-ninjas.com/v1/country?name={}",
            country.to_string()
        );
        let country: Vec<CountryInfo> = agent
            .get(url.as_str())
            .set("X-Api-Key", api_key.as_str())
            .call()
            .expect("failed fetching country")
            .into_json()
            .expect("failed turning info of country into json");
        countries.push(country[0].clone());
        i += 1;
    }
    println!("done fetching, making CSV now");
    let file =
        std::fs::File::create("world_countries_info.csv").expect("failure creating a csv file");
    let mut wtr = csv::Writer::from_writer(file);
    wtr.write_record(&[
        "Name",
        "ISO2 Code",
        "Capital",
        "Region",
        "GDP",
        "GDP Per Capita",
        "GDP Growth",
        "Surface Area",
        "Sex Ratio",
        "Male Life Expectancy",
        "Female Life Expectancy",
        "Infant Mortality",
        "Employment in services",
        "Employment in industry",
        "Employment in agriculture",
        "Unemployment",
        "Imports",
        "Exports",
        "Homicide Rate",
        "Population",
        "Population Growth",
        "Population Density",
        "Urban Population",
        "Urban Population Growth",
        "Internet Users",
        "Fertility",
        "Refugees",
        "Male Primary School Enrollment",
        "Female Primary School Enrollment",
        "Male Secondary School Enrollment",
        "Female Secondary School Enrollment",
        "Male Post-Secondary Enrollment",
        "Female Post-Secondary Enrollment",
        "CO2 Emissions",
        "Forested Area",
        "Tourists",
        "Threatened Species",
    ])
    .expect("failed writing title records");
    i = 1;
    for country in countries {
        println!("{} [{}/{}]", country.name, i, Countries::COUNT);
        wtr.write_record(&[
            country.name,
            country.iso2,
            country.capital,
            country.region,
            handle_option_f64(country.gdp),
            handle_option_f32(country.gdp_per_capita),
            handle_option_f32(country.gdp_growth),
            handle_option_f64(country.surface_area),
            country.sex_ratio.to_string(),
            handle_option_f32(country.life_expectancy_male),
            handle_option_f32(country.life_expectancy_female),
            handle_option_f32(country.infant_mortality),
            handle_option_f32(country.employment_services),
            handle_option_f32(country.employment_industry),
            handle_option_f32(country.employment_agriculture),
            handle_option_f32(country.unemployment),
            handle_option_f64(country.imports),
            handle_option_f64(country.exports),
            handle_option_f32(country.homicide_rate),
            country.population.to_string(),
            country.pop_growth.to_string(),
            country.pop_density.to_string(),
            country.urban_population.to_string(),
            country.urban_population_growth.to_string(),
            handle_option_f32(country.internet_users),
            handle_option_f32(country.fertility),
            handle_option_f64(country.refugees),
            handle_option_f32(country.primary_school_enrollment_male),
            handle_option_f32(country.primary_school_enrollment_female),
            handle_option_f32(country.secondary_school_enrollment_male),
            handle_option_f32(country.secondary_school_enrollment_female),
            handle_option_f32(country.post_secondary_enrollment_male),
            handle_option_f32(country.post_secondary_enrollment_female),
            handle_option_f32(country.co2_emissions),
            handle_option_f32(country.forested_area),
            handle_option_f64(country.tourists),
            country.threatened_species.to_string(),
        ])
        .expect("failed to write record");
        i += 1;
    }
    wtr.flush().expect("failed flushing CSV writer");
}
