const BUILD_NUMBER: u32 = 0;

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
    format!("build-{}", get_build())
}

pub fn get_build() -> u32 {
    BUILD_NUMBER
}
