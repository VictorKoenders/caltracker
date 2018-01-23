#![allow(dead_code)]
#![recursion_limit="128"]

#[macro_use]
extern crate yew;
#[macro_use]
extern crate stdweb;
extern crate shared;
extern crate serde_json;

use yew::services::fetch::{FetchService, Method};
use yew::services::console::ConsoleService;
use yew::html::{App, Html, InputData};
use shared::{Day, Model, Entry};
use yew::format::Json;

struct Context {
    pub console: ConsoleService,
    pub fetch: FetchService<Msg>,
    pub sender: yew::html::AppSender<Msg>,
}

enum Msg {
    Load,
    Loaded(Result<Vec<Day>, ()>),
    LoadedEntry {
        day_index: usize,
        entry_index: usize,
        result: Entry,
    },
    SelectDay(usize),
    EditEntry(usize),
    UpdateEntryName(String),
    UpdateEntryValue(String),
    SaveEntry,
    NewEntry,
    Nop,
}

fn main() {
    yew::initialize();
    let mut app = App::new();
    let mut sender = app.sender();

    sender.send(Msg::Load);

    let context = Context {
        console: ConsoleService,
        sender: sender.clone(),
        fetch: FetchService::new(sender),
    };
    let model = Model::default();

    app.mount(context, model, update, view);
    yew::run_loop();
}

fn view(model: &Model) -> Html<Msg> {
    html! {
        <div>
            <ul class="test", >
                {for model.days.iter().enumerate().map(render_day_tile)}
            </ul>
            { if let Some(idx) = model.current_day {
               render_day(&model, &model.days[idx])
            } else { html! { <div /> } } }
        </div>
    }
}

fn render_day_tile((index, day): (usize, &Day)) -> Html<Msg> {
    html! {
        <li onclick=move|_| Msg::SelectDay(index), >{day.label()}</li>
    }
}

fn render_day(model: &Model, day: &Day) -> Html<Msg> {
    html! {
        <div>
            <b>
                {"Day "}
                {day.label()}
            </b>
            <dl>
                {for day.entries.iter().enumerate().map(|e| render_day_entry(model, e))}
                <dt><a onclick=|_| Msg::NewEntry, >{"New"}</a></dt>
                <dd></dd>
            </dl>
        </div>
    }
}

fn render_day_entry(model: &Model, (index, entry): (usize, &Entry)) -> Html<Msg> {
    match model.current_entry {
        Some(i) if i == index => html! {
            <dt>
                <input type="text",
                       value={&entry.name},
                       oninput=|e: InputData| Msg::UpdateEntryName(e.value),
                />
            </dt>
            <dd>
                <input type="text",
                       value={&entry.value},
                       oninput=|e: InputData| Msg::UpdateEntryValue(e.value),
                />
            </dd>
            <input type="button",
                   value={"Save"},
                   onclick=|_| Msg::SaveEntry, />
        },
        _ => html! {
            <dt ondoubleclick=move|_| Msg::EditEntry(index), >{&entry.name}</dt>
            <dd ondoubleclick=move|_| Msg::EditEntry(index), >{&entry.value}</dd>
        }
    }
}

fn save_model(context: &mut Context, model: &Model) {
    let day_index = model.current_day.unwrap();
    let entry_index = model.current_entry.unwrap();
    let day = &model.days[day_index];
    let entry = &day.entries[entry_index];

    // TODO: Once fetch POST is landed, use that instead of this monstrocity
    //  https://github.com/DenisKolodin/yew/pull/95

    let json: yew::format::Storable = Json(&entry).into();
    let mut sender = context.sender.clone();
    
    let callback = move |response: String| {
        let entry: Entry = serde_json::from_str(&response).unwrap();
        sender.send(Msg::LoadedEntry { day_index, entry_index, result: entry });
    };
    js! {
        var cb = @{callback};
        fetch(@{&format!("/api/entry/{}/{}/{}", day.date.year, day.date.month, day.date.day)}, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: @{json}
        }).then(function(r) {
            return r.text();
        }).then(function(r){
            cb(r);
            cb.drop();
        });
    }
}


fn update(context: &mut Context, model: &mut Model, msg: Msg) {
    match msg {
        Msg::Load => {
            context.fetch.fetch(Method::Get, "/api/list", None, |r| {
                let Json(r) = r;
                Msg::Loaded(r)
            });
        }
        Msg::Loaded(days) => {
            match days {
                Ok(d) => model.days = d,
                Err(_) => {
                    js! {
                        console.error("Could not load data from server");
                    };
                }
            }
        }
        Msg::LoadedEntry {  day_index,  entry_index,  result } => {
            let day = &mut model.days[day_index];
            day.entries[entry_index] = result;
        }
        Msg::SelectDay(index) => {
            model.current_day = Some(index);
        }
        Msg::EditEntry(index) => {
            if let Some(_) = model.current_entry {
                save_model(context, model);
            }
            model.current_entry = Some(index);
        }
        Msg::NewEntry => {
            let day = &mut model.days[model.current_day.unwrap()];
            model.current_entry = Some(day.entries.len());
            day.entries.push(Default::default());
        }
        Msg::UpdateEntryName(name) => {
            let day = &mut model.days[model.current_day.unwrap()];
            let entry = &mut day.entries[model.current_entry.unwrap()];
            entry.name = name;
        }
        Msg::UpdateEntryValue(value) => {
            let day = &mut model.days[model.current_day.unwrap()];
            let entry = &mut day.entries[model.current_entry.unwrap()];
            entry.value = value.parse().unwrap_or(0f32);
        }
        Msg::SaveEntry => {
            save_model(context, model);
            model.current_entry = None;
        }
        Msg::Nop => {}
    };
}

