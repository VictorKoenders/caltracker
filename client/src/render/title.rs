use super::Renderable;
use yew::html::Html;
use {Context, Msg, Model};
use shared::Day;

pub struct Title<R>(pub R);

impl<'a> Renderable for Title<&'a Day>
{
    fn render(&self, _: &Model, _: usize) -> Html<Context, Model> {
        let date = self.0.date.clone();
        html! {
            <li onclick=move|_| Msg::SelectDay(date.clone()), >{self.0.label()}</li>
        }
    }
}

