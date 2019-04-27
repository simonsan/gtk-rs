// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
use IconTheme;
use StyleContext;
use cairo;
#[cfg(feature = "futures")]
use futures::future;
use gdk;
use gdk_pixbuf;
use gio;
use gio_sys;
use glib::object::IsA;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std;
#[cfg(feature = "futures")]
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct IconInfo(Object<gtk_sys::GtkIconInfo, gtk_sys::GtkIconInfoClass, IconInfoClass>);

    match fn {
        get_type => || gtk_sys::gtk_icon_info_get_type(),
    }
}

impl IconInfo {
    pub fn new_for_pixbuf<P: IsA<IconTheme>>(icon_theme: &P, pixbuf: &gdk_pixbuf::Pixbuf) -> IconInfo {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gtk_sys::gtk_icon_info_new_for_pixbuf(icon_theme.as_ref().to_glib_none().0, pixbuf.to_glib_none().0))
        }
    }
}

pub const NONE_ICON_INFO: Option<&IconInfo> = None;

pub trait IconInfoExt: 'static {
    fn get_base_scale(&self) -> i32;

    fn get_base_size(&self) -> i32;

    fn get_filename(&self) -> Option<std::path::PathBuf>;

    fn is_symbolic(&self) -> bool;

    fn load_icon(&self) -> Result<gdk_pixbuf::Pixbuf, Error>;

    fn load_icon_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<gdk_pixbuf::Pixbuf, Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q);

    #[cfg(feature = "futures")]
    fn load_icon_async_future(&self) -> Box_<future::Future<Output = Result<gdk_pixbuf::Pixbuf, Error>> + std::marker::Unpin>;

    fn load_surface<P: IsA<gdk::Window>>(&self, for_window: Option<&P>) -> Result<cairo::Surface, Error>;

    fn load_symbolic(&self, fg: &gdk::RGBA, success_color: Option<&gdk::RGBA>, warning_color: Option<&gdk::RGBA>, error_color: Option<&gdk::RGBA>) -> Result<(gdk_pixbuf::Pixbuf, bool), Error>;

    fn load_symbolic_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(gdk_pixbuf::Pixbuf, bool), Error>) + Send + 'static>(&self, fg: &gdk::RGBA, success_color: Option<&gdk::RGBA>, warning_color: Option<&gdk::RGBA>, error_color: Option<&gdk::RGBA>, cancellable: Option<&P>, callback: Q);

    #[cfg(feature = "futures")]
    fn load_symbolic_async_future(&self, fg: &gdk::RGBA, success_color: Option<&gdk::RGBA>, warning_color: Option<&gdk::RGBA>, error_color: Option<&gdk::RGBA>) -> Box_<future::Future<Output = Result<(gdk_pixbuf::Pixbuf, bool), Error>> + std::marker::Unpin>;

    fn load_symbolic_for_context<P: IsA<StyleContext>>(&self, context: &P) -> Result<(gdk_pixbuf::Pixbuf, bool), Error>;

    fn load_symbolic_for_context_async<P: IsA<StyleContext>, Q: IsA<gio::Cancellable>, R: FnOnce(Result<(gdk_pixbuf::Pixbuf, bool), Error>) + Send + 'static>(&self, context: &P, cancellable: Option<&Q>, callback: R);

    #[cfg(feature = "futures")]
    fn load_symbolic_for_context_async_future<P: IsA<StyleContext> + Clone + 'static>(&self, context: &P) -> Box_<future::Future<Output = Result<(gdk_pixbuf::Pixbuf, bool), Error>> + std::marker::Unpin>;
}

impl<O: IsA<IconInfo>> IconInfoExt for O {
    fn get_base_scale(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_icon_info_get_base_scale(self.as_ref().to_glib_none().0)
        }
    }

    fn get_base_size(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_icon_info_get_base_size(self.as_ref().to_glib_none().0)
        }
    }

    fn get_filename(&self) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_none(gtk_sys::gtk_icon_info_get_filename(self.as_ref().to_glib_none().0))
        }
    }

    fn is_symbolic(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_icon_info_is_symbolic(self.as_ref().to_glib_none().0))
        }
    }

    fn load_icon(&self) -> Result<gdk_pixbuf::Pixbuf, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gtk_sys::gtk_icon_info_load_icon(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn load_icon_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<gdk_pixbuf::Pixbuf, Error>) + Send + 'static>(&self, cancellable: Option<&P>, callback: Q) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn load_icon_async_trampoline<Q: FnOnce(Result<gdk_pixbuf::Pixbuf, Error>) + Send + 'static>(_source_object: *mut gobject_sys::GObject, res: *mut gio_sys::GAsyncResult, user_data: glib_sys::gpointer) {
            let mut error = ptr::null_mut();
            let ret = gtk_sys::gtk_icon_info_load_icon_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = load_icon_async_trampoline::<Q>;
        unsafe {
            gtk_sys::gtk_icon_info_load_icon_async(self.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn load_icon_async_future(&self) -> Box_<future::Future<Output = Result<gdk_pixbuf::Pixbuf, Error>> + std::marker::Unpin> {
        use gio::GioFuture;
        use fragile::Fragile;

        GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            obj.load_icon_async(
                Some(&cancellable),
                move |res| {
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    fn load_surface<P: IsA<gdk::Window>>(&self, for_window: Option<&P>) -> Result<cairo::Surface, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gtk_sys::gtk_icon_info_load_surface(self.as_ref().to_glib_none().0, for_window.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn load_symbolic(&self, fg: &gdk::RGBA, success_color: Option<&gdk::RGBA>, warning_color: Option<&gdk::RGBA>, error_color: Option<&gdk::RGBA>) -> Result<(gdk_pixbuf::Pixbuf, bool), Error> {
        unsafe {
            let mut was_symbolic = mem::uninitialized();
            let mut error = ptr::null_mut();
            let ret = gtk_sys::gtk_icon_info_load_symbolic(self.as_ref().to_glib_none().0, fg.to_glib_none().0, success_color.to_glib_none().0, warning_color.to_glib_none().0, error_color.to_glib_none().0, &mut was_symbolic, &mut error);
            if error.is_null() { Ok((from_glib_full(ret), from_glib(was_symbolic))) } else { Err(from_glib_full(error)) }
        }
    }

    fn load_symbolic_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result<(gdk_pixbuf::Pixbuf, bool), Error>) + Send + 'static>(&self, fg: &gdk::RGBA, success_color: Option<&gdk::RGBA>, warning_color: Option<&gdk::RGBA>, error_color: Option<&gdk::RGBA>, cancellable: Option<&P>, callback: Q) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn load_symbolic_async_trampoline<Q: FnOnce(Result<(gdk_pixbuf::Pixbuf, bool), Error>) + Send + 'static>(_source_object: *mut gobject_sys::GObject, res: *mut gio_sys::GAsyncResult, user_data: glib_sys::gpointer) {
            let mut error = ptr::null_mut();
            let mut was_symbolic = mem::uninitialized();
            let ret = gtk_sys::gtk_icon_info_load_symbolic_finish(_source_object as *mut _, res, &mut was_symbolic, &mut error);
            let result = if error.is_null() { Ok((from_glib_full(ret), from_glib(was_symbolic))) } else { Err(from_glib_full(error)) };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = load_symbolic_async_trampoline::<Q>;
        unsafe {
            gtk_sys::gtk_icon_info_load_symbolic_async(self.as_ref().to_glib_none().0, fg.to_glib_none().0, success_color.to_glib_none().0, warning_color.to_glib_none().0, error_color.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn load_symbolic_async_future(&self, fg: &gdk::RGBA, success_color: Option<&gdk::RGBA>, warning_color: Option<&gdk::RGBA>, error_color: Option<&gdk::RGBA>) -> Box_<future::Future<Output = Result<(gdk_pixbuf::Pixbuf, bool), Error>> + std::marker::Unpin> {
        use gio::GioFuture;
        use fragile::Fragile;

        let fg = fg.clone();
        let success_color = success_color.map(ToOwned::to_owned);
        let warning_color = warning_color.map(ToOwned::to_owned);
        let error_color = error_color.map(ToOwned::to_owned);
        GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            obj.load_symbolic_async(
                &fg,
                success_color.as_ref().map(::std::borrow::Borrow::borrow),
                warning_color.as_ref().map(::std::borrow::Borrow::borrow),
                error_color.as_ref().map(::std::borrow::Borrow::borrow),
                Some(&cancellable),
                move |res| {
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }

    fn load_symbolic_for_context<P: IsA<StyleContext>>(&self, context: &P) -> Result<(gdk_pixbuf::Pixbuf, bool), Error> {
        unsafe {
            let mut was_symbolic = mem::uninitialized();
            let mut error = ptr::null_mut();
            let ret = gtk_sys::gtk_icon_info_load_symbolic_for_context(self.as_ref().to_glib_none().0, context.as_ref().to_glib_none().0, &mut was_symbolic, &mut error);
            if error.is_null() { Ok((from_glib_full(ret), from_glib(was_symbolic))) } else { Err(from_glib_full(error)) }
        }
    }

    fn load_symbolic_for_context_async<P: IsA<StyleContext>, Q: IsA<gio::Cancellable>, R: FnOnce(Result<(gdk_pixbuf::Pixbuf, bool), Error>) + Send + 'static>(&self, context: &P, cancellable: Option<&Q>, callback: R) {
        let user_data: Box<R> = Box::new(callback);
        unsafe extern "C" fn load_symbolic_for_context_async_trampoline<R: FnOnce(Result<(gdk_pixbuf::Pixbuf, bool), Error>) + Send + 'static>(_source_object: *mut gobject_sys::GObject, res: *mut gio_sys::GAsyncResult, user_data: glib_sys::gpointer) {
            let mut error = ptr::null_mut();
            let mut was_symbolic = mem::uninitialized();
            let ret = gtk_sys::gtk_icon_info_load_symbolic_for_context_finish(_source_object as *mut _, res, &mut was_symbolic, &mut error);
            let result = if error.is_null() { Ok((from_glib_full(ret), from_glib(was_symbolic))) } else { Err(from_glib_full(error)) };
            let callback: Box<R> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = load_symbolic_for_context_async_trampoline::<R>;
        unsafe {
            gtk_sys::gtk_icon_info_load_symbolic_for_context_async(self.as_ref().to_glib_none().0, context.as_ref().to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn load_symbolic_for_context_async_future<P: IsA<StyleContext> + Clone + 'static>(&self, context: &P) -> Box_<future::Future<Output = Result<(gdk_pixbuf::Pixbuf, bool), Error>> + std::marker::Unpin> {
        use gio::GioFuture;
        use fragile::Fragile;

        let context = context.clone();
        GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let send = Fragile::new(send);
            obj.load_symbolic_for_context_async(
                &context,
                Some(&cancellable),
                move |res| {
                    let _ = send.into_inner().send(res);
                },
            );

            cancellable
        })
    }
}

impl fmt::Display for IconInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IconInfo")
    }
}
