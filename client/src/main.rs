#![allow(dead_code)]

#[macro_use]
extern crate yew;
#[macro_use]
extern crate stdweb;
extern crate shared;

use yew::services::fetch::{FetchService, Method};
use yew::services::console::ConsoleService;
use shared::{Day, Model, Entry};
use yew::html::{App, Html};
use yew::format::Json;

struct Context {
    pub console: ConsoleService,
    pub fetch: FetchService<Msg>,
}

enum Msg {
    Load,
    Loaded(Result<Vec<Day>, ()>),
    SelectDay(usize),
    EditEntry(usize),
    Nop,
}

fn main() {
    yew::initialize();
    let mut app = App::new();
    let mut sender = app.sender();

    sender.send(Msg::Load);

    let context = Context {
        console: ConsoleService,
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
               render_day(&model.days[idx])
            } else { html! { <div /> } } }
        </div>
    }
}

fn render_day_tile((index, day): (usize, &Day)) -> Html<Msg> {
    html! {
        <li onclick=move|_| Msg::SelectDay(index), >{day.label()}</li>
    }
}

fn render_day(day: &Day) -> Html<Msg> {
    html! {
        <div>
            <b>
                {"Day "}
                {day.label()}
            </b>
            <dl>
                {for day.entries.iter().enumerate().map(render_day_entry)}
            </dl>
        </div>
    }
}

fn render_day_entry((index, entry): (usize, &Entry)) -> Html<Msg> {
    html! {
        <dt>{&entry.name}</dt>
        <dd ondoubleclick=move|_| Msg::EditEntry(index), >{&entry.value}</dd>
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
        Msg::SelectDay(index) => {
            model.current_day = Some(index);
        }
        Msg::EditEntry(index) => {
            js!{ 
                console.log("Editing entry", @{index as i32});
            }
        }
        Msg::Nop => {}
    };
}

