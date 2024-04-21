pub static VERSION: [u16; 3] = [0, 0, 0];
pub static STATE: &str = "dev";
pub static COMMIT: &str = env!("GIT_HASH");

pub fn get_version_string() -> String {
    format!("{}.{}.{}", VERSION[0], VERSION[1], VERSION[2])
}
