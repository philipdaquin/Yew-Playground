use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListInfo { 
    
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all= "camelCase")]
pub struct ArticleListInfo { 
    pub article: Vec<ListInfo>  
}