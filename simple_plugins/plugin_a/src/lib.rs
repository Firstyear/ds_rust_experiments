
extern crate interfaces;

use interfaces::PluginRegistration;
use interfaces::PluginCallbacks;
use interfaces::CallbackParameters;

pub struct PluginA {}

impl PluginA {
    fn pa_pre_cb<T: CallbackParameters>(pb: &T) {
        println!("Hello From a plugin!");
    }
}

impl PluginRegistration for PluginA {
    fn register(pcb: &mut PluginCallbacks) {
        pcb.pre_cb = Some(PluginA::pa_pre_cb);
    }
}

