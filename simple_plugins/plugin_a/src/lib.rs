
extern crate interfaces;

use interfaces::PluginRegistration;
use interfaces::PluginCallbacks;
use interfaces::CallbackParameters;
use interfaces::PluginOperationError;

pub struct PluginA {}

impl PluginA {
    fn pa_pre_cb<T: CallbackParameters>(pb: &T) -> Result<(), PluginOperationError> {
        println!("Hello From plugin A!");
        println!("Plugin A: cb_test is {}", pb.cb_test());
        Ok(())
    }
}

impl PluginRegistration for PluginA {
    fn register(pcb: &mut PluginCallbacks) {
        pcb.pre_cb = Some(PluginA::pa_pre_cb);
    }
}

