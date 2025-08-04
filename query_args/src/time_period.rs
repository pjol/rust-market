pub enum TimePeriod {
    Daily,
    Hourly
}

impl TimePeriod {
    pub fn from_str(s: &str) -> Self {
      match s {
        "daily" => TimePeriod::Daily,
        "hourly" => TimePeriod::Hourly,
        _ => TimePeriod::Hourly
      }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            TimePeriod::Daily => "daily",
            TimePeriod::Hourly => "hourly"
        }
    }
}