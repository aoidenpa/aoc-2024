pub fn execute_day(day: u32, part: u32, test: bool) -> String {
    match day {
        1 => crate::day1::run(part, test),
        2 => crate::day2::run(part, test),
        3 => crate::day3::run(part, test),
        4 => crate::day4::run(part, test),
        5 => crate::day5::run(part, test),
        6 => crate::day6::run(part, test),
        7 => crate::day7::run(part, test),
        8 => crate::day8::run(part, test),
        9 => crate::day9::run(part, test),
        10 => crate::day10::run(part, test),
        11 => crate::day11::run(part, test),
        12 => crate::day12::run(part, test),
        13 => crate::day13::run(part, test),
        14 => crate::day14::run(part, test),
        15 => crate::day15::run(part, test),
        16 => crate::day16::run(part, test),
        17 => crate::day17::run(part, test),
        18 => crate::day18::run(part, test),
        19 => crate::day19::run(part, test),
        20 => crate::day20::run(part, test),
        21 => crate::day21::run(part, test),
        22 => crate::day22::run(part, test),
        23 => crate::day23::run(part, test),
        24 => crate::day24::run(part, test),
        25 => crate::day25::run(part, test),
        _ => panic!("Invalid day or part"),
    }
}