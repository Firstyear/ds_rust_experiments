
// Plugins register their callbacks to this function
pub struct PluginCallbacks {
    pub pre_cb: Option<fn(&CoreApi)>,
}

pub struct CoreApi {
    internal_test: i64,
}

impl CallbackParameters for CoreApi {
    fn cb_test(&self) -> i64 {
        self.internal_test
    }
}

impl PluginCallbacks {
    pub fn new() -> Self {
        PluginCallbacks{
            pre_cb: None
        }
    }
}

// When a callback is made to a plugin function, it has a structure
// bound by this trait provided first as a gateway
// to our server api.
pub trait CallbackParameters {
    fn cb_test(&self) -> i64;
}

// Every trait that is able to be registered is bound by this
// type.
pub trait PluginRegistration {
    fn register(&mut PluginCallbacks);
}

