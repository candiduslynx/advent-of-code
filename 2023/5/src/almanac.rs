use std::io::Lines;

use crate::interval;
use crate::interval::{Interval, Mapping};
use crate::range::Range;

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

    pub fn locations_for_ranges(&self, ranges: Vec<Range>) -> Vec<Range> {
        let soil = Almanac::apply_mapping(&self.seed_to_soil, ranges);
        let fertilizer = Almanac::apply_mapping(&self.soil_to_fertilizer, soil);
        let water = Almanac::apply_mapping(&self.fertilizer_to_water, fertilizer);
        let light = Almanac::apply_mapping(&self.water_to_light, water);
        let temperature = Almanac::apply_mapping(&self.light_to_temperature, light);
        let humidity = Almanac::apply_mapping(&self.temperature_to_humidity, temperature);
        Almanac::apply_mapping(&self.humidity_to_location, humidity)
    }

    pub(crate) fn from_lines(lines: Lines<&[u8]>) -> Self {
        let rows: Vec<String> = lines.map(|l| l.unwrap().trim().to_owned()).collect();

        let seeds: Vec<u64> = rows[0].split_whitespace().filter_map(|s| s.trim().parse().ok()).collect();

        let mut idx = 2;
        assert!(idx < rows.len());
        assert_eq!(rows[idx], "seed-to-soil map:");
        let seed_to_soil: Mapping = Almanac::mapping(&rows, idx + 1);

        idx += seed_to_soil.len() + 2;
        assert!(idx < rows.len());
        assert_eq!(rows[idx], "soil-to-fertilizer map:");
        let soil_to_fertilizer: Mapping = Almanac::mapping(&rows, idx + 1);

        idx += soil_to_fertilizer.len() + 2;
        assert!(idx < rows.len());
        assert_eq!(rows[idx], "fertilizer-to-water map:");
        let fertilizer_to_water: Mapping = Almanac::mapping(&rows, idx + 1);

        idx += fertilizer_to_water.len() + 2;
        assert!(idx < rows.len());
        assert_eq!(rows[idx], "water-to-light map:");
        let water_to_light: Mapping = Almanac::mapping(&rows, idx + 1);

        idx += water_to_light.len() + 2;
        assert!(idx < rows.len());
        assert_eq!(rows[idx], "light-to-temperature map:");
        let light_to_temperature: Mapping = Almanac::mapping(&rows, idx + 1);

        idx += light_to_temperature.len() + 2;
        assert!(idx < rows.len());
        assert_eq!(rows[idx], "temperature-to-humidity map:");
        let temperature_to_humidity: Mapping = Almanac::mapping(&rows, idx + 1);

        idx += temperature_to_humidity.len() + 2;
        assert!(idx < rows.len());
        assert_eq!(rows[idx], "humidity-to-location map:");
        let humidity_to_location: Mapping = Almanac::mapping(&rows, idx + 1);

        Almanac {
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

    fn mapping(rows: &Vec<String>, start: usize) -> Mapping {
        let mut m: Mapping = rows.iter().skip(start).map_while(|s| Interval::from_str(s)).collect();
        m.sort_by(|a, b| u64::cmp(&a.start, &b.start));
        m
    }

    fn apply_mapping(m: &Mapping, ranges: Vec<Range>) -> Vec<Range> {
        Range::reduce(ranges.iter().
            flat_map(|r| Almanac::apply_mapping_to_range(m, r)).collect())
    }

    fn apply_mapping_to_range(m: &Mapping, range: &Range) -> Vec<Range> {
        assert!(range.start <= range.end);
        let mut start = range.start;
        let end = range.end;
        let mut res: Vec<Range> = Vec::new();

        m.iter().for_each(|m| {
            if start > end {
                // we're done, just iterate through
                return;
            }
            if m.end < start || end < m.start {
                // don't have intersection, leave
                return;
            }

            if start < m.start {
                // have an idempotent part
                res.push(Range { start, end: m.start - 1 });
                start = m.start;
            }

            if m.end >= end {
                // our last bit
                res.push(Range {
                    start: m.value_for(&start).unwrap(),
                    end: m.value_for(&end).unwrap(),
                });
                start = end + 1;
                return;
            }

            // push range for current bit
            res.push(Range {
                start: m.value_for(&start).unwrap(),
                end: m.value_for(&m.end).unwrap(),
            });

            start = m.end + 1;
        });

        if start <= end {
            // have an idempotent tail
            res.push(Range { start, end });
        }
        return Range::reduce(res);
    }
}
