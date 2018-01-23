use super::schema::dayentry;
use super::schema::day;
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

#[derive(Insertable, Queryable, Debug)]
#[table_name = "day"]
pub struct Day {
    pub id: Uuid,
    pub year: i16,
    pub month: i16,
    pub day_of_month: i16
}

impl From<Day> for shared::Day {
    fn from(day: Day) -> shared::Day {
        shared::Day {
            date: shared::Date {
                year: day.year as u16,
                month: day.month as u8,
                day: day.day_of_month as u8,
            },
            entries: Vec::new()
        }
    }
}


#[derive(Insertable, Queryable, Debug)]
#[table_name = "dayentry"]
pub struct DayEntry {
    pub id: Uuid,
    pub day: Uuid,
    pub name: String,
    pub value: f64
}

impl From<DayEntry> for shared::Entry {
    fn from(entry: DayEntry) -> shared::Entry {
        shared::Entry {
            id: entry.id.to_string(),
            name: entry.name,
            value: entry.value as f32
        }
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

