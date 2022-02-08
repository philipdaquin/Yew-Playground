use yew::{prelude::*, functional};

#[function_component(Columns)]
pub fn columns() -> Html { 
    let items = vec!["hello"];
    items.iter().map(|x| {
        return html! { 
            <>
                <div>{x}</div>
            </>
        }
    });
    html! { 
        <>
            
            <div>
                {items}
                <td>{"Name: "}</td>
                <td>{"Philip Daquin"}</td>
            </div>
        </>
    }
}