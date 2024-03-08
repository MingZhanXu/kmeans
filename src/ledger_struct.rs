/// 使用者IP位置帳本
///
/// # 變數說明
///
/// * `user`: 使用者結構樹。
/// * `ip_addr`: ip位置(IPv6)。
pub struct UserIPAddrLedger{
    pub user: String,
    pub ip_addr: String,
}
/// 使用者
/// 
/// # 變數說明
///
/// * `area`: 地區名稱(創建時間Hash)。
/// * `main_user_id`: 使用者樹所有人公鑰。
/// * `id`: 使用者公鑰。
/// * `next_id_hash`: 下一筆使用者公鑰Hash。
/// * `permissions`: 權限。
/// * `subusers`: 子使用者群。
/// * `subusers_permissions`: 子使用者群權限。
pub struct User{
    pub area: String,
    pub main_user_id: Option<String>,
    pub id: String,
    pub next_id_hash: String,
    pub permissions: u32,
    pub subusers: Option<Vec<User>>,
    pub subusers_permissions: u32,
}
// pub struct AreaLedger{
//     pub name: String,
//     pub log: String,
// }
// pub struct 