// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Object;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "NMDhcpConfig")]
    pub struct DhcpConfig(Object<ffi::NMDhcpConfig, ffi::NMDhcpConfigClass>) @extends Object;

    match fn {
        type_ => || ffi::nm_dhcp_config_get_type(),
    }
}

impl DhcpConfig {
    /// Gets the IP address family of the configuration
    ///
    /// # Returns
    ///
    /// the IP address family; either `<literal>`AF_INET`</literal>` or
    ///  `<literal>`AF_INET6`</literal>`
    #[doc(alias = "nm_dhcp_config_get_family")]
    #[doc(alias = "get_family")]
    pub fn family(&self) -> i32 {
        unsafe { ffi::nm_dhcp_config_get_family(self.to_glib_none().0) }
    }

    /// Gets one option by option name.
    /// ## `option`
    /// the option to retrieve
    ///
    /// # Returns
    ///
    /// the configuration option's value. This is the internal string used by the
    /// configuration, and must not be modified.
    #[doc(alias = "nm_dhcp_config_get_one_option")]
    #[doc(alias = "get_one_option")]
    pub fn one_option(&self, option: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::nm_dhcp_config_get_one_option(
                self.to_glib_none().0,
                option.to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "nm_dhcp_config_get_options")]
    //#[doc(alias = "get_options")]
    //pub fn options(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 } {
    //    unsafe { TODO: call ffi:nm_dhcp_config_get_options() }
    //}

    #[doc(alias = "family")]
    pub fn connect_family_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_family_trampoline<F: Fn(&DhcpConfig) + 'static>(
            this: *mut ffi::NMDhcpConfig,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::family\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_family_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "options")]
    pub fn connect_options_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_options_trampoline<F: Fn(&DhcpConfig) + 'static>(
            this: *mut ffi::NMDhcpConfig,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::options\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_options_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DhcpConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DhcpConfig")
    }
}
