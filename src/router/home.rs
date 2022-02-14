use std::rc::Rc;

use yew::prelude::*;
use crate::components::{
    lifecycleA::LifeCycleA,
    table::Table,
    purecomp::PureComp,
    regcomp::RegComp,
   // noderefs::Ref,
   portaldemo::PortalDemo,
   errorhandling::Hero,
   errorboundary::ErrorBoundary
};
use crate::contexts::{compc::CompC};
use crate::service::postform::*;
use crate::higherordercomp::{
    clickcounter::ClickCounter,
    hovercounter::HoverCount,
    with_counter::OriginalComponent,
};
use crate::renderprop::{clickcountertwo::ClickCounterTwo,
    hovercountertwo::HoverCounterTwo,
    user::User
};
use crate::service::{postform::*, };
use crate::hooks::{};
use crate::hooks::{hookcounter::*, hookcounter2::*, hookcounter3::*};
use crate::use_effect::{
    hookcounterone::HookCounterOne
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
                    <HookCounter/>

                    <HookCounterTwo />

                    <HookCounterThree />

                    <HookCounterOne/>

                    //<PostForm />
                    // <ClickCounterTwo />
                    // <HoverCounterTwo />
                    //<User name={}/>
                </div>
                    // <ClickCounter />
                    // <HoverCount />
                    // < OriginalComponent/>
                
                    // <ErrorBoundary >
                    //     <Hero name={"Man"}/>
                    //     <Hero name={"Man"}/>
                    //     <Hero name={"Man"}/>
                    // </ ErrorBoundary>

                    // <ErrorBoundary> 
                    //     <Hero name={"Joker"}/>
                    // </ErrorBoundary>
                    
                    // <ErrorBoundary> 
                    //     <Hero name={"hello"}/>
                    // </ErrorBoundary>
                    
                    // <ErrorBoundary> 
                    //     <Hero name={"hello"}/>
                    // </ErrorBoundary>   
                    // <PortalDemo />
                    // <Hero name={"Hello"}/>                   
                    // <div class="root"></div>
                    // <div id="portal-root"></div>
                    // <PostList />
                    // <Ref />
                    // <PureComp />
                    // <RegComp />
                    //  <Table />
                    //  <LifeCycleA />
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
            </>
        }
    }
}