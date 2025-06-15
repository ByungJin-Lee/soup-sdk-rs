pub mod freeze_target_flags {
    pub const NORMAL: u32 = 1 << 4;
    pub const FAN: u32 = 1 << 5;
    pub const SUPPORTER: u32 = 1 << 6;
    pub const TOP_FAN: u32 = 1 << 7;
    pub const FOLLOWER: u32 = 1 << 8;
    pub const MANAGER: u32 = 1 << 9;
}

pub const SUPER_USERS: [&str; 5] = ["streamer", "manager", "operator", "operator", "cleaner"];
