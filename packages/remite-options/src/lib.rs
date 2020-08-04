use yew::prelude::*;
use wasm_bindgen::prelude::*;
use yew::web_sys::console;
use yew::web_sys::window;
use yew::MouseEvent;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


struct Model {
    link: ComponentLink<Self>,
    mite_instance: String,
    mite_api_key: String,
}

enum Msg {
    ClickSave(MouseEvent),
}

impl Model {
    fn save_changes(&mut self) {
        let document = window().unwrap().document().unwrap();

        let instance_element = document.query_selector("input#instanceField").unwrap().unwrap();
        let instance_attributes = instance_element;
        console::log_1(&format!("{:?}", instance_attributes).into());
        console::log_1(&"Saving changes.".into());
        console::log_1(&self.mite_api_key.clone().into());
        console::log_1(&self.mite_instance.clone().into());
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
            Msg::ClickSave (MouseEvent) => {
			    MouseEvent.prevent_default();
			    self.save_changes()
		    }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="container" style="padding-top: 1rem">
                <form>
                    <fieldset>
                        <label for="instanceField">{"Your mite instance."}</label>
                        <input type="text" id="instanceField" />
                        
                        <label for="instanceField">{"Your mite api key."}</label>
                        <input type="text" id="apiKeyField" />

                        <button onclick=self.link.callback(|event| Msg::ClickSave(event))>{ "Save" }</button>
                    </fieldset>
                </form>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}