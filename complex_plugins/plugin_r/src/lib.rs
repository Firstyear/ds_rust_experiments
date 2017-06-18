extern crate library;

use library::Plugin;

#[no_mangle]
pub extern fn plugin_init(plugin: &mut Plugin) -> i32 {
    println!("plugin_r: init");
    plugin.register_pre_operation();
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
