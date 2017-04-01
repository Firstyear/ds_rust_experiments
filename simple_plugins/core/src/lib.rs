
extern crate plugina;
extern crate interfaces;

use interfaces::CallbackParameters;
use interfaces::PluginCallbacks;
use interfaces::PluginRegistration;

// This actually handles and runs plugins when required.
// The registration and removal is external

struct PluginCore {
}

impl PluginCore {
    // Create a new plugin core.
    pub fn new() -> Self {
        PluginCore{}
    }

}




#[cfg(test)]
mod tests {

    use interfaces::PluginCallbacks;
    use super::PluginCore;
    use interfaces::PluginRegistration;
    use plugina::PluginA;

    #[test]
    fn plugin_core() {
        let core = PluginCore::new();

        let mut pcb = PluginCallbacks::new();

        PluginA::register(&mut pcb);

    }
}
