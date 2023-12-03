use ndarray::Array2;
use std::{
    ptr::{self, NonNull},
    str::FromStr,
};

#[derive(Debug)]
struct SchematicNumber {
    number: u16,
    is_part: bool,
}

#[derive(Debug)]
struct SchematicPart {
    symbol: char,
    position: (usize, usize),
}

#[derive(Debug)]
struct Schematic {
    parts: Vec<SchematicPart>,
    numbers: Vec<Box<SchematicNumber>>,
    grid: Array2<Option<NonNull<SchematicNumber>>>,
}

impl Schematic {
    fn update_neighbours(&mut self) {
        for part in &self.parts {
            for x_offset in -1..=1 {
                for y_offset in -1..=1 {
                    if let Some(number_ptr) = &mut self.grid[(
                        (part.position.0 as isize + x_offset) as usize,
                        (part.position.1 as isize + y_offset) as usize,
                    )] {
                        // SAFETY: We have a &mut reference and therefore only we are accessing this data.
                        unsafe { number_ptr.as_mut() }.is_part = true;
                    }
                }
            }
        }
    }

    fn sum_parts(&self) -> u32 {
        self.numbers
            .iter()
            .filter_map(|num| {
                if num.is_part {
                    Some(num.number as u32)
                } else {
                    None
                }
            })
            .sum()
    }

    fn gear_ratios(&self) -> u32 {
        let mut sum = 0;

        'parts: for part in &self.parts {
            if part.symbol != '*' {
                continue;
            }

            let mut first: *const SchematicNumber = ptr::null();
            let mut second: *const SchematicNumber = ptr::null();
            for x_offset in -1..=1 {
                for y_offset in -1..=1 {
                    if let Some(number_ptr) = &self.grid[(
                        (part.position.0 as isize + x_offset) as usize,
                        (part.position.1 as isize + y_offset) as usize,
                    )] {
                        if !second.is_null()
                            && second as usize != number_ptr.as_ptr() as usize
                            && first as usize != number_ptr.as_ptr() as usize
                        {
                            continue 'parts;
                        }

                        if first.is_null() {
                            first = number_ptr.as_ptr() as *const SchematicNumber;
                        } else if second.is_null() && first as usize != number_ptr.as_ptr() as usize
                        {
                            second = number_ptr.as_ptr() as *const SchematicNumber;
                        }
                    }
                }
            }

            // SAFETY: We have a reference and therefore the data is not being written to.
            if let (Some(first), Some(second)) = unsafe { (first.as_ref(), second.as_ref()) } {
                sum += first.number as u32 * second.number as u32;
            }
        }

        sum
    }
}

impl FromStr for Schematic {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = vec![];
        let mut numbers = vec![];
        let mut grid: Array2<Option<NonNull<SchematicNumber>>> = Array2::default((
            s.lines().count(),
            s.lines()
                .next()
                .ok_or(eyre::eyre!("Empty input"))?
                .chars()
                .count(),
        ));

        let mut y = 0;
        for line in s.lines() {
            let mut line_iter = line.chars().peekable();
            let mut x = 0;

            while let Some(ch) = line_iter.next() {
                if ch == '.' {
                    x += 1;
                    continue;
                }

                if ch.is_numeric() {
                    let start = x;

                    let mut buf = String::from(ch);
                    while let Some(ch) = line_iter.peek() {
                        if ch.is_numeric() {
                            buf.push(*ch);
                            line_iter.next();
                            x += 1;
                        } else {
                            break;
                        }
                    }

                    let number = Box::into_raw(Box::new(SchematicNumber {
                        number: buf.parse()?,
                        is_part: false,
                    }));
                    let number_ptr = unsafe { NonNull::new_unchecked(number) };
                    numbers.push(unsafe { Box::from_raw(number) });

                    for offset in 0..buf.len() {
                        grid[(start + offset, y)] = Some(number_ptr);
                    }
                } else {
                    parts.push(SchematicPart {
                        symbol: ch,
                        position: (x, y),
                    });
                }

                x += 1;
            }

            y += 1;
        }

        Ok(Schematic {
            parts,
            numbers,
            grid,
        })
    }
}

pub fn first(input: &str) -> eyre::Result<String> {
    let mut schematic: Schematic = input.parse()?;
    schematic.update_neighbours();

    Ok(schematic.sum_parts().to_string())
}

pub fn second(input: &str) -> eyre::Result<String> {
    let schematic: Schematic = input.parse()?;
    Ok(schematic.gear_ratios().to_string())
}
