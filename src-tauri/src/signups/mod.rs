mod sabrina_handler;

#[tauri::command]
pub async fn sabrina_royal_arena(app_handle: tauri::AppHandle, proxy_group: String, mode: String) -> Result<(), String> {
    let product_id: &str = "bdd68f6b-dd1c-4551-89d7-c168f8d6c3d0";
    sabrina_handler::run(app_handle, product_id, proxy_group, mode).await
}

#[tauri::command]
pub async fn sabrina_avicii_arena(app_handle: tauri::AppHandle, proxy_group: String, mode: String) -> Result<(), String> {
    let product_id: &str = "a280873c-4ae9-44c9-9cc7-8f70f9d38cdd";
    sabrina_handler::run(app_handle, product_id, proxy_group, mode).await
}

#[tauri::command]
pub async fn sabrina_hallenstadion(app_handle: tauri::AppHandle, proxy_group: String, mode: String) -> Result<(), String> {
    let product_id: &str = "49bdb6bf-1f63-4a46-a5dc-ab1cf79924f0";
    sabrina_handler::run(app_handle, product_id, proxy_group, mode).await
}
