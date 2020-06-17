pub fn get_full_version_string() -> String {
    format!("{} {} {}", get_name(), get_stage(), get_version_string())
}

pub fn get_name() -> &'static str {
    "gotg"
}

pub fn get_stage() -> &'static str {
    "prototype"
}

pub fn get_version_string() -> String {
    format!("V{}.{}.{}", get_major(), get_minor(), get_patch())
}

pub fn get_major() -> u32 {
    0
}

pub fn get_minor() -> u32 {
    0
}

pub fn get_patch() -> u32 {
    0
}
