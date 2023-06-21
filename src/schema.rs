use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateNoteSchema {
    pub barcodestring:i64,
    pub title: String,
    pub price: i32,
    //pub barstring:i64,
  //  #[serde(skip_serializing_if = "Option::is_none")]
    
   
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateNoteSchema {
    pub barcodestring: Option<i64>,
    pub title: Option<String>,
    pub price: Option<i32>
 
    
    
}
