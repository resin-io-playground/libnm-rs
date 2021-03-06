// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Device;
use crate::Object;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "NMDeviceVlan")]
    pub struct DeviceVlan(Object<ffi::NMDeviceVlan, ffi::NMDeviceVlanClass>) @extends Device, Object;

    match fn {
        type_ => || ffi::nm_device_vlan_get_type(),
    }
}

impl DeviceVlan {
    /// Whether the device has carrier.
    ///
    /// # Returns
    ///
    /// [`true`] if the device has carrier
    #[doc(alias = "nm_device_vlan_get_carrier")]
    #[doc(alias = "get_carrier")]
    pub fn is_carrier(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_vlan_get_carrier(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the device's parent device
    #[doc(alias = "nm_device_vlan_get_parent")]
    #[doc(alias = "get_parent")]
    pub fn parent(&self) -> Option<Device> {
        unsafe { from_glib_none(ffi::nm_device_vlan_get_parent(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the device's VLAN ID
    #[doc(alias = "nm_device_vlan_get_vlan_id")]
    #[doc(alias = "get_vlan_id")]
    pub fn vlan_id(&self) -> u32 {
        unsafe { ffi::nm_device_vlan_get_vlan_id(self.to_glib_none().0) }
    }

    #[doc(alias = "carrier")]
    pub fn connect_carrier_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_carrier_trampoline<F: Fn(&DeviceVlan) + 'static>(
            this: *mut ffi::NMDeviceVlan,
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
                b"notify::carrier\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_carrier_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "parent")]
    pub fn connect_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<F: Fn(&DeviceVlan) + 'static>(
            this: *mut ffi::NMDeviceVlan,
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
                b"notify::parent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_parent_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "vlan-id")]
    pub fn connect_vlan_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vlan_id_trampoline<F: Fn(&DeviceVlan) + 'static>(
            this: *mut ffi::NMDeviceVlan,
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
                b"notify::vlan-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_vlan_id_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceVlan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceVlan")
    }
}
