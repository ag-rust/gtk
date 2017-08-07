// This file was generated by gir (3294959) from gir-files (0bcaef9)
// DO NOT EDIT

use Container;
use DirectionType;
use NotebookTab;
use PackType;
use PositionType;
use Widget;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Notebook(Object<ffi::GtkNotebook>): Container, Widget;

    match fn {
        get_type => || ffi::gtk_notebook_get_type(),
    }
}

impl Notebook {
    pub fn new() -> Notebook {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_notebook_new()).downcast_unchecked()
        }
    }
}

impl Default for Notebook {
    fn default() -> Self {
        Self::new()
    }
}

pub trait NotebookExt {
    #[cfg(feature = "v3_16")]
    fn detach_tab<P: IsA<Widget>>(&self, child: &P);

    fn get_action_widget(&self, pack_type: PackType) -> Option<Widget>;

    fn get_group_name(&self) -> Option<String>;

    fn get_menu_label<P: IsA<Widget>>(&self, child: &P) -> Option<Widget>;

    fn get_menu_label_text<P: IsA<Widget>>(&self, child: &P) -> Option<String>;

    fn get_scrollable(&self) -> bool;

    fn get_show_border(&self) -> bool;

    fn get_show_tabs(&self) -> bool;

    fn get_tab_detachable<P: IsA<Widget>>(&self, child: &P) -> bool;

    fn get_tab_label<P: IsA<Widget>>(&self, child: &P) -> Option<Widget>;

    fn get_tab_label_text<P: IsA<Widget>>(&self, child: &P) -> Option<String>;

    fn get_tab_pos(&self) -> PositionType;

    fn get_tab_reorderable<P: IsA<Widget>>(&self, child: &P) -> bool;

    fn next_page(&self);

    fn popup_disable(&self);

    fn popup_enable(&self);

    fn prev_page(&self);

    fn set_action_widget<P: IsA<Widget>>(&self, widget: &P, pack_type: PackType);

    fn set_group_name<'a, P: Into<Option<&'a str>>>(&self, group_name: P);

    fn set_menu_label<'a, P: IsA<Widget>, Q: IsA<Widget> + 'a, R: Into<Option<&'a Q>>>(&self, child: &P, menu_label: R);

    fn set_menu_label_text<P: IsA<Widget>>(&self, child: &P, menu_text: &str);

    fn set_scrollable(&self, scrollable: bool);

    fn set_show_border(&self, show_border: bool);

    fn set_show_tabs(&self, show_tabs: bool);

    fn set_tab_detachable<P: IsA<Widget>>(&self, child: &P, detachable: bool);

    fn set_tab_label<'a, P: IsA<Widget>, Q: IsA<Widget> + 'a, R: Into<Option<&'a Q>>>(&self, child: &P, tab_label: R);

    fn set_tab_label_text<P: IsA<Widget>>(&self, child: &P, tab_text: &str);

    fn set_tab_pos(&self, pos: PositionType);

    fn set_tab_reorderable<P: IsA<Widget>>(&self, child: &P, reorderable: bool);

    fn get_property_enable_popup(&self) -> bool;

    fn set_property_enable_popup(&self, enable_popup: bool);

    fn get_property_page(&self) -> i32;

    fn set_property_page(&self, page: i32);

    #[doc(hidden)]
    fn get_child_detachable<T: IsA<Widget>>(&self, item: &T) -> bool;

    #[doc(hidden)]
    fn set_child_detachable<T: IsA<Widget>>(&self, item: &T, detachable: bool);

    #[doc(hidden)]
    fn get_child_menu_label<T: IsA<Widget>>(&self, item: &T) -> Option<String>;

    #[doc(hidden)]
    fn set_child_menu_label<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, menu_label: P);

    #[doc(hidden)]
    fn get_child_position<T: IsA<Widget>>(&self, item: &T) -> i32;

    #[doc(hidden)]
    fn set_child_position<T: IsA<Widget>>(&self, item: &T, position: i32);

    #[doc(hidden)]
    fn get_child_reorderable<T: IsA<Widget>>(&self, item: &T) -> bool;

    #[doc(hidden)]
    fn set_child_reorderable<T: IsA<Widget>>(&self, item: &T, reorderable: bool);

    #[doc(hidden)]
    fn get_child_tab_expand<T: IsA<Widget>>(&self, item: &T) -> bool;

    #[doc(hidden)]
    fn set_child_tab_expand<T: IsA<Widget>>(&self, item: &T, tab_expand: bool);

    #[doc(hidden)]
    fn get_child_tab_fill<T: IsA<Widget>>(&self, item: &T) -> bool;

    #[doc(hidden)]
    fn set_child_tab_fill<T: IsA<Widget>>(&self, item: &T, tab_fill: bool);

    #[doc(hidden)]
    fn get_child_tab_label<T: IsA<Widget>>(&self, item: &T) -> Option<String>;

    #[doc(hidden)]
    fn set_child_tab_label<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, tab_label: P);

    fn connect_change_current_page<F: Fn(&Self, i32) -> bool + 'static>(&self, f: F) -> u64;

    fn connect_create_window<F: Fn(&Self, &Widget, i32, i32) -> Notebook + 'static>(&self, f: F) -> u64;

    fn connect_focus_tab<F: Fn(&Self, NotebookTab) -> bool + 'static>(&self, f: F) -> u64;

    fn connect_move_focus_out<F: Fn(&Self, DirectionType) + 'static>(&self, f: F) -> u64;

    fn connect_page_added<F: Fn(&Self, &Widget, u32) + 'static>(&self, f: F) -> u64;

    fn connect_page_removed<F: Fn(&Self, &Widget, u32) + 'static>(&self, f: F) -> u64;

    fn connect_page_reordered<F: Fn(&Self, &Widget, u32) + 'static>(&self, f: F) -> u64;

    fn connect_reorder_tab<F: Fn(&Self, DirectionType, bool) -> bool + 'static>(&self, f: F) -> u64;

    fn connect_select_page<F: Fn(&Self, bool) -> bool + 'static>(&self, f: F) -> u64;

    fn connect_switch_page<F: Fn(&Self, &Widget, u32) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<Notebook> + IsA<Container> + IsA<glib::object::Object>> NotebookExt for O {
    #[cfg(feature = "v3_16")]
    fn detach_tab<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_notebook_detach_tab(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    fn get_action_widget(&self, pack_type: PackType) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_action_widget(self.to_glib_none().0, pack_type.to_glib()))
        }
    }

    fn get_group_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_group_name(self.to_glib_none().0))
        }
    }

    fn get_menu_label<P: IsA<Widget>>(&self, child: &P) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_menu_label(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    fn get_menu_label_text<P: IsA<Widget>>(&self, child: &P) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_menu_label_text(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    fn get_scrollable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_notebook_get_scrollable(self.to_glib_none().0))
        }
    }

    fn get_show_border(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_notebook_get_show_border(self.to_glib_none().0))
        }
    }

    fn get_show_tabs(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_notebook_get_show_tabs(self.to_glib_none().0))
        }
    }

    fn get_tab_detachable<P: IsA<Widget>>(&self, child: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_notebook_get_tab_detachable(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    fn get_tab_label<P: IsA<Widget>>(&self, child: &P) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_tab_label(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    fn get_tab_label_text<P: IsA<Widget>>(&self, child: &P) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_notebook_get_tab_label_text(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    fn get_tab_pos(&self) -> PositionType {
        unsafe {
            from_glib(ffi::gtk_notebook_get_tab_pos(self.to_glib_none().0))
        }
    }

    fn get_tab_reorderable<P: IsA<Widget>>(&self, child: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_notebook_get_tab_reorderable(self.to_glib_none().0, child.to_glib_none().0))
        }
    }

    fn next_page(&self) {
        unsafe {
            ffi::gtk_notebook_next_page(self.to_glib_none().0);
        }
    }

    fn popup_disable(&self) {
        unsafe {
            ffi::gtk_notebook_popup_disable(self.to_glib_none().0);
        }
    }

    fn popup_enable(&self) {
        unsafe {
            ffi::gtk_notebook_popup_enable(self.to_glib_none().0);
        }
    }

    fn prev_page(&self) {
        unsafe {
            ffi::gtk_notebook_prev_page(self.to_glib_none().0);
        }
    }

    fn set_action_widget<P: IsA<Widget>>(&self, widget: &P, pack_type: PackType) {
        unsafe {
            ffi::gtk_notebook_set_action_widget(self.to_glib_none().0, widget.to_glib_none().0, pack_type.to_glib());
        }
    }

    fn set_group_name<'a, P: Into<Option<&'a str>>>(&self, group_name: P) {
        let group_name = group_name.into();
        let group_name = group_name.to_glib_none();
        unsafe {
            ffi::gtk_notebook_set_group_name(self.to_glib_none().0, group_name.0);
        }
    }

    fn set_menu_label<'a, P: IsA<Widget>, Q: IsA<Widget> + 'a, R: Into<Option<&'a Q>>>(&self, child: &P, menu_label: R) {
        let menu_label = menu_label.into();
        let menu_label = menu_label.to_glib_none();
        unsafe {
            ffi::gtk_notebook_set_menu_label(self.to_glib_none().0, child.to_glib_none().0, menu_label.0);
        }
    }

    fn set_menu_label_text<P: IsA<Widget>>(&self, child: &P, menu_text: &str) {
        unsafe {
            ffi::gtk_notebook_set_menu_label_text(self.to_glib_none().0, child.to_glib_none().0, menu_text.to_glib_none().0);
        }
    }

    fn set_scrollable(&self, scrollable: bool) {
        unsafe {
            ffi::gtk_notebook_set_scrollable(self.to_glib_none().0, scrollable.to_glib());
        }
    }

    fn set_show_border(&self, show_border: bool) {
        unsafe {
            ffi::gtk_notebook_set_show_border(self.to_glib_none().0, show_border.to_glib());
        }
    }

    fn set_show_tabs(&self, show_tabs: bool) {
        unsafe {
            ffi::gtk_notebook_set_show_tabs(self.to_glib_none().0, show_tabs.to_glib());
        }
    }

    fn set_tab_detachable<P: IsA<Widget>>(&self, child: &P, detachable: bool) {
        unsafe {
            ffi::gtk_notebook_set_tab_detachable(self.to_glib_none().0, child.to_glib_none().0, detachable.to_glib());
        }
    }

    fn set_tab_label<'a, P: IsA<Widget>, Q: IsA<Widget> + 'a, R: Into<Option<&'a Q>>>(&self, child: &P, tab_label: R) {
        let tab_label = tab_label.into();
        let tab_label = tab_label.to_glib_none();
        unsafe {
            ffi::gtk_notebook_set_tab_label(self.to_glib_none().0, child.to_glib_none().0, tab_label.0);
        }
    }

    fn set_tab_label_text<P: IsA<Widget>>(&self, child: &P, tab_text: &str) {
        unsafe {
            ffi::gtk_notebook_set_tab_label_text(self.to_glib_none().0, child.to_glib_none().0, tab_text.to_glib_none().0);
        }
    }

    fn set_tab_pos(&self, pos: PositionType) {
        unsafe {
            ffi::gtk_notebook_set_tab_pos(self.to_glib_none().0, pos.to_glib());
        }
    }

    fn set_tab_reorderable<P: IsA<Widget>>(&self, child: &P, reorderable: bool) {
        unsafe {
            ffi::gtk_notebook_set_tab_reorderable(self.to_glib_none().0, child.to_glib_none().0, reorderable.to_glib());
        }
    }

    fn get_property_enable_popup(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "enable-popup".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_enable_popup(&self, enable_popup: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "enable-popup".to_glib_none().0, Value::from(&enable_popup).to_glib_none().0);
        }
    }

    fn get_property_page(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "page".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_page(&self, page: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "page".to_glib_none().0, Value::from(&page).to_glib_none().0);
        }
    }

    #[doc(hidden)]
    fn get_child_detachable<T: IsA<Widget>>(&self, item: &T) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "detachable".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    #[doc(hidden)]
    fn set_child_detachable<T: IsA<Widget>>(&self, item: &T, detachable: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "detachable".to_glib_none().0, Value::from(&detachable).to_glib_none().0);
        }
    }

    #[doc(hidden)]
    fn get_child_menu_label<T: IsA<Widget>>(&self, item: &T) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "menu-label".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    #[doc(hidden)]
    fn set_child_menu_label<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, menu_label: P) {
        let menu_label = menu_label.into();
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "menu-label".to_glib_none().0, Value::from(menu_label).to_glib_none().0);
        }
    }

    #[doc(hidden)]
    fn get_child_position<T: IsA<Widget>>(&self, item: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "position".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    #[doc(hidden)]
    fn set_child_position<T: IsA<Widget>>(&self, item: &T, position: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "position".to_glib_none().0, Value::from(&position).to_glib_none().0);
        }
    }

    #[doc(hidden)]
    fn get_child_reorderable<T: IsA<Widget>>(&self, item: &T) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "reorderable".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    #[doc(hidden)]
    fn set_child_reorderable<T: IsA<Widget>>(&self, item: &T, reorderable: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "reorderable".to_glib_none().0, Value::from(&reorderable).to_glib_none().0);
        }
    }

    #[doc(hidden)]
    fn get_child_tab_expand<T: IsA<Widget>>(&self, item: &T) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "tab-expand".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    #[doc(hidden)]
    fn set_child_tab_expand<T: IsA<Widget>>(&self, item: &T, tab_expand: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "tab-expand".to_glib_none().0, Value::from(&tab_expand).to_glib_none().0);
        }
    }

    #[doc(hidden)]
    fn get_child_tab_fill<T: IsA<Widget>>(&self, item: &T) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "tab-fill".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    #[doc(hidden)]
    fn set_child_tab_fill<T: IsA<Widget>>(&self, item: &T, tab_fill: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "tab-fill".to_glib_none().0, Value::from(&tab_fill).to_glib_none().0);
        }
    }

    #[doc(hidden)]
    fn get_child_tab_label<T: IsA<Widget>>(&self, item: &T) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "tab-label".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    #[doc(hidden)]
    fn set_child_tab_label<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, tab_label: P) {
        let tab_label = tab_label.into();
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "tab-label".to_glib_none().0, Value::from(tab_label).to_glib_none().0);
        }
    }

    fn connect_change_current_page<F: Fn(&Self, i32) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "change-current-page",
                transmute(change_current_page_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_create_window<F: Fn(&Self, &Widget, i32, i32) -> Notebook + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Widget, i32, i32) -> Notebook + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "create-window",
                transmute(create_window_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_focus_tab<F: Fn(&Self, NotebookTab) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, NotebookTab) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "focus-tab",
                transmute(focus_tab_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_move_focus_out<F: Fn(&Self, DirectionType) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, DirectionType) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-focus-out",
                transmute(move_focus_out_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_page_added<F: Fn(&Self, &Widget, u32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Widget, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "page-added",
                transmute(page_added_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_page_removed<F: Fn(&Self, &Widget, u32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Widget, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "page-removed",
                transmute(page_removed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_page_reordered<F: Fn(&Self, &Widget, u32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Widget, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "page-reordered",
                transmute(page_reordered_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_reorder_tab<F: Fn(&Self, DirectionType, bool) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, DirectionType, bool) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "reorder-tab",
                transmute(reorder_tab_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_select_page<F: Fn(&Self, bool) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, bool) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "select-page",
                transmute(select_page_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_switch_page<F: Fn(&Self, &Widget, u32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Widget, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "switch-page",
                transmute(switch_page_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn change_current_page_trampoline<P>(this: *mut ffi::GtkNotebook, object: libc::c_int, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Notebook> {
    callback_guard!();
    let f: &&(Fn(&P, i32) -> bool + 'static) = transmute(f);
    f(&Notebook::from_glib_none(this).downcast_unchecked(), object).to_glib()
}

unsafe extern "C" fn create_window_trampoline<P>(this: *mut ffi::GtkNotebook, page: *mut ffi::GtkWidget, x: libc::c_int, y: libc::c_int, f: glib_ffi::gpointer) -> *mut ffi::GtkNotebook
where P: IsA<Notebook> {
    callback_guard!();
    let f: &&(Fn(&P, &Widget, i32, i32) -> Notebook + 'static) = transmute(f);
    f(&Notebook::from_glib_none(this).downcast_unchecked(), &from_glib_none(page), x, y)/*Not checked*/.to_glib_none().0
}

unsafe extern "C" fn focus_tab_trampoline<P>(this: *mut ffi::GtkNotebook, object: ffi::GtkNotebookTab, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Notebook> {
    callback_guard!();
    let f: &&(Fn(&P, NotebookTab) -> bool + 'static) = transmute(f);
    f(&Notebook::from_glib_none(this).downcast_unchecked(), from_glib(object)).to_glib()
}

unsafe extern "C" fn move_focus_out_trampoline<P>(this: *mut ffi::GtkNotebook, object: ffi::GtkDirectionType, f: glib_ffi::gpointer)
where P: IsA<Notebook> {
    callback_guard!();
    let f: &&(Fn(&P, DirectionType) + 'static) = transmute(f);
    f(&Notebook::from_glib_none(this).downcast_unchecked(), from_glib(object))
}

unsafe extern "C" fn page_added_trampoline<P>(this: *mut ffi::GtkNotebook, child: *mut ffi::GtkWidget, page_num: libc::c_uint, f: glib_ffi::gpointer)
where P: IsA<Notebook> {
    callback_guard!();
    let f: &&(Fn(&P, &Widget, u32) + 'static) = transmute(f);
    f(&Notebook::from_glib_none(this).downcast_unchecked(), &from_glib_none(child), page_num)
}

unsafe extern "C" fn page_removed_trampoline<P>(this: *mut ffi::GtkNotebook, child: *mut ffi::GtkWidget, page_num: libc::c_uint, f: glib_ffi::gpointer)
where P: IsA<Notebook> {
    callback_guard!();
    let f: &&(Fn(&P, &Widget, u32) + 'static) = transmute(f);
    f(&Notebook::from_glib_none(this).downcast_unchecked(), &from_glib_none(child), page_num)
}

unsafe extern "C" fn page_reordered_trampoline<P>(this: *mut ffi::GtkNotebook, child: *mut ffi::GtkWidget, page_num: libc::c_uint, f: glib_ffi::gpointer)
where P: IsA<Notebook> {
    callback_guard!();
    let f: &&(Fn(&P, &Widget, u32) + 'static) = transmute(f);
    f(&Notebook::from_glib_none(this).downcast_unchecked(), &from_glib_none(child), page_num)
}

unsafe extern "C" fn reorder_tab_trampoline<P>(this: *mut ffi::GtkNotebook, object: ffi::GtkDirectionType, p0: glib_ffi::gboolean, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Notebook> {
    callback_guard!();
    let f: &&(Fn(&P, DirectionType, bool) -> bool + 'static) = transmute(f);
    f(&Notebook::from_glib_none(this).downcast_unchecked(), from_glib(object), from_glib(p0)).to_glib()
}

unsafe extern "C" fn select_page_trampoline<P>(this: *mut ffi::GtkNotebook, object: glib_ffi::gboolean, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Notebook> {
    callback_guard!();
    let f: &&(Fn(&P, bool) -> bool + 'static) = transmute(f);
    f(&Notebook::from_glib_none(this).downcast_unchecked(), from_glib(object)).to_glib()
}

unsafe extern "C" fn switch_page_trampoline<P>(this: *mut ffi::GtkNotebook, page: *mut ffi::GtkWidget, page_num: libc::c_uint, f: glib_ffi::gpointer)
where P: IsA<Notebook> {
    callback_guard!();
    let f: &&(Fn(&P, &Widget, u32) + 'static) = transmute(f);
    f(&Notebook::from_glib_none(this).downcast_unchecked(), &from_glib_none(page), page_num)
}
