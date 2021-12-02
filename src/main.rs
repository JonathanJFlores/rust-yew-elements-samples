use yew::{prelude::*};
use log::*;
use uuid::Uuid;

#[derive(Debug)]
enum Message {
    OptionChange,
    OptionSelected(Uuid)
}

struct Model {
    link: ComponentLink<Self>,
    class_groups: Vec<(Uuid, String)>,
    node_options: NodeRef,
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            class_groups: vec![
                    (Uuid::new_v4(), "Option Three".to_string()),
                    (Uuid::new_v4(), "Option Two".to_string()), 
                    (Uuid::new_v4(), "Option Four".to_string())],
            node_options: NodeRef::default()

        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_render = true;
        match msg {
            Message::OptionChange => {
                let node = self
                    .node_options
                    .cast::<web_sys::HtmlSelectElement>()
                    .unwrap();
                info!("OPTION VAlUE {}", node.value() );

                let option_id = Uuid::parse_str(&node.value()).unwrap();
                self.link.send_message(Message::OptionSelected(option_id));
            }
            Message::OptionSelected(option_id) => {
            }
        }
        should_render
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let list_options = self.class_groups.iter().map(|option| {
            let option_id = format!("{:?}", option.0);
            html!{
                <option value=option_id >{&option.1}</option>
            }
        }).collect::<Html>();

        // let on_change_option = self.link.callback(|_: ChangeData| Message::OptionChange);

        let drop_down = html! {
            <div class="select is-medium">
                <select ref=self.node_options.clone()  onchange=self.link.callback(|_: ChangeData| Message::OptionChange)>
                    <option value="NONE">
                        <span>{""}</span>
                    </option>
                    {list_options}
                </select>
            </div>
        };
        html! {
            <div class="m-5">
                <h3 class="title is-4">{"Dropdown Example"}</h3>
                {drop_down}
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::start_app::<Model>();
}