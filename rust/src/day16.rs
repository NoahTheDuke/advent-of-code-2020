use std::collections::{HashMap};

// --- Day 16: Ticket Translation ---

// departure location: 27-840 or 860-957
// departure station: 28-176 or 183-949
// departure platform: 44-270 or 277-967
// departure track: 33-197 or 203-957
// departure date: 47-660 or 677-955
// departure time: 45-744 or 758-971
// arrival location: 42-636 or 642-962
// arrival station: 44-243 or 252-962
// arrival platform: 46-428 or 449-949
// arrival track: 25-862 or 876-951
// class: 26-579 or 585-963
// duration: 38-683 or 701-949
// price: 41-453 or 460-970
// route: 48-279 or 292-963
// row: 33-617 or 637-955
// seat: 39-328 or 351-970
// train: 35-251 or 264-957
// type: 25-380 or 389-951
// wagon: 42-461 or 480-965
// zone: 33-768 or 789-954
//
// your ticket:
// 83,53,73,139,127,131,97,113,61,101,107,67,79,137,89,109,103,59,149,71
//
// nearby tickets:
// 541,797,657,536,243,821,805,607,97,491,714,170,714,533,363,491,896,399,710,865

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rule {
    left: usize,
    right: usize,
}

impl Rule {
    fn check_range(self, number: usize) -> bool {
        self.left <= number && number <= self.right
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Field<'a> {
    name: &'a str,
    low: Rule,
    high: Rule,
}

impl Field<'_> {
    fn check_rules(self, number: usize) -> bool {
        self.low.check_range(number) || self.high.check_range(number)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TicketField<'a> {
    r#departure_location: Field<'a>,
    r#departure_station: Field<'a>,
    r#departure_platform: Field<'a>,
    r#departure_track: Field<'a>,
    r#departure_date: Field<'a>,
    r#departure_time: Field<'a>,
    r#arrival_location: Field<'a>,
    r#arrival_station: Field<'a>,
    r#arrival_platform: Field<'a>,
    r#arrival_track: Field<'a>,
    r#class: Field<'a>,
    r#duration: Field<'a>,
    r#price: Field<'a>,
    r#route: Field<'a>,
    r#row: Field<'a>,
    r#seat: Field<'a>,
    r#train: Field<'a>,
    r#type: Field<'a>,
    r#wagon: Field<'a>,
    r#zone: Field<'a>,
}

impl TicketField<'_> {
    fn new(fields: &str) -> TicketField {
        // let mut map = HashMap::new();
        let mut v = Vec::new();

        for field_line in fields.lines() {
            let mut field_split = field_line.split(": ");
            let name = field_split.next().unwrap();
            let mut rules = field_split.next().unwrap().split(" or ");
            let mut low_rules = rules.next().unwrap().split("-");
            let low_left: usize = low_rules.next().unwrap().parse().unwrap();
            let low_right: usize = low_rules.next().unwrap().parse().unwrap();
            let low_rule = Rule {
                left: low_left,
                right: low_right,
            };

            let mut high_rules = rules.next().unwrap().split("-");
            let high_left: usize = high_rules.next().unwrap().parse().unwrap();
            let high_right: usize = high_rules.next().unwrap().parse().unwrap();
            let high_rule = Rule {
                left: high_left,
                right: high_right,
            };

            let new_field = Field {
                name: name,
                low: low_rule,
                high: high_rule,
            };
            v.push(new_field);
            // map.insert(name, new_field);
        }

        let mut v_iter = v.iter();

        TicketField {
            r#departure_location: *v_iter.next().unwrap(),
            r#departure_station: *v_iter.next().unwrap(),
            r#departure_platform: *v_iter.next().unwrap(),
            r#departure_track: *v_iter.next().unwrap(),
            r#departure_date: *v_iter.next().unwrap(),
            r#departure_time: *v_iter.next().unwrap(),
            r#arrival_location: *v_iter.next().unwrap(),
            r#arrival_station: *v_iter.next().unwrap(),
            r#arrival_platform: *v_iter.next().unwrap(),
            r#arrival_track: *v_iter.next().unwrap(),
            r#class: *v_iter.next().unwrap(),
            r#duration: *v_iter.next().unwrap(),
            r#price: *v_iter.next().unwrap(),
            r#route: *v_iter.next().unwrap(),
            r#row: *v_iter.next().unwrap(),
            r#seat: *v_iter.next().unwrap(),
            r#train: *v_iter.next().unwrap(),
            r#type: *v_iter.next().unwrap(),
            r#wagon: *v_iter.next().unwrap(),
            r#zone: *v_iter.next().unwrap(),
        }
    }

    fn bad_number(self, number: usize) -> Option<usize> {
        if self.departure_location.check_rules(number)
            || self.departure_station.check_rules(number)
            || self.departure_platform.check_rules(number)
            || self.departure_track.check_rules(number)
            || self.departure_date.check_rules(number)
            || self.departure_time.check_rules(number)
            || self.arrival_location.check_rules(number)
            || self.arrival_station.check_rules(number)
            || self.arrival_platform.check_rules(number)
            || self.arrival_track.check_rules(number)
            || self.class.check_rules(number)
            || self.duration.check_rules(number)
            || self.price.check_rules(number)
            || self.route.check_rules(number)
            || self.row.check_rules(number)
            || self.seat.check_rules(number)
            || self.train.check_rules(number)
            || self.r#type.check_rules(number)
            || self.wagon.check_rules(number)
            || self.zone.check_rules(number)
        {
            None
        } else {
            Some(number)
        }
    }
}

// 19070
pub fn part1(input: String) -> usize {
    let chunks = input.split("\n\n").collect::<Vec<_>>();
    let fields = chunks[0];
    let rest = chunks[2];
    let ticket_fields = TicketField::new(fields);

    rest.lines()
        .flat_map(|l| l.split(",").filter_map(|n| n.parse::<usize>().ok()))
        .filter_map(|n| ticket_fields.bad_number(n))
        .sum()
}

// goal number: 161926544831
pub fn part2(input: String) -> usize {
    let chunks = input.split("\n\n").collect::<Vec<_>>();
    let fields = chunks[0];
    let mut rest = chunks[1].to_owned();
    rest.push_str("\n");
    rest.push_str(chunks[2]);
    let ticket_fields = TicketField::new(fields);

    let lines = rest
        .lines()
        .map(|l| {
            l.split(",")
                .filter_map(|n| n.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .filter(|l| {
            !l.is_empty()
                && l.iter()
                    .filter_map(|n| ticket_fields.bad_number(*n))
                    .count()
                    == 0
        })
        .collect::<Vec<_>>();

    let fns: Vec<Field> = vec![
        ticket_fields.departure_location,
        ticket_fields.departure_station,
        ticket_fields.departure_platform,
        ticket_fields.departure_track,
        ticket_fields.departure_date,
        ticket_fields.departure_time,
        ticket_fields.arrival_location,
        ticket_fields.arrival_station,
        ticket_fields.arrival_platform,
        ticket_fields.arrival_track,
        ticket_fields.class,
        ticket_fields.duration,
        ticket_fields.price,
        ticket_fields.route,
        ticket_fields.row,
        ticket_fields.seat,
        ticket_fields.train,
        ticket_fields.r#type,
        ticket_fields.wagon,
        ticket_fields.zone,
    ];

    let mut pairs = HashMap::new();

    for idx in 0..20 {
        let mut found: Option<&str> = None;
        'fields: for field in &fns {
            for line in &lines {
                let col = line[idx];
                if !field.check_rules(col) {
                    continue 'fields;
                }
            }
            // println!("{:?}, {:?}", idx, field.name);
            found = Some(field.name);
            break 'fields;
        }
        if let Some(f) = found {
            pairs.insert(f, idx);
        }
    }

    // println!("{:?}", pairs.keys());

    // let locs = vec![
    //     pairs.get("departure location").unwrap(),
    //     pairs.get("departure station").unwrap(),
    //     pairs.get("departure platform").unwrap(),
    //     pairs.get("departure track").unwrap(),
    //     pairs.get("departure date").unwrap(),
    //     pairs.get("departure time").unwrap()
    // ];

    // println!("{:?}", locs.iter().fold(1, |acc, idx| acc * lines[0][**idx]));

    0
}
