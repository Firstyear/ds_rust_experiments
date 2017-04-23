

extern crate interfaces;

use interfaces::PluginOperationError;
use interfaces::PluginRegistration;
use interfaces::PluginCallbacks;
use interfaces::CallbackParameters;

pub struct PluginB {}

impl PluginB {
    fn pb_post_cb<T: CallbackParameters>(pb: &T) -> Result<(), PluginOperationError> {
        println!("Hello From plugin B!");
        println!("Plugin B: cb_test is {}", pb.cb_test());
        Ok(())
    }
}

impl PluginRegistration for PluginB {
    fn register(pcb: &mut PluginCallbacks) {
        pcb.post_cb = Some(PluginB::pb_post_cb);
    }
}

