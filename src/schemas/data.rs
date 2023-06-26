use juniper::{FieldResult};
use juniper::{EmptySubscription, RootNode};
use juniper::{GraphQLInputObject, GraphQLObject};
use crate::models::DataDevice;
use crate::services::{data,log_data};
#[derive(GraphQLObject)]
#[graphql(description = "rust-actix-postgresql-graphql")]
pub struct Result{
    message: String,
}
#[derive(GraphQLObject)]
#[graphql(description = "rust-actix-postgresql-graphql")]
pub struct Device{
    pub device:String,
    pub value :String,
    pub date: String,
}
#[derive(GraphQLInputObject)]
#[graphql(description = "rust-actix-postgresql-graphql")]
pub struct NewDevice{
    pub device:String,
    pub value :String,
}
pub struct QueryRoot;
#[juniper::graphql_object]
impl QueryRoot {
    async fn data(device: String) -> FieldResult<Device> {  
        let data = data::getbydevice(device).await?;
        Ok(Device{device: data[0].device.to_owned(),date:data[0].date.to_owned(),value:data[0].value.to_owned()})       
    }
    async fn log_data(minute: String) -> FieldResult<Vec<Device>> {  
        let data = log_data::gets(minute).await?;
        let mut datas:Vec<Device> = vec![];
        for i in 0..data.len(){
            datas.push(Device{device: data[i].device.to_owned(),date:data[i].date.to_owned(),value:data[i].value.to_owned() })
        }
        Ok(datas)      
    }
}
pub struct MutationRoot;
#[juniper::graphql_object]
impl MutationRoot {
    async fn create_data(data: NewDevice) -> FieldResult<Result> {
        let d = DataDevice{id:0,date:"".to_string(),device:data.device,value:data.value};
       let result =  data::insert(d).await?;
       if result == 1{
            Ok(Result{message : String::from("OK")})     
       }else{
            Ok(Result{message : String::from("Error")})  
       }
    }
}
pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;
pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}