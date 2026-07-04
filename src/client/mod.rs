use crate::common::Command;
use crate::network::connect_client;
use crate::platform::get_active_window_info;
use std::time::Duration;
use tokio::time::sleep;

pub async fn run(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let default_ip = "127.0.0.1".to_string();
    let default_port = "8080".to_string();
    
    let ip = args.get(3).unwrap_or(&default_ip);
    let port = args.get(5).unwrap_or(&default_port);
    let addr = format!("{}:{}", ip, port);
    
    println!("[Client] Connecting to server at {}", addr);
    
    let mut stream = connect_client(&addr).await?;
    println!("[Client] Connected to server");

    let mut last_process = String::new();
    let mut last_layout = 0;

    loop {
        if let Some((process_name, layout, _)) = get_active_window_info() {
            if process_name != last_process || layout != last_layout {
                last_process = process_name.clone();
                last_layout = layout;
                
                let command = Command::SetLayout {
                    process_name: process_name.clone(),
                    layout_id: layout,
                };
                let data = serde_json::to_vec(&command)?;
                crate::network::send_message(&mut stream, &data).await?;
                println!("[Client] Sent layout for {}: {:X}", process_name, layout);
            }
        }
        sleep(Duration::from_millis(500)).await;
    }
}