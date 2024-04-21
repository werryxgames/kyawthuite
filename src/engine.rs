use std::sync::{Arc, Mutex, MutexGuard};

use cfg_if::cfg_if;
use wgpu::Color;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use crate::renderer::State;

pub struct Engine {
    clear_color: Color,
}

impl Engine {
    pub fn get_clear_color(&self) -> Color {
        self.clear_color
    }

    pub fn set_clear_color(&mut self, r: f64, g: f64, b: f64, a: f64) {
        self.clear_color.r = r;
        self.clear_color.g = g;
        self.clear_color.b = b;
        self.clear_color.a = a;
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            clear_color: Color {
                r: 0.02,
                g: 0.02,
                b: 0.02,
                a: 1.0,
            }.into()
        }
    }
}

pub static VERSION: [u16; 3] = [0, 0, 0];
pub static STATE: &str = "dev";
pub static COMMIT: &str = env!("GIT_HASH");
pub static mut ENGINE: Option<Engine> = None;

pub fn get_version_string() -> String {
    format!("{}.{}.{}", VERSION[0], VERSION[1], VERSION[2])
}

pub fn get_engine_mut() -> &'static mut Engine {
    if unsafe { ENGINE.is_none() } {
        unsafe { ENGINE = Some(Engine::default()); }
    }

    // let mutex = ;
    unsafe { ENGINE.as_mut().unwrap_unchecked() }
}

pub fn get_engine() -> &'static Engine {
    if unsafe { ENGINE.is_none() } {
        unsafe { ENGINE = Some(Engine::default()); }
    }

    // let mutex = ;
    unsafe { ENGINE.as_ref().unwrap_unchecked() }
}

// pub fn get_engine() -> &Engine {
//     if unsafe { ENGINE.is_none() } {
//         unsafe { ENGINE = Some(Arc::new(Mutex::new(Engine::default()))); }
//     }

//     // let mutex = ;
//     unsafe { ENGINE }
// }

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn init() {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).unwrap();
        }
        else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new().unwrap();
    let mut window_orig = WindowBuilder::new().build(&event_loop).unwrap();
    let window = Arc::new(&mut window_orig);
    let orig_window_id = window.id();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::dpi::PhysicalSize;
        use winit::platform::web::WindowExtWebSys;
        // window.set_inner_size(PhysicalSize::new(800, 600));

        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("kyawthuite").unwrap();
                let canvas = web_sys::Element::from(window.canvas().unwrap());
                dst.append_child(&canvas).ok().unwrap();
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    let mut state = State::new(window).await;

    event_loop
        .run(move |event, window_target| {
            match event {
                Event::WindowEvent {
                    window_id,
                    event: ref win_event,
                } => {
                    if state.input(win_event) {
                        return;
                    }

                    match win_event {
                        WindowEvent::Resized(new_size) => {
                            state.resize(*new_size);
                        }
                        WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer: _ } => {
                            state.resize_scale(*scale_factor);
                        }
                        WindowEvent::CloseRequested => {
                            window_target.exit();
                        }
                        WindowEvent::RedrawRequested => {
                            if window_id != state.get_window().id() {
                                return;
                            }

                            state.update();

                            match state.render() {
                                Ok(_) => {}
                                Err(wgpu::SurfaceError::Lost) => {
                                    state.reconfigure();
                                }
                                Err(wgpu::SurfaceError::OutOfMemory) => {
                                    window_target.exit();
                                }
                                Err(err) => eprintln!("{:?}", err),
                            }
                        }
                        _ => {}
                    }
                }
                Event::AboutToWait => {
                    state.get_window().request_redraw();
                }
                _ => {}
            }
        })
        .unwrap();
}
