pub fn get_full_version_string() -> String {
    return format!("{} {} {}", get_name(), get_stage(), get_version_string());
}

pub fn get_name() -> &'static str {
    return &"gotg";
}

pub fn get_stage() -> &'static str {
    return &"prototype";
}

pub fn get_version_string() -> String {
    return format!("build-{}", get_build());
}

pub fn get_build() -> u32 {
    return 0;
}
