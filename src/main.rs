use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;
use chrono;

// 定义设备数据结构
#[derive(Serialize, Deserialize, Clone)]
struct DeviceResponse {
    name: String,
    device_type: String,
    status: String,
}

// 定义应用状态
struct AppState {
    devices: Mutex<HashMap<String, DeviceResponse>>,
}

// 保存设备数据到文件
fn save_devices(devices: &HashMap<String, DeviceResponse>) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(devices)?;
    fs::write("device.json", json)?;
    Ok(())
}

// 从文件加载设备数据
fn load_devices() -> HashMap<String, DeviceResponse> {
    match fs::read_to_string("device.json") {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => HashMap::new(),
    }
}

// GET 请求处理器
#[get("/device/{id}")]
async fn get_device(
    id: web::Path<String>,
    data: web::Data<AppState>
) -> impl Responder {
    let devices = data.devices.lock().unwrap();
    match devices.get(&id.into_inner()) {
        Some(device) => HttpResponse::Ok().json(device),
        None => HttpResponse::NotFound().body("Device not found"),
    }
}

// POST 请求处理器
#[post("/device/{id}")]
async fn create_device(
    id: web::Path<String>,
    device: web::Json<DeviceResponse>,
    data: web::Data<AppState>
) -> impl Responder {
    let mut devices = data.devices.lock().unwrap();
    devices.insert(id.into_inner(), device.into_inner());
    
    // 保存到文件
    if let Err(e) = save_devices(&devices) {
        eprintln!("Error saving devices: {}", e);
        return HttpResponse::InternalServerError().body("Failed to save device");
    }
    
    HttpResponse::Created().body("Device created successfully")
}

// 获取所有设备
#[get("/devices")]
async fn get_all_devices(data: web::Data<AppState>) -> impl Responder {
    let devices = data.devices.lock().unwrap();
    HttpResponse::Ok().json(&*devices)
}

// 健康检查接口
#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting at http://127.0.0.1:8080");
    
    // 初始化应用状态
    let app_state = web::Data::new(AppState {
        devices: Mutex::new(load_devices()),
    });
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(get_device)
            .service(create_device)
            .service(get_all_devices)
            .service(health_check)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
