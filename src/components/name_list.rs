
use yew::prelude::*;
use core::fmt;
use std::fmt::{Display};
use crate::components::{person::Person};
struct Names(Vec<String>);
//  Implement Display Trait for String 
impl Display for Names { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "\n")?;
        for v in &self.0 { 
            write!(f, "\t{}", v)?;
        }
        Ok(())
    }
} 

#[derive(PartialEq, Properties, Clone)]
pub struct Persons { 
    pub id: i32, 
    pub name: String, 
    pub age: i32,
    pub skill: String
}

impl Persons { 
    pub fn new(id: i32, name: String, age: i32, skill: String) -> Self { 
        Self { 
            id, 
            name, 
            age, 
            skill
        }
    }
    fn render(&self) -> Html { 
        html! { 
            <div>
                <p>{format!("{}", self.id)}</p>
                <p>{format!("{}", self.name)}</p>
                <p>{format!("{}", self.age)}</p>
                <p>{format!("{}", self.skill)}</p>
            </div>
        }
    }
}


#[function_component(NameList)]
pub fn name_list() -> Html { 
    let names: Vec<String> = vec!["Bruce", "Clark", "Diana", "Bruce"]
        .into_iter()
        .map(String::from)
        .collect(); 
    let persons: Vec<Persons> = vec![
        Persons { 
            id: 1,
            name: "Philip".to_string(),
            age: 21,
            skill: "Rust".to_string()
        },
        Persons { 
            id: 2,
            name: "Thor".to_string(),
            age: 100,
            skill: "Lightning".to_string()
        }
    ];
    let list = 
    persons
    .iter()
    .map(|person: &Persons| { 
        html! { 
            <>
            <div>
                <div key={person.id.clone()}>{ format!("Hello, I'am {}!", person.name) }</div>
                
            </div>
            </>
        }
    }).collect::<Vec<_>>();
    let list_names = names.iter().enumerate().map(|(index, name)| { 
        
        html! { 
            <>
                <div key={(index)}>
                    {index}
                    {name}

                </div>

            </>
        }
    }).collect::<Html>(); 

    html! { 
        <>
            <div>
          
                <h2>{for names.iter()}</h2>
                <h2>{&names[0]}</h2>
                <h2>{&names[1]}</h2>            
                <h2>{&names[2]}</h2>
                <h2>{&names[3]}</h2>
                {list_names}

                <h2 >{Names(names)}</h2> 
                <span >{list}</span>     
            </div>
            <div></div>
        </>
    }
}