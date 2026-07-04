use crate::common::Command;
use crate::network::{start_server, read_message, send_message};
use crate::platform::{get_active_window_info, set_layout};
use std::collections::HashMap;

pub async fn run(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let default_port = "8080".to_string();
    let port = args.get(3).unwrap_or(&default_port);
    let addr = format!("0.0.0.0:{}", port);
    let listener = start_server(&addr).await?;
    
    println!("[Server] Listening on {}", addr);
    println!("[Server] Synchronizing keyboard layouts...");
    
    let mut layout_map: HashMap<String, u32> = HashMap::new();

    loop {
        let (mut stream, addr) = listener.accept().await?;
        println!("[Server] Client connected: {}", addr);

        let data = read_message(&mut stream).await?;
        if let Ok(command) = serde_json::from_slice::<Command>(&data) {
            match command {
                Command::SetLayout { process_name, layout_id } => {
                    layout_map.insert(process_name.clone(), layout_id);
                    println!("[Server] Saved layout for process: {} -> {:X}", process_name, layout_id);
                    
                    if let Some((active_process, active_layout, _)) = get_active_window_info() {
                        if active_process == process_name && active_layout != layout_id {
                            println!("[Server] Switching layout for {} to {:X}", active_process, layout_id);
                            set_layout(layout_id);
                        }
                    }
                }
                Command::GetLayout { process_name } => {
                    let layout_id = layout_map.get(&process_name).unwrap_or(&0x0409);
                    let response = serde_json::to_vec(&Command::SetLayout {
                        process_name: process_name.clone(),
                        layout_id: *layout_id,
                    })?;
                    send_message(&mut stream, &response).await?;
                }
            }
        }
    }
}