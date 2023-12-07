struct MapEntry {
    dest_start: usize,
    source_start: usize,
    range: usize,
}

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

    fn dest_to_source(&self, dest: usize) -> usize {
        for entry in &self.0 {
            if dest >= entry.dest_start && dest < entry.dest_start + entry.range {
                return entry.source_start + (dest - entry.dest_start);
            }
        }

        dest
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

    data.seeds
        .iter()
        .step_by(2)
        .zip(data.seeds.iter().skip(1).step_by(2))
        .map(|(s_start, s_range)| {
            (*s_start..(*s_start + s_range))
                .map(|seed| {
                    let soil = data.seed_to_soil.source_to_dest(seed);
                    let fertilizer = data.soil_to_fertilizer.source_to_dest(soil);
                    let water = data.fertilizer_to_water.source_to_dest(fertilizer);
                    let light = data.water_to_light.source_to_dest(water);
                    let temperature = data.light_to_temperature.source_to_dest(light);
                    let humidity = data.temperature_to_humidity.source_to_dest(temperature);
                    data.humidity_to_location.source_to_dest(humidity)
                })
                .min()
                .unwrap_or(usize::MAX)
        })
        .min()
        .unwrap_or_default()
}
