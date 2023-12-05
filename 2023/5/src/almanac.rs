use std::io::Lines;

use crate::interval;
use crate::interval::{Interval, Mapping};

pub(crate) struct Almanac {
    pub seeds: Vec<u64>,
    pub seed_to_soil: Mapping,
    pub soil_to_fertilizer: Mapping,
    pub fertilizer_to_water: Mapping,
    pub water_to_light: Mapping,
    pub light_to_temperature: Mapping,
    pub temperature_to_humidity: Mapping,
    pub humidity_to_location: Mapping,
}

impl Almanac {
    pub fn location_for_seed(&self, seed: &u64) -> u64 {
        let soil = interval::value_for(&self.seed_to_soil, seed);
        let fertilizer = interval::value_for(&self.soil_to_fertilizer, &soil);
        let water = interval::value_for(&self.fertilizer_to_water, &fertilizer);
        let light = interval::value_for(&self.water_to_light, &water);
        let temperature = interval::value_for(&self.light_to_temperature, &light);
        let humidity = interval::value_for(&self.temperature_to_humidity, &temperature);
        interval::value_for(&self.humidity_to_location, &humidity)
    }

    pub(crate) fn from_lines(lines: Lines<&[u8]>) -> Self {
        let rows:Vec<String> = lines.map(|l|l.unwrap().trim().to_owned()).collect();

        let seeds: Vec<u64> = rows[0].split_whitespace().filter_map(|s| s.trim().parse().ok()).collect();

        let mut idx = 2;
        assert!(idx < rows.len());
        assert_eq!(rows[idx], "seed-to-soil map:");
        let seed_to_soil:Mapping = Almanac::mapping(&rows, idx + 1);

        idx += seed_to_soil.len() + 2;
        assert!(idx < rows.len());
        assert_eq!(rows[idx], "soil-to-fertilizer map:");
        let soil_to_fertilizer:Mapping = Almanac::mapping(&rows, idx + 1);

        idx += soil_to_fertilizer.len() + 2;
        assert!(idx < rows.len());
        assert_eq!(rows[idx], "fertilizer-to-water map:");
        let fertilizer_to_water:Mapping = Almanac::mapping(&rows, idx + 1);

        idx += fertilizer_to_water.len() + 2;
        assert!(idx < rows.len());
        assert_eq!(rows[idx], "water-to-light map:");
        let water_to_light:Mapping = Almanac::mapping(&rows, idx + 1);

        idx += water_to_light.len() + 2;
        assert!(idx < rows.len());
        assert_eq!(rows[idx], "light-to-temperature map:");
        let light_to_temperature:Mapping = Almanac::mapping(&rows, idx + 1);

        idx += light_to_temperature.len() + 2;
        assert!(idx < rows.len());
        assert_eq!(rows[idx], "temperature-to-humidity map:");
        let temperature_to_humidity:Mapping = Almanac::mapping(&rows, idx + 1);

        idx += temperature_to_humidity.len() + 2;
        assert!(idx < rows.len());
        assert_eq!(rows[idx], "humidity-to-location map:");
        let humidity_to_location:Mapping = Almanac::mapping(&rows, idx + 1);
        
        Almanac{
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }

    fn mapping(rows: &Vec<String>, start:usize) -> Mapping {
        rows.iter().skip(start).map_while(|s|Interval::from_str(s)).collect()
    }
}