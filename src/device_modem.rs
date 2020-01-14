// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_20", feature = "dox"))]
use glib::GString;
use glib_sys;
use nm_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Device;
use DeviceModemCapabilities;
use Object;

glib_wrapper! {
    pub struct DeviceModem(Object<nm_sys::NMDeviceModem, nm_sys::NMDeviceModemClass, DeviceModemClass>) @extends Device, Object;

    match fn {
        get_type => || nm_sys::nm_device_modem_get_type(),
    }
}

impl DeviceModem {
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    pub fn get_apn(&self) -> Option<GString> {
        unsafe { from_glib_none(nm_sys::nm_device_modem_get_apn(self.to_glib_none().0)) }
    }

    pub fn get_current_capabilities(&self) -> DeviceModemCapabilities {
        unsafe {
            from_glib(nm_sys::nm_device_modem_get_current_capabilities(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    pub fn get_device_id(&self) -> Option<GString> {
        unsafe { from_glib_none(nm_sys::nm_device_modem_get_device_id(self.to_glib_none().0)) }
    }

    pub fn get_modem_capabilities(&self) -> DeviceModemCapabilities {
        unsafe {
            from_glib(nm_sys::nm_device_modem_get_modem_capabilities(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    pub fn get_operator_code(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_device_modem_get_operator_code(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    pub fn connect_property_apn_notify<F: Fn(&DeviceModem) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_apn_trampoline<F: Fn(&DeviceModem) + 'static>(
            this: *mut nm_sys::NMDeviceModem,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::apn\0".as_ptr() as *const _,
                Some(transmute(notify_apn_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_current_capabilities_notify<F: Fn(&DeviceModem) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_current_capabilities_trampoline<
            F: Fn(&DeviceModem) + 'static,
        >(
            this: *mut nm_sys::NMDeviceModem,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::current-capabilities\0".as_ptr() as *const _,
                Some(transmute(
                    notify_current_capabilities_trampoline::<F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    pub fn connect_property_device_id_notify<F: Fn(&DeviceModem) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_device_id_trampoline<F: Fn(&DeviceModem) + 'static>(
            this: *mut nm_sys::NMDeviceModem,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::device-id\0".as_ptr() as *const _,
                Some(transmute(notify_device_id_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_modem_capabilities_notify<F: Fn(&DeviceModem) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_modem_capabilities_trampoline<F: Fn(&DeviceModem) + 'static>(
            this: *mut nm_sys::NMDeviceModem,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::modem-capabilities\0".as_ptr() as *const _,
                Some(transmute(
                    notify_modem_capabilities_trampoline::<F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_20", feature = "dox"))]
    pub fn connect_property_operator_code_notify<F: Fn(&DeviceModem) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_operator_code_trampoline<F: Fn(&DeviceModem) + 'static>(
            this: *mut nm_sys::NMDeviceModem,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::operator-code\0".as_ptr() as *const _,
                Some(transmute(notify_operator_code_trampoline::<F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceModem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceModem")
    }
}