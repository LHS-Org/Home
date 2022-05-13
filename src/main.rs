use yew::prelude::*;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div id="welcome">
                <h1>{"Loek Sangers"}</h1>
                <p>{"Feel free to contact me on "}<a href="https://www.linkedin.com/in/loeksangers/">{"LinkedIn"}</a>{" or see what I have been up to lately on "}<a href="https://github.com/LoekSangers">{"GitHub"}</a>{"."}</p>
                <p>{"If you were expecting an introduction, those places are also where you need to be ;)."}</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Home>();
}
