#![recursion_limit = "128"]

extern crate serde_json;
extern crate shared;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;

mod msg;
mod render;

use yew::services::fetch::{FetchService, Method};
use yew::services::console::ConsoleService;
use yew::html::{App, Html};
use shared::{Entry, Model};
use render::title::Title;
use render::Renderable;
use yew::format::Json;
use msg::Msg;

struct Context {
    pub console: ConsoleService,
    pub fetch: FetchService<Msg>,
    pub sender: yew::html::AppSender<Msg>,
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
                {for model.days.iter().enumerate().map(|(i, day)| Title(day).render(&model, i))}
            </ul>
            { if let Some(idx) = model.current_day {
                model.days[idx].render(&model, idx)
            } else { html! { <div /> } } }
        </div>
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
        sender.send(Msg::LoadedEntry {
            day_index,
            entry_index,
            result: entry,
        });
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
        Msg::Loaded(days) => match days {
            Ok(d) => model.days = d,
            Err(_) => {
                js! {
                    console.error("Could not load data from server");
                };
            }
        },
        Msg::LoadedEntry {
            day_index,
            entry_index,
            result,
        } => {
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
