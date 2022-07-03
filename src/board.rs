use std::collections::HashMap;

struct Board {
    positions: HashMap<Position, Player>
}

impl State {
    fn new() -> Self {
        State {
            positions: HashMap::new()
        }
    }
}

impl Component for State {
    type Message = Action;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        State::new()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">
                <div class="inner-container">
                    <div class="cell">{"x1y1"}</div>
                    <div class="cell cell-x">{"x2y1"}</div>
                    <div class="cell">{"x3y1"}</div>

                    <div class="cell cell-y">{"x1y2"}</div>
                    <div class="cell cell-center">{"x2y2"}</div>
                    <div class="cell cell-y">{"x3y2"}</div>

                    <div class="cell">{"x1y3"}</div>
                    <div class="cell cell-x">{"x2y3"}</div>
                    <div class="cell">{"x3y3"}</div>
                </div>
            </div>
        }
    }
}
