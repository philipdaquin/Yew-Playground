use gloo::timers::callback::Interval;
use yew::prelude::*;

pub struct ClassTimer { 
    pub timer: Option<Interval>,
    pub time: String
}

pub enum Msg {
    SetInterval,
    UpdateTime
}

impl ClassTimer { 
    fn get_current() -> String { 
        let date = js_sys::Date::new_0();
        String::from(date.to_locale_time_string("en-US"))
    }
}



impl Component for ClassTimer {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            timer: None,
            time: String::new()
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg { 
            Msg::SetInterval => {
                let handle = { 
                    let link = ctx.link().clone();
                    Interval::new(1, move || link.send_message(Msg::UpdateTime))
                };
                self.timer = Some(handle)
                
            }
            Msg::UpdateTime => {
                self.time = ClassTimer::get_current();
                
            }
        }
        true 
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(move |e: MouseEvent| Msg::SetInterval);
        html! {
            <>
                <div>
                    <button onclick={onclick}>{"Set Timer"}</button>
                    {self.time.clone()}
                </div>
            </>     
        }
    }
}