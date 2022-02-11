use yew::prelude::*;

pub struct HoverCounterTwo { 
    pub count: i32
}

pub enum Msg {
    IncrementCount
}

impl Component for HoverCounterTwo {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { count: 0 }
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
        let HoverCounterTwo {count } = self;
        html! {
            <h1 onmouseover={ctx.link().callback(|_| Msg::IncrementCount)}>
                {format!("Hovered {} Times", count)}
            </h1>
        }
    }
}