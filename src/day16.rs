use std::collections::HashMap;

pub fn solve1(lines: &Vec<&str>) -> usize {
    interpret_notes(&lines).1
}

pub fn solve2(lines: &Vec<&str>) -> usize {
    let ticket = your_ticket(&lines);
    let result = ticket
        .iter()
        .filter(|(k, _)| k.starts_with("departure"))
        .map(|(_, v)| v)
        .take(6)
        .product();
    result
}

fn interpret_notes<'a>(lines: &Vec<&'a str>) -> (Vec<&'a str>, usize, Vec<usize>) {
    let mut ranges = HashMap::new();
    lines
        .iter()
        .take_while(|&&l| !l.is_empty())
        .for_each(|line| {
            let mut parts = line.split(':');
            let key = parts.next().unwrap();
            let valid_nums = parts
                .next()
                .unwrap()
                .split(" or ")
                .flat_map(|range_str| {
                    let nums = range_str.trim().split("-").collect::<Vec<_>>();
                    let min = nums[0].parse::<usize>().unwrap();
                    let max = nums[1].parse::<usize>().unwrap();
                    (min..=max).collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            ranges.insert(key, valid_nums);
        });

    let your_vals = lines
        .iter()
        .skip_while(|&&line| line != "your ticket:")
        .skip(1)
        .take(1)
        .flat_map(|s| {
            s.split(',')
                .flat_map(str::parse::<usize>)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut error_rate = 0;
    let mut valid_tickets = vec![your_vals.clone()];
    lines
        .iter()
        .skip_while(|&&line| line != "nearby tickets:")
        .skip(1)
        .for_each(|line| {
            let mut valid = true;
            let vals = line
                .split(',')
                .flat_map(str::parse::<usize>)
                .collect::<Vec<_>>();
            vals.iter().for_each(|n| {
                if ranges.values().all(|v| !v.contains(&n)) {
                    error_rate += n;
                    valid = false;
                }
            });
            if valid {
                valid_tickets.push(vals);
            }
        });

    let number_of_positions = your_vals.len();
    let mut number_of_fixed_positions = 0;
    let mut fixed_fields = HashMap::new();

    let mut fixed_positions = vec![false; number_of_positions];

    while number_of_fixed_positions < number_of_positions {
        for pos in 0..number_of_positions {
            if fixed_positions[pos] {
                continue;
            }

            let mut last_matching_field = "";
            let mut number_of_matching_fields = 0;

            'rule: for (&field, rule) in ranges.iter() {
                if fixed_fields.contains_key(field) {
                    continue;
                }

                for ticket in valid_tickets.iter() {
                    let field_value = ticket[pos];
                    if !rule.contains(&field_value) {
                        continue 'rule;
                    }
                }

                last_matching_field = field;
                number_of_matching_fields += 1;
            }

            if number_of_matching_fields == 1 {
                fixed_fields.insert(last_matching_field, pos);
                fixed_positions[pos] = true;
                number_of_fixed_positions += 1;
            }
        }
    }

    let mut fields_indexes = fixed_fields.iter().collect::<Vec<(&&str, &usize)>>();
    fields_indexes.sort_by(|(_, i), (_, j)| i.cmp(j));
    let fields = fields_indexes.iter().map(|(&f, _)| f).collect::<Vec<_>>();

    (fields, error_rate, your_vals)
}

fn your_ticket<'a>(lines: &Vec<&'a str>) -> HashMap<&'a str, usize> {
    let (fields, _, your_vals) = interpret_notes(lines);
    fields
        .iter()
        .zip(your_vals)
        .map(|(&field, val)| (field, val))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let lines = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
            .lines()
            .collect::<Vec<_>>();

        assert_eq!(solve1(&lines), 71);
    }

    #[test]
    fn example2() {
        let lines = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"
            .lines()
            .collect::<Vec<_>>();
        let mut expected_ticket = HashMap::new();
        expected_ticket.insert("class", 12);
        expected_ticket.insert("row", 11);
        expected_ticket.insert("seat", 13);

        assert_eq!(your_ticket(&lines), expected_ticket);
    }
}
