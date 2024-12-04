mod sabrina_handler;

#[tauri::command]
pub async fn sabrina_hallenstadion(app_handle: tauri::AppHandle, proxy_group: String, mode: String) -> Result<(), String> {
    const product_id: &str = "bdd68f6b-dd1c-4551-89d7-c168f8d6c3d0";
    sabrina_handler::run(app_handle, proxy_group, mode, product_id).await
}
