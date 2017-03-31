
// Plugins register their callbacks to this function
pub struct PluginCallbacks {}

// When a callback is made to a plugin function, it has a structure
// bound by this trait provided first as a gateway
// to our server api.
pub trait CallbackParameters {
    fn cb_test(&self) -> i64;
}

// Every trait that is able to be registered is bound by this
// type.
pub trait PluginRegistration {
    fn register(*mut PluginCallbacks);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
