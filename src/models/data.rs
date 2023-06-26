use serde::{Deserialize,Serialize};
#[derive(Deserialize,Serialize,Debug)]
pub struct DataDevice{
    #[serde(skip_deserializing)]
    pub id:i32,
    pub device:String,
    pub value :String,
    #[serde(skip_deserializing)]
    pub date: String,
}