/// 使用者IP位置
///
/// # 變數說明
///
/// * `user`: 使用者結構樹。
/// * `ip_addr`: ip位置(IPv6)。
pub struct UserIPAddr{
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

/// 標頭結構
/// 
/// # 變數說明
/// 
/// *`parent_hash`: 父區塊；指向該區塊的父區塊哈希值
/// *`state_root`: 狀態根；代表區塊應用所有交易後，整個系統的MPT的根哈希值
/// *`transactions_root`: 交易根；所有交易組成的MPT的根哈希值
/// *`receipts_root`: 收據根；所有交易收據組成的MPT的根哈希值
/// *`logs_bloom`: 日誌；一種數據結構，用於快速搜索和過濾日誌
/// *`difficulty`: 難度；當前區塊的挖礦難度
/// *`number`: 區塊號；區塊鏈中該區塊的編號
/// *`gas_limit`: gas限制；該區塊中所有交易可使用的最大gas總量
/// *`gas_used`: gas用量；該區塊中所有交易實際使用的gas量
/// *`timestamp`: 時間戳；區塊創建的時間
/// *`extra_data`: 額外數據；礦工可在其中放置額外訊息
/// *`mix_hash`: 混合哈希；與nonce一起用於挖礦證明
/// *`nonce`: 一個只能使用一次的隨機值，用於區塊的挖礦過程
pub struct Header{
    pub parent_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub transactions_root: [u8; 32],
    pub receipts_root: [u8; 32],    //
    pub difficulty: u32,
    pub number: [u8; 32],   //
    pub gas_limit: u32, //
    pub gas_used: u32, //
    pub timestamp: u32, //
    pub extra_data: [u8; 32], //
    pub mix_hash: [u8; 32], //
    pub nonce: [u8; 32], 
}

/// 帳本結構
/// 
/// # 變數說明
/// 
/// *`header`: 標頭
/// *`data`: 資料
const MAX_DATA_SIZE: u32 = 32;
pub struct Ledger{
    pub header: Header,
    pub data: [u8; MAX_DATA_SIZE],
}