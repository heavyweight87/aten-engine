use crate::window::WindowManager;

pub struct Engine {
    window: WindowManager,
}

impl Engine {
    /// Creates a new window in current thread using default settings.
    fn new() -> Self {
        Self {
            window: WindowManager::new(),
        }
    }

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let mut engine = Engine::new();
        engine.window.run()
    }
}