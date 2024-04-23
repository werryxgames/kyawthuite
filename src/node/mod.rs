use winit::event::WindowEvent;

pub enum Notification {
    Quit,
}

pub trait BaseNode {
    fn get_parent(&self) -> &Option<*mut dyn BaseNode>;
    fn get_children(&self) -> &Vec<*mut dyn BaseNode>;
    fn add_child(&mut self, child: *mut dyn BaseNode);

    fn ready(&mut self) {}

    fn update(&mut self, _delta: f64) {}

    fn input(&mut self, _event: WindowEvent) -> bool {
        false
    }

    fn unhandled_input(&mut self, _event: WindowEvent) -> bool {
        false
    }

    fn render(&mut self) {}

    fn notification(&mut self, _notification: Notification) -> bool {
        false
    }

    fn unhandled_notification(&mut self, _notification: Notification) -> bool {
        false
    }
}

pub struct Node {
    parent: Option<*mut dyn BaseNode>,
    children: Vec<*mut dyn BaseNode>,
}

impl BaseNode for Node {
    fn get_parent(&self) -> &Option<*mut dyn BaseNode> {
        &self.parent
    }

    fn get_children(&self) -> &Vec<*mut dyn BaseNode> {
        &self.children
    }

    fn add_child(&mut self, child: *mut dyn BaseNode) {
        self.children.push(child);
    }
}

pub static mut ROOT: Option<*mut dyn BaseNode> = None;

pub fn get_root() -> *mut dyn BaseNode {
    if unsafe { ROOT.is_none() } {
        unsafe {
            ROOT = Some(&mut Node {
                parent: None,
                children: vec![],
            } as *mut Node);
        }
    }

    unsafe { ROOT.unwrap_unchecked() }
}

pub fn change_root(new_node: *mut dyn BaseNode) {
    unsafe {
        ROOT = Some(new_node);
    }
}
