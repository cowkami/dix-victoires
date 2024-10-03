mod state;

use state::*;
use web_sys::HtmlInputElement;
use yew::html::Scope;
use yew::prelude::*;
use yew::NodeRef;

fn main() {
    yew::Renderer::<App>::new().render();
}

pub enum Msg {
    Add(AddressFieldType, String),
    NoOp,
}

struct App {
    state: State,
    focus_ref: NodeRef,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            state: State {
                zipcode: ZipCode("".to_string()),
                prefecture: Prefecture("".to_string()),
                city: City("".to_string()),
                address: Address("".to_string()),
                building: Building("".to_string()),
                room: Room("".to_string()),
            },
            focus_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add(AddressFieldType::ZipCode, value) => {
                self.state.zipcode = ZipCode(value);
            }
            Msg::Add(AddressFieldType::Prefecture, value) => {
                self.state.prefecture = Prefecture(value);
            }
            Msg::Add(AddressFieldType::City, value) => {
                self.state.city = City(value);
            }
            Msg::Add(AddressFieldType::Address, value) => {
                self.state.address = Address(value);
            }
            Msg::Add(AddressFieldType::Building, value) => {
                self.state.building = Building(value);
            }
            Msg::Add(AddressFieldType::Room, value) => {
                self.state.room = Room(value);
            }
            Msg::NoOp => {
                // Do nothing or handle as needed
            }
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <div class="max-w-lg mx-auto p-6 bg-white rounded-lg shadow-md">
                <div class="text-2xl font-bold text-center text-gray-800">
                    {"JPN ▶ US"}
                </div>
                <div class="mt-4">
                    <div class="text-lg font-semibold text-gray-700">
                        {"JPN"}
                    </div>
                    { self.view_input(
                        ctx.link(),
                        AddressFieldType::ZipCode,
                        "郵便番号",
                        "123-4567",
                        "tel",
                        "numeric",
                        "8",
                      )
                    }
                    { self.view_input(
                        ctx.link(),
                        AddressFieldType::Prefecture,
                        "都道府県",
                        "東京都",
                        "text",
                        "text",
                        "10",
                      )
                    }
                    { self.view_input(
                        ctx.link(),
                        AddressFieldType::City,
                        "市区町村",
                        "千代田区",
                        "text",
                        "text",
                        "30",
                      )
                    }
                    { self.view_input(
                        ctx.link(),
                        AddressFieldType::Address,
                        "町域・番地",
                        "1-1",
                        "text",
                        "text",
                        "30",
                      )
                    }
                    { self.view_input(
                        ctx.link(),
                        AddressFieldType::Building,
                        "建物名",
                        "Chateau Noir",
                        "text",
                        "text",
                        "30",
                      )
                    }
                    { self.view_input(
                        ctx.link(),
                        AddressFieldType::Room,
                        "部屋番号",
                        "101",
                        "text",
                        "text",
                        "30",
                      )
                    }
                </div>
                <div class="mt-6">
                    { self.view_output_address(ctx.link()) }
                </div>
            </div>
        }
    }
}

impl App {
    fn handle_input(&self, link: &Scope<Self>, field: AddressFieldType) -> Callback<InputEvent> {
        link.batch_callback(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Some(Msg::Add(field.clone(), input.value()))
        })
    }

    fn view_input(
        &self,
        link: &Scope<Self>,
        field: AddressFieldType,
        label: &str,
        placeholder: &str,
        r#type: &str,
        inputmode: &str,
        maxlength: &str,
    ) -> Html {
        let oninput = self.handle_input(link, field);
        html! {
            <div class="mb-4">
                <label class="mb-2 block text-gray-600">{label}</label>
                <input
                    class="w-full pl-4 p-2 border border-gray-300 rounded-full focus:outline-none focus:ring focus:ring-blue-300"
                    placeholder={placeholder.to_string()}
                    type={r#type.to_string()}
                    inputmode={inputmode.to_string()}
                    maxlength={maxlength.to_string()}
                    {oninput}
                />
            </div>
        }
    }

    fn view_output_address(&self, link: &Scope<Self>) -> Html {
        html! {
            <div>
                <div class="text-lg font-semibold text-gray-700">{"US"}</div>
                <div class="p-4 border-2 border-blue-500 bg-blue-100 rounded-xl">
                    <div class="text-lg font-semibold">
                        { self.state.render() }
                    </div>
                    <div class="flex justify-end">
                        <button
                            onclick={self.copy_to_clipboard(link)}
                            class="p-2 copy-button rounded-full"
                        >
                            <svg
                                class="h-8 w-8 text-gray-500"  width="12" height="12" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                                <path stroke="none" d="M0 0h24v24H0z"/>
                                <rect x="8" y="8" width="12" height="12" rx="2" />
                                <path d="M16 8v-2a2 2 0 0 0 -2 -2h-8a2 2 0 0 0 -2 2v8a2 2 0 0 0 2 2h2" />
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }

    fn copy_to_clipboard(&self, link: &Scope<Self>) -> Callback<MouseEvent> {
        let address = self.state.render();
        link.callback(move |_| {
            let clipboard = web_sys::window().unwrap().navigator().clipboard();
            let _ = clipboard.write_text(&address);
            Msg::NoOp
        })
    }
}
