pub enum Interval {
  Hourly,
  Daily,
  Weekly,
  Monthly,
  Yearly
}

impl Interval {
  pub fn default() -> Self {
    Interval::Monthly
  }

  pub fn from_str(s: &str) -> Option<Self> {
    match s {
      "hourly" => Some(Interval::Hourly),
      "daily" => Some(Interval::Daily),
      "weekly" => Some(Interval::Weekly),
      "monthly" => Some(Interval::Monthly),
      "yearly" => Some(Interval::Yearly),
      _ => None
    }
  }

  pub fn to_str(&self) -> &'static str {
    match self {
      Interval::Hourly => "hourly",
      Interval::Daily => "daily",
      Interval::Weekly => "weekly",
      Interval::Monthly => "monthly",
      Interval::Yearly => "yearly",
    }
  }
}