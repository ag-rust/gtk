// This file was generated by gir (26020e8) from gir-files (11e0e6d)
// DO NOT EDIT

use Actionable;
use Adjustment;
use Bin;
use Button;
use Container;
use Orientable;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct ScaleButton(Object<ffi::GtkScaleButton>): Button, Bin, Container, Widget, Actionable, Orientable;

    match fn {
        get_type => || ffi::gtk_scale_button_get_type(),
    }
}

impl ScaleButton {
    pub fn new(size: i32, min: f64, max: f64, step: f64, icons: &[&str]) -> ScaleButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_scale_button_new(size, min, max, step, icons.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait ScaleButtonExt {
    fn get_adjustment(&self) -> Adjustment;

    fn get_minus_button(&self) -> Option<Widget>;

    fn get_plus_button(&self) -> Option<Widget>;

    fn get_popup(&self) -> Option<Widget>;

    fn get_value(&self) -> f64;

    fn set_adjustment(&self, adjustment: &Adjustment);

    fn set_icons(&self, icons: &[&str]);

    fn set_value(&self, value: f64);
}

impl<O: IsA<ScaleButton>> ScaleButtonExt for O {
    fn get_adjustment(&self) -> Adjustment {
        unsafe {
            from_glib_none(ffi::gtk_scale_button_get_adjustment(self.to_glib_none().0))
        }
    }

    fn get_minus_button(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_scale_button_get_minus_button(self.to_glib_none().0))
        }
    }

    fn get_plus_button(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_scale_button_get_plus_button(self.to_glib_none().0))
        }
    }

    fn get_popup(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_scale_button_get_popup(self.to_glib_none().0))
        }
    }

    fn get_value(&self) -> f64 {
        unsafe {
            ffi::gtk_scale_button_get_value(self.to_glib_none().0)
        }
    }

    fn set_adjustment(&self, adjustment: &Adjustment) {
        unsafe {
            ffi::gtk_scale_button_set_adjustment(self.to_glib_none().0, adjustment.to_glib_none().0);
        }
    }

    fn set_icons(&self, icons: &[&str]) {
        unsafe {
            ffi::gtk_scale_button_set_icons(self.to_glib_none().0, icons.to_glib_none().0);
        }
    }

    fn set_value(&self, value: f64) {
        unsafe {
            ffi::gtk_scale_button_set_value(self.to_glib_none().0, value);
        }
    }
}
