use pollster::FutureExt;

pub mod editor;
pub mod engine;
pub mod node;
pub mod renderer;

pub fn start(function: fn() -> ()) {
    log::info!(
        "Kyawthuite Engine {} by Werryx Games v.{} ({})",
        engine::STATE,
        engine::get_version_string(),
        engine::COMMIT,
    );
    engine::init(function).block_on();
}

pub fn start_editor() {
    start(editor::main);
}
