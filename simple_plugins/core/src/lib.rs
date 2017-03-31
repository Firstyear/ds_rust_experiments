
extern crate interfaces;

use interfaces::CallbackParameters;
use interfaces::PluginCallbacks;
use interfaces::PluginRegistration;

struct CoreApi {
    internal_test: i64,
}

impl CallbackParameters for CoreApi {
    fn cb_test(&self) -> i64 {
        self.internal_test
    }
}

struct PluginCore {
}

impl PluginCore {
    // Create a new plugin core.
    fn new() -> Self {
        PluginCore{}
    }

    fn register<T: PluginRegistration>(plugin: T) {
        // Create a new set of PluginCallbacks
        // Call register on the 
    }
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
