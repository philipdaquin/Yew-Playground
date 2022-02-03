use web_sys::{Element, Node};
use yew::{prelude::*, Html, Children};
use gloo_utils::document;

#[derive(PartialEq, Properties, Clone)]
pub struct Props { 
    pub name: String,
    pub num: i32,
    pub children: Children
}

#[function_component(Hello)]
pub fn return_hello(props: &Props) -> Html { 
    let Props { name, num, children } = props; 
    return html! { 
        <div>
            // <h1>{"Hello "}
            //     {props.name.clone()}
            //     {props.num.clone()}
            //     //  For multiple children, you need to use for props.children.iter() 
            //     { for props.children.iter()}
            // </h1>
            <h1>{"Hello "}
                {name}
                {num}
                //  For multiple children, you need to use for props.children.iter() 
                { for children.iter()}
            </h1>
        </div>
    }

    // let div: Element = document().create_element("div").unwrap();
    // div.set_inner_html("Manually Managing DOM Nodes in Yew");
    // let node: Node = div.into();
    // Html::VRef(node)

    // let level = 1;
    // let text = "Hello World!".to_owned();
    // html! {
    //     <>
    //         <@{format!("h{}", level)} class="title">{ text.clone() }</@>
    //         <h1>{text.clone()}</h1>
    //     </>
    // }

}