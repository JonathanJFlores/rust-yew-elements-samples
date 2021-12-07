use yew::{prelude::*};
use log::*;

mod elements;

#[derive(Debug)]
enum Message {
}

struct Model {
    _link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            _link: link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_render = true;
        // match msg {
        // }
        should_render
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html!{
            <elements::dropdown::DropdownElement />
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::start_app::<Model>();
}