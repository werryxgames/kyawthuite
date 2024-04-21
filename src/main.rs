pub mod node;
pub mod renderer;
pub mod editor;
pub mod engine;
pub mod logger;

fn main() {
    logger::log::info!(
        "Kyawthuite Engine {} by Werryx Games v.{} ({})",
        engine::STATE,
        engine::get_version_string(),
        engine::COMMIT,
    );
}
