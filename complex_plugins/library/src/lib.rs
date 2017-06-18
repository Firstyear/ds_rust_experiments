extern crate dylib;

use std::path::Path;
use dylib::DynamicLibrary;

pub enum PluginError {
    Dlopen,
    Symbol,
    Init,
}

pub struct Plugin {
    name: &'static str,
    priority: u64,
    sohandle: DynamicLibrary,
    // optional function pointers here.
}

pub struct PluginSystem;

// Needs to implement ordering for sorting.
impl Plugin {
    // init, create a ref to the dl
    // Shuold this actually return a result type?
    pub fn new(name: &'static str, sopath: &Path) -> Result<Self, PluginError> {
        // Need to take a name, version, path
        // need to stash the handle somehow?
        match DynamicLibrary::open(Some(sopath)) {
            Ok(handle) => {
                let mut plg = Plugin{
                    name: name,
                    priority: 0,
                    sohandle: handle,
                };
                let fptr = match unsafe { plg.sohandle.symbol("plugin_init") } {
                    // Now we get dangerous: We have to unsafe access
                    // the raw pointer of the function.
                    Ok(mfptr) => unsafe { std::mem::transmute::<*mut (), fn(&mut Plugin) -> i32>(mfptr) },
                    Err(_) => return Err(PluginError::Symbol),
                };
                let init_result = fptr(&mut plg);
                println!("plugin: {} init_result: {}", plg.name, init_result);
                if init_result == 0 {
                    Ok(plg)
                } else {
                    Err(PluginError::Init)
                }
            },
            Err(e) => Err(PluginError::Dlopen),
        }

    }

    // Call a fn on this plugin? Or do we just hold the fn pointers?
    #[no_mangle]
    pub fn register_pre_operation(&self) {
        // Do nothing ...
        println!("Pre op reg!");
    }
}

impl PluginSystem {
    // Need an init
    pub fn new() -> Self {
        let plgs = PluginSystem{};
        // for now this has to hardcode the .so locations, but this can change later ....
        // open
        let plg_r_path = Path::new("./.libs/libplugin_r.so");
        let mut plg_r = Plugin::new("plugin_r", plg_r_path);

        let plg_c_path = Path::new("./.libs/libplugin_c.so");
        let mut plg_c = Plugin::new("plugin_c", plg_c_path);

        plgs
    }

    // Add / remove plg?

    // jobs for plugs

    // do we need a destroy type?
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
