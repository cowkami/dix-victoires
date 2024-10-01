use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <h1>{"DixVictoires"}</h1>
            <InputAddress/>
            <EnglishAddress/>
        </div>
    }
}

#[function_component]
fn InputAddress() -> Html {
    html! {
        <div>
            <div>
                <h3>{"住所"}</h3>
            </div>
            <div>
                <label for="zipcode">{"郵便番号"}</label>
                <input id="zipcode" />
            </div>
            <div>
                <label for="prefecture">{"都道府県"}</label>
                <input id="prefucture" />
            </div>
            <div>
                <label for="city">{"市区町村"}</label>
                <input id="city" />
            </div>
            <div>
                <label for="address">{"町域・番地"}</label>
                <input id="address" />
            </div>
            <div>
                <label for="building">{"建物名・部屋番号"}</label>
                <input id="building" />
            </div>
            <div>
                <label for="room">{"部屋番号"}</label>
                <input id="room" />
            </div>
        </div>
    }
}

#[function_component]
fn EnglishAddress() -> Html {
    html! {
        <div>
            <div>
                <h3>{"Address"}</h3>
            </div>
            <div>
                {"output"}
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct AddressProps {
    zipcode: String,
}

#[derive(Clone, PartialEq, Properties)]
struct EnglishAddressProps {
    zipcode: String,
}
