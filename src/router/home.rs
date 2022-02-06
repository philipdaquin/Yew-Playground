use std::rc::Rc;

use yew::prelude::*;
use crate::components::{
    lifecycleA::LifeCycleA
    
};
pub struct Home;
impl Component for Home { 
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        
        
        html! {
            <>
                <div>
                    <LifeCycleA />


                // <FormInput />
                // <FormArea />
                // <Inline/>
                // <StyleSheets styles={true} />
                // <NameList/>
                // <Person  id ={1} name={"Philip"} age={21} skill={"Rust"}/>
                // <Person  id={1} name={"Philip"} age={31} skill={"Substrate"}/>
                // <Conditional/>
                //<ParentComponent prop1={"Lorem"} prop2={"Ipsum"}/>
                //<EventBind /> 
                // <FunctionClick />
                // <ClassClick />
                // <Counter/>
                // <Message/>
                // <div class="home">
                    // <Hello name={"WALICE"} num={2}>
                    //     <p>{"THis is Children Props "}</p>
                    //     <p>{"THis is Children Props "}</p>
                    //     <p>{"THis is Children Props "}</p>
                    //     <p>{"THis is Children Props "}</p>
                    //     <button>{"Hello"}</button>
                    // </Hello>
                    // <Class name={"This Is Coming From a Impl Components"} num={2012}>
                    //     <p>{"sdasdasd"}</p>
                    //     <p>{"This is oming from a shared property "}</p>
                    // </Class>
                //
                //     <img class="profile-picture" src="assets/images/avatar.jpg" alt="ShironCat's avatar" />
                //     <h1>{ "Hello, World!" }</h1>
                //     <img src="https://http.cat/404.jpg" />
                //</div>

                </div>
            </>
        }
    }
}