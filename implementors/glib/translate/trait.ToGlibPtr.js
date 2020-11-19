(function() {var implementors = {};
implementors["cairo"] = [{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *mut cairo_t&gt; for &amp;'a Context","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *mut cairo_device_t&gt; for Device","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *mut cairo_surface_t&gt; for ImageSurface","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *mut cairo_surface_t&gt; for RecordingSurface","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *mut cairo_surface_t&gt; for Surface","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *mut xcb_connection_t&gt; for &amp;'a XCBConnection","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *mut xcb_render_pictforminfo_t&gt; for &amp;'a XCBRenderPictFormInfo","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *mut xcb_screen_t&gt; for &amp;'a XCBScreen","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *mut cairo_surface_t&gt; for XCBSurface","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *mut xcb_visualtype_t&gt; for &amp;'a XCBVisualType","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *mut cairo_surface_t&gt; for PdfSurface","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *mut cairo_surface_t&gt; for PsSurface","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *mut cairo_surface_t&gt; for SvgSurface","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *mut cairo_surface_t&gt; for QuartzSurface","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *mut cairo_surface_t&gt; for Win32Surface","synthetic":false,"types":[]}];
implementors["gdk"] = [{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *const GdkEventAny&gt; for Event","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *mut _GdkAtom&gt; for Atom","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *const GdkEventButton&gt; for EventButton","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *const GdkEventConfigure&gt; for EventConfigure","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *const GdkEventCrossing&gt; for EventCrossing","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *const GdkEventDND&gt; for EventDND","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *const GdkEventExpose&gt; for EventExpose","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *const GdkEventFocus&gt; for EventFocus","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *const GdkEventGrabBroken&gt; for EventGrabBroken","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *const GdkEventKey&gt; for EventKey","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *const GdkEventMotion&gt; for EventMotion","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *const GdkEventOwnerChange&gt; for EventOwnerChange","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *const GdkEventPadAxis&gt; for EventPadAxis","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *const GdkEventPadButton&gt; for EventPadButton","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *const GdkEventPadGroupMode&gt; for EventPadGroupMode","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *const GdkEventProperty&gt; for EventProperty","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *const GdkEventProximity&gt; for EventProximity","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *const GdkEventScroll&gt; for EventScroll","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *const GdkEventSelection&gt; for EventSelection","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *const GdkEventSetting&gt; for EventSetting","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *const GdkEventTouch&gt; for EventTouch","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *const GdkEventTouchpadPinch&gt; for EventTouchpadPinch","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *const GdkEventTouchpadSwipe&gt; for EventTouchpadSwipe","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *const GdkEventVisibility&gt; for EventVisibility","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *const GdkEventWindowState&gt; for EventWindowState","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; ToGlibPtr&lt;'a, *mut GdkWindowAttr&gt; for WindowAttr","synthetic":false,"types":[]}];
implementors["glib"] = [];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()