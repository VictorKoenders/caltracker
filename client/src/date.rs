use stdweb::unstable::TryInto;
use shared::Date;

pub trait DateUtils {
    fn today() -> Date;
}

impl DateUtils for Date {
    fn today() -> Date {
        let values: Vec<i32> = js! {
            var date = new Date();
            return [date.getFullYear(), date.getMonth() + 1, date.getDate()];
        }.try_into().unwrap();
        Date {
            year: values[0] as u16,
            month: values[1] as u8,
            day: values[2] as u8,
        }
    }
}

