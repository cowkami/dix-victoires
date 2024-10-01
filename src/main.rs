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
    Add(AddressType, String),
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
                zipcode: "".to_string(),
                prefecture: "".to_string(),
                city: "".to_string(),
                address: "".to_string(),
                building: "".to_string(),
                room: "".to_string(),
            },
            focus_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add(AddressType::ZipCode, value) => {
                self.state.zipcode = value;
            }
            Msg::Add(AddressType::Prefecture, value) => {
                self.state.prefecture = value;
            }
            Msg::Add(AddressType::City, value) => {
                self.state.city = value;
            }
            Msg::Add(AddressType::Address, value) => {
                self.state.address = value;
            }
            Msg::Add(AddressType::Building, value) => {
                self.state.building = value;
            }
            Msg::Add(AddressType::Room, value) => {
                self.state.room = value;
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
                    <div>
                        { format!("#{}", &self.state.zipcode.to_string()) }
                        { &self.state.prefecture.to_string() }
                        { &self.state.city.to_string() }
                        { &self.state.address.to_string() }
                        { &self.state.building.to_string() }
                        { &self.state.room.to_string() }
                    </div>
                </div>
            </div>
        }
    }
}

impl App {
    fn handle_input(&self, link: &Scope<Self>, field: AddressType) -> Callback<InputEvent> {
        link.batch_callback(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            Some(Msg::Add(field.clone(), value))
        })
    }

    fn view_input_zipcode(&self, link: &Scope<Self>) -> Html {
        let oninput = self.handle_input(link, AddressType::ZipCode); // 関数化したoninputを呼び出す
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
        let oninput = self.handle_input(link, AddressType::Prefecture);
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
        let oninput = self.handle_input(link, AddressType::City);
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
        let oninput = self.handle_input(link, AddressType::Address);
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
        let oninput = self.handle_input(link, AddressType::Building);
        html! {
            <div>
                <label for="building">{"建物名・部屋番号"}</label>
                <input
                    class="building"
                    placeholder="101"
                    {oninput}
                />
            </div>
        }
    }

    fn view_input_room(&self, link: &Scope<Self>) -> Html {
        let oninput = self.handle_input(link, AddressType::Room);
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
}
