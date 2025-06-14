pub fn is(flags: u32, flag: u32) -> bool {
    flags & flag == flag
}
