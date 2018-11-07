#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Duration {
    years: Option<f32>,
    months: Option<f32>,
    weeks: Option<f32>,
    days: Option<f32>,
    hours: Option<f32>,
    minutes: Option<f32>,
    seconds: Option<f32>,
}

impl Duration {
    pub fn new() -> Self {
        Duration {
            years: None,
            months: None,
            weeks: None,
            days: None,
            hours: None,
            minutes: None,
            seconds: None,
        }
    }
    pub fn years(&mut self, years: f32) -> &mut Self {
        self.set_years(years);
        self
    }
    pub fn set_years(&mut self, years: f32) {
        self.years = Some(years);
    }
    pub fn months(&mut self, months: f32) -> &mut Self {
        self.set_months(months);
        self
    }
    pub fn set_months(&mut self, months: f32) {
        self.months = Some(months);
    }
    pub fn weeks(&mut self, weeks: f32) -> &mut Self {
        self.set_weeks(weeks);
        self
    }
    pub fn set_weeks(&mut self, weeks: f32) {
        self.weeks = Some(weeks);
    }
    pub fn days(&mut self, days: f32) -> &mut Self {
        self.set_days(days);
        self
    }
    pub fn set_days(&mut self, days: f32) {
        self.days = Some(days);
    }
    pub fn hours(&mut self, hours: f32) -> &mut Self {
        self.set_hours(hours);
        self
    }
    pub fn set_hours(&mut self, hours: f32) {
        self.hours = Some(hours);
    }
    pub fn minutes(&mut self, minutes: f32) -> &mut Self {
        self.set_minutes(minutes);
        self
    }
    pub fn set_minutes(&mut self, minutes: f32) {
        self.minutes = Some(minutes);
    }
    pub fn seconds(&mut self, seconds: f32) -> &mut Self {
        self.set_seconds(seconds);
        self
    }
    pub fn set_seconds(&mut self, seconds: f32) {
        self.seconds = Some(seconds);
    }
    pub fn build(&mut self) -> Self {
        *self
    }
    pub fn is_empty(&self) -> bool {
        self.years.is_none()
        && self.months.is_none()
        && self.weeks.is_none()
        && self.days.is_none()
        && self.empty_time()
    }

    pub fn empty_time(&self) -> bool {
        self.hours.is_none()
        && self.minutes.is_none()
        && self.seconds.is_none()
    }
}

impl ::std::fmt::Display for Duration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut ret = String::from("P");
        if let Some(y) = self.years {
            ret.push_str(&format!("{}Y", y));
        }
        if let Some(m) = self.months {
            ret.push_str(&format!("{}M", m));
        }
        if let Some(w) = self.weeks {
            ret.push_str(&format!("{}W", w));
        }
        if let Some(d) = self.days {
            ret.push_str(&format!("{}D", d))
        }
        if self.empty_time() {
            return write!(f, "{}", ret);
        }
        ret.push('T');
        if let Some(h) = self.hours {
            ret.push_str(&format!("{}H", h));
        }
        if let Some(m) = self.minutes {
            ret.push_str(&format!("{}M", m));
        }
        if let Some(s) = self.seconds {
            ret.push_str(&format!("{}S", s));
        }
        write!(f, "{}", ret)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn builder_test() {
        let _d = Duration::new()
                        .years(1.0)
                        .months(1.0)
                        .weeks(1.0)
                        .days(1.0)
                        .hours(1.0)
                        .minutes(1.0)
                        .seconds(1.0)
                        .build();

    }
    #[test]
    fn display_test() {
        let one = Duration::new()
            .years(1.0)
            .hours(3.0)
            .minutes(1.0)
            .seconds(3.5)
            .build();
        assert_eq!(String::from("P1YT3H1M3.5S"), format!("{}", one));
        let two = Duration::new()
            .years(10.0)
            .build();
        assert_eq!(String::from("P10Y"), format!("{}", two));
        let three = Duration::new()
            .minutes(100.558)
            .build();
        assert_eq!(String::from("PT100.558M"), format!("{}", three));
    }
}