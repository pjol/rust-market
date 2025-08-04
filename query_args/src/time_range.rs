use chrono::{DateTime, Days, Utc};

pub struct TimeRange {
    start: DateTime<Utc>,
    length: u64,
    end: DateTime<Utc>
}

impl TimeRange {

    // Initializes a new TimeRange
    //   start: The starting date of the query.
    //   length: The period in days that each iteration spans.
    //   end: The date at which to stop iteration (exclusive)
    pub fn new(start: DateTime<Utc>, length: u64, end: DateTime<Utc> ) -> Self {
        TimeRange{ start , length, end }
    }

    pub fn next(&mut self) -> Option<Self> {
        if self.end <= self.start {
          return None
        }
        match self
            .start
            .checked_add_days(Days::new(self.length))
        {
            Some(t) => {
              let nt = Some(TimeRange::new(self.start, self.length, self.end));
              self.start = t;
              nt
            }
            None => None
        }
    }

    pub fn fmt_start(&self) -> Option<String> {
        match self
            .start
            .to_rfc3339()
            .split("T")
            .next()
        {
            Some(s) => Some(String::from(s)),
            None => None
        }
    }

    pub fn fmt_end(&self) -> Option<String> {
        match self
            .start
            .checked_add_days(Days::new(self.length))
        {
          Some(mut t) => {
            if t > self.end {
              t = self.end
            }
            return match t
            .to_rfc3339()
            .split("T")
            .next()
            {
              Some(s) => Some(String::from(s)),
              None => None
            }
          },
          None => None
        }
    }
}