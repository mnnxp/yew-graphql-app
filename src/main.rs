mod requests;

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::services::console::ConsoleService;
use requests::*;

// Define the possible messages which can be sent to the component
pub enum Msg {
    Increment,
    Decrement,
    SendResponse,
}

pub struct Model {
    link: ComponentLink<Self>,
    value: i64, // This will store the counter value
    response: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0,  response: ("Not response".to_owned())}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value += 1;
                ConsoleService::log("plus one"); // Will output a string to the browser console
                true // Return true to cause the displayed change to update
            }
            Msg::Decrement => {
                self.value -= 1;
                ConsoleService::log("minus one");
                true
            }
            Msg::SendResponse => {
                // self.response = "Update".to_owned();
                // let mut req = "";
                self.response.run_query();
                // ConsoleService::log(format!("SendResponse {:?}", req).as_str());
                ConsoleService::log("SendResponse");
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div class="panel">
                    // A button to send the Increment message
                    <button class="button" onclick=self.link.callback(|_| Msg::Increment)>
                        { "+1" }
                    </button>

                    // A button to send the Decrement message
                    <button onclick=self.link.callback(|_| Msg::Decrement)>
                        { "-1" }
                    </button>

                    // A button to send two Increment messages
                    <button onclick=self.link.batch_callback(|_| vec![Msg::Increment, Msg::Increment])>
                        { "+1, +1" }
                    </button>

                    // A button to send two Increment messages
                    <button onclick=self.link.callback(|_| Msg::SendResponse)>
                        { "SendResponse" }
                    </button>

                </div>

                // Display the current value of the counter
                <p class="counter">
                    { self.value }
                </p>

                // Display the current value of the counter
                <p class="response">
                    { format!("{}", self.response) }
                </p>

                // Display the current date and time the page was rendered
                <p class="footer">
                    { ":)" }
                </p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
