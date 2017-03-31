This is an example of the use of plugins defined by traits.

* core, the server that runs the plugins.
* plugin_a, a first plugin. (pre and post)
* plugin_b, a second plugin. (pre)
* interfaces, the module containing our traits and struct definitions

Note that plugin_b doesn't have a post plugin, so we can demonstrate how optional
functions work.

This doesn't handle anything related to dynamic libraries: plugins are built
into the core as dependencies.

The callbacks function types are bound by the plugin registration function: Rather
than trying to implement traits on the plugins.




