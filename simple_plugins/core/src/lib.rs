
extern crate plugina;
extern crate pluginb;
extern crate interfaces;

use interfaces::CallbackParameters;
use interfaces::PluginCallbacks;
use interfaces::PluginRegistration;
use interfaces::PluginOperationError;
use interfaces::CoreApi;

// This actually handles and runs plugins when required.
// The registration and removal is external

struct PluginCore {
    plgs: Vec<PluginCallbacks>,
}

// This type actually does the plugin dispatch and operation.

impl PluginCore {
    // Create a new plugin core.
    pub fn new() -> Self {
        PluginCore{
            plgs: Vec::new(),
        }
    }

    // Given a types registration fn, we register it to our server.
    pub fn submit(&mut self, reg_func: fn(&mut PluginCallbacks)) {
        let mut pcb_a = PluginCallbacks::new();
        reg_func(&mut pcb_a);
        self.plgs.push(pcb_a);
    }

    pub fn extract(&mut self) {
    }

    pub fn pre(&self) {
        for plg in &self.plgs {
            // Build a new param, or this could be passed in. depends on situation.
            let core_api = CoreApi::new();
            let inner_result = plg.pre_cb.map_or(Ok(()), |f| f(&core_api) );
            // Now map on the result to determine what to do, or Ok().
        }
    }

    pub fn post(&self) {
        for plg in &self.plgs {
            // Build a new param, or this could be passed in. depends on situation.
            let core_api = CoreApi::new();
            let inner_result = plg.post_cb.map_or(Ok(()), |f| f(&core_api) );
            // Now map on the result to determine what to do, or Ok().
        }
    }

}




#[cfg(test)]
mod tests {

    use interfaces::PluginCallbacks;
    use interfaces::PluginRegistration;
    use super::PluginCore;
    use plugina::PluginA;
    use pluginb::PluginB;

    #[test]
    fn plugin_core() {
        let mut core = PluginCore::new();

        core.submit(PluginA::register);
        core.submit(PluginB::register);

        // Try a callback!

        core.pre();
        core.post();

    }
}
