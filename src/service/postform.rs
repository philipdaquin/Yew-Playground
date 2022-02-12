use yew::prelude::*;

pub struct PostForm { 
    user_id: String, 
    title: String, 
    body: String
}

pub enum Msg {
    OnChange(String)
}

impl Component for PostForm {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            user_id: String::new(),
            title: String::new(),
            body: String::new()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let PostForm { user_id, title, body} = self;
        let onchange = ctx.link().callback(|e: Event| Msg::OnChange(e.target().to_strng()))

        return html! {
            <>
                <div>
                    <form>
                        <div>
                            <input 
                                type="text" 
                                name="user_id" 
                                value={user_id.clone()}
                                onchange={onchange}    
                            />                      
                         </div>
                         <div>
                            <input type="text" name="title" value={title.clone()}/>
                        </div>
                        <div>
                            <input type="text" name="body" value={body.clone()}/>
                        </div>
                    </form>          
                </div>
            </>
        }
    }
}