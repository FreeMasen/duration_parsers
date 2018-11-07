extern crate duration;
#[macro_use]
extern crate nom;
use nom::types::CompleteStr;
use duration::Duration;
pub fn parse(s: &str) -> Result<Duration, String> {
    let pair = duration(s.into()).map_err(|e| format!("{}", e))?;
    Ok(pair.1)
}

named!(duration<CompleteStr, Duration>,
    do_parse!(
        tag!("P") >>
        date_parts: opt!(many_m_n!(1, 4, date_part)) >>
        time_parts: opt!(time_parts) >>
        (combine_duration_parts(date_parts, time_parts))
    )
);

fn combine_duration_parts(date_parts: Option<Vec<DurationPart>>, time_parts: Option<Vec<DurationPart>>) -> Duration {
    let mut duration = Duration::new();
    for part in date_parts.unwrap_or(Vec::new()).iter().chain(time_parts.unwrap_or(Vec::new()).iter()) {
        match part {
            DurationPart::Years(value) => duration.set_years(*value),
            DurationPart::Months(value) => duration.set_months(*value),
            DurationPart::Weeks(value) => duration.set_weeks(*value),
            DurationPart::Days(value) => duration.set_days(*value),
            DurationPart::Hours(value) => duration.set_hours(*value),
            DurationPart::Minutes(value) => duration.set_minutes(*value),
            DurationPart::Seconds(value) => duration.set_seconds(*value),
        }
    }
    duration
}

named!(time_parts<CompleteStr, Vec<DurationPart>>,
    do_parse!(
        tag!("T") >>
        time_parts: many_m_n!(1, 3, time_part) >>
        (time_parts)
    )
);

named!(date_part<CompleteStr, DurationPart>,
    do_parse!(
        part: duration_part >>
        (combine_duration_part(part.0, part.1, false))
    )
);


named!(time_part<CompleteStr, DurationPart>,
    do_parse!(
        part: duration_part >>
        (combine_duration_part(part.0, part.1, true))
    )
);

fn combine_duration_part(value: f32, flag: CompleteStr, is_time: bool) -> DurationPart {
    match *flag {
        "Y" => DurationPart::Years(value),
        "M" => if is_time {
            DurationPart::Minutes(value)
        } else {
            DurationPart::Months(value)
        },
        "W" => DurationPart::Weeks(value),
        "D" => DurationPart::Days(value),
        "H" => DurationPart::Hours(value),
        "S" => DurationPart::Seconds(value),
        _ => unreachable!()
    }
}

named!(duration_part<CompleteStr, (f32, CompleteStr)>,
    do_parse!(
        value: float >>
        flag: take!(1) >>
        (value, flag)
    )
);


named!(float<CompleteStr, f32>,
    do_parse!(
        int: integer >>
        rem: opt!(remainder) >>
        (parse_float(int, rem))
    )
);

named!(remainder<CompleteStr, CompleteStr>,
    do_parse!(
        char!('.') >>
        rem: integer >>
        (rem)
    )
);

named!(integer<CompleteStr, CompleteStr>,
    take_while1!(digit)
);

fn digit(c: char) -> bool {
    c.is_digit(10)
}

fn parse_float(int: CompleteStr, rem: Option<CompleteStr>) -> f32 {
    let value = if let Some(rem) = rem {
        format!("{}.{}", int, rem)
    } else {
        int.to_string()
    };
    value.parse().unwrap()
}


enum DurationPart {
    Years(f32),
    Months(f32),
    Weeks(f32),
    Days(f32),
    Hours(f32),
    Minutes(f32),
    Seconds(f32),
}