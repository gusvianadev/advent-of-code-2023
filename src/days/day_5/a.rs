enum Phase {
    Seeds,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Debug)]
struct Item {
    source: u64,
    target: u64,
    len: u64,
}

impl Item {
    fn init() -> Self {
        Self {
            source: 0,
            target: 0,
            len: 0,
        }
    }
}

pub fn solution(file: &str) -> u64 {
    let mut lowest_location = u64::MAX;
    let mut phase = Phase::Seeds;
    let mut seeds = vec![];
    let mut seed_to_soil = vec![];
    let mut soil_to_fertilizer = vec![];
    let mut fertilizer_to_water = vec![];
    let mut water_to_light = vec![];
    let mut light_to_temperature = vec![];
    let mut temperature_to_humidity = vec![];
    let mut humidity_to_location = vec![];

    let mut num = String::new();
    for line in file.lines() {
        if line.ends_with(':') {
            continue;
        }

        let mut create_item = || -> Item {
            let mut item = Item::init();

            let mut target_assigned = false;
            for ch in line.chars() {
                if ch.is_ascii_digit() {
                    num.push(ch);
                } else {
                    let parsed_num = num.parse::<u64>().unwrap();
                    if !target_assigned {
                        item.target = parsed_num;
                        target_assigned = true;
                    } else {
                        item.source = parsed_num;
                    }
                    num.clear();
                }
            }
            item.len = num.parse::<u64>().unwrap();
            num.clear();
            item
        };

        match phase {
            Phase::Seeds => {
                if line.is_empty() {
                    phase = Phase::SeedToSoil;
                    continue;
                }

                for ch in line.chars().skip(7) {
                    if ch.is_ascii_digit() {
                        num.push(ch);
                    } else {
                        seeds.push(num.parse::<u64>().unwrap());
                        num.clear();
                    }
                }

                seeds.push(num.parse::<u64>().unwrap());
                num.clear();
            }

            Phase::SeedToSoil => {
                if line.is_empty() {
                    phase = Phase::SoilToFertilizer;
                    continue;
                }
                seed_to_soil.push(create_item());
            }
            Phase::SoilToFertilizer => {
                if line.is_empty() {
                    phase = Phase::FertilizerToWater;
                    continue;
                }
                soil_to_fertilizer.push(create_item());
            }
            Phase::FertilizerToWater => {
                if line.is_empty() {
                    phase = Phase::WaterToLight;
                    continue;
                }
                fertilizer_to_water.push(create_item());
            }
            Phase::WaterToLight => {
                if line.is_empty() {
                    phase = Phase::LightToTemperature;
                    continue;
                }
                water_to_light.push(create_item());
            }
            Phase::LightToTemperature => {
                if line.is_empty() {
                    phase = Phase::TemperatureToHumidity;
                    continue;
                }
                light_to_temperature.push(create_item());
            }
            Phase::TemperatureToHumidity => {
                if line.is_empty() {
                    phase = Phase::HumidityToLocation;
                    continue;
                }
                temperature_to_humidity.push(create_item());
            }
            Phase::HumidityToLocation => {
                humidity_to_location.push(create_item());
            }
        }
    }

    for seed in seeds {
        let mut soil = seed;

        for item in &seed_to_soil {
            if item.source <= seed && seed <= item.source + item.len {
                soil = seed - item.source + item.target;
                break;
            }
        }

        let mut fertilizer = soil;
        for item in &soil_to_fertilizer {
            if item.source <= soil && soil <= item.source + item.len {
                fertilizer = soil - item.source + item.target;
                break;
            }
        }

        let mut water = fertilizer;
        for item in &fertilizer_to_water {
            if item.source <= fertilizer && fertilizer <= item.source + item.len {
                water = fertilizer - item.source + item.target;
                break;
            }
        }

        let mut light = water;
        for item in &water_to_light {
            if item.source <= water && water <= item.source + item.len {
                light = water - item.source + item.target;
                break;
            }
        }

        let mut temperature = light;
        for item in &light_to_temperature {
            if item.source <= light && light <= item.source + item.len {
                temperature = light - item.source + item.target;
                break;
            }
        }

        let mut humidity = temperature;
        for item in &temperature_to_humidity {
            if item.source <= temperature && temperature <= item.source + item.len {
                humidity = temperature - item.source + item.target;
                break;
            }
        }

        let mut location = humidity;
        for item in &humidity_to_location {
            if item.source <= humidity && humidity <= item.source + item.len {
                location = humidity - item.source + item.target;
                break;
            }
        }

        if location < lowest_location {
            lowest_location = location;
        }

        // dbg!(&seed);
        // dbg!(&soil);
        // dbg!(&fertilizer);
        // dbg!(&water);
        // dbg!(&light);
        // dbg!(&temperature);
        // dbg!(&humidity);
        // dbg!(&location);
        // println!("-----------------------------------------------------");
    }

    lowest_location
}

