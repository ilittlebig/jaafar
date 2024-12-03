#[tauri::command]
pub fn sabrina_hallenstadion(proxy_group: String, mode: String) {
    println!("I am going to start the signups in Rust now! {:?} {:?}", proxy_group, mode);
}
