use stdweb::unstable::TryInto;

#[derive(Default)]
pub struct Model {
    pub days: Vec<Day>,
    pub current_day: Option<usize>,
}

#[derive(Deserialize)]
pub struct ModelFromServer {
    pub days: Vec<Day>,
}

#[derive(Deserialize)]
pub struct Day {
    pub date: Date,
    pub entries: Vec<Entry>,
}

#[derive(Deserialize)]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

impl Date {
    pub fn today() -> Date {
        let date: Vec<u32> = js! {
            var dt = new Date();
            var result = [dt.getFullYear(), dt.getMonth() + 1, dt.getDate()];
            return result;
        }.try_into()
            .unwrap();
        Date {
            day: date[2] as u8,
            month: date[1] as u8,
            year: date[0] as u16,
        }
    }
}

impl Day {
    pub fn today() -> Day {
        Day {
            entries: Vec::new(),
            date: Date::today(),
        }
    }

    pub fn label(&self) -> String {
        format!(
            "{}/{}: {} cals",
            self.date.month,
            self.date.day,
            self.total()
        )
    }

    pub fn total(&self) -> f32 {
        self.entries.iter().map(|e| e.value).sum()
    }
}

#[derive(Deserialize)]
pub struct Entry {
    pub name: String,
    pub value: f32,
}
