use tokio;
use crate::databases::db::DB;
use tokio_postgres::{NoTls};
use crate::models::{DataDevice};
use chrono::{DateTime,Utc,Local};
pub async fn gets()->Result<Vec<DataDevice>,tokio_postgres::Error>{
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
    let rows = client.query("select * from datas order by id", &[]).await?;
    for row in rows{
        id = row.get(0);
        device = row.get(1);
        value = row.get(2);
        date = row.get(3);
                        
      datas.push(DataDevice{id:id,device:device,value:value,date:date.format("%Y-%m-%d %H:%M:%S").to_string()});
    } 
    Ok(datas)      
}
pub async fn getbydevice(_device:String)->Result<Vec<DataDevice>,tokio_postgres::Error>{
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
    let rows = client.query("select * from datas where device = $1", &[&_device]).await?;
    for row in rows{
        id = row.get(0);
        device = row.get(1);
        value = row.get(2);
        date = row.get(3);
                        
      datas.push(DataDevice{id:id,device:device,value:value,date:date.format("%Y-%m-%d %H:%M:%S").to_string()});
    } 
    Ok(datas)      
}
pub async fn insert(p:DataDevice)->Result<u64,tokio_postgres::Error>{
    let (client, connection) =
        tokio_postgres::connect(DB::url().url, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let datetime = Utc::now();
    let command = format!("with i as (
        update datas set date = $3 ,   value = $2 where device = $1 and exists (select 1 from datas where  device = $1)),  u as ( insert into datas (device,value,date) 
          SELECT $1,$2,$3
          WHERE NOT EXISTS (SELECT 1 FROM datas WHERE device = $1)
      )
      insert into log_datas (device,value,date) values($1,$2,$3);");
    
      let rows = client.execute(&command, & [&p.device,&p.value,&datetime]).await?;
    Ok(rows)      
}