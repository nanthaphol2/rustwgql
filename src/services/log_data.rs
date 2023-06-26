use tokio;
use crate::databases::db::DB;
use tokio_postgres::{NoTls};
use crate::models::{DataDevice};
use chrono::{DateTime,Local};
pub async fn gets(n :String)->Result<Vec<DataDevice>,tokio_postgres::Error>{
    let (client, connection) =
        tokio_postgres::connect(DB::url().url, NoTls).await?;
tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let mut datas :Vec<DataDevice> = vec![];
    let mut id:i32;
    let mut device:String;
    let mut value:String;
    let mut date:DateTime<Local>;
    let rows = client.query("SELECT * FROM log_datas WHERE date >= NOW() - ($1 ||' minutes')::INTERVAL order by date desc;", &[&n]).await?;
    for row in rows{
        id = row.get(0);
        device = row.get(1);
        value = row.get(2);
        date = row.get(3); 
   datas.push(DataDevice{id:id,device:device,value:value,date:date.format("%Y-%m-%d %H:%M:%S").to_string()});
    } 
    Ok(datas)      
}