use yew::prelude::*;

pub struct PostForm;

pub enum Msg {
}

impl Component for PostForm {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        return html! {
            <>
                <div>
                    <form>
                        <div>
                            <form action=""><input type="text" name="user_id"/></form>
                        </div>
                         <div>
                            <form action=""><input type="text" name="title"/></form>
                        </div>
                        <div>
                            <form action=""><input type="text" name="body"/></form>
                        </div>
                    </form>
                </div>
            </>
        }
    }
}