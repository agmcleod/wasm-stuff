use stdweb::web::Date;
use yew::{html, html_impl, prelude::*, services::console::ConsoleService};

pub struct Model {
    value: i64,
}

pub enum Msg {
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
}

impl<C> Component<C> for Model
where
    C: AsMut<ConsoleService>,
{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<C, Self>) -> Self {
        Model { value: 0 }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<C, Self>) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value += 1;
                env.as_mut().log("plus one");
            }
            Msg::Decrement => {
                self.value -= 1;
                env.as_mut().log("minus one");
            }
            Msg::Bulk(list) => {
                for msg in list {
                    self.update(msg, env);
                    env.as_mut().log("Bulk action");
                }
            }
        }

        true
    }
}

impl<C> Renderable<C, Model> for Model
where
    C: AsMut<ConsoleService> + 'static,
{
    fn view(&self) -> Html<C, Self> {
        html! {
            <div>
                <nav class="menu",>
                    <button onclick=|_| Msg::Increment,>{"Increment"}</button>
                    <button onclick=|_| Msg::Decrement,>{"Decrement"}</button>
                    <button onclick=|_| Msg::Bulk(vec![Msg::Increment, Msg::Increment]),>{"Increment twice"}</button>
                </nav>
                <p>{self.value}</p>
                <p>{Date::new().to_string()}</p>
            </div>
        }
    }
}
