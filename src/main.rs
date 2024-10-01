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
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <div class="max-w-lg mx-auto p-6 bg-white rounded-lg shadow-md">
                <h1 class="text-2xl font-bold text-center text-gray-800">{"JPN ▶ US"}</h1>
                <div class="mt-4">
                    <div class="text-lg font-semibold text-gray-700">{"住所"}</div>
                    { self.view_input(ctx.link(), AddressFieldType::ZipCode, "郵便番号", "123-4567") }
                    { self.view_input(ctx.link(), AddressFieldType::Prefecture, "都道府県", "東京都") }
                    { self.view_input(ctx.link(), AddressFieldType::City, "市区町村", "千代田区") }
                    { self.view_input(ctx.link(), AddressFieldType::Address, "町域・番地", "1-1") }
                    { self.view_input(ctx.link(), AddressFieldType::Building, "建物名", "Chateau") }
                    { self.view_input(ctx.link(), AddressFieldType::Room, "部屋番号", "101") }
                </div>
                <div class="mt-6">
                    <div class="text-lg font-semibold text-gray-700">{"Address"}</div>
                    <div class="p-4 border-2 border-blue-500 bg-blue-100 rounded">
                        { self.view_output_address(ctx.link()) }
                    </div>
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
    ) -> Html {
        let oninput = self.handle_input(link, field);
        html! {
            <div class="mb-4">
                <label class="block text-gray-600">{label}</label>
                <input
                    class="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring focus:ring-blue-300"
                    placeholder={placeholder.to_string()}
                    {oninput}
                />
            </div>
        }
    }

    fn view_output_address(&self, _link: &Scope<Self>) -> Html {
        let State {
            zipcode,
            prefecture,
            city,
            address,
            building,
            room,
        } = self.state.clone();
        html! {
            // 1st Row ： From: First name Last name
            // 2nd Row ： Name of the building, like an apartment bldg. and room number (if applicable)
            // 3rd Row ： House number, street, town/village
            // 4th Row ： City, Prefecture/State/Province
            // 5th Row ： Postal Code, Country
            <div>
                {
                    format!("{} {}, {}, {}, {}, {}, Japan",
                        building.render(),
                        room.render(),
                        address.render(),
                        city.render(),
                        prefecture.render(),
                        zipcode.render(),
                    )
                }
            </div>
        }
    }
}
