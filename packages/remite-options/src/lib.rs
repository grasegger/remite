use yew::prelude::*;
use wasm_bindgen::prelude::*;
use yew::web_sys::console;

struct Model {
    link: ComponentLink<Self>,
    mite_instance: String,
    mite_api_key: String,
}

enum Msg {
    ClickSave,
}

impl Model {
    fn save_changes(&mut self) {
        console::log_1(&"Saving changes.".into());
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            mite_instance: "".to_string(),
            mite_api_key: "".to_string(),
        }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ClickSave => self.save_changes()
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::ClickSave)>{ "+1" }</button>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}