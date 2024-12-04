mod sabrina_handler;

#[tauri::command]
pub async fn sabrina_hallenstadion(proxy_group: String, mode: String) -> Result<(), String> {
    sabrina_handler::run(proxy_group, mode).await
}
