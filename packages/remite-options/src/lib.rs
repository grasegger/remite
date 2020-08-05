use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::web_sys::console;
use yew::web_sys::HtmlInputElement;
use yew::MouseEvent;
use browser_storage_sync as settings;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


struct Model {
    link: ComponentLink<Self>,
    mite_instance: String,
    mite_api_key: String,
    instance_ref: NodeRef,
    apikey_ref: NodeRef,
}

enum Msg {
    ClickSave(MouseEvent),
}

impl Model {
    fn save_changes(&mut self) {
        let instance = self.instance_ref
            .cast::<HtmlInputElement>()
            .unwrap()
            .value();

        settings::Sync::set_string("instance", &instance);

        let api_key = self.apikey_ref
            .cast::<HtmlInputElement>()
            .unwrap()
            .value();

        

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
            instance_ref: NodeRef::default(),
            apikey_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ClickSave(mouse_event) => {
                mouse_event.prevent_default();
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
                        <input type="text" id="instanceField" ref={self.instance_ref.clone()}  />
                        <label for="instanceField">{"Your mite api key."}</label>
                        <input type="text" id="apiKeyField" ref={self.apikey_ref.clone()}  />

                        <button onclick=self.link.callback(|event| Msg::ClickSave(event))>{ "Save" }</button>
                    </fieldset>
                </form>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    console_error_panic_hook::set_once();
    App::<Model>::new().mount_to_body();
}
