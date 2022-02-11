use yew::prelude::*;

pub struct ClickCounterTwo  {
    pub count: i32
}

pub enum Msg {
    IncrementCount
}

impl Component for ClickCounterTwo {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            count: 0
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg { 
            Msg::IncrementCount => { 
                self.count = self.count + 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ClickCounterTwo { count } = self;

        html! {

            <button onclick={ctx.link().callback(|_| Msg::IncrementCount)}>
                {format!("Click {} times", count)}
            </button>
        }
    }
}