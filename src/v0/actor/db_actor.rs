use crate::v0::actor::data_actor::SensorData;
use chrono::{DateTime, Utc};
use ractor::{Actor, ActorProcessingErr, ActorRef};
use sqlx::PgPool;
use std::sync::Arc;

// DB Actor 接收的訊息類型
pub enum DbMessage {
    StoreSensorData {
        data: Vec<SensorData>, // 感測器資料向量
    },
}

// DB Actor 結構體
pub struct DbActor;

// Actor 狀態，包含資料庫連線池
pub struct DbState {
    pool: Arc<PgPool>,
}

impl Actor for DbActor {
    type Msg = DbMessage;
    type State = DbState;
    type Arguments = String; // 資料庫連線字串

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        db_url: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        println!("[INFO] 初始化 DB Actor，連線至：{}", db_url);

        // 初始化資料庫連線池
        let pgsql_pool = PgPool::connect(&db_url)
            .await
            .expect("Failed to crate Pgsql pool");

        println!("[INFO] 資料庫連線池已建立");

        Ok(DbState {
            pool: Arc::new(pgsql_pool),
        })
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            DbMessage::StoreSensorData { data } => {
                let mut success_count = 0;
                let mut error_count = 0;

                for sensor in &data {
                    // 嘗試將字串時間戳解析為 DateTime<Utc>
                    let timestamp = match DateTime::parse_from_rfc3339(&sensor.timestamp) {
                        Ok(dt) => dt.with_timezone(&Utc),
                        Err(_) => {
                            println!("[ERROR] 時間戳解析失敗：{}", sensor.timestamp);
                            error_count += 1;
                            continue;
                        }
                    };

                    // 插入資料到資料庫
                    let result = sqlx::query!(
                        r#"
                        INSERT INTO sensor_data (time, device_id, sensor_name, value, unit)
                        VALUES ($1, $2, $3, $4, $5)
                        "#,
                        timestamp,
                        sensor.sensor_name,
                        sensor.value,
                        sensor.unit
                    )
                    .execute(&*state.pool)
                    .await;

                    match result {
                        Ok(_) => {}
                        Err(e) => {
                            println!("[ERROR] 儲存感測器資料失敗：{:?}", e);
                            error_count += 1;
                        }
                    }
                }

                println!(
                    "[INFO] 資料庫儲存結果：{} 筆成功，{} 筆失敗",
                    success_count, error_count
                );
            }
        }
        Ok(())
    }
}
