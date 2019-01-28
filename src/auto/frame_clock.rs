// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v3_8", feature = "dox"))]
use FrameClockPhase;
#[cfg(any(feature = "v3_8", feature = "dox"))]
use FrameTimings;
use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct FrameClock(Object<ffi::GdkFrameClock, ffi::GdkFrameClockClass, FrameClockClass>);

    match fn {
        get_type => || ffi::gdk_frame_clock_get_type(),
    }
}

pub const NONE_FRAME_CLOCK: Option<&FrameClock> = None;

pub trait FrameClockExt: 'static {
    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn begin_updating(&self);

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn end_updating(&self);

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn get_current_timings(&self) -> Option<FrameTimings>;

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn get_frame_counter(&self) -> i64;

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn get_frame_time(&self) -> i64;

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn get_history_start(&self) -> i64;

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn get_timings(&self, frame_counter: i64) -> Option<FrameTimings>;

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn request_phase(&self, phase: FrameClockPhase);

    fn connect_after_paint<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_before_paint<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_flush_events<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_layout<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_paint<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_resume_events<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_update<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FrameClock>> FrameClockExt for O {
    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn begin_updating(&self) {
        unsafe {
            ffi::gdk_frame_clock_begin_updating(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn end_updating(&self) {
        unsafe {
            ffi::gdk_frame_clock_end_updating(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn get_current_timings(&self) -> Option<FrameTimings> {
        unsafe {
            from_glib_none(ffi::gdk_frame_clock_get_current_timings(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn get_frame_counter(&self) -> i64 {
        unsafe {
            ffi::gdk_frame_clock_get_frame_counter(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn get_frame_time(&self) -> i64 {
        unsafe {
            ffi::gdk_frame_clock_get_frame_time(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn get_history_start(&self) -> i64 {
        unsafe {
            ffi::gdk_frame_clock_get_history_start(self.as_ref().to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn get_timings(&self, frame_counter: i64) -> Option<FrameTimings> {
        unsafe {
            from_glib_none(ffi::gdk_frame_clock_get_timings(self.as_ref().to_glib_none().0, frame_counter))
        }
    }

    #[cfg(any(feature = "v3_8", feature = "dox"))]
    fn request_phase(&self, phase: FrameClockPhase) {
        unsafe {
            ffi::gdk_frame_clock_request_phase(self.as_ref().to_glib_none().0, phase.to_glib());
        }
    }

    fn connect_after_paint<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"after-paint\0".as_ptr() as *const _,
                Some(transmute(after_paint_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_before_paint<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"before-paint\0".as_ptr() as *const _,
                Some(transmute(before_paint_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_flush_events<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"flush-events\0".as_ptr() as *const _,
                Some(transmute(flush_events_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_layout<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"layout\0".as_ptr() as *const _,
                Some(transmute(layout_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_paint<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"paint\0".as_ptr() as *const _,
                Some(transmute(paint_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_resume_events<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"resume-events\0".as_ptr() as *const _,
                Some(transmute(resume_events_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_update<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"update\0".as_ptr() as *const _,
                Some(transmute(update_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn after_paint_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GdkFrameClock, f: glib_ffi::gpointer)
where P: IsA<FrameClock> {
    let f: &F = transmute(f);
    f(&FrameClock::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn before_paint_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GdkFrameClock, f: glib_ffi::gpointer)
where P: IsA<FrameClock> {
    let f: &F = transmute(f);
    f(&FrameClock::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn flush_events_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GdkFrameClock, f: glib_ffi::gpointer)
where P: IsA<FrameClock> {
    let f: &F = transmute(f);
    f(&FrameClock::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn layout_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GdkFrameClock, f: glib_ffi::gpointer)
where P: IsA<FrameClock> {
    let f: &F = transmute(f);
    f(&FrameClock::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn paint_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GdkFrameClock, f: glib_ffi::gpointer)
where P: IsA<FrameClock> {
    let f: &F = transmute(f);
    f(&FrameClock::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn resume_events_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GdkFrameClock, f: glib_ffi::gpointer)
where P: IsA<FrameClock> {
    let f: &F = transmute(f);
    f(&FrameClock::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn update_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GdkFrameClock, f: glib_ffi::gpointer)
where P: IsA<FrameClock> {
    let f: &F = transmute(f);
    f(&FrameClock::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for FrameClock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FrameClock")
    }
}
