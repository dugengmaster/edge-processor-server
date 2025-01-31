use std::time::Instant;
use super::parser::MessageProcessor;

pub fn benchmark_parse_topic() {
    let processor = MessageProcessor::new();
    let topic = "sensor/00:11:22:33:44:55/temperature";
    
    // 整體執行時間
    let total_start = Instant::now();
    
    // 分割操作時間
    let split_start = Instant::now();
    let parts: Vec<&str> = topic.split("/").collect();
    let split_duration = split_start.elapsed();
    
    // 檢查和轉換時間
    let conversion_start = Instant::now();
    let _result = processor.parse_topic(topic);
    let conversion_duration = conversion_start.elapsed();
    
    let total_duration = total_start.elapsed();
    
    println!("效能測試結果:");
    println!("字串分割時間: {:?}", split_duration);
    println!("解析與轉換時間: {:?}", conversion_duration);
    println!("總執行時間: {:?}", total_duration);
}