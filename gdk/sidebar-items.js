initSidebarItems({"constant":[["ATOM_NONE",""],["BUTTON_MIDDLE","The middle button."],["BUTTON_PRIMARY","The primary button. This is typically the left mouse button, or the right button in a left-handed setup."],["BUTTON_SECONDARY","The secondary button. This is typically the right mouse button, or the left button in a left-handed setup."],["EVENT_PROPAGATE",""],["EVENT_STOP",""],["NONE_DEVICE_PAD",""],["NONE_WINDOW",""],["SELECTION_CLIPBOARD",""],["SELECTION_PRIMARY",""],["SELECTION_SECONDARY",""],["SELECTION_TYPE_ATOM",""],["SELECTION_TYPE_BITMAP",""],["SELECTION_TYPE_COLORMAP",""],["SELECTION_TYPE_DRAWABLE",""],["SELECTION_TYPE_INTEGER",""],["SELECTION_TYPE_PIXMAP",""],["SELECTION_TYPE_STRING",""],["SELECTION_TYPE_WINDOW",""],["TARGET_BITMAP",""],["TARGET_COLORMAP",""],["TARGET_DRAWABLE",""],["TARGET_PIXMAP",""],["TARGET_STRING",""]],"enum":[["AxisUse","An enumeration describing the way in which a device axis (valuator) maps onto the predefined valuator types that GTK+ understands."],["ByteOrder","A set of values describing the possible byte-orders for storing pixel values in memory."],["ChangeData",""],["CrossingMode","Specifies the crossing mode for `EventCrossing`."],["CursorType","Predefined cursors."],["DevicePadFeature","A pad feature."],["DeviceToolType","Indicates the specific type of tool being used being a tablet. Such as an airbrush, pencil, etc."],["DeviceType","Indicates the device type. See [above][`DeviceManager`.description] for more information about the meaning of these device types."],["DragCancelReason","Used in `DragContext` to the reason of a cancelled DND operation."],["DragProtocol","Used in `DragContext` to indicate the protocol according to which DND is done."],["EventType","Specifies the type of the event."],["FullscreenMode","Indicates which monitor (in a multi-head setup) a window should span over when in fullscreen mode."],["GLError","Error enumeration for `GLContext`."],["GrabOwnership","Defines how device grabs interact with other devices."],["GrabStatus","Returned by `Device::grab`, `gdk_pointer_grab` and `gdk_keyboard_grab` to indicate success or the reason for the failure of the grab attempt."],["Gravity","Defines the reference point of a window and the meaning of coordinates passed to `gtk_window_move`. See `gtk_window_move` and the \"implementation notes\" section of the Extended Window Manager Hints specification for more details. The `Gravity` type represents the orientation of glyphs in a segment of text. This is useful when rendering vertical text layouts. In those situations, the layout is rotated using a non-identity PangoMatrix, and then glyph orientation is controlled using `Gravity`. Not every value in this enumeration makes sense for every usage of `Gravity`; for example, `Gravity::Auto` only can be passed to `Context::set_base_gravity` and can only be returned by `Context::get_base_gravity`."],["InputMode","An enumeration that describes the mode of an input device."],["InputSource","An enumeration describing the type of an input device in general terms."],["ModifierIntent","This enum is used with `Keymap::get_modifier_mask` in order to determine what modifiers the currently used windowing system backend uses for particular purposes. For example, on X11/Windows, the Control key is used for invoking menu shortcuts (accelerators), whereas on Apple computers it’s the Command key (which correspond to `ModifierType::ControlMask` and `ModifierType::Mod2Mask`, respectively)."],["NotifyType","Specifies the kind of crossing for `EventCrossing`."],["OwnerChange","Specifies why a selection ownership was changed."],["PropMode","Describes how existing data is combined with new data when using `gdk_property_change`."],["PropertyState","Specifies the type of a property change for a `EventProperty`."],["ScrollDirection","Specifies the direction for `EventScroll`."],["SettingAction","Specifies the kind of modification applied to a setting in a `EventSetting`."],["SubpixelLayout","This enumeration describes how the red, green and blue components of physical pixels on an output device are laid out."],["VisibilityState","Specifies the visiblity status of a window for a `EventVisibility`."],["VisualType","A set of values that describe the manner in which the pixel values for a visual are converted into RGB values for display."],["WindowEdge","Determines a window edge or corner."],["WindowType","Describes the kind of window. A `Window` can be one of these types. Most things you’d consider a “window” should have type `WindowType::Toplevel`; windows with this type are managed by the window manager and have a frame by default (call `GtkWindowExt::set_decorated` to toggle the frame). Windows with type `WindowType::Popup` are ignored by the window manager; window manager keybindings won’t work on them, the window manager won’t decorate the window with a frame, many GTK+ features that rely on the window manager will not work (e.g. resize grips and maximization/minimization). `WindowType::Popup` is used to implement widgets such as `Menu` or tooltips that you normally don’t think of as windows per se. Nearly all windows should be `WindowType::Toplevel`. In particular, do not use `WindowType::Popup` just to turn off the window borders; use `GtkWindowExt::set_decorated` for that."],["WindowTypeHint","These are hints for the window manager that indicate what type of function the window has. The window manager can use this when determining decoration and behaviour of the window. The hint must be set before mapping the window."],["WindowWindowClass","`WindowWindowClass::InputOutput` windows are the standard kind of window you might expect. Such windows receive events and are also displayed on screen. `WindowWindowClass::InputOnly` windows are invisible; they are usually placed above other windows in order to trap or filter the events. You can’t draw on `WindowWindowClass::InputOnly` windows."]],"fn":[["beep",""],["error_trap_pop",""],["error_trap_pop_ignored",""],["error_trap_push",""],["events_get_angle",""],["events_get_center",""],["events_get_distance",""],["events_pending",""],["flush",""],["get_display_arg_name",""],["get_program_class",""],["get_show_events",""],["init",""],["keyval_convert_case",""],["keyval_from_name",""],["keyval_is_lower",""],["keyval_is_upper",""],["keyval_to_lower",""],["keyval_to_upper",""],["list_visuals",""],["notify_startup_complete",""],["notify_startup_complete_with_id",""],["pango_context_get",""],["pango_context_get_for_display",""],["pango_context_get_for_screen",""],["pango_layout_get_clip_region",""],["pango_layout_line_get_clip_region",""],["pixbuf_get_from_surface",""],["pre_parse_libgtk_only",""],["property_change",""],["property_delete",""],["property_get",""],["query_depths",""],["selection_convert",""],["selection_owner_get",""],["selection_owner_get_for_display",""],["selection_owner_set",""],["selection_owner_set_for_display",""],["selection_send_notify",""],["selection_send_notify_for_display",""],["set_allowed_backends",""],["set_double_click_time",""],["set_initialized","Informs this crate that GDK has been initialized and the current thread is the main one."],["set_program_class",""],["set_show_events",""],["setting_get",""],["synthesize_window_state",""],["test_render_sync",""],["test_simulate_button",""],["test_simulate_key",""],["text_property_to_utf8_list_for_display",""],["unicode_to_keyval",""],["utf8_to_string_target",""]],"mod":[["functions",""],["keys",""],["prelude","Traits intended for blanket imports."]],"struct":[["AnchorHints",""],["AppLaunchContext",""],["Atom",""],["AxisFlags",""],["Color",""],["Cursor",""],["Device",""],["DeviceManager",""],["DevicePad",""],["DeviceTool",""],["Display",""],["DisplayManager",""],["DragAction",""],["DragContext",""],["DrawingContext",""],["Event","A generic GDK event."],["EventButton",""],["EventConfigure",""],["EventCrossing",""],["EventDND",""],["EventExpose",""],["EventFocus",""],["EventGrabBroken",""],["EventKey",""],["EventMask",""],["EventMotion",""],["EventOwnerChange",""],["EventPadAxis",""],["EventPadButton",""],["EventPadGroupMode",""],["EventProperty",""],["EventProximity",""],["EventScroll",""],["EventSelection",""],["EventSequence",""],["EventSetting",""],["EventTouch",""],["EventTouchpadPinch",""],["EventTouchpadSwipe",""],["EventVisibility",""],["EventWindowState",""],["FrameClock",""],["FrameClockPhase",""],["FrameTimings",""],["GLContext",""],["GRange",""],["Geometry",""],["Keymap",""],["KeymapKey",""],["ModifierType",""],["Monitor",""],["RGBA","A `RGBA` is used to represent a (possibly translucent) color, in a way that is compatible with cairo’s notion of color."],["Rectangle",""],["RgbaParseError",""],["Screen",""],["Seat",""],["SeatCapabilities",""],["TimeCoord",""],["Visual",""],["WMDecoration",""],["WMFunction",""],["Window",""],["WindowAttr",""],["WindowHints",""],["WindowState",""]],"trait":[["DevicePadExt","Trait containing all `DevicePad` methods."],["WindowExt","Trait containing all `Window` methods."]],"type":[["key",""]]});