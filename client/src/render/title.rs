use super::Renderable;
use yew::html::Html;
use {Context, Msg, Model};
use shared::Day;

pub struct Title<R: Labelled>(pub R);

impl<R> Renderable for Title<R>
where
    R: Labelled,
{
    fn render(&self, _: &Model, index: usize) -> Html<Context, Model> {
        html! {
            <li onclick=move|_| Msg::SelectDay(index), >{self.0.label()}</li>
        }
    }
}

pub trait Labelled {
    fn label(&self) -> String;
}

impl<'a> Labelled for &'a Day {
    fn label(&self) -> String {
        Day::label(self)
    }
}
