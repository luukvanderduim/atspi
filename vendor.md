<!-- file * -->
<!-- struct Accessible -->


# Implements

[`AccessibleExt`](trait.AccessibleExt.html), [`ObjectExt`](trait.ObjectExt.html), [`ActionExt`](trait.ActionExt.html), [`CollectionExt`](trait.CollectionExt.html), [`ComponentExt`](trait.ComponentExt.html), [`DocumentExt`](trait.DocumentExt.html), [`EditableTextExt`](trait.EditableTextExt.html), [`HypertextExt`](trait.HypertextExt.html), [`ImageExt`](trait.ImageExt.html), [`SelectionExt`](trait.SelectionExt.html), [`TableExt`](trait.TableExt.html), [`TableCellExt`](trait.TableCellExt.html), [`TextExt`](trait.TextExt.html), [`ValueExt`](trait.ValueExt.html)
<!-- trait AccessibleExt -->
Trait containing all `Accessible` methods.

# Implementors

[`Accessible`](struct.Accessible.html)
<!-- trait AccessibleExt::fn clear_cache -->
Clears the cached information for the given accessible and all of its
descendants.
<!-- trait AccessibleExt::fn get_action -->
Gets the `Action` interface for an `Accessible`.

# Deprecated since 2.10

Use atspi_accessible_get_action_iface instead.

# Returns

a pointer to an `Action` interface
 instance, or NULL if `self` does not implement `Action`.
<!-- trait AccessibleExt::fn get_action_iface -->
Gets the `Action` interface for an `Accessible`.

# Returns

a pointer to an `Action` interface
 instance, or NULL if `self` does not implement `Action`.
<!-- trait AccessibleExt::fn get_application -->
Gets the containing `Application` for an object.

# Returns

the containing `Application` instance for
 this object.
<!-- trait AccessibleExt::fn get_atspi_version -->
Gets the AT-SPI IPC specification version supported by the application
pointed to by the `Accessible` object.
Only works on application root objects.

# Returns

a UTF-8 string indicating the AT-SPI version for the `Accessible` object or NULL on exception.
<!-- trait AccessibleExt::fn get_attributes -->
Gets the `AttributeSet` representing any assigned
name-value pair attributes or annotations for this object.
For typographic, textual, or textually-semantic attributes, see
atspi_text_get_attributes instead.

# Returns

The name-value-pair
attributes assigned to this object.
<!-- trait AccessibleExt::fn get_attributes_as_array -->
Gets a `glib::Array` representing any assigned
name-value pair attributes or annotations for this object.
For typographic, textual, or textually-semantic attributes, see
atspi_text_get_attributes_as_array instead.

# Returns

The name-value-pair
 attributes assigned to this object.
<!-- trait AccessibleExt::fn get_child_at_index -->
Gets the `Accessible` child of an `Accessible` object at a given index.
## `child_index`
a `long` indicating which child is specified.

# Returns

a pointer to the `Accessible` child object at
index `child_index` or NULL on exception.
<!-- trait AccessibleExt::fn get_child_count -->
Gets the number of children contained by an `Accessible` object.

# Returns

a `long` indicating the number of `Accessible` children
 contained by an `Accessible` object or -1 on exception.
<!-- trait AccessibleExt::fn get_collection -->
Gets the `Collection` interface for an `Accessible`.

# Deprecated since 2.10

Use atspi_accessible_get_collection_iface instead.

# Returns

a pointer to an `Collection` interface
 instance, or NULL if `self` does not implement `Collection`.
<!-- trait AccessibleExt::fn get_collection_iface -->
Gets the `Collection` interface for an `Accessible`.

# Returns

a pointer to an `Collection` interface
 instance, or NULL if `self` does not implement `Collection`.
<!-- trait AccessibleExt::fn get_component -->
Gets the `Component` interface for an `Accessible`.

# Deprecated since 2.10

Use atspi_accessible_get_component_iface instead.

# Returns

a pointer to an `Component` interface
 instance, or NULL if `self` does not implement `Component`.
<!-- trait AccessibleExt::fn get_component_iface -->
Gets the `Component` interface for an `Accessible`.

# Returns

a pointer to an `Component` interface
 instance, or NULL if `self` does not implement `Component`.
<!-- trait AccessibleExt::fn get_description -->
Gets the description of an `Accessible` object.

# Returns

a UTF-8 string describing the `Accessible` object
or NULL on exception.
<!-- trait AccessibleExt::fn get_document -->
Gets the `Document` interface for an `Accessible`.

# Deprecated since 2.10

Use atspi_accessible_get_document_iface instead.

# Returns

a pointer to an `Document` interface
 instance, or NULL if `self` does not implement `Document`.
<!-- trait AccessibleExt::fn get_document_iface -->
Gets the `Document` interface for an `Accessible`.

# Returns

a pointer to an `Document` interface
 instance, or NULL if `self` does not implement `Document`.
<!-- trait AccessibleExt::fn get_editable_text -->
Gets the `EditableText` interface for an `Accessible`.

# Deprecated since 2.10

Use atspi_accessible_get_editable_text_iface instead.

# Returns

a pointer to an `EditableText` interface
 instance, or NULL if `self` does not implement `EditableText`.
<!-- trait AccessibleExt::fn get_editable_text_iface -->
Gets the `EditableText` interface for an `Accessible`.

# Returns

a pointer to an `EditableText` interface
 instance, or NULL if `self` does not implement `EditableText`.
<!-- trait AccessibleExt::fn get_hyperlink -->
Gets the `Hyperlink` interface for an `Accessible`.

# Returns

the `Hyperlink` object associated with
 the given `Accessible`, or NULL if not supported.
<!-- trait AccessibleExt::fn get_hypertext -->
Gets the `Hypertext` interface for an `Accessible`.

# Deprecated since 2.10

Use atspi_accessible_get_hypertext_iface instead.

# Returns

a pointer to an `Hypertext` interface
 instance, or NULL if `self` does not implement `Hypertext`.
<!-- trait AccessibleExt::fn get_hypertext_iface -->
Gets the `Hypertext` interface for an `Accessible`.

# Returns

a pointer to an `Hypertext` interface
 instance, or NULL if `self` does not implement `Hypertext`.
<!-- trait AccessibleExt::fn get_id -->
Gets the application id for a `Accessible` object.
Only works on application root objects.

# Returns

a positive `gint` indicating the id for the `Accessible` object
or -1 on exception.
<!-- trait AccessibleExt::fn get_image -->
Gets the `Image` interface for an `Accessible`.

# Deprecated since 2.10

Use atspi_accessible_get_image_iface instead.

# Returns

a pointer to an `Image` interface instance, or
 NULL if `self` does not implement `Image`.
<!-- trait AccessibleExt::fn get_image_iface -->
Gets the `Image` interface for an `Accessible`.

# Returns

a pointer to an `Image` interface instance, or
 NULL if `self` does not implement `Image`.
<!-- trait AccessibleExt::fn get_index_in_parent -->
Gets the index of an `Accessible` object within its parent's
`Accessible` children list.

# Returns

a `glong` indicating the index of the `Accessible` object
 in its parent,
 or -1 if `self` has no containing parent or on exception.
<!-- trait AccessibleExt::fn get_interfaces -->
A set of pointers to all interfaces supported by an `Accessible`.

# Returns

A `glib::Array` of strings
 describing the interfaces supported by the object. Interfaces are
 denoted in short-hand (i.e. "Component", "Text" etc.).
<!-- trait AccessibleExt::fn get_localized_role_name -->
Gets a UTF-8 string corresponding to the name of the role played by an
object, translated to the current locale.
This method will return useful values for roles that fall outside the
enumeration used in atspi_accessible_getRole ().

# Returns

a localized, UTF-8 string specifying the type of UI role played
by an `Accessible` object.
<!-- trait AccessibleExt::fn get_name -->
Gets the name of an `Accessible` object.

# Returns

a UTF-8 string indicating the name of the `Accessible` object
or NULL on exception.
<!-- trait AccessibleExt::fn get_parent -->
Gets an `Accessible` object's parent container.

# Returns

a pointer to the
 `Accessible` object which contains the given
 `Accessible` instance, or NULL if the `self` has no
 parent container.
<!-- trait AccessibleExt::fn get_process_id -->
Returns the process id associated with the given accessible. Mainly
added for debugging; it is a shortcut to explicitly querying the
accessible's app->bus_name and then calling GetConnectionUnixProcessID.

# Returns

The process ID or undetermined value if `error` is set.
<!-- trait AccessibleExt::fn get_relation_set -->
Gets the set of `Relation` objects which describes this `Accessible` object's
relationships with other `Accessible` objects.

# Returns

a `glib::Array` of
 `Relation` pointers or NULL on exception.
<!-- trait AccessibleExt::fn get_role -->
Gets the UI role played by an `Accessible` object.
This role's name can be obtained via atspi_accessible_get_role_name ().

# Returns

the `Role` of an `Accessible` object.
<!-- trait AccessibleExt::fn get_role_name -->
Gets a UTF-8 string corresponding to the name of the role played by an object.
This method will return useful values for roles that fall outside the
enumeration used in atspi_accessible_get_role ().

# Returns

a UTF-8 string specifying the type of UI role played by an
`Accessible` object.
<!-- trait AccessibleExt::fn get_selection -->
Gets the `Selection` interface for an `Accessible`.

# Deprecated since 2.10

Use atspi_accessible_get_selection_iface instead.

# Returns

a pointer to an `Selection` interface
 instance, or NULL if `self` does not implement `Selection`.
<!-- trait AccessibleExt::fn get_selection_iface -->
Gets the `Selection` interface for an `Accessible`.

# Returns

a pointer to an `Selection` interface
 instance, or NULL if `self` does not implement `Selection`.
<!-- trait AccessibleExt::fn get_state_set -->
Gets the states currently held by an object.

# Returns

a pointer to an `StateSet` representing an
object's current state set.
<!-- trait AccessibleExt::fn get_table -->
Gets the `Table` interface for an `Accessible`.

# Deprecated since 2.10

Use atspi_accessible_get_table_iface instead.

# Returns

a pointer to an `Table` interface instance, or
 NULL if `self` does not implement `Table`.
<!-- trait AccessibleExt::fn get_table_cell -->
Gets the `TableCell` interface for an `Accessible`.

# Returns

a pointer to an `TableCell` interface instance,
 or NULL if `self` does not implement `Table`.
<!-- trait AccessibleExt::fn get_table_iface -->
Gets the `Table` interface for an `Accessible`.

# Returns

a pointer to an `Table` interface instance, or
 NULL if `self` does not implement `Table`.
<!-- trait AccessibleExt::fn get_text -->
Gets the `Table` interface for an `Accessible`.

# Deprecated since 2.10

Use atspi_accessible_get_text_iface instead.

# Returns

a pointer to an `Text` interface instance, or
 NULL if `self` does not implement `Text`.
<!-- trait AccessibleExt::fn get_text_iface -->
Gets the `Table` interface for an `Accessible`.

# Returns

a pointer to an `Text` interface instance, or
 NULL if `self` does not implement `Text`.
<!-- trait AccessibleExt::fn get_toolkit_name -->
Gets the toolkit name for an `Accessible` object.
Only works on application root objects.

# Returns

a UTF-8 string indicating the toolkit name for the `Accessible` object or NULL on exception.
<!-- trait AccessibleExt::fn get_toolkit_version -->
Gets the toolkit version for an `Accessible` object.
Only works on application root objects.

# Returns

a UTF-8 string indicating the toolkit version for the `Accessible` object or NULL on exception.
<!-- trait AccessibleExt::fn get_value -->
Gets the `Table` interface for an `Accessible`.

# Deprecated since 2.10

Use atspi_accessible_get_value_iface instead.

# Returns

a pointer to an `Value` interface instance, or
 NULL if `self` does not implement `Value`.
<!-- trait AccessibleExt::fn get_value_iface -->
Gets the `Table` interface for an `Accessible`.

# Returns

a pointer to an `Value` interface instance, or
 NULL if `self` does not implement `Value`.
<!-- trait AccessibleExt::fn set_cache_mask -->
Sets the type of data to cache for accessibles.
If this is not set for an application or is reset to ATSPI_CACHE_UNDEFINED,
then the desktop's cache flag will be used.
If the desktop's cache flag is also undefined, then all possible data will
be cached.
This function is intended to work around bugs in toolkits where the proper
events are not raised / to aid in testing for such bugs.
## `mask`
An `Cache` specifying a bit mask of the types of data to cache.
<!-- struct Action -->


# Implements

[`ActionExt`](trait.ActionExt.html)
<!-- trait ActionExt -->
Trait containing all `Action` methods.

# Implementors

[`Accessible`](struct.Accessible.html), [`Action`](struct.Action.html)
<!-- trait ActionExt::fn do_action -->
Invoke the action indicated by `index`.
## `i`
an integer specifying which action to invoke.

# Returns

`true` if the action is successfully invoked, otherwise `false`.
<!-- trait ActionExt::fn get_action_description -->
Get the description of '`i`-th' action invocable on an
 object implementing `Action`.
## `i`
an integer indicating which action to query.

# Returns

a UTF-8 string describing the '`i`-th' invocable action.
<!-- trait ActionExt::fn get_action_name -->
Get the name of the '`i`-th' action invocable on an
 object implementing `Action`.
## `i`
an integer indicating which action to query.

# Returns

the non-localized name of the action, as a UTF-8 string.
<!-- trait ActionExt::fn get_description -->
Get the description of '`i`-th' action invocable on an
 object implementing `Action`.

# Deprecated since 2.10

Use atspi_action_get_action_description instead.
## `i`
an integer indicating which action to query.

# Returns

a UTF-8 string describing the '`i`-th' invocable action.
<!-- trait ActionExt::fn get_key_binding -->
Get the keybindings for the `i`-th action invocable on an
 object implementing `Action`, if any are defined.
 The keybindings string format is as follows:
 there are multiple parts to a keybinding string (typically 3).
 They are delimited with ";". The first is the action's
 keybinding which is usable if the object implementing the action
 is currently posted to the screen, e.g. if a menu is posted
 then these keybindings for the corresponding menu-items are
 available. The second keybinding substring is the full key sequence
 necessary to post the action's widget and activate it, e.g. for
 a menu item such as "File->Open" it would both post the menu and
 activate the item. Thus the second keybinding string is available
 during the lifetime of the containing toplevel window as a whole,
 whereas the first keybinding string only works while the object
 implementing AtkAction is posted. The third (and optional)
 keybinding string is the "keyboard shortcut" which invokes the
 action without posting any menus.
 Meta-keys are indicated by the conventional strings
 "&lt;Control&gt;", "&lt;Alt&gt;", "&lt;Shift&gt;", "&lt;Mod2&gt;",
 etc. (we use the same string as `gtk_accelerator_name` in
 gtk+-2.X.
## `i`
an integer indicating which action to query.

# Returns

a UTF-8 string which can be parsed to determine the `i`-th
 invocable action's keybindings.
<!-- trait ActionExt::fn get_localized_name -->
Get the name of the '`i`-th' action invocable on an
 object implementing `Action`.
## `i`
an integer indicating which action to query.

# Returns

the name of the action, as a UTF-8 string.
<!-- trait ActionExt::fn get_n_actions -->
Get the number of actions invokable on an `Action` implementor.

# Returns

an integer indicating the number of invocable actions.
<!-- trait ActionExt::fn get_name -->
Get the name of the '`i`-th' action invocable on an
 object implementing `Action`.

# Deprecated since 2.10

Use atspi_action_get_action_name instead.
## `i`
an integer indicating which action to query.

# Returns

the non-localized name of the action, as a UTF-8 string.
<!-- struct Collection -->


# Implements

[`CollectionExt`](trait.CollectionExt.html)
<!-- trait CollectionExt -->
Trait containing all `Collection` methods.

# Implementors

[`Accessible`](struct.Accessible.html), [`Collection`](struct.Collection.html)
<!-- trait CollectionExt::fn get_active_descendant -->

# Returns

The active descendant of the given object.
Not yet implemented.
<!-- trait CollectionExt::fn get_matches -->
Gets all `Accessible` objects from the `self` matching a given
`rule`.
## `rule`
An `MatchRule` describing the match criteria.
## `sortby`
An `CollectionSortOrder` specifying the way the results are to
 be sorted.
## `count`
The maximum number of results to return, or 0 for no limit.
## `traverse`
Not supported.

# Returns

All
 `Accessible` objects matching the given match rule.
<!-- trait CollectionExt::fn get_matches_from -->
Gets all `Accessible` objects from the `self`, before
`current_object`, matching a given `rule`.
## `current_object`
Upon reaching this object, searching should stop.
## `rule`
An `MatchRule` describing the match criteria.
## `sortby`
An `CollectionSortOrder` specifying the way the results are to
 be sorted.
## `tree`
An `CollectionTreeTraversalType` specifying restrictions on
 the objects to be traversed.
## `count`
The maximum number of results to return, or 0 for no limit.
## `traverse`
Not supported.

# Returns

All
 `Accessible` objects matching the given match rule that preceed
 `current_object`.
<!-- trait CollectionExt::fn get_matches_to -->
Gets all `Accessible` objects from the `self`, after
`current_object`, matching a given `rule`.
## `current_object`
The object at which to start searching.
## `rule`
An `MatchRule` describing the match criteria.
## `sortby`
An `CollectionSortOrder` specifying the way the results are to
 be sorted.
## `tree`
An `CollectionTreeTraversalType` specifying restrictions on
 the objects to be traversed.
## `limit_scope`
If `true`, only descendants of `current_object`'s parent
 will be returned. Otherwise (if `false`), any accessible may be
 returned if it would preceed `current_object` in a flattened
 hierarchy.
## `count`
The maximum number of results to return, or 0 for no limit.
## `traverse`
Not supported.

# Returns

All
 `Accessible` objects matching the given match rule after
 `current_object`.
<!-- trait CollectionExt::fn is_ancestor_of -->
Not yet implemented.
<!-- enum CollectionMatchType -->
Enumeration used by `MatchRule` to specify
how to interpret `Accessible` objects.
<!-- enum CollectionMatchType::variant Invalid -->
Indicates an error condition or
uninitialized value.
<!-- enum CollectionMatchType::variant All -->
`true` if all of the criteria are met.
<!-- enum CollectionMatchType::variant Any -->
`true` if any of the criteria are met.
<!-- enum CollectionMatchType::variant None -->
`true` if none of the criteria are met.
<!-- enum CollectionMatchType::variant Empty -->
Same as `CollectionMatchType::All` if
the criteria is non-empty; for empty criteria this rule requires returned
value to also have empty set.
<!-- enum CollectionMatchType::variant LastDefined -->
Used only to determine the end of the
enumeration.
<!-- enum CollectionSortOrder -->
Enumeration used by interface `Collection` to specify
the way `AtspiAccesible` objects should be sorted.
<!-- enum CollectionTreeTraversalType -->
Enumeration used by interface `Collection` to specify
restrictions on `AtspiAccesible` objects to be traversed.
<!-- struct Component -->


# Implements

[`ComponentExt`](trait.ComponentExt.html)
<!-- trait ComponentExt -->
Trait containing all `Component` methods.

# Implementors

[`Accessible`](struct.Accessible.html), [`Component`](struct.Component.html)
<!-- trait ComponentExt::fn contains -->
Queries whether a given `Component` contains a particular point.
## `x`
a `gint` specifying the x coordinate in question.
## `y`
a `gint` specifying the y coordinate in question.
## `ctype`
the desired coordinate system of the point (`x`, `y`)
 (e.g. CSPI_COORD_TYPE_WINDOW, CSPI_COORD_TYPE_SCREEN).

# Returns

`true` if the specified component contains the point (`x`, `y`),
 `false` otherwise.
<!-- trait ComponentExt::fn get_accessible_at_point -->
Gets the accessible child at a given coordinate within an `Component`.
## `x`
a `gint` specifying the x coordinate of the point in question.
## `y`
a `gint` specifying the y coordinate of the point in question.
## `ctype`
the coordinate system of the point (`x`, `y`)
 (e.g. ATSPI_COORD_TYPE_WINDOW, ATSPI_COORD_TYPE_SCREEN).

# Returns

a pointer to an
 `Accessible` child of the specified component which
 contains the point (`x`, `y`), or NULL if no child contains
 the point.
<!-- trait ComponentExt::fn get_alpha -->
Gets the opacity/alpha value of a component, if alpha blending is in use.

# Returns

the opacity value of a component, as a `gdouble` between 0.0 and 1.0.
<!-- trait ComponentExt::fn get_extents -->
Gets the bounding box of the specified `Component`.
## `ctype`
the desired coordinate system into which to return the results,
 (e.g. ATSPI_COORD_TYPE_WINDOW, ATSPI_COORD_TYPE_SCREEN).

# Returns

An `Rect` giving the accessible's extents.
<!-- trait ComponentExt::fn get_layer -->
Queries which layer the component is painted into, to help determine its
 visibility in terms of stacking order.

# Returns

the `ComponentLayer` into which this component is painted.
<!-- trait ComponentExt::fn get_mdi_z_order -->
Queries the z stacking order of a component which is in the MDI or window
 layer. (Bigger z-order numbers mean nearer the top)

# Returns

a `gshort` indicating the stacking order of the component
 in the MDI layer, or -1 if the component is not in the MDI layer.
<!-- trait ComponentExt::fn get_position -->
Gets the minimum x and y coordinates of the specified `Component`.
## `ctype`
the desired coordinate system into which to return the results,
 (e.g. ATSPI_COORD_TYPE_WINDOW, ATSPI_COORD_TYPE_SCREEN).

# Returns

An `Point` giving the `self`'s position.
<!-- trait ComponentExt::fn get_size -->
Gets the size of the specified `Component`.

# Returns

An `Point` giving the `self`'s size.
<!-- trait ComponentExt::fn grab_focus -->
Attempts to set the keyboard input focus to the specified
 `Component`.

# Returns

`true` if successful, `false` otherwise.
<!-- trait ComponentExt::fn set_extents -->
Moves and resizes the specified component.
## `x`
the new vertical position to which the component should be moved.
## `y`
the new horizontal position to which the component should be moved.
## `width`
the width to which the component should be resized.
## `height`
the height to which the component should be resized.
## `ctype`
the coordinate system in which the position is specified.
 (e.g. ATSPI_COORD_TYPE_WINDOW, ATSPI_COORD_TYPE_SCREEN).

# Returns

`true` if successful; `false` otherwise.
<!-- trait ComponentExt::fn set_position -->
Moves the component to the specified position.
## `x`
the new vertical position to which the component should be moved.
## `y`
the new horizontal position to which the component should be moved.
## `ctype`
the coordinate system in which the position is specified.
 (e.g. ATSPI_COORD_TYPE_WINDOW, ATSPI_COORD_TYPE_SCREEN).

# Returns

`true` if successful; `false` otherwise.
<!-- trait ComponentExt::fn set_size -->
Resizes the specified component to the given coordinates.
## `width`
the width to which the component should be resized.
## `height`
the height to which the component should be resized.

# Returns

`true` if successful; `false` otherwise.
<!-- enum ComponentLayer -->
The `ComponentLayer` of an `Component` instance indicates its
relative stacking order with respect to the onscreen visual representation
of the UI. `ComponentLayer`, in combination with `Component` bounds
information, can be used to compute the visibility of all or part of a
component. This is important in programmatic determination of
region-of-interest for magnification, and in
flat screen review models of the screen, as well as
for other uses. Objects residing in two of the `ComponentLayer`
categories support further z-ordering information, with respect to their
peers in the same layer: namely, `ComponentLayer::Window` and
`ComponentLayer::Mdi`. Relative stacking order for other objects within the
same layer is not available; the recommended heuristic is
first child paints first. In other words, assume that the
first siblings in the child list are subject to being overpainted by later
siblings if their bounds intersect. The order of layers, from bottom to top,
 is: `ComponentLayer::Background`, `ComponentLayer::Window`, `ComponentLayer::Mdi`,
`ComponentLayer::Canvas`, `ComponentLayer::Widget`, `ComponentLayer::Popup`, and
`ComponentLayer::Overlay`.
<!-- enum ComponentLayer::variant Invalid -->
Indicates an error condition or uninitialized value.
<!-- enum ComponentLayer::variant Background -->
The bottom-most layer, over which everything else
is painted. The 'desktop background' is generally in this layer.
<!-- enum ComponentLayer::variant Canvas -->
The 'background' layer for most content renderers and
UI `Component` containers.
<!-- enum ComponentLayer::variant Widget -->
The layer in which the majority of ordinary
'foreground' widgets reside.
<!-- enum ComponentLayer::variant Mdi -->
A special layer between `ComponentLayer::Canvas` and
`ComponentLayer::Widget`, in which the 'pseudo windows' (e.g. the MDI frames)
reside. See `Component::get_mdi_z_order`.
<!-- enum ComponentLayer::variant Popup -->
A layer for popup window content, above
`ComponentLayer::Widget`.
<!-- enum ComponentLayer::variant Overlay -->
The topmost layer.
<!-- enum ComponentLayer::variant Window -->
The layer in which a toplevel window background usually
resides.
<!-- enum ComponentLayer::variant LastDefined -->
Used only to determine the end of the
enumeration.
<!-- enum CoordType -->
Enumeration used by `Component`, `Image`, and `Text` interfaces
to specify whether coordinates are relative to the window or the screen.
<!-- enum CoordType::variant Screen -->
Specifies xy coordinates relative to the screen.
<!-- enum CoordType::variant Window -->
Specifies xy coordinates relative to the widget's
top-level window.
<!-- struct DeviceEvent -->
<!-- struct DeviceListener -->


# Implements

[`DeviceListenerExt`](trait.DeviceListenerExt.html)
<!-- trait DeviceListenerExt -->
Trait containing all `DeviceListener` methods.

# Implementors

[`DeviceListener`](struct.DeviceListener.html)
<!-- impl DeviceListener::fn new -->
Creates a new `DeviceListener` with a specified callback function.
## `callback`
an `AtspiDeviceListenerCB` callback function,
 or NULL.
## `user_data`
a pointer to data which will be passed to the
callback when invoked.
## `callback_destroyed`
A `GDestroyNotify` called when the listener is freed
and data associated with the callback should be freed. It can be NULL.

# Returns

a pointer to a newly-created `DeviceListener`.
<!-- impl DeviceListener::fn new_simple -->
Creates a new `DeviceListener` with a specified callback function.
This method is similar to `DeviceListener::new`, but callback
takes no user data.
## `callback`
an `AtspiDeviceListenerCB` callback function,
 or NULL.
## `callback_destroyed`
A `GDestroyNotify` called when the listener is freed
and data associated with the callback should be freed. It an be NULL.

# Returns

a pointer to a newly-created `DeviceListener`.
<!-- trait DeviceListenerExt::fn add_callback -->
Adds an in-process callback function to an existing `DeviceListener`.
## `callback`
an `AtspiDeviceListenerCB` function pointer.
## `callback_destroyed`
A `GDestroyNotify` called when the listener is freed
and data associated with the callback should be freed. It can be NULL.
## `user_data`
a pointer to data which will be passed to the
 callback when invoked.
<!-- trait DeviceListenerExt::fn remove_callback -->
Removes an in-process callback function from an existing
`DeviceListener`.
## `callback`
an `AtspiDeviceListenerCB` function pointer.
<!-- struct Document -->


# Implements

[`DocumentExt`](trait.DocumentExt.html)
<!-- trait DocumentExt -->
Trait containing all `Document` methods.

# Implementors

[`Accessible`](struct.Accessible.html), [`Document`](struct.Document.html)
<!-- trait DocumentExt::fn get_attribute_value -->
Gets the value of a single attribute, if specified for the document as a whole.

# Deprecated since 2.10

Use atspi_document_get_document_attribute_value instead.
## `attribute`
a string indicating the name of a specific attribute.

# Returns

a string corresponding to the value of the specified attribute, or
an empty string if the attribute is unspecified for the object.
<!-- trait DocumentExt::fn get_attributes -->
Gets all constant attributes for the document as a whole. For attributes
that change within the document content, see `Text::get_attribute_run` instead.

# Deprecated since 2.10

Use atspi_document_get_document_attributes instead.

# Returns

a `glib::HashTable`
 containing the constant attributes of the document, as name-value pairs.
<!-- trait DocumentExt::fn get_current_page_number -->
Gets the current page number of an `AccessibleDocument` object.

# Returns

a `gint` indicating the current page number in the
`AccessibleDocument` object.
<!-- trait DocumentExt::fn get_document_attribute_value -->
Gets the value of a single attribute, if specified for the document as a whole.
## `attribute`
a string indicating the name of a specific attribute.

# Returns

a string corresponding to the value of the specified attribute, or
an empty string if the attribute is unspecified for the object.
<!-- trait DocumentExt::fn get_document_attributes -->
Gets all constant attributes for the document as a whole. For attributes
that change within the document content, see `Text::get_attribute_run` instead.

# Returns

a `glib::HashTable`
 containing the constant attributes of the document, as name-value pairs.
<!-- trait DocumentExt::fn get_locale -->
Gets the locale associated with the document's content,
e.g. the locale for LOCALE_TYPE_MESSAGES.

# Returns

a string compliant with the POSIX standard for locale description.
<!-- trait DocumentExt::fn get_page_count -->
Gets the page count of an `AccessibleDocument` object.

# Returns

a `gint` indicating the page count of an
`AccessibleDocument` object.
<!-- struct EditableText -->


# Implements

[`EditableTextExt`](trait.EditableTextExt.html)
<!-- trait EditableTextExt -->
Trait containing all `EditableText` methods.

# Implementors

[`Accessible`](struct.Accessible.html), [`EditableText`](struct.EditableText.html)
<!-- trait EditableTextExt::fn copy_text -->
Copies text from an `EditableText` object into the system clipboard.

see: `EditableText::paste_text`
## `start_pos`
a `gint` indicating the starting character offset
 of the text to copy.
## `end_pos`
a `gint` indicating the offset of the first character
 past the end of the text section to be copied.

# Returns

`true` if the operation was successful, otherwise `false`.
<!-- trait EditableTextExt::fn cut_text -->
Deletes text from an `EditableText` object, copying the
 excised portion into the system clipboard.

see: `EditableText::paste_text`
## `start_pos`
a `gint` indicating the starting character offset
 of the text to cut.
## `end_pos`
a `gint` indicating the offset of the first character
 past the end of the text section to be cut.

# Returns

`true` if operation was successful, `false` otherwise.
<!-- trait EditableTextExt::fn delete_text -->
Deletes text from an `EditableText` object, without copying the
 excised portion into the system clipboard.

see: `EditableText::cut_text`
## `start_pos`
a `gint` indicating the starting character offset
 of the text to delete.
## `end_pos`
a `gint` indicating the offset of the first character
 past the end of the text section to be deleted.

# Returns

`true` if the operation was successful, otherwise `false`.
<!-- trait EditableTextExt::fn insert_text -->
Inserts text into an `EditableText` object.
As with all character offsets, the specified `position` may not be the
same as the resulting byte offset, since the text is in a
variable-width encoding.
## `position`
a `gint` indicating the character offset at which to insert
 the new text.
## `text`
a string representing the text to insert, in UTF-8 encoding.
## `length`
the number of characters of text to insert. If the character
count of text is less than or equal to length, the entire contents
of text will be inserted.

# Returns

`true` if the operation was successful, otherwise `false`.
<!-- trait EditableTextExt::fn paste_text -->
Inserts text from the system clipboard into an `EditableText` object.
As with all character offsets, the specified `position` may not be the
 same as the resulting byte offset, since the text is in a
 variable-width encoding.
## `position`
a `gint` indicating the character offset at which to insert
 the new text.

# Returns

`true` if the operation was successful, otherwise `false`.
<!-- trait EditableTextExt::fn set_text_contents -->
Replace the entire text contents of an `EditableText` object.
## `new_contents`
a character string, encoded in UTF-8, which is to
 become the new text contents of the `EditableText` object.

# Returns

`true` if the operation was successful, otherwise `false`.
<!-- struct Event -->
<!-- impl Event::fn main -->
Starts/enters the main event loop for the AT-SPI services.

NOTE: This method does not return control; it is exited via a call to
`Event::quit` from within an event handler.
<!-- impl Event::fn quit -->
Quits the last main event loop for the AT-SPI services,
See: `Event::main`
<!-- struct Hyperlink -->


# Implements

[`HyperlinkExt`](trait.HyperlinkExt.html), [`ObjectExt`](trait.ObjectExt.html)
<!-- trait HyperlinkExt -->
Trait containing all `Hyperlink` methods.

# Implementors

[`Hyperlink`](struct.Hyperlink.html)
<!-- trait HyperlinkExt::fn get_end_index -->
Gets the ending character offset of the text range associated with
 an `Hyperlink`, in its originating `Hypertext`.
<!-- trait HyperlinkExt::fn get_index_range -->
Gets the starting and ending character offsets of the text range
associated with an `Hyperlink`, in its originating `Hypertext`.
<!-- trait HyperlinkExt::fn get_n_anchors -->
Gets the total number of anchors which an `Hyperlink` implementor has.
Though typical hyperlinks have only one anchor, client-side image maps and
other hypertext objects may potentially activate or refer to multiple
URIs. For each anchor there is a corresponding URI and object.

see: `HyperlinkExt::get_uri` and `HyperlinkExt::get_object`.

# Returns

a `gint` indicating the number of anchors in this hyperlink.
<!-- trait HyperlinkExt::fn get_object -->
Gets the object associated with a particular hyperlink anchor, as an
`Accessible`.
## `i`
a (zero-index) `gint` indicating which hyperlink anchor to query.

# Returns

an `Accessible` that represents the object
 associated with the `ith` anchor of the specified `Hyperlink`.
<!-- trait HyperlinkExt::fn get_start_index -->
Gets the starting character offset of the text range associated with
 an `Hyperlink`, in its originating `Hypertext`.
<!-- trait HyperlinkExt::fn get_uri -->
Gets the URI associated with a particular hyperlink anchor.
## `i`
a (zero-index) integer indicating which hyperlink anchor to query.

# Returns

a UTF-8 string giving the URI of the `ith` hyperlink anchor.
<!-- trait HyperlinkExt::fn is_valid -->
Tells whether an `Hyperlink` object is still valid with respect to its
 originating hypertext object.

# Returns

`true` if the specified `Hyperlink` is still valid with respect
 to its originating `Hypertext` object, `false` otherwise.
<!-- struct Hypertext -->


# Implements

[`HypertextExt`](trait.HypertextExt.html)
<!-- trait HypertextExt -->
Trait containing all `Hypertext` methods.

# Implementors

[`Accessible`](struct.Accessible.html), [`Hypertext`](struct.Hypertext.html)
<!-- trait HypertextExt::fn get_link -->
Gets the `Hyperlink` object at a specified index.
## `link_index`
a (zero-index) `gint` indicating which hyperlink to query.

# Returns

the `Hyperlink` object
 specified by `link_index`.
<!-- trait HypertextExt::fn get_link_index -->
Gets the index of the `Hyperlink` object at a specified
 character offset.
## `character_offset`
a `gint` specifying the character offset to query.

# Returns

the linkIndex of the `Hyperlink` active at
 character offset `character_offset`, or -1 if there is
 no hyperlink at the specified character offset.
<!-- trait HypertextExt::fn get_n_links -->
Gets the total number of `Hyperlink` objects that an
`Hypertext` implementor has.

# Returns

a `gint` indicating the number of `Hyperlink` objects
 of the `Hypertext` implementor, or -1 if
 the number cannot be determined (for example, if the
 `Hypertext` object is so large that it is not
 all currently in the memory cache).
<!-- struct Image -->


# Implements

[`ImageExt`](trait.ImageExt.html)
<!-- trait ImageExt -->
Trait containing all `Image` methods.

# Implementors

[`Accessible`](struct.Accessible.html), [`Image`](struct.Image.html)
<!-- trait ImageExt::fn get_image_description -->
Gets the description of the image displayed in an `Image` object.

# Returns

a UTF-8 string describing the image.
<!-- trait ImageExt::fn get_image_extents -->
Gets the bounding box of the image displayed in a
 specified `Image` implementor.
## `ctype`
the desired coordinate system into which to return the results,
 (e.g. ATSPI_COORD_TYPE_WINDOW, ATSPI_COORD_TYPE_SCREEN).

# Returns

a pointer to an `Rect` corresponding to the image's bounding box. The minimum x and y coordinates,
width, and height are specified.
<!-- trait ImageExt::fn get_image_locale -->
Gets the locale associated with an image and its textual representation.

# Returns

A POSIX LC_MESSAGES-style locale value for image description and text.
<!-- trait ImageExt::fn get_image_position -->
Gets the minimum x and y coordinates of the image displayed in a
 specified `Image` implementor.
## `ctype`
the desired coordinate system into which to return the results,
 (e.g. ATSPI_COORD_TYPE_WINDOW, ATSPI_COORD_TYPE_SCREEN).

# Returns

a pointer to an `Point` where x and y correspond to the
minimum coordinates of the displayed image.
<!-- trait ImageExt::fn get_image_size -->
Gets the size of the image displayed in a specified `Image` object.

# Returns

a pointer to an `Point` where x corresponds to
the image's width and y corresponds to the image's height.
<!-- struct KeyDefinition -->
<!-- enum KeySynthType -->
Enumeration used when synthesizing keyboard input via
`atspi_generate_keyboard_event`.
<!-- enum KeySynthType::variant Press -->
Emulates the pressing of a hardware keyboard key.
<!-- enum KeySynthType::variant Release -->
Emulates the release of a hardware keyboard key.
<!-- enum KeySynthType::variant Pressrelease -->
Emulates the pressing and immediate releasing
of a hardware keyboard key.
<!-- enum KeySynthType::variant Sym -->
A symbolic key event is generated, without specifying a
hardware key. Note: if the keysym is not present in the current keyboard
map, the `AtspiDeviceEventController` instance has a limited ability to
generate such keysyms on-the-fly. Reliability of GenerateKeyboardEvent
calls using out-of-keymap keysyms will vary from system to system, and on
the number of different out-of-keymap keysyms being generated in quick
succession.
In practice this is rarely significant, since the keysyms of interest to
AT clients and keyboard emulators are usually part of the current keymap,
i.e., present on the system keyboard for the current locale (even if a
physical hardware keyboard is not connected).
<!-- enum KeySynthType::variant String -->
A string is converted to its equivalent keyboard events
and emitted. If the string consists of complex characters or composed
characters which are not in the current keymap, string emission is
subject to the out-of-keymap limitations described for
`KeySynthType::Sym`. In practice this limitation primarily effects
Chinese and Japanese locales.
<!-- struct MatchRule -->

<!-- impl MatchRule::fn new -->
Creates a new `MatchRule` with specified `states`, `attributes`,
`interfaces`, and `roles`.
## `states`
An `StateSet` specifying the states to match or NULL if none.
## `statematchtype`
An `CollectionMatchType` specifying how to interpret
 `states`.
## `attributes`
A `glib::HashTable` specifying
 attributes to match. To specify multiple attribute values,
 separate each value with a :: If an attribute value contains a :,
 then it can be escaped by preceding it with a \. A backslash can
 likewise be escaped by inserting a double backslash.
## `attributematchtype`
An `CollectionMatchType` specifying how to
 interpret `attributes`.
## `roles`
A `glib::Array` of roles to match, or NULL if
 not applicable.
## `rolematchtype`
An `CollectionMatchType` specifying how to
 interpret `roles`.
## `interfaces`
An array of interfaces to match, or
 NULL if not applicable. Interface names should be specified
 by their DBus names (org.a11y.Atspi.Accessible,
 org.a11y.Atspi.Component, etc).
## `interfacematchtype`
An `CollectionMatchType` specifying how to
 interpret `interfaces`.
## `invert`
if `true`, the match rule should be denied (inverted); if `false`,
 it should not. For example, if the match rule defines that a match is
 an object of ROLE_HEADING which has STATE_FOCUSABLE and a click action,
 inverting it would match all objects that are not of ROLE_HEADING,
 focusable and clickable at the same time.

# Returns

A new `MatchRule`.
<!-- struct Object -->

<!-- struct Point -->
<!-- struct Range -->
<!-- impl Range::fn copy -->
Gets a copy of an `Range` object.

# Returns

the `Range` copy of an `Range` object.
<!-- struct Rect -->
<!-- struct Relation -->


# Implements

[`RelationExt`](trait.RelationExt.html)
<!-- trait RelationExt -->
Trait containing all `Relation` methods.

# Implementors

[`Relation`](struct.Relation.html)
<!-- trait RelationExt::fn get_n_targets -->
Gets the number of objects which this relationship has as its
 target objects (the subject is the `Accessible` from which this
 `Relation` originated).

# Returns

a `gint` indicating how many target objects which the
 originating `Accessible` object has the `Relation`
 relationship with.
<!-- trait RelationExt::fn get_relation_type -->
Gets the type of relationship represented by an `Relation`.

# Returns

an `RelationType` indicating the type of relation
 encapsulated in this `Relation` object.
<!-- trait RelationExt::fn get_target -->
Gets the `i`-th target of a specified `Relation` relationship.
## `i`
a (zero-index) `gint` indicating which (of possibly several) target is requested.

# Returns

an `Accessible` which is the `i`-th object
 with which the originating `Accessible` has relationship
 specified in the `Relation` object.
<!-- enum RelationType -->
`RelationType` specifies a relationship between objects
(possibly one-to-many
or many-to-one) outside of the normal parent/child hierarchical
relationship. It allows better semantic identification of how objects
are associated with one another. For instance the
`RelationType::LabelledBy`
relationship may be used to identify labelling information that should
accompany the accessible name property when presenting an object's content or
identity to the end user. Similarly,
`RelationType::ControllerFor` can be used
to further specify the context in which a valuator is useful, and/or the
other UI components which are directly effected by user interactions with
the valuator. Common examples include association of scrollbars with the
viewport or panel which they control.


Enumeration used to specify
the type of relation encapsulated in an `Relation` object.
<!-- enum RelationType::variant Null -->
Not a meaningful relationship; clients should not
normally encounter this `RelationType` value.
<!-- enum RelationType::variant LabelFor -->
Object is a label for one or more other objects.
<!-- enum RelationType::variant LabelledBy -->
Object is labelled by one or more other
objects.
<!-- enum RelationType::variant ControllerFor -->
Object is an interactive object which
modifies the state, onscreen location, or other attributes of one or more
target objects.
<!-- enum RelationType::variant ControlledBy -->
Object state, position, etc. is
modified/controlled by user interaction with one or more other objects.
For instance a viewport or scroll pane may be `RelationType::ControlledBy`
scrollbars.
<!-- enum RelationType::variant MemberOf -->
Object has a grouping relationship (e.g. 'same
group as') to one or more other objects.
<!-- enum RelationType::variant TooltipFor -->
Object is a tooltip associated with another
object.
<!-- enum RelationType::variant NodeChildOf -->
Object is a child of the target.
<!-- enum RelationType::variant NodeParentOf -->
Object is a parent of the target.
<!-- enum RelationType::variant Extended -->
Used to indicate that a relationship exists, but
its type is not specified in the enumeration.
<!-- enum RelationType::variant FlowsTo -->
Object renders content which flows logically to
another object. For instance, text in a paragraph may flow to another
object which is not the 'next sibling' in the accessibility hierarchy.
<!-- enum RelationType::variant FlowsFrom -->
Reciprocal of `RelationType::FlowsTo`.
<!-- enum RelationType::variant SubwindowOf -->
Object is visually and semantically considered
a subwindow of another object, even though it is not the object's child.
Useful when dealing with embedded applications and other cases where the
widget hierarchy does not map cleanly to the onscreen presentation.
<!-- enum RelationType::variant Embeds -->
Similar to `RelationType::SubwindowOf`, but
specifically used for cross-process embedding.
<!-- enum RelationType::variant EmbeddedBy -->
Reciprocal of `RelationType::Embeds`. Used to
denote content rendered by embedded renderers that live in a separate process
space from the embedding context.
<!-- enum RelationType::variant PopupFor -->
Denotes that the object is a transient window or
frame associated with another onscreen object. Similar to `ATSPI_TOOLTIP_FOR`,
but more general. Useful for windows which are technically toplevels
but which, for one or more reasons, do not explicitly cause their
associated window to lose 'window focus'. Creation of an `Role::Window`
object with the `RelationType::PopupFor` relation usually requires
some presentation action on the part
of assistive technology clients, even though the previous toplevel
`Role::Frame` object may still be the active window.
<!-- enum RelationType::variant ParentWindowOf -->
This is the reciprocal relation to
`RelationType::PopupFor`.
<!-- enum RelationType::variant DescriptionFor -->
Reciprocal of `RelationType::DescribedBy`.
Indicates that this object provides descriptive information about the target
object(s). See also `RelationType::DetailsFor` and `RelationType::ErrorFor`.
<!-- enum RelationType::variant DescribedBy -->
Reciprocal of `RelationType::DescriptionFor`.
Indicates that one or more target objects provide descriptive information
about this object. This relation type is most appropriate for information
that is not essential as its presentation may be user-configurable and/or
limited to an on-demand mechanism such as an assistive technology command.
For brief, essential information such as can be found in a widget's on-screen
label, use `RelationType::LabelledBy`. For an on-screen error message, use
`RelationType::ErrorMessage`. For lengthy extended descriptive information
contained in an on-screen object, consider using `RelationType::Details` as
assistive technologies may provide a means for the user to navigate to
objects containing detailed descriptions so that their content can be more
closely reviewed.
<!-- enum RelationType::variant Details -->
Reciprocal of `RelationType::DetailsFor`. Indicates
that this object has a detailed or extended description, the contents of
which can be found in the target object(s). This relation type is most
appropriate for information that is sufficiently lengthy as to make
navigation to the container of that information desirable. For less verbose
information suitable for announcement only, see `RelationType::DescribedBy`.
If the detailed information describes an error condition,
`RelationType::ErrorFor` should be used instead. `Since`: 2.26.
<!-- enum RelationType::variant DetailsFor -->
Reciprocal of `RelationType::Details`. Indicates
that this object provides a detailed or extended description about the target
object(s). See also `RelationType::DescriptionFor` and
`RelationType::ErrorFor`. `Since`: 2.26.
<!-- enum RelationType::variant ErrorMessage -->
Reciprocal of `RelationType::ErrorFor`.
Indicates that this object has one or more errors, the nature of which is
described in the contents of the target object(s). Objects that have this
relation type should also contain `StateType::InvalidEntry` in their
`StateSet`. `Since`: 2.26.
<!-- enum RelationType::variant ErrorFor -->
Reciprocal of `RelationType::ErrorMessage`.
Indicates that this object contains an error message describing an invalid
condition in the target object(s). `Since`: 2.26.
<!-- enum RelationType::variant LastDefined -->
Do not use as a parameter value, used to
determine the size of the enumeration.
<!-- enum Role -->
Enumeration used by interface `Accessible` to specify the role
of an `Accessible` object.
<!-- enum Role::variant Invalid -->
A role indicating an error condition, such as
uninitialized Role data.
<!-- enum Role::variant AcceleratorLabel -->
Object is a label indicating the keyboard
accelerators for the parent.
<!-- enum Role::variant Alert -->
Object is used to alert the user about something.
<!-- enum Role::variant Animation -->
Object contains a dynamic or moving image of some
kind.
<!-- enum Role::variant Arrow -->
Object is a 2d directional indicator.
<!-- enum Role::variant Calendar -->
Object contains one or more dates, usually arranged
into a 2d list.
<!-- enum Role::variant Canvas -->
Object that can be drawn into and is used to trap
events.
<!-- enum Role::variant CheckBox -->
A choice that can be checked or unchecked and
provides a separate indicator for the current state.
<!-- enum Role::variant CheckMenuItem -->
A menu item that behaves like a check box. See
`Role::CheckBox`.
<!-- enum Role::variant ColorChooser -->
A specialized dialog that lets the user choose a
color.
<!-- enum Role::variant ColumnHeader -->
The header for a column of data.
<!-- enum Role::variant ComboBox -->
A list of choices the user can select from.
<!-- enum Role::variant DateEditor -->
An object which allows entry of a date.
<!-- enum Role::variant DesktopIcon -->
An inconifed internal frame within a DESKTOP_PANE.
<!-- enum Role::variant DesktopFrame -->
A pane that supports internal frames and
iconified versions of those internal frames.
<!-- enum Role::variant Dial -->
An object that allows a value to be changed via rotating a
visual element, or which displays a value via such a rotating element.
<!-- enum Role::variant Dialog -->
A top level window with title bar and a border.
<!-- enum Role::variant DirectoryPane -->
A pane that allows the user to navigate through
and select the contents of a directory.
<!-- enum Role::variant DrawingArea -->
A specialized dialog that displays the files in
the directory and lets the user select a file, browse a different
directory, or specify a filename.
<!-- enum Role::variant FileChooser -->
An object used for drawing custom user interface
elements.
<!-- enum Role::variant Filler -->
A object that fills up space in a user interface.
<!-- enum Role::variant FocusTraversable -->
Don't use, reserved for future use.
<!-- enum Role::variant FontChooser -->
Allows selection of a display font.
<!-- enum Role::variant Frame -->
A top level window with a title bar, border, menubar,
etc.
<!-- enum Role::variant GlassPane -->
A pane that is guaranteed to be painted on top of
all panes beneath it.
<!-- enum Role::variant HtmlContainer -->
A document container for HTML, whose children
represent the document content.
<!-- enum Role::variant Icon -->
A small fixed size picture, typically used to decorate
components.
<!-- enum Role::variant Image -->
An image, typically static.
<!-- enum Role::variant InternalFrame -->
A frame-like object that is clipped by a desktop
pane.
<!-- enum Role::variant Label -->
An object used to present an icon or short string in an
interface.
<!-- enum Role::variant LayeredPane -->
A specialized pane that allows its children to be
drawn in layers, providing a form of stacking order.
<!-- enum Role::variant List -->
An object that presents a list of objects to the user and
allows the user to select one or more of them.
<!-- enum Role::variant ListItem -->
An object that represents an element of a list.
<!-- enum Role::variant Menu -->
An object usually found inside a menu bar that contains a
list of actions the user can choose from.
<!-- enum Role::variant MenuBar -->
An object usually drawn at the top of the primary
dialog box of an application that contains a list of menus the user can
choose from.
<!-- enum Role::variant MenuItem -->
An object usually contained in a menu that presents
an action the user can choose.
<!-- enum Role::variant OptionPane -->
A specialized pane whose primary use is inside a
dialog.
<!-- enum Role::variant PageTab -->
An object that is a child of a page tab list.
<!-- enum Role::variant PageTabList -->
An object that presents a series of panels (or
page tabs), one at a time,through some mechanism provided by the
object.
<!-- enum Role::variant Panel -->
A generic container that is often used to group objects.
<!-- enum Role::variant PasswordText -->
A text object uses for passwords, or other places
where the text content is not shown visibly to the user.
<!-- enum Role::variant PopupMenu -->
A temporary window that is usually used to offer the
user a list of choices, and then hides when the user selects one of those
choices.
<!-- enum Role::variant ProgressBar -->
An object used to indicate how much of a task has
been completed.
<!-- enum Role::variant PushButton -->
An object the user can manipulate to tell the
application to do something.
<!-- enum Role::variant RadioButton -->
A specialized check box that will cause other
radio buttons in the same group to become unchecked when this one is
checked.
<!-- enum Role::variant RadioMenuItem -->
Object is both a menu item and a "radio button"
. See `Role::RadioButton`.
<!-- enum Role::variant RootPane -->
A specialized pane that has a glass pane and a
layered pane as its children.
<!-- enum Role::variant RowHeader -->
The header for a row of data.
<!-- enum Role::variant ScrollBar -->
An object usually used to allow a user to
incrementally view a large amount of data by moving the bounds of a
viewport along a one-dimensional axis.
<!-- enum Role::variant ScrollPane -->
An object that allows a user to incrementally view
a large amount of information. `Role::ScrollPane` objects are usually
accompanied by `Role::ScrollBar` controllers, on which the
`RelationType::ControllerFor` and `RelationType::ControlledBy`
reciprocal relations are set. See `atspi_get_relation_set`.
<!-- enum Role::variant Separator -->
An object usually contained in a menu to provide a
visible and logical separation of the contents in a menu.
<!-- enum Role::variant Slider -->
An object that allows the user to select from a bounded
range.
<!-- enum Role::variant SpinButton -->
An object which allows one of a set of choices to
be selected, and which displays the current choice. Unlike
`Role::ScrollBar`, `Role::Slider` objects need not control
'viewport'-like objects.
<!-- enum Role::variant SplitPane -->
A specialized panel that presents two other panels
at the same time.
<!-- enum Role::variant StatusBar -->
Object displays non-quantitative status information
(c.f. `Role::ProgressBar`)
<!-- enum Role::variant Table -->
An object used to repesent information in terms of rows
and columns.
<!-- enum Role::variant TableCell -->
A 'cell' or discrete child within a Table. Note:
Table cells need not have `Role::TableCell`, other
`AtspiRoleType` values are valid as well.
<!-- enum Role::variant TableColumnHeader -->
An object which labels a particular column
in an `Table`.
<!-- enum Role::variant TableRowHeader -->
An object which labels a particular row in a
`Table`. `Table` rows and columns may also be labelled via the
`RelationType::LabelFor`/`RelationType::LabelledBy` relationships.
See `atspi_get_relation_set`.
<!-- enum Role::variant TearoffMenuItem -->
Object allows menu to be removed from menubar
and shown in its own window.
<!-- enum Role::variant Terminal -->
An object that emulates a terminal.
<!-- enum Role::variant Text -->
An interactive widget that supports multiple lines of text
and optionally accepts user input, but whose purpose is not to solicit user
input. Thus `Role::Text` is appropriate for the text view in a plain text
editor but inappropriate for an input field in a dialog box or web form. For
widgets whose purpose is to solicit input from the user, see `Role::Entry`
and `Role::PasswordText`. For generic objects which display a brief amount
of textual information, see `Role::Static`.
<!-- enum Role::variant ToggleButton -->
A specialized push button that can be checked or
unchecked, but does not procide a separate indicator for the current
state.
<!-- enum Role::variant ToolBar -->
A bar or palette usually composed of push buttons or
toggle buttons.
<!-- enum Role::variant ToolTip -->
An object that provides information about another
object.
<!-- enum Role::variant Tree -->
An object used to repsent hierarchical information to the
user.
<!-- enum Role::variant TreeTable -->
An object that presents both tabular and
hierarchical info to the user.
<!-- enum Role::variant Unknown -->
The object contains some `Accessible` information,
but its role is not known.
<!-- enum Role::variant Viewport -->
An object usually used in a scroll pane, or to
otherwise clip a larger object or content renderer to a specific
onscreen viewport.
<!-- enum Role::variant Window -->
A top level window with no title or border.
<!-- enum Role::variant Extended -->
means that the role for this item is known, but not
included in the core enumeration. Deprecated since 2.24.
<!-- enum Role::variant Header -->
An object that serves as a document header.
<!-- enum Role::variant Footer -->
An object that serves as a document footer.
<!-- enum Role::variant Paragraph -->
An object which is contains a single paragraph of
text content. See also `Role::Text`.
<!-- enum Role::variant Ruler -->
An object which describes margins and tab stops, etc.
 for text objects which it controls (should have
`RelationType::ControllerFor` relation to such).
<!-- enum Role::variant Application -->
An object corresponding to the toplevel accessible
of an application, which may contain `Role::Frame` objects or other
accessible objects. Children of `AccessibleDesktop` objects are generally
`Role::Application` objects.
<!-- enum Role::variant Autocomplete -->
The object is a dialog or list containing items
for insertion into an entry widget, for instance a list of words for
completion of a text entry.
<!-- enum Role::variant Editbar -->
The object is an editable text object in a toolbar.
<!-- enum Role::variant Embedded -->
The object is an embedded component container. This
role is a "grouping" hint that the contained objects share a context
which is different from the container in which this accessible is
embedded. In particular, it is used for some kinds of document embedding,
and for embedding of out-of-process component, "panel applets", etc.
<!-- enum Role::variant Entry -->
The object is a component whose textual content may be
entered or modified by the user, provided `StateType::Editable` is present.
A readonly `Role::Entry` object (i.e. where `StateType::Editable` is
not present) implies a read-only 'text field' in a form, as opposed to a
title, label, or caption.
<!-- enum Role::variant Chart -->
The object is a graphical depiction of quantitative data.
It may contain multiple subelements whose attributes and/or description
may be queried to obtain both the quantitative data and information about
how the data is being presented. The `ATSPI_LABELLED_BY` relation is
particularly important in interpreting objects of this type, as is the
accessible description property. See `Role::Caption`.
<!-- enum Role::variant Caption -->
The object contains descriptive information, usually
textual, about another user interface element such as a table, chart, or
image.
<!-- enum Role::variant DocumentFrame -->
The object is a visual frame or container which
contains a view of document content. `Document` frames may occur within
another `Document` instance, in which case the second document may be
said to be embedded in the containing instance. HTML frames are often
ATSPI_ROLE_DOCUMENT_FRAME: Either this object, or a singleton descendant,
should implement the `Document` interface.
<!-- enum Role::variant Heading -->
The object serves as a heading for content which
follows it in a document. The 'heading level' of the heading, if
availabe, may be obtained by querying the object's attributes.
<!-- enum Role::variant Page -->
The object is a containing instance which encapsulates a
page of information. `Role::Page` is used in documents and content which
support a paginated navigation model.
<!-- enum Role::variant Section -->
The object is a containing instance of document content
which constitutes a particular 'logical' section of the document. The
type of content within a section, and the nature of the section division
itself, may be obtained by querying the object's attributes. Sections
may be nested.
<!-- enum Role::variant RedundantObject -->
The object is redundant with another object in
the hierarchy, and is exposed for purely technical reasons. Objects of
this role should be ignored by clients, if they are encountered at all.
<!-- enum Role::variant Form -->
The object is a containing instance of document content
which has within it components with which the user can interact in order
to input information; i.e. the object is a container for pushbuttons,
comboboxes, text input fields, and other 'GUI' components. `Role::Form`
should not, in general, be used for toplevel GUI containers or dialogs,
but should be reserved for 'GUI' containers which occur within document
content, for instance within Web documents, presentations, or text
documents. Unlike other GUI containers and dialogs which occur inside
application instances, `Role::Form` containers' components are
associated with the current document, rather than the current foreground
application or viewer instance.
<!-- enum Role::variant Link -->
The object is a hypertext anchor, i.e. a "link" in a
hypertext document. Such objects are distinct from 'inline' content
which may also use the `Hypertext`/`Hyperlink` interfacesto indicate
the range/location within a text object where an inline or embedded object
lies.
<!-- enum Role::variant InputMethodWindow -->
The object is a window or similar viewport
which is used to allow composition or input of a 'complex character',
in other words it is an "input method window".
<!-- enum Role::variant TableRow -->
A row in a table.
<!-- enum Role::variant TreeItem -->
An object that represents an element of a tree.
<!-- enum Role::variant DocumentSpreadsheet -->
A document frame which contains a
spreadsheet.
<!-- enum Role::variant DocumentPresentation -->
A document frame which contains a
presentation or slide content.
<!-- enum Role::variant DocumentText -->
A document frame which contains textual content,
such as found in a word processing
application.
<!-- enum Role::variant DocumentWeb -->
A document frame which contains HTML or other
markup suitable for display in a web browser.
<!-- enum Role::variant DocumentEmail -->
A document frame which contains email content
to be displayed or composed either in plain text or
HTML.
<!-- enum Role::variant Comment -->
An object found within a document and designed to
present a comment, note, or other annotation. In some cases, this object
might not be visible until activated.
<!-- enum Role::variant ListBox -->
A non-collapsible list of choices the user can
select from.
<!-- enum Role::variant Grouping -->
A group of related widgets. This group typically has
a label.
<!-- enum Role::variant ImageMap -->
An image map object. Usually a graphic with multiple
hotspots, where each hotspot can be activated resulting in the loading of
another document or section of a document.
<!-- enum Role::variant Notification -->
A transitory object designed to present a
message to the user, typically at the desktop level rather than inside a
particular application.
<!-- enum Role::variant InfoBar -->
An object designed to present a message to the user
within an existing window.
<!-- enum Role::variant LevelBar -->
A bar that serves as a level indicator to, for
instance, show the strength of a password or the state of a battery.
 Since: 2.8
<!-- enum Role::variant TitleBar -->
A bar that serves as the title of a window or a
dialog. `Since`: 2.12
<!-- enum Role::variant BlockQuote -->
An object which contains a text section
that is quoted from another source. `Since`: 2.12
<!-- enum Role::variant Audio -->
An object which represents an audio
element. `Since`: 2.12
<!-- enum Role::variant Video -->
An object which represents a video
element. `Since`: 2.12
<!-- enum Role::variant Definition -->
A definition of a term or concept. `Since`: 2.12
<!-- enum Role::variant Article -->
A section of a page that consists of a
composition that forms an independent part of a document, page, or
site. Examples: A blog entry, a news story, a forum post. `Since`:
2.12
<!-- enum Role::variant Landmark -->
A region of a web page intended as a
navigational landmark. This is designed to allow Assistive
Technologies to provide quick navigation among key regions within a
document. `Since`: 2.12
<!-- enum Role::variant Log -->
A text widget or container holding log content, such
as chat history and error logs. In this role there is a
relationship between the arrival of new items in the log and the
reading order. The log contains a meaningful sequence and new
information is added only to the end of the log, not at arbitrary
points. `Since`: 2.12
<!-- enum Role::variant Marquee -->
A container where non-essential information
changes frequently. Common usages of marquee include stock tickers
and ad banners. The primary difference between a marquee and a log
is that logs usually have a meaningful order or sequence of
important content changes. `Since`: 2.12
<!-- enum Role::variant Math -->
A text widget or container that holds a mathematical
expression. `Since`: 2.12
<!-- enum Role::variant Rating -->
A widget whose purpose is to display a rating,
such as the number of stars associated with a song in a media
player. Objects of this role should also implement
AtspiValue. `Since`: 2.12
<!-- enum Role::variant Timer -->
An object containing a numerical counter which
indicates an amount of elapsed time from a start point, or the time
remaining until an end point. `Since`: 2.12
<!-- enum Role::variant Static -->
A generic non-container object whose purpose is to display
a brief amount of information to the user and whose role is known by the
implementor but lacks semantic value for the user. Examples in which
`Role::Static` is appropriate include the message displayed in a message
box and an image used as an alternative means to display text.
`Role::Static` should not be applied to widgets which are traditionally
interactive, objects which display a significant amount of content, or any
object which has an accessible relation pointing to another object. The
displayed information, as a general rule, should be exposed through the
accessible name of the object. For labels which describe another widget, see
`Role::Label`. For text views, see `Role::Text`. For generic
containers, see `Role::Panel`. For objects whose role is not known by the
implementor, see `Role::Unknown`. `Since`: 2.16.
<!-- enum Role::variant MathFraction -->
An object that represents a mathematical fraction.
<!-- enum Role::variant MathRoot -->
An object that represents a mathematical expression
displayed with a radical. `Since`: 2.16.
<!-- enum Role::variant Subscript -->
An object that contains text that is displayed as a
subscript. `Since`: 2.16.
<!-- enum Role::variant Superscript -->
An object that contains text that is displayed as a
superscript. `Since`: 2.16.
<!-- enum Role::variant DescriptionList -->
An object that represents a list of term-value
groups. A term-value group represents an individual description and consist
of one or more names (`Role::DescriptionTerm`) followed by one or more
values (`Role::DescriptionValue`). For each list, there should not be
more than one group with the same term name. `Since`: 2.26.
<!-- enum Role::variant DescriptionTerm -->
An object that represents a term or phrase
with a corresponding definition. `Since`: 2.26.
<!-- enum Role::variant DescriptionValue -->
An object that represents the description,
definition, or value of a term. `Since`: 2.26.
<!-- enum Role::variant Footnote -->
An object that contains the text of a footnote. `Since`: 2.26.
<!-- enum Role::variant LastDefined -->
Not a valid role, used for finding end of
enumeration.
<!-- struct Selection -->


# Implements

[`SelectionExt`](trait.SelectionExt.html)
<!-- trait SelectionExt -->
Trait containing all `Selection` methods.

# Implementors

[`Accessible`](struct.Accessible.html), [`Selection`](struct.Selection.html)
<!-- trait SelectionExt::fn clear_selection -->
Clears the current selection, removing all selected children from the
 specified `Selection` implementor's selection list.

# Returns

`true` if successful, `false` otherwise.
<!-- trait SelectionExt::fn deselect_child -->
Deselects a specific child of an `Selection`.
 Note that `child_index` is the index of the child
 in the parent container.

See `Selection::deselect_selected_child`
## `child_index`
a `gint` indicating which of the children
 of the `Accessible` is to be de-selected.

# Returns

`true` if the child was successfully deselected, `false` otherwise.
<!-- trait SelectionExt::fn deselect_selected_child -->
Removes a child from the selected children list of an `Selection`.
 Note that `child_index` is the index in the selected-children list,
 not the index in the parent container. `selectedChildIndex` in this
 method, and `child_index` in `Selection::select_child`
 are asymmetric.
## `selected_child_index`
a `gint` indicating which of the selected children
 of the `Accessible` is to be selected.

# Returns

`true` if the child was successfully deselected, `false` otherwise.
<!-- trait SelectionExt::fn get_n_selected_children -->
Gets the number of children of an `Selection` implementor which are
 currently selected.

# Returns

a `gint` indicating the number of `Accessible` children
 of the `Selection` implementor which are currently selected.
<!-- trait SelectionExt::fn get_selected_child -->
Gets the i-th selected `Accessible` child of an `Selection`.
 Note that `selected_child_index` refers to the index in the list
 of 'selected'
 children and generally differs from that used in
 `AccessibleExt::get_child_at_index` or returned by
 `AccessibleExt::get_index_in_parent`.
 `selected_child_index` must lie between 0
 and `Selection::get_n_selected_children` - 1, inclusive.
## `selected_child_index`
a `gint` indicating which of the selected
 children is specified.

# Returns

a pointer to a selected `Accessible` child
 object, specified by `selected_child_index`.
<!-- trait SelectionExt::fn is_child_selected -->
Determines whether a particular child of an `Selection` implementor
 is currently selected. Note that `child_index` is the index into the
 standard `Accessible` container's list of children.
## `child_index`
an index into the `Selection`'s list of children.

# Returns

`true` if the specified child is currently selected,
 `false` otherwise.
<!-- trait SelectionExt::fn select_all -->
Attempts to select all of the children of an `Selection` implementor.
Not all `Selection` implementors support this operation.

# Returns

`true` if successful, `false` otherwise.
<!-- trait SelectionExt::fn select_child -->
Adds a child to the selected children list of an `Selection`.
 For `Selection` implementors that only allow
 single selections, this may replace the (single) current
 selection.
## `child_index`
a `gint` indicating which child of the `Accessible`
 is to be selected.

# Returns

`true` if the child was successfully selected, `false` otherwise.
<!-- struct StateSet -->


# Implements

[`StateSetExt`](trait.StateSetExt.html)
<!-- trait StateSetExt -->
Trait containing all `StateSet` methods.

# Implementors

[`StateSet`](struct.StateSet.html)
<!-- impl StateSet::fn new -->
Generates an `StateSet` with the given `states`.
## `states`
An array of states with which the
 method initializes the state set.

# Returns

A new `StateSet` with the given states.
<!-- trait StateSetExt::fn add -->
Adds a particular `AtspiState` to an `StateSet` (i.e. sets the
 given state to `true` in the stateset).
## `state`
an `StateType` to be added to the specified `StateSet`.
<!-- trait StateSetExt::fn compare -->
Determines the differences between two instances of `StateSet`.

`see` `StateSetExt::equals`.
## `set2`
a pointer to the second `StateSet` object on which to operate.

# Returns

an `StateSet` object containing all states
contained on one of the two sets but not the other.
<!-- trait StateSetExt::fn contains -->
Determines whether a given `StateSet` includes a given state; that is,
 whether `state` is true for the `self` in question.
## `state`
an `StateType` for which the specified `StateSet`
 will be queried.

# Returns

`true` if `state` is true/included in the given `StateSet`,
 otherwise `false`.
<!-- trait StateSetExt::fn equals -->
Determines whether two instances of `StateSet` are equivalent (i.e.
 consist of the same `AtspiStates`). Useful for checking multiple
 state variables at once.

`see` `StateSetExt::compare`.
## `set2`
a pointer to the second `StateSet` object on which to operate.

# Returns

`true` if the two `AtspiStateSets` are equivalent,
otherwise `false`.
<!-- trait StateSetExt::fn get_states -->
Returns the states in an `StateSet` as an array.

# Returns

A `glib::Array` of state
 types representing the current state.
<!-- trait StateSetExt::fn is_empty -->

# Returns

`true` if the state set contains no states; `false` otherwise.
<!-- trait StateSetExt::fn remove -->
Removes a particular `AtspiState` to an `StateSet` (i.e. sets the
 given state to `false` in the stateset.)
## `state`
an `StateType` to remove from the specified `self`.
<!-- trait StateSetExt::fn set_by_name -->
Enables/disables a state in an `StateSet` according to its `name`.
## `name`
a string corresponding to a state name.
## `enabled`
if `true`, `name` should be enabled in the `self` in question;
 otherwise, it should be disabled.
<!-- enum StateType -->
Enumeration used by various interfaces indicating every possible state
an `AtspiAccesible` object can assume.
<!-- enum StateType::variant Invalid -->
Indicates an invalid state - probably an error
condition.
<!-- enum StateType::variant Active -->
Indicates a window is currently the active window, or
an object is the active subelement within a container or table.
`StateType::Active` should not be used for objects which have
`StateType::Focusable` or `StateType::Selectable`: Those objects should use
`StateType::Focused` and `StateType::Selected` respectively.
`StateType::Active` is a means to indicate that an object which is not
focusable and not selectable is the currently-active item within its
parent container.
<!-- enum StateType::variant Armed -->
Indicates that the object is armed.
<!-- enum StateType::variant Busy -->
Indicates the current object is busy, i.e. onscreen
representation is in the process of changing, or the object is
temporarily unavailable for interaction due to activity already in progress.
<!-- enum StateType::variant Checked -->
Indicates this object is currently checked.
<!-- enum StateType::variant Collapsed -->
Indicates this object is collapsed.
<!-- enum StateType::variant Defunct -->
Indicates that this object no longer has a valid
backing widget (for instance, if its peer object has been destroyed).
<!-- enum StateType::variant Editable -->
Indicates the user can change the contents of this
object.
<!-- enum StateType::variant Enabled -->
Indicates that this object is enabled, i.e. that it
currently reflects some application state. Objects that are "greyed out"
may lack this state, and may lack the `StateType::Sensitive` if direct
user interaction cannot cause them to acquire `StateType::Enabled`.
See `StateType::Sensitive`.
<!-- enum StateType::variant Expandable -->
Indicates this object allows progressive
disclosure of its children.
<!-- enum StateType::variant Expanded -->
Indicates this object is expanded.
<!-- enum StateType::variant Focusable -->
Indicates this object can accept keyboard focus,
which means all events resulting from typing on the keyboard will
normally be passed to it when it has focus.
<!-- enum StateType::variant Focused -->
Indicates this object currently has the keyboard
focus.
<!-- enum StateType::variant HasTooltip -->
Indicates that the object has an associated
tooltip.
<!-- enum StateType::variant Horizontal -->
Indicates the orientation of this object is
horizontal.
<!-- enum StateType::variant Iconified -->
Indicates this object is minimized and is
represented only by an icon.
<!-- enum StateType::variant Modal -->
Indicates something must be done with this object
before the user can interact with an object in a different window.
<!-- enum StateType::variant MultiLine -->
Indicates this (text) object can contain multiple
lines of text.
<!-- enum StateType::variant Multiselectable -->
Indicates this object allows more than one of
its children to be selected at the same time, or in the case of text
objects, that the object supports non-contiguous text selections.
<!-- enum StateType::variant Opaque -->
Indicates this object paints every pixel within its
rectangular region. It also indicates an alpha value of unity, if it
supports alpha blending.
<!-- enum StateType::variant Pressed -->
Indicates this object is currently pressed.
<!-- enum StateType::variant Resizable -->
Indicates the size of this object's size is not
fixed.
<!-- enum StateType::variant Selectable -->
Indicates this object is the child of an object
that allows its children to be selected and that this child is one of
those children that can be selected.
<!-- enum StateType::variant Selected -->
Indicates this object is the child of an object that
allows its children to be selected and that this child is one of those
children that has been selected.
<!-- enum StateType::variant Sensitive -->
Indicates this object is sensitive, e.g. to user
interaction. `StateType::Sensitive` usually accompanies.
`StateType::Enabled` for user-actionable controls, but may be found in the
absence of `StateType::Enabled` if the current visible state of the control
is "disconnected" from the application state. In such cases, direct user
interaction can often result in the object gaining `StateType::Sensitive`,
for instance if a user makes an explicit selection using an object whose
current state is ambiguous or undefined. See `StateType::Enabled`,
`StateType::Indeterminate`.
<!-- enum StateType::variant Showing -->
Indicates this object, the object's parent, the
object's parent's parent, and so on, are all 'shown' to the end-user,
i.e. subject to "exposure" if blocking or obscuring objects do not
interpose between this object and the top of the window stack.
<!-- enum StateType::variant SingleLine -->
Indicates this (text) object can contain only a
single line of text.
<!-- enum StateType::variant Stale -->
Indicates that the information returned for this object
may no longer be synchronized with the application state. This can occur
if the object has `StateType::Transient`, and can also occur towards the
end of the object peer's lifecycle.
<!-- enum StateType::variant Transient -->
Indicates this object is transient.
<!-- enum StateType::variant Vertical -->
Indicates the orientation of this object is vertical;
for example this state may appear on such objects as scrollbars, text
objects (with vertical text flow), separators, etc.
<!-- enum StateType::variant Visible -->
Indicates this object is visible, e.g. has been
explicitly marked for exposure to the user. `StateType::Visible` is no
guarantee that the object is actually unobscured on the screen, only that
it is 'potentially' visible, barring obstruction, being scrolled or clipped
out of the field of view, or having an ancestor container that has not yet
made visible. A widget is potentially onscreen if it has both
`StateType::Visible` and `StateType::Showing`. The absence of
`StateType::Visible` and `StateType::Showing` is
semantically equivalent to saying that an object is 'hidden'.
<!-- enum StateType::variant ManagesDescendants -->
Indicates that "active-descendant-changed"
event is sent when children become 'active' (i.e. are selected or
navigated to onscreen). Used to prevent need to enumerate all children
in very large containers, like tables. The presence of
`StateType::ManagesDescendants` is an indication to the client that the
children should not, and need not, be enumerated by the client.
Objects implementing this state are expected to provide relevant state
notifications to listening clients, for instance notifications of
visibility changes and activation of their contained child objects, without
the client having previously requested references to those children.
<!-- enum StateType::variant Indeterminate -->
Indicates that a check box or other boolean
indicator is in a state other than checked or not checked. This
usually means that the boolean value reflected or controlled by the
object does not apply consistently to the entire current context.
For example, a checkbox for the "Bold" attribute of text may have
`StateType::Indeterminate` if the currently selected text contains a mixture
of weight attributes. In many cases interacting with a
`StateType::Indeterminate` object will cause the context's corresponding
boolean attribute to be homogenized, whereupon the object will lose
`StateType::Indeterminate` and a corresponding state-changed event will be
fired.
<!-- enum StateType::variant Required -->
Indicates that user interaction with this object is
'required' from the user, for instance before completing the
processing of a form.
<!-- enum StateType::variant Truncated -->
Indicates that an object's onscreen content
is truncated, e.g. a text value in a spreadsheet cell.
<!-- enum StateType::variant Animated -->
Indicates this object's visual representation is
dynamic, not static. This state may be applied to an object during an
animated 'effect' and be removed from the object once its visual
representation becomes static. Some applications, notably content viewers,
may not be able to detect all kinds of animated content. Therefore the
absence of this state should not be taken as
definitive evidence that the object's visual representation is
static; this state is advisory.
<!-- enum StateType::variant InvalidEntry -->
This object has indicated an error condition
due to failure of input validation. For instance, a form control may
acquire this state in response to invalid or malformed user input.
<!-- enum StateType::variant SupportsAutocompletion -->
This state indicates that the object
in question implements some form of typeahead or
pre-selection behavior whereby entering the first character of one or more
sub-elements causes those elements to scroll into view or become
selected. Subsequent character input may narrow the selection further as
long as one or more sub-elements match the string. This state is normally
only useful and encountered on objects that implement `Selection`.
In some cases the typeahead behavior may result in full or partial
completion of the data in the input field, in which case
these input events may trigger text-changed events from the source.
<!-- enum StateType::variant SelectableText -->
This state indicates that the object in
question supports text selection. It should only be exposed on objects
which implement the `Text` interface, in order to distinguish this state
from `StateType::Selectable`, which infers that the object in question is a
selectable child of an object which implements `Selection`. While
similar, text selection and subelement selection are distinct operations.
<!-- enum StateType::variant IsDefault -->
This state indicates that the object in question is
the 'default' interaction object in a dialog, i.e. the one that gets
activated if the user presses "Enter" when the dialog is initially
posted.
<!-- enum StateType::variant Visited -->
This state indicates that the object (typically a
hyperlink) has already been activated or invoked, with the result that
some backing data has been downloaded or rendered.
<!-- enum StateType::variant Checkable -->
Indicates this object has the potential to
 be checked, such as a checkbox or toggle-able table cell. `Since`:
 2.12
<!-- enum StateType::variant HasPopup -->
Indicates that the object has a popup
context menu or sub-level menu which may or may not be
showing. This means that activation renders conditional content.
Note that ordinary tooltips are not considered popups in this
context. `Since`: 2.12
<!-- enum StateType::variant ReadOnly -->
Indicates that an object which is ENABLED and
SENSITIVE has a value which can be read, but not modified, by the
user. `Since`: 2.16
<!-- enum StateType::variant LastDefined -->
This value of the enumeration should not be used
as a parameter, it indicates the number of items in the `StateType`
enumeration.
<!-- struct Table -->


# Implements

[`TableExt`](trait.TableExt.html)
<!-- trait TableExt -->
Trait containing all `Table` methods.

# Implementors

[`Accessible`](struct.Accessible.html), [`Table`](struct.Table.html)
<!-- trait TableExt::fn add_column_selection -->
Selects the specified column, adding it to the current column selection.
Not all tables support column selection.
## `column`
the zero-indexed column number of the column being selected.

# Returns

`true` if the specified column was successfully selected, `false` if not.
<!-- trait TableExt::fn add_row_selection -->
Selects the specified row, adding it to the current row selection.
Not all tables support row selection.
## `row`
the zero-indexed row number of the row being selected.

# Returns

`true` if the specified row was successfully selected, `false` if not.
<!-- trait TableExt::fn get_accessible_at -->
Gets the table cell at the specified row and column indices.
To get the accessible object at a particular (x, y) screen
coordinate, use `Component::get_accessible_at_point`.
## `row`
the specified table row, zero-indexed.
## `column`
the specified table column, zero-indexed.

# Returns

an `Accessible` object representing the
 specified table cell.
<!-- trait TableExt::fn get_caption -->
Gets an accessible representation of the caption for an `Table`.

# Returns

an `Accessible` object that serves as
the table's caption.
<!-- trait TableExt::fn get_column_at_index -->
Gets the table column index occupied by the child at a particular 1-D
child index.

`see` `Table::get_index_at`, `Table::get_row_at_index`
## `index`
the specified child index, zero-indexed.

# Returns

a `gint` indicating the first column spanned by the child of a
 table, at the specified 1-D (zero-offset) `index`.
<!-- trait TableExt::fn get_column_description -->
Gets a text description of a particular table column. This differs from
`Table::get_column_header`, which returns an `Accessible`.
## `column`
the specified table column, zero-indexed.

# Returns

a UTF-8 string describing the specified table column, if available.
<!-- trait TableExt::fn get_column_extent_at -->
Gets the number of columns spanned by the table cell at the specific
row and column (some tables can have cells which span multiple
rows and/or columns).
## `row`
the specified table row, zero-indexed.
## `column`
the specified table column, zero-indexed.

# Returns

a `gint` indicating the number of columns spanned by the specified cell.
<!-- trait TableExt::fn get_column_header -->
Gets the header associated with a table column, if available.
This differs from `Table::get_column_description`, which
returns a string.
## `column`
the specified table column, zero-indexed.

# Returns

an `Accessible` representation of the
 specified table column, if available.
<!-- trait TableExt::fn get_index_at -->
Gets the 1-D child index corresponding to the specified 2-D row and
column indices. To get the accessible object at a particular (x, y) screen
coordinate, use `Component::get_accessible_at_point`.

`see` `Table::get_row_at_index`, `Table::get_column_at_index`
## `row`
the specified table row, zero-indexed.
## `column`
the specified table column, zero-indexed.

# Returns

a `gint` which serves as the index of a specified cell in the
 table, in a form usable by `atspi_get_child_at_index`.
<!-- trait TableExt::fn get_n_columns -->
Gets the number of columns in an `Table`,
 exclusive of any columns that are programmatically hidden, but inclusive
 of columns that may be outside of the current scrolling window or viewport.

# Returns

a `gint` indicating the number of columns in the table.
<!-- trait TableExt::fn get_n_rows -->
Gets the number of rows in an `Table`,
 exclusive of any rows that are programmatically hidden, but inclusive
 of rows that may be outside of the current scrolling window or viewport.

# Returns

a `gint` indicating the number of rows in the table.
<!-- trait TableExt::fn get_n_selected_columns -->
Queries a table to find out how many columns are currently selected.
Not all tables support column selection.

# Returns

a `gint` indicating the number of columns currently selected.
<!-- trait TableExt::fn get_n_selected_rows -->
Query a table to find out how many rows are currently selected.
Not all tables support row selection.

# Returns

a `gint` indicating the number of rows currently selected.
<!-- trait TableExt::fn get_row_at_index -->
Gets the table row index occupied by the child at a particular 1-D
child index.

`see` `Table::get_index_at`, `Table::get_column_at_index`
## `index`
the specified child index, zero-indexed.

# Returns

a `gint` indicating the first row spanned by the child of a
 table, at the specified 1-D (zero-offset) `index`.
<!-- trait TableExt::fn get_row_column_extents_at_index -->
Given a child index, determines the row and column indices and
extents, and whether the cell is currently selected. If
the child at index is not a cell (for instance, if it is
a summary, caption, etc.), `false` is returned.

Example:
If the `Table` child at index '6' extends across columns 5 and 6 of
row 2 of an `Table` instance, and is currently selected, then

retval = atspi_table_get_row_column_extents_at_index (table, 6,
 row, col,
 row_extents,
 col_extents,
 is_selected);

will return `true`, and after the call
row, col, row_extents, col_extents,
and is_selected will contain 2, 5, 1, 2, and
`true`, respectively.
## `index`
the index of the `Table` child whose row/column
extents are requested.
## `row`
back-filled with the first table row associated with
the cell with child index.
## `col`
back-filled with the first table column associated
with the cell with child index.
## `row_extents`
back-filled with the number of table rows
across which child i extends.
## `col_extents`
back-filled with the number of table columns
across which child i extends.
## `is_selected`
a boolean which is back-filled with `true`
if the child at index i corresponds to a selected table cell,
`false` otherwise.

# Returns

`true` if the index is associated with a valid table
cell, `false` if the index does not correspond to a cell. If
`false` is returned, the values of the out parameters are
undefined.
<!-- trait TableExt::fn get_row_description -->
Gets a text description of a particular table row. This differs from
`Table::get_row_header`, which returns an `Accessible`.
## `row`
the specified table row, zero-indexed.

# Returns

a UTF-8 string describing the specified table row, if available.
<!-- trait TableExt::fn get_row_extent_at -->
Gets the number of rows spanned by the table cell at the specific row
and column. (some tables can have cells which span multiple rows
and/or columns).
## `row`
the specified table row, zero-indexed.
## `column`
the specified table column, zero-indexed.

# Returns

a `gint` indicating the number of rows spanned by the specified cell.
<!-- trait TableExt::fn get_row_header -->
Gets the header associated with a table row, if available. This differs from
`Table::get_row_description`, which returns a string.
## `row`
the specified table row, zero-indexed.

# Returns

an `Accessible` representation of the specified
 table row, if available.
<!-- trait TableExt::fn get_selected_columns -->
Queries a table for a list of indices of columns which are currently
selected.

# Returns

an array of `gint` values,
 specifying which columns are currently selected.
<!-- trait TableExt::fn get_selected_rows -->
Queries a table for a list of indices of rows which are currently selected.

# Returns

an array of `gint` values,
 specifying which rows are currently selected.
<!-- trait TableExt::fn get_summary -->
Gets an accessible object which summarizes the contents of an `Table`.

# Returns

an `Accessible` object that serves as the
 table's summary (often a reduced `Table`).
<!-- trait TableExt::fn is_column_selected -->
Determines whether specified table column is selected.
Not all tables support column selection.
## `column`
the zero-indexed column number of the column being queried.

# Returns

`true` if the specified column is currently selected, `false` if not.
<!-- trait TableExt::fn is_row_selected -->
Determines whether a table row is selected. Not all tables support
row selection.
## `row`
the zero-indexed row number of the row being queried.

# Returns

`true` if the specified row is currently selected, `false` if not.
<!-- trait TableExt::fn is_selected -->
Determines whether the cell at a specific row and column is selected.
## `row`
the zero-indexed row of the cell being queried.
## `column`
the zero-indexed column of the cell being queried.

# Returns

`true` if the specified cell is currently selected, `false` if not.
<!-- trait TableExt::fn remove_column_selection -->
De-selects the specified column, removing it from the current column
selection.
Not all tables support column selection.
## `column`
the zero-indexed column number of the column being de-selected.

# Returns

`true` if the specified column was successfully de-selected,
`false` if not.
<!-- trait TableExt::fn remove_row_selection -->
De-selects the specified row, removing it from the current row selection.
Not all tables support row selection.
## `row`
the zero-indexed number of the row being de-selected.

# Returns

`true` if the specified row was successfully de-selected,
`false` if not.
<!-- struct TableCell -->


# Implements

[`TableCellExt`](trait.TableCellExt.html)
<!-- trait TableCellExt -->
Trait containing all `TableCell` methods.

# Implementors

[`Accessible`](struct.Accessible.html), [`TableCell`](struct.TableCell.html)
<!-- trait TableCellExt::fn get_column_header_cells -->
Returns the column headers as an array of cell accessibles.

# Returns

a GPtrArray of
AtspiAccessibles representing the column header cells.
<!-- trait TableCellExt::fn get_column_span -->
Returns the number of columns occupied by this cell accessible.

# Returns

a gint representing the number of columns occupied by this cell,
or 0 if the cell does not implement this method.
<!-- trait TableCellExt::fn get_position -->
Retrieves the tabular position of this cell.
## `row`
the row of the given cell.
## `column`
the column of the given cell.

# Returns

TRUE if successful, FALSE otherwise.
<!-- trait TableCellExt::fn get_row_column_span -->
Gets the row and column indexes and extents of this cell accessible.
## `row`
the row index of the given cell.
## `column`
the column index of the given cell.
## `row_span`
the number of rows occupied by this cell.
## `column_span`
the number of columns occupied by this cell.
<!-- trait TableCellExt::fn get_row_header_cells -->
Returns the row headers as an array of cell accessibles.

# Returns

a GPtrArray of
AtspiAccessibles representing the row header cells.
<!-- trait TableCellExt::fn get_row_span -->
Returns the number of rows occupied by this cell accessible.

# Returns

a gint representing the number of rows occupied by this cell,
or 0 if the cell does not implement this method.
<!-- trait TableCellExt::fn get_table -->
Returns a reference to the accessible of the containing table.

# Returns

the AtspiAccessible for the containing table.
<!-- struct Text -->


# Implements

[`TextExt`](trait.TextExt.html)
<!-- trait TextExt -->
Trait containing all `Text` methods.

# Implementors

[`Accessible`](struct.Accessible.html), [`Text`](struct.Text.html)
<!-- trait TextExt::fn add_selection -->
Selects some text (adds a text selection) in an `Text` object.
## `start_offset`
the starting offset of the desired new selection.
## `end_offset`
the offset of the first character after the new selection.

# Returns

`true` if successful, `false` otherwise.
<!-- trait TextExt::fn get_attribute_run -->
Gets a set of attributes applied to a range of text from an `Text` object, optionally
including its 'default' attributes.
## `offset`
a `gint` indicating the offset from which the attribute
 search is based.
## `include_defaults`
a `bool` that, when set as `false`, indicates the call
should only return those attributes which are explicitly set on the current
attribute run, omitting any attributes which are inherited from the
default values.
## `start_offset`
a `gint` pointer indicating the start of the desired text
 range.
## `end_offset`
a `gint` pointer indicating the first character past the desired
 range.

# Returns

a `glib::HashTable` with attributes
 defined at the indicated offset, optionally including the 'default' ones.
<!-- trait TextExt::fn get_attribute_value -->
Gets the value of a named attribute at a given offset.

# Deprecated since 2.10

Use atspi_text_get_text_attribute_value instead.
## `offset`
The character offset at which to query the attribute.
## `attribute_name`
The attribute to query.

# Returns

the value of a given attribute at the given
offset, or `None` if not present.
<!-- trait TextExt::fn get_attributes -->
Gets the attributes applied to a range of text from an `Text`
object. The text attributes correspond to CSS attributes
where possible.
`<em>`DEPRECATED`</em>`

# Deprecated since 2.10

Use atspi_text_get_text_attributes instead.
## `offset`
a `gint` indicating the offset from which the attribute
 search is based.
## `start_offset`
a `gint` pointer indicating the start of the desired text
 range.
## `end_offset`
a `gint` pointer indicating the first character past the desired
 range.

# Returns

a `glib::HashTable`
describing the attributes at the given character offset.
<!-- trait TextExt::fn get_bounded_ranges -->
Gets the ranges of text from an `Text` object which lie within the
 bounds defined by (`x`, `y`) and (`x`+`width`, `y`+`height`).
## `x`
the 'starting' x coordinate of the bounding box.
## `y`
the 'starting' y coordinate of the bounding box.
## `width`
the x extent of the bounding box.
## `height`
the y extent of the bounding box.
## `type_`
an `AccessibleCoordType` indicating the coordinate system to use
 for the returned values.
## `clipTypeX`
an `TextClipType` indicating how to treat characters that
 intersect the bounding box's x extents.
## `clipTypeY`
an `TextClipType` indicating how to treat characters that
 intersect the bounding box's y extents.

# Returns

a null-terminated list of
 pointers to `TextRange` structs detailing the bounded text.
<!-- trait TextExt::fn get_caret_offset -->
Gets the current offset of the text caret in an `Text` object.

# Returns

a `gint` indicating the current position of the text caret.
<!-- trait TextExt::fn get_character_at_offset -->
Gets the character at a given offset for an `Text` object.
## `offset`
a `gint` indicating the text offset where the desired
 character is located.

# Returns

a `guint` representing the
 UCS-4 unicode code point of the given character, or
 0xFFFFFFFF if the character in question cannot be represented
 in the UCS-4 encoding.
<!-- trait TextExt::fn get_character_count -->
Gets the character count of an `AccessibleText` object.

# Returns

a `gint` indicating the total number of
 characters in the `AccessibleText` object.
<!-- trait TextExt::fn get_character_extents -->
Gets a bounding box containing the glyph representing
 the character at a particular text offset.
## `offset`
a `gint` indicating the offset of the text character for
 whom boundary information is requested.
## `type_`
an `AccessibleCoordType` indicating the coordinate system to use
 for the returned values.

# Returns

An `Rect` specifying the position and size of the character.
<!-- trait TextExt::fn get_default_attributes -->
Gets the default attributes applied to an `Text`
object. The text attributes correspond to CSS attributes
where possible. The combination of this attribute set and
the attributes reported by `Text::get_attributes`
describes the entire set of text attributes over a range.

# Returns

a `glib::HashTable`
 containing the default attributes applied to a text object,
 (exclusive of explicitly-set attributes), encoded as UTF-8.
<!-- trait TextExt::fn get_n_selections -->
Gets the number of active non-contiguous selections for an
 `Text` object.

# Returns

a `gint` indicating the current
 number of non-contiguous text selections active
 within an `Text` object.
<!-- trait TextExt::fn get_offset_at_point -->
Gets the character offset into the text at a given point.
## `x`
the x coordinate of the point to be queried.
## `y`
the y coordinate of the point to be queried.
## `type_`
an `CoordType` indicating the coordinate system in which
 the values should be returned.

# Returns

the offset (as a `gint`) at the point (`x`, `y`)
 in the specified coordinate system.
<!-- trait TextExt::fn get_range_extents -->
Gets the bounding box for text within a range in an `Text` object.
## `start_offset`
a `gint` indicating the offset of the first text character for
 whom boundary information is requested.
## `end_offset`
a `gint` indicating the offset of the text character
 after the last character for whom boundary information is requested.
## `type_`
an `CoordType` indicating the coordinate system to use
 for the returned values.

# Returns

An `Rect` giving the position and size of the specified range
 of text.
<!-- trait TextExt::fn get_selection -->
Gets the bounds of the `selection_num`-th active text selection for an
 `Text` object.
## `selection_num`
a `gint` indicating which selection to query.
<!-- trait TextExt::fn get_string_at_offset -->
Gets a portion of the text exposed through an `Text` according to a given `offset`
and a specific `granularity`, along with the start and end offsets defining the
boundaries of such a portion of text.

If `granularity` is ATSPI_TEXT_GRANULARITY_CHAR the character at the
offset is returned.

If `granularity` is ATSPI_TEXT_GRANULARITY_WORD the returned string
is from the word start at or before the offset to the word start after
the offset.

The returned string will contain the word at the offset if the offset
is inside a word and will contain the word before the offset if the
offset is not inside a word.

If `granularity` is ATSPI_TEXT_GRANULARITY_SENTENCE the returned string
is from the sentence start at or before the offset to the sentence
start after the offset.

The returned string will contain the sentence at the offset if the offset
is inside a sentence and will contain the sentence before the offset
if the offset is not inside a sentence.

If `granularity` is ATSPI_TEXT_GRANULARITY_LINE the returned string
is from the line start at or before the offset to the line
start after the offset.

If `granularity` is ATSPI_TEXT_GRANULARITY_PARAGRAPH the returned string
is from the start of the paragraph at or before the offset to the start
of the following paragraph after the offset.
## `offset`
position
## `granularity`
An `TextGranularity`

# Returns

a newly allocated string containing the text at the `offset` bounded
 by the specified `granularity`. Use `g_free` to free the returned string.
 Returns `None` if the offset is invalid or no implementation is available.
<!-- trait TextExt::fn get_text -->
Gets a range of text from an `Text` object. The number of bytes
 in the returned string may exceed either end_offset or start_offset, since
 UTF-8 is a variable-width encoding.
## `start_offset`
a `gint` indicating the start of the desired text range.
## `end_offset`
a `gint` indicating the first character past the desired range.

# Returns

a text string containing characters from `start_offset`
 to `end_offset`-1, inclusive, encoded as UTF-8.
<!-- trait TextExt::fn get_text_after_offset -->
Gets delimited text from an `Text` object which follows a given
 text offset.
## `offset`
a `gint` indicating the offset from which the delimiter
 search is based.
## `type_`
an `TextBoundaryType` indicating whether the desired
 text string is a word, sentence, line, or attribute run.

# Returns

an `TextRange` containing a UTF-8 string representing the
 delimited text, both of whose delimiting boundaries are after or
 inclusive of the current offset, or an empty string if no such
 text exists.
<!-- trait TextExt::fn get_text_at_offset -->
Gets delimited text from an `Text` object which includes a given
 text offset.

# Deprecated since 2.10

Use atspi_text_get_string_at_offset.
## `offset`
a `gint` indicating the offset from which the delimiter
 search is based.
## `type_`
an `TextBoundaryType` indicating whether the desired
 text string is a word, sentence, line, or attribute run.

# Returns

an `TextRange` containing a UTF-8 string representing the
 delimited text, whose delimiting boundaries bracket the
 current offset, or an empty string if no such text exists.
<!-- trait TextExt::fn get_text_attribute_value -->
Gets the value of a named attribute at a given offset.
## `offset`
The character offset at which to query the attribute.
## `attribute_name`
The attribute to query.

# Returns

the value of a given attribute at the given offset, or `None` if
not present.
<!-- trait TextExt::fn get_text_attributes -->
Gets the attributes applied to a range of text from an `Text`
object. The text attributes correspond to CSS attributes
where possible.
`<em>`DEPRECATED`</em>`
## `offset`
a `gint` indicating the offset from which the attribute
 search is based.
## `start_offset`
a `gint` pointer indicating the start of the desired text
 range.
## `end_offset`
a `gint` pointer indicating the first character past the desired
 range.

# Returns

a `glib::HashTable`
describing the attributes at the given character offset.
<!-- trait TextExt::fn get_text_before_offset -->
Gets delimited text from an `Text` object which precedes a given
 text offset.
## `offset`
a `gint` indicating the offset from which the delimiter
 search is based.
## `type_`
an `TextBoundaryType` indicating whether the desired
 text string is a word, sentence, line, or attribute run.

# Returns

an `TextRange` containing a UTF-8 string representing the
 delimited text, both of whose delimiting boundaries are before the
 current offset, or an empty string if no such text exists.
<!-- trait TextExt::fn remove_selection -->
De-selects a text selection.
## `selection_num`
a `gint` indicating which text selection to remove.

# Returns

`true` if successful, `false` otherwise.
<!-- trait TextExt::fn set_caret_offset -->
Moves the text caret to a given position.
## `new_offset`
the offset to which the text caret is to be moved.

# Returns

`true` if successful, `false` otherwise.
<!-- trait TextExt::fn set_selection -->
Changes the bounds of an existing `Text` text selection.
## `selection_num`
a zero-offset index indicating which text selection to modify.
## `start_offset`
a `gint` indicating the new starting offset for the selection.
## `end_offset`
a `gint` indicating the desired new offset of the first character
 after the selection.

# Returns

`true` if successful, `false` otherwise.
<!-- enum TextBoundaryType -->
Specifies the boundary conditions determining a run of text as returned from
`Text::get_text_at_offset`, `Text::get_text_after_offset`, and
`Text::get_text_before_offset`.

This enumerationis deprecated since 2.9.90 and should not be used. Use
AtspiTextGranularity with `Text::get_string_at_offset` instead.
<!-- enum TextBoundaryType::variant Char -->
An `Text` instance is bounded by this
character only. Start and end offsets differ by one, by definition,
for this value.
<!-- enum TextBoundaryType::variant WordStart -->
Boundary condition is start of a word; i.e.
range is from start of one word to the start of another word.
<!-- enum TextBoundaryType::variant WordEnd -->
Boundary condition is the end of a word; i.e.
range is from the end of one word to the end of another. Some locales
may not distinguish between words and characters or glyphs. In particular,
those locales which use wholly or partially ideographic character sets.
In these cases, characters may be returned in lieu of multi-character
substrings.
<!-- enum TextBoundaryType::variant SentenceStart -->
Boundary condition is start of a
sentence, as determined by the application. Some locales or
character sets may not include explicit sentence delimiters, so this
boundary type can not always be honored. Some locales will return lines
of text instead of grammatical sentences.
<!-- enum TextBoundaryType::variant SentenceEnd -->
Boundary condition is end of a sentence,
as determined by the application, including the sentence-delimiting
character, for instance '.' Some locales or character sets may not
include explicit sentence delimiters, so this boundary type can not
always be honored. Some locales will return lines of text instead of
grammatical sentences.
<!-- enum TextBoundaryType::variant LineStart -->
Boundary condition is the start of a line;
i.e. range is from start of one line to the start of another. This
generally means that an end-of-line character will appear at the end of
the range.
<!-- enum TextBoundaryType::variant LineEnd -->
Boundary condition is the end of a line; i.e.
range is from start of one line to the start of another. This generally
means that an end-of-line character will be the first character of the
range.
<!-- enum TextClipType -->
Enumeration used by interface `Text` to indicate
how to treat characters intersecting bounding boxes.
<!-- enum TextClipType::variant None -->
No characters/glyphs are omitted.
<!-- enum TextClipType::variant Min -->
Characters/glyphs clipped by the minimum coordinate
are omitted.
<!-- enum TextClipType::variant Max -->
Characters/glyphs which intersect the maximum
coordinate are omitted.
<!-- enum TextClipType::variant Both -->
Only glyphs falling entirely within the region
bounded by min and max are retained.
<!-- enum TextGranularity -->
Text granularity types used for specifying the granularity of the region of
text we are interested in.
<!-- enum TextGranularity::variant Char -->
Granularity is defined by the boundaries between characters
(including non-printing characters)
<!-- enum TextGranularity::variant Word -->
Granularity is defined by the boundaries of a word,
starting at the beginning of the current word and finishing at the beginning of
the following one, if present.
<!-- enum TextGranularity::variant Sentence -->
Granularity is defined by the boundaries of a sentence,
starting at the beginning of the current sentence and finishing at the beginning of
the following one, if present.
<!-- enum TextGranularity::variant Line -->
Granularity is defined by the boundaries of a line,
starting at the beginning of the current line and finishing at the beginning of
the following one, if present.
<!-- enum TextGranularity::variant Paragraph -->
Granularity is defined by the boundaries of a paragraph,
starting at the beginning of the current paragraph and finishing at the beginning of
the following one, if present.
<!-- struct TextRange -->
<!-- struct Value -->


# Implements

[`ValueExt`](trait.ValueExt.html)
<!-- trait ValueExt -->
Trait containing all `Value` methods.

# Implementors

[`Accessible`](struct.Accessible.html), [`Value`](struct.Value.html)
<!-- trait ValueExt::fn get_current_value -->
Gets the current value for an `Value`.

# Returns

the current value for this object.
<!-- trait ValueExt::fn get_maximum_value -->
Gets the maximum allowed value for an `Value`.

# Returns

the maximum allowed value for this object.
<!-- trait ValueExt::fn get_minimum_increment -->
Gets the minimum increment by which an `Value` can be adjusted.

# Returns

the minimum increment by which the value may be changed, or
zero if the minimum increment cannot be determined.
<!-- trait ValueExt::fn get_minimum_value -->
Gets the minimum allowed value for an `Value`.

# Returns

the minimum allowed value for this object.
<!-- trait ValueExt::fn set_current_value -->
Sets the current value of an `Value`.
## `new_value`
a `gdouble` value which is the desired new value of the object.

# Returns

`true` if the value could be assigned the specified value,
 `false` otherwise.
