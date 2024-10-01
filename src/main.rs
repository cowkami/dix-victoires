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
            <div>
                <h1>{"DixVictoires"}</h1>
                <div>
                    <div>
                        <h3>{"住所"}</h3>
                    </div>
                    { self.view_input_zipcode(ctx.link()) }
                    { self.view_input_prefecture(ctx.link()) }
                    { self.view_input_city(ctx.link()) }
                    { self.view_input_address(ctx.link()) }
                    { self.view_input_building(ctx.link()) }
                    { self.view_input_room(ctx.link()) }
                </div>
                <div>
                    <div>
                        <h3>{"Address"}</h3>
                    </div>
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

    fn view_input_zipcode(&self, link: &Scope<Self>) -> Html {
        let oninput = self.handle_input(link, AddressFieldType::ZipCode); // 関数化したoninputを呼び出す
        html! {
            <div>
                <label for="zipcode">{"郵便番号"}</label>
                // todo: validate
                <input
                    class="zipcode"
                    placeholder="123-4567"
                    {oninput}
                />
            </div>
        }
    }

    fn view_input_prefecture(&self, link: &Scope<Self>) -> Html {
        let oninput = self.handle_input(link, AddressFieldType::Prefecture);
        html! {
            <div>
                <label for="prefecture">{"都道府県"}</label>
                // todo: validate
                <input
                    class="prefecture"
                    placeholder="東京都"
                    {oninput}
                />
            </div>
        }
    }

    fn view_input_city(&self, link: &Scope<Self>) -> Html {
        let oninput = self.handle_input(link, AddressFieldType::City);
        html! {
            <div>
                <label for="city">{"市区町村"}</label>
                <input
                    class="city"
                    placeholder="千代田区"
                    {oninput}
                />
            </div>
        }
    }

    fn view_input_address(&self, link: &Scope<Self>) -> Html {
        let oninput = self.handle_input(link, AddressFieldType::Address);
        html! {
            <div>
                <label for="address">{"町域・番地"}</label>
                <input
                    class="address"
                    placeholder="1-1"
                    {oninput}
                />
            </div>
        }
    }

    fn view_input_building(&self, link: &Scope<Self>) -> Html {
        let oninput = self.handle_input(link, AddressFieldType::Building);
        html! {
            <div>
                <label for="building">{"建物名"}</label>
                <input
                    class="building"
                    placeholder="Chateau"
                    {oninput}
                />
            </div>
        }
    }

    fn view_input_room(&self, link: &Scope<Self>) -> Html {
        let oninput = self.handle_input(link, AddressFieldType::Room);
        html! {
            <div>
                <label for="room">{"部屋番号"}</label>
                <input
                    class="room"
                    placeholder="101"
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
            <div>
                { format!("#{} {} {} {} {} {}", zipcode.0, prefecture.0, city.0, address.0, building.0, room.0,) }
            </div>
        }
    }
}
