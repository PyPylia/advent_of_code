use crate::collect_to_array;
use std::{mem, ops::Range, str::FromStr};

struct MapItem {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl FromStr for MapItem {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [destination_str, source_str, length_str] =
            collect_to_array(s.split(" ")).ok_or_else(|| eyre::eyre!("Invalid map item"))?;

        Ok(Self {
            destination_start: lexical_core::parse(destination_str.as_bytes())?,
            source_start: lexical_core::parse(source_str.as_bytes())?,
            length: lexical_core::parse(length_str.as_bytes())?,
        })
    }
}

struct Map {
    items: heapless::Vec<MapItem, 48>,
}

impl Map {
    fn map_type(&self, item_type: u64) -> u64 {
        for item in &self.items {
            if item_type >= item.source_start && item_type < item.source_start + item.length {
                return item_type + item.destination_start - item.source_start;
            }
        }

        item_type
    }

    fn map_range(&self, type_range: Range<u64>) -> Vec<Range<u64>> {
        let mut new_unmapped = Vec::with_capacity(2);
        let mut unmapped = vec![type_range];
        let mut mapped = vec![];

        for item in &self.items {
            while let Some(range) = unmapped.pop() {
                let source_end = item.source_start + item.length;
                if range.end <= item.source_start || range.start >= source_end {
                    new_unmapped.push(range);
                    continue;
                }

                let destination_end = item.destination_start + item.length;
                let mapped_start = range.start + item.destination_start - item.source_start;
                let mapped_end = range.end + item.destination_start - item.source_start;

                match (range.start >= item.source_start, range.end > source_end) {
                    (false, false) => {
                        mapped.push(item.destination_start..mapped_end);
                        new_unmapped.push(range.start..item.source_start);
                    }
                    (false, true) => {
                        mapped.push(item.destination_start..destination_end);
                        new_unmapped.push(range.start..item.source_start);
                        new_unmapped.push(source_end..range.end);
                    }
                    (true, false) => {
                        mapped.push(mapped_start..mapped_end);
                    }
                    (true, true) => {
                        mapped.push(mapped_start..destination_end);
                        new_unmapped.push(source_end..range.end);
                    }
                }
            }

            mem::swap(&mut unmapped, &mut new_unmapped);
        }

        unmapped.append(&mut mapped);
        unmapped
    }
}

impl FromStr for Map {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let _header_info = lines.next().ok_or_else(|| eyre::eyre!("Invalid map"))?;

        let mut items = heapless::Vec::new();
        while let Some(line) = lines.next() {
            items.push(line.trim_end().parse()?).ok();
        }

        Ok(Self { items })
    }
}

fn get_sections(input: &str) -> eyre::Result<(impl Iterator<Item = &str>, heapless::Vec<Map, 7>)> {
    let sections: heapless::Vec<&str, 8> = input.split("\r\n\r\n").collect();
    let (seeds, maps) = sections
        .split_first()
        .ok_or_else(|| eyre::eyre!("No seed header"))?;

    let maps: eyre::Result<heapless::Vec<Map, 7>> = maps.into_iter().map(|s| s.parse()).collect();
    Ok((
        seeds
            .strip_prefix("seeds: ")
            .ok_or_else(|| eyre::eyre!("Invalid seed header"))?
            .trim_end()
            .split(" "),
        maps?,
    ))
}

pub fn first(input: &str) -> eyre::Result<u64> {
    let (seeds, maps) = get_sections(input)?;
    let seeds: Result<heapless::Vec<u64, 20>, lexical_core::Error> =
        seeds.map(|s| lexical_core::parse(s.as_bytes())).collect();
    let mut seeds = seeds?;

    for map in maps {
        for seed in &mut seeds {
            *seed = map.map_type(*seed)
        }
    }

    seeds
        .into_iter()
        .min()
        .ok_or_else(|| eyre::eyre!("No minimum seed"))
}

pub fn second(input: &str) -> eyre::Result<u64> {
    let (seeds_iter, maps) = get_sections(input)?;
    let mut seeds_iter = seeds_iter.array_chunks();

    let mut first = vec![];
    while let Some([start_str, length_str]) = seeds_iter.next() {
        let range_start: u64 = lexical_core::parse(start_str.as_bytes())?;
        let length: u64 = lexical_core::parse(length_str.as_bytes())?;
        first.push(range_start..range_start + length);
    }

    let mut second = Vec::with_capacity(first.len() * 2);
    for map in maps {
        while let Some(range) = first.pop() {
            second.append(&mut map.map_range(range))
        }

        mem::swap(&mut first, &mut second);
    }

    first
        .into_iter()
        .map(|range| range.start)
        .min()
        .ok_or_else(|| eyre::eyre!("No minimum seed"))
}
