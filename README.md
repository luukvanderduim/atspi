#Atspi Rust bindings
[![Build Status](https://travis-ci.com/luukvanderduim/atspi.svg?branch=master)](https://travis-ci.com/luukvanderduim/atspi)

Gir generated Rust bindings for the atspi accessibility library

The accessibility library libatspi, part of AT-SPI2-CORE, is a C library for writing client programs to aid people in using their computers more conveniently.

Please note that these bindings are incomplete, though largely useable.

At time of writing 26-11-2020 (DDMMYYYY) the following roughly remains on the to-do list before publishing on Crates.io 

```bash
Gir generated raw bindings for the atspi accessibility library for Rust
[WARN  libgir::library_postprocessing] Field `MatchRule::roles` missing c:type assumed to be `fixed_array`
[WARN  libgir::analysis::functions] function `atspi_device_listener_new_simple`'s callback `callback` without associated user data
[WARN  libgir::analysis::functions] `atspi_device_listener_new_simple`: no user data point to the destroy callback
[WARN  libgir::analysis::functions] `atspi_device_listener_new_simple`: destructor without linked callback
[WARN  libgir::analysis::functions] function `atspi_device_listener_remove_callback`'s callback `callback` without associated user data
[WARN  libgir::analysis::functions] `remove_callback`: this is supposed to be a callback function but no callback was found...
[WARN  libgir::analysis::functions] function `atspi_event_listener_new_simple`'s callback `callback` without associated user data
[WARN  libgir::analysis::functions] `atspi_event_listener_new_simple`: no user data point to the destroy callback
[WARN  libgir::analysis::functions] `atspi_event_listener_new_simple`: destructor without linked callback
[WARN  libgir::analysis::functions] function `atspi_event_listener_deregister_no_data`'s callback `callback` without associated user data
[WARN  libgir::analysis::functions] `deregister_no_data`: this is supposed to be a callback function but no callback was found...
[WARN  libgir::analysis::functions] function `atspi_event_listener_register_no_data`'s callback `callback` without associated user data
[WARN  libgir::analysis::functions] `atspi_event_listener_register_no_data`: no user data point to the destroy callback
[WARN  libgir::analysis::functions] `atspi_event_listener_register_no_data`: destructor without linked callback
[NOT GENERATED METHOD] Atspi.Component::scroll_to because of Atspi.ScrollType
[NOT GENERATED METHOD] Atspi.Document::get_document_attributes because of *.HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }
[NOT GENERATED METHOD] Atspi.Text::get_attribute_run because of *.HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }
[NOT GENERATED METHOD] Atspi.Text::get_default_attributes because of *.HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }
[NOT GENERATED METHOD] Atspi.Text::get_text_attributes because of *.HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }
[NOT GENERATED METHOD] Atspi.Text::scroll_substring_to because of Atspi.ScrollType
[NOT GENERATED METHOD] Atspi.Accessible::get_attributes because of *.HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }
[NOT GENERATED METHOD] Atspi.MatchRule::new because of *.HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }
[NOT GENERATED] Atspi.ScrollType
[NOT GENERATED] Atspi.EventType
[NOT GENERATED] Atspi.EventListenerMode
[NOT GENERATED] Atspi.KeyEventType
[NOT GENERATED] Atspi.KeySet
[NOT GENERATED] Atspi.LocaleType
[NOT GENERATED] Atspi.ModifierType
[NOT GENERATED FUNCTION] Atspi.dbus_connection_setup_with_g_main because of Gio.DBusConnection
[NOT GENERATED FUNCTION] Atspi.dbus_server_setup_with_g_main because of Gio.DBusServer
[NOT GENERATED FUNCTION] Atspi.get_a11y_bus because of Gio.DBusConnection
```

