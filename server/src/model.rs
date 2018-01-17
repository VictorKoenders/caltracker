use uuid::Uuid;
use shared;

#[derive(Queryable, Debug)]
pub struct DayEntryJoinedWithDay {
    pub entry: (Uuid, Uuid, String, f64),
    pub day: (Uuid, i16, i16, i16),
}

impl DayEntryJoinedWithDay {
    pub fn matches(&self, day: &shared::Day) -> bool {
        day.date.year == self.day.1 as u16
            && day.date.month == self.day.2 as u8
            && day.date.day == self.day.3 as u8
    }
}

/*
impl Into<shared::Day> for Day {
    fn into(self) -> shared::Day {
        shared::Day {
            date: shared::Date {
                year: self.year as u16,
                month: self.month as u8,
                day: self.day as u8,
            },
            entries: Vec::new()
        }
    }
}
*/

