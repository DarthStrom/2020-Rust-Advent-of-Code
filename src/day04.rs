use regex::Regex;
use std::collections::HashMap;

pub fn solve1(lines: &Vec<&str>) -> usize {
    solve(lines, &all_fields_present)
}

pub fn solve2(lines: &Vec<&str>) -> usize {
    solve(lines, &all_fields_valid)
}

fn solve(lines: &Vec<&str>, verification: &dyn Fn(&HashMap<&str, &str>) -> bool) -> usize {
    let mut count = 0;
    let mut fields = HashMap::new();

    for line in lines {
        if !line.is_empty() {
            let items = line.split(' ');
            for item in items {
                let mut parts = item.split(':');
                let key = parts.next().unwrap();
                let val = parts.next().unwrap();
                fields.insert(key, val);
            }
        } else {
            if verification(&fields) {
                count += 1;
            }
            fields.clear();
        }
    }

    if verification(&fields) {
        count += 1;
    }

    count
}

fn all_fields_present(fields: &HashMap<&str, &str>) -> bool {
    fields.contains_key("byr")
        && fields.contains_key("iyr")
        && fields.contains_key("eyr")
        && fields.contains_key("hgt")
        && fields.contains_key("hcl")
        && fields.contains_key("ecl")
        && fields.contains_key("pid")
}

fn all_fields_valid(fields: &HashMap<&str, &str>) -> bool {
    if all_fields_present(fields) {
        let byr = fields["byr"];
        let iyr = fields["iyr"];
        let eyr = fields["eyr"];
        let hgt = fields["hgt"];
        let hcl = fields["hcl"];
        let ecl = fields["ecl"];
        let pid = fields["pid"];

        byr_valid(byr)
            && iyr_valid(iyr)
            && eyr_valid(eyr)
            && hgt_valid(hgt)
            && hcl_valid(hcl)
            && ecl_valid(ecl)
            && pid_valid(pid)
    } else {
        false
    }
}

fn byr_valid(byr: &str) -> bool {
    if let Ok(year) = byr.parse::<u32>() {
        year >= 1920 && year <= 2002
    } else {
        false
    }
}

fn iyr_valid(iyr: &str) -> bool {
    if let Ok(year) = iyr.parse::<u32>() {
        year >= 2010 && year <= 2020
    } else {
        false
    }
}

fn eyr_valid(eyr: &str) -> bool {
    if let Ok(year) = eyr.parse::<u32>() {
        year >= 2020 && year <= 2030
    } else {
        false
    }
}

fn hgt_valid(hgt: &str) -> bool {
    if let Ok(height) = hgt[..hgt.len() - 2].parse::<u32>() {
        if hgt.ends_with("in") {
            height >= 59 && height <= 76
        } else if hgt.ends_with("cm") {
            height >= 150 && height <= 193
        } else {
            false
        }
    } else {
        false
    }
}

fn hcl_valid(hcl: &str) -> bool {
    let re = Regex::new(r"#[a-f0-9]{6}").unwrap();
    re.is_match(hcl)
}

fn ecl_valid(ecl: &str) -> bool {
    match ecl {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

fn pid_valid(pid: &str) -> bool {
    pid.len() == 9 && pid.parse::<u32>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in

ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm
";

    #[test]
    fn example1() {
        let lines = INPUT.lines().collect::<Vec<_>>();

        assert_eq!(solve1(&lines), 3);
    }

    #[test]
    fn invalid_passports() {
        let input = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

        let lines = input.lines().collect::<Vec<_>>();

        assert_eq!(solve2(&lines), 0);
    }

    #[test]
    fn valid_passports() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let lines = input.lines().collect::<Vec<_>>();

        assert_eq!(solve2(&lines), 4);
    }
}
