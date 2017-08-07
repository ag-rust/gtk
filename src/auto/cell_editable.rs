// This file was generated by gir (3294959) from gir-files (0bcaef9)
// DO NOT EDIT

use Widget;
use ffi;
use gdk;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct CellEditable(Object<ffi::GtkCellEditable>): Widget;

    match fn {
        get_type => || ffi::gtk_cell_editable_get_type(),
    }
}

pub trait CellEditableExt {
    fn editing_done(&self);

    fn remove_widget(&self);

    fn start_editing<'a, P: Into<Option<&'a gdk::Event>>>(&self, event: P);

    fn get_property_editing_canceled(&self) -> bool;

    fn set_property_editing_canceled(&self, editing_canceled: bool);

    fn connect_editing_done<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_remove_widget<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<CellEditable> + IsA<glib::object::Object>> CellEditableExt for O {
    fn editing_done(&self) {
        unsafe {
            ffi::gtk_cell_editable_editing_done(self.to_glib_none().0);
        }
    }

    fn remove_widget(&self) {
        unsafe {
            ffi::gtk_cell_editable_remove_widget(self.to_glib_none().0);
        }
    }

    fn start_editing<'a, P: Into<Option<&'a gdk::Event>>>(&self, event: P) {
        let event = event.into();
        unsafe {
            ffi::gtk_cell_editable_start_editing(self.to_glib_none().0, mut_override(event.to_glib_none().0));
        }
    }

    fn get_property_editing_canceled(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "editing-canceled".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_editing_canceled(&self, editing_canceled: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "editing-canceled".to_glib_none().0, Value::from(&editing_canceled).to_glib_none().0);
        }
    }

    fn connect_editing_done<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "editing-done",
                transmute(editing_done_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_remove_widget<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "remove-widget",
                transmute(remove_widget_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn editing_done_trampoline<P>(this: *mut ffi::GtkCellEditable, f: glib_ffi::gpointer)
where P: IsA<CellEditable> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellEditable::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn remove_widget_trampoline<P>(this: *mut ffi::GtkCellEditable, f: glib_ffi::gpointer)
where P: IsA<CellEditable> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellEditable::from_glib_none(this).downcast_unchecked())
}
