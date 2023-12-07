use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct MapEntry {
    pub dest_start: usize,
    pub source_start: usize,
    pub range: usize,
}

#[derive(Debug, Clone)]
struct Map(Vec<MapEntry>);

impl Map {
    fn from_input(input: &str) -> Self {
        let data = input
            .lines()
            .skip(1)
            .map(|line| {
                let mut parts = line.split(' ');
                let dest_start = parts.next().unwrap().parse::<usize>().unwrap();
                let source_start = parts.next().unwrap().parse::<usize>().unwrap();
                let range = parts.next().unwrap().parse::<usize>().unwrap();

                MapEntry {
                    dest_start,
                    source_start,
                    range,
                }
            })
            .collect::<Vec<_>>();

        Self(data)
    }

    fn source_to_dest(&self, source: usize) -> usize {
        for entry in &self.0 {
            if source >= entry.source_start && source < entry.source_start + entry.range {
                return entry.dest_start + (source - entry.source_start);
            }
        }

        source
    }

    fn sr_to_dr(&self, ranges: &[(usize, usize)]) -> Vec<(usize, usize)> {
        let mut source_ranges = ranges.to_vec();
        let mut d_ranges = Vec::new();

        'queue: while let Some((s_start, s_end)) = source_ranges.pop() {
            for entry in &self.0 {
                let esrc_end = entry.source_start + entry.range;

                if s_end <= entry.source_start || s_start >= esrc_end {
                    continue;
                }

                let common_start = s_start.max(entry.source_start);
                let common_end = s_end.min(esrc_end);

                if entry.source_start <= entry.dest_start {
                    let offset = entry.dest_start - entry.source_start;

                    d_ranges.push((common_start + offset, common_end + offset));
                } else {
                    let offset = entry.source_start - entry.dest_start;

                    d_ranges.push((common_start - offset, common_end - offset));
                }

                if s_start < common_start {
                    source_ranges.push((s_start, common_start));
                }

                if s_end > common_end {
                    source_ranges.push((common_end, s_end));
                }

                continue 'queue;
            }

            d_ranges.push((s_start, s_end));
        }

        d_ranges
    }
}

struct Data {
    pub seeds: Vec<usize>,

    pub seed_to_soil: Map,
    pub soil_to_fertilizer: Map,
    pub fertilizer_to_water: Map,
    pub water_to_light: Map,
    pub light_to_temperature: Map,
    pub temperature_to_humidity: Map,
    pub humidity_to_location: Map,
}

impl Data {
    fn from_input(input: &str) -> Self {
        let chunks = input.split("\n\n").collect::<Vec<_>>();

        let seeds = chunks[0][7..]
            .split(' ')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let seed_to_soil = Map::from_input(chunks[1]);
        let soil_to_fertilizer = Map::from_input(chunks[2]);
        let fertilizer_to_water = Map::from_input(chunks[3]);
        let water_to_light = Map::from_input(chunks[4]);
        let light_to_temperature = Map::from_input(chunks[5]);
        let temperature_to_humidity = Map::from_input(chunks[6]);
        let humidity_to_location = Map::from_input(chunks[7]);

        Self {
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
}

#[inline]
#[must_use]
pub fn part1(input: &str) -> usize {
    let data = Data::from_input(input);

    data.seeds
        .iter()
        .map(|seed| {
            let soil = data.seed_to_soil.source_to_dest(*seed);
            let fertilizer = data.soil_to_fertilizer.source_to_dest(soil);
            let water = data.fertilizer_to_water.source_to_dest(fertilizer);
            let light = data.water_to_light.source_to_dest(water);
            let temperature = data.light_to_temperature.source_to_dest(light);
            let humidity = data.temperature_to_humidity.source_to_dest(temperature);
            data.humidity_to_location.source_to_dest(humidity)
        })
        .min()
        .unwrap_or_default()
}

#[inline]
#[must_use]
pub fn part2(input: &str) -> usize {
    let data = Data::from_input(input);

    let seeds = data
        .seeds
        .iter()
        .step_by(2)
        .zip(data.seeds.iter().skip(1).step_by(2))
        .map(|(s1, s2)| (*s1, *s2))
        .collect::<HashMap<_, _>>();

    seeds
        .iter()
        .map(|(s_start, s_range)| {
            let soil = data
                .seed_to_soil
                .sr_to_dr(&[(*s_start, *s_start + *s_range)]);
            let fert = data.soil_to_fertilizer.sr_to_dr(&soil);
            let water = data.fertilizer_to_water.sr_to_dr(&fert);
            let light = data.water_to_light.sr_to_dr(&water);
            let temp = data.light_to_temperature.sr_to_dr(&light);
            let humid = data.temperature_to_humidity.sr_to_dr(&temp);
            let loc = data.humidity_to_location.sr_to_dr(&humid);

            loc.iter().map(|(s, _)| *s).min().unwrap_or_default()
        })
        .min()
        .unwrap_or_default()
}
