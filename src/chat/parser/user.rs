use crate::chat::{
    constants::user_flags,
    parser::{types::UserFlags, util::is},
    types::UserStatus,
};

pub fn parse_user_status(flag_str: &str) -> UserStatus {
    let flags = parse_user_flags(flag_str);

    UserStatus {
        follow: get_follow(flags.follow),
        is_bj: is(flags.combined, user_flags::BJ),
        is_manager: is(flags.combined, user_flags::MANAGER),
        is_top_fan: is(flags.combined, user_flags::TOP_FAN),
        is_fan: is(flags.combined, user_flags::FAN),
        is_supporter: is(flags.combined, user_flags::SUPPORTER),
    }
}

fn parse_user_flags(flag_str: &str) -> UserFlags {
    let flags: Vec<u32> = flag_str
        .split("|")
        .map(|val| val.parse::<u32>().unwrap())
        .collect();

    return UserFlags {
        follow: flags[1],
        combined: flags[0],
    };
}

fn get_follow(flags: u32) -> u8 {
    // 1티어
    if is(flags, user_flags::FOLLOWER_TIER1) {
        return 1;
        // 2티어
    } else if is(flags, user_flags::FOLLOWER_TIER2) {
        return 2;
    }
    return 0;
}
