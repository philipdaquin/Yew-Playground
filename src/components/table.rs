use yew::prelude::*;
use crate::components::columns::Columns;


#[function_component(Table)]
pub fn table() -> Html { 
    html! { 
        <>
            <table>
                <tbody>
                    <tr>
                        <Columns />                    
                    </tr>
                </tbody>
            </table>
        </>
    }
}