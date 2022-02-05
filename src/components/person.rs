use yew::prelude::*;
use crate::components::name_list::Persons;

#[function_component(Person)]
pub fn person(person: &Persons) -> Html { 
    html! { 
        <div>
            <h2>{"My Name is "}{person.name.clone()}</h2>
            <h3>{"I am "}{person.age.clone()}{" years old"}</h3>
            <h4>{"I know "}{person.skill.clone()}</h4>
            
        </div>
    }
}