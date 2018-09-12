extern crate clap;
extern crate futures;
extern crate gio;
extern crate glib;

extern crate nm;

use std::str;

use clap::{App, Arg};

use glib::translate::FromGlib;

use futures::prelude::*;

use nm::*;

#[derive(Debug)]
struct Config {
    ssid: String,
    passphrase: String,
    interface: Option<String>,
}

fn get_config() -> Config {
    let matches = App::new("connect-wifi")
        .version("0.0.1")
        .arg(
            Arg::with_name("ssid")
                .value_name("SSID")
                .help("WiFi network SSID")
                .index(1)
                .required(true),
        ).arg(
            Arg::with_name("passphrase")
                .value_name("PASSPHRASE")
                .help("WiFi network passphrase")
                .index(2)
                .required(true),
        ).arg(
            Arg::with_name("interface")
                .short("i")
                .long("interface")
                .value_name("interface")
                .help("WiFi interface name")
                .takes_value(true),
        ).get_matches();

    let ssid = matches.value_of("ssid").unwrap().to_string();
    let passphrase = matches.value_of("passphrase").unwrap().to_string();
    let interface: Option<String> = matches.value_of("interface").map(str::to_string);

    Config {
        ssid,
        passphrase,
        interface,
    }
}

struct AccessPointData {
    ssid: String,
    strength: u8,
    security: String,
}

fn main() {
    let config = get_config();

    let context = glib::MainContext::default();
    let loop_ = glib::MainLoop::new(Some(&context), false);

    context.push_thread_default();

    let client = Client::new(None).unwrap();

    let device = find_device(&client, &config.interface);

    let device_wifi = if let Ok(device_wifi) = device.clone().downcast::<DeviceWifi>() {
        device_wifi
    } else {
        panic!("Not a WiFi device!");
    };

    println!("Choose a WiFi network:");

    let access_points = device_wifi.get_access_points();

    let mut access_points_data = vec![];
    let mut max_ssid = 0;
    for ap in &access_points {
        if let Some(ssid) = ap.get_ssid() {
            if let Ok(ssid) = str::from_utf8(&ssid) {
                let ssid = ssid.to_string();
                if ssid.len() > max_ssid {
                    max_ssid = ssid.len();
                }
                let strength = ap.get_strength();

                let mut security = String::new();

                let flags = ap.get_flags();
                let rsn_flags = ap.get_rsn_flags();
                let wpa_flags = ap.get_wpa_flags();

                if flags.contains(_80211ApFlags::PRIVACY)
                    && wpa_flags == _80211ApSecurityFlags::NONE
                    && rsn_flags == _80211ApSecurityFlags::NONE
                {
                    security.push_str("WEP ");
                }

                if wpa_flags != _80211ApSecurityFlags::NONE {
                    security.push_str("WPA1 ");
                }

                if rsn_flags != _80211ApSecurityFlags::NONE {
                    security.push_str("WPA2 ");
                }

                if wpa_flags.contains(_80211ApSecurityFlags::KEY_MGMT_802_1X)
                    || rsn_flags.contains(_80211ApSecurityFlags::KEY_MGMT_802_1X)
                {
                    security.push_str("802.1X ");
                }

                security.pop();

                access_points_data.push(AccessPointData {
                    ssid,
                    strength,
                    security,
                });
            }
        }
    }

    access_points_data.sort_by_key(|ap| ap.strength);
    access_points_data.reverse();

    for (index, ap) in access_points_data.iter().enumerate() {
        let bars = utils_wifi_strength_bars(ap.strength).unwrap();
        println!(
            "[{:2}] {1:2$} {3:3} {4} {5}",
            index, ap.ssid, max_ssid, ap.strength, bars, ap.security
        );
    }

    panic!("bye");

    let s_connection = SettingConnection::new();
    s_connection.set_property_type(Some(&SETTING_WIRELESS_SETTING_NAME));
    s_connection.set_property_id(Some(&config.ssid));

    let s_wireless = SettingWireless::new();
    s_wireless.set_property_ssid(Some(&(config.ssid.as_bytes().into())));

    let s_wireless_security = SettingWirelessSecurity::new();
    s_wireless_security.set_property_key_mgmt(Some("wpa-psk"));
    s_wireless_security.set_property_psk(Some(&config.passphrase));

    let connection = SimpleConnection::new();

    connection.add_setting(&s_connection);
    connection.add_setting(&s_wireless);
    connection.add_setting(&s_wireless_security);

    if let Err(e) = connection.normalize() {
        panic!("Verification error: {:?}", e);
    }

    let l_clone = loop_.clone();

    let future = client.add_and_activate_connection_async_future(&connection, &device, None);
    let new_future = future
        .map(|(_con, active_con)| {
            active_con.connect_state_changed(move |active_con, state, reason| {
                let state = ActiveConnectionState::from_glib(state as _);
                let reason = ActiveConnectionStateReason::from_glib(reason as _);
                println!("Connection state: {:?} / {:?}", state, reason);

                match state {
                    ActiveConnectionState::Activated => {
                        println!("Connection successfully activated.");
                        l_clone.quit();
                    }
                    ActiveConnectionState::Deactivated => {
                        println!("Connection NOT activated!");
                        let r_con = active_con.get_connection().unwrap();
                        r_con.delete(None).unwrap();
                        l_clone.quit();
                    }
                    _ => {}
                }
            });
        }).map_err(|(_con, e)| {
            eprintln!("{:?}", e);
        }).then(move |_| Ok(()));

    context.spawn_local(new_future);

    loop_.run();

    context.pop_thread_default();
}

fn find_device(client: &Client, interface: &Option<String>) -> Device {
    if let Some(ref interface) = *interface {
        match client.get_device_by_iface(interface) {
            Some(device) => device,
            _ => panic!("Interface not found: {}", interface),
        }
    } else {
        let devices = client.get_devices();

        let device = devices
            .iter()
            .find(|d| d.get_device_type() == DeviceType::Wifi);

        if let Some(device) = device {
            println!("WiFi device: {}", device.get_iface().unwrap());
            device.clone()
        } else {
            panic!("Could not find a WiFi device");
        }
    }
}
