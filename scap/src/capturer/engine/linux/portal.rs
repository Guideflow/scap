use std::{
    sync::{atomic::AtomicBool, Arc, Mutex},
    time::Duration,
};

use dbus::{
    arg::{self, PropMap, RefArg, Variant},
    blocking::{Connection, Proxy},
    message::MatchRule,
    strings::{BusName, Interface},
};

use super::error::LinCapError;

// This code was autogenerated with `dbus-codegen-rust -d org.freedesktop.portal.Desktop -p /org/freedesktop/portal/desktop -f org.freedesktop.portal.ScreenCast`, see https://github.com/diwic/dbus-rs
// {
use dbus::blocking;

trait OrgFreedesktopPortalScreenCast {
    fn create_session(&self, options: arg::PropMap) -> Result<dbus::Path<'static>, dbus::Error>;
    fn select_sources(
        &self,
        session_handle: dbus::Path,
        options: arg::PropMap,
    ) -> Result<dbus::Path<'static>, dbus::Error>;
    fn start(
        &self,
        session_handle: dbus::Path,
        parent_window: &str,
        options: arg::PropMap,
    ) -> Result<dbus::Path<'static>, dbus::Error>;
    fn open_pipe_wire_remote(
        &self,
        session_handle: dbus::Path,
        options: arg::PropMap,
    ) -> Result<arg::OwnedFd, dbus::Error>;
    fn available_source_types(&self) -> Result<u32, dbus::Error>;
    fn available_cursor_modes(&self) -> Result<u32, dbus::Error>;
    fn version(&self) -> Result<u32, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>>
    OrgFreedesktopPortalScreenCast for blocking::Proxy<'a, C>
{
    fn create_session(&self, options: arg::PropMap) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call(
            "org.freedesktop.portal.ScreenCast",
            "CreateSession",
            (options,),
        )
        .and_then(|r: (dbus::Path<'static>,)| Ok(r.0))
    }

    fn select_sources(
        &self,
        session_handle: dbus::Path,
        options: arg::PropMap,
    ) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call(
            "org.freedesktop.portal.ScreenCast",
            "SelectSources",
            (session_handle, options),
        )
        .and_then(|r: (dbus::Path<'static>,)| Ok(r.0))
    }

    fn start(
        &self,
        session_handle: dbus::Path,
        parent_window: &str,
        options: arg::PropMap,
    ) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call(
            "org.freedesktop.portal.ScreenCast",
            "Start",
            (session_handle, parent_window, options),
        )
        .and_then(|r: (dbus::Path<'static>,)| Ok(r.0))
    }

    fn open_pipe_wire_remote(
        &self,
        session_handle: dbus::Path,
        options: arg::PropMap,
    ) -> Result<arg::OwnedFd, dbus::Error> {
        self.method_call(
            "org.freedesktop.portal.ScreenCast",
            "OpenPipeWireRemote",
            (session_handle, options),
        )
        .and_then(|r: (arg::OwnedFd,)| Ok(r.0))
    }

    fn available_source_types(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.portal.ScreenCast",
            "AvailableSourceTypes",
        )
    }

    fn available_cursor_modes(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.portal.ScreenCast",
            "AvailableCursorModes",
        )
    }

    fn version(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.portal.ScreenCast",
            "version",
        )
    }
}
// }

// This code was autogenerated with `dbus-codegen-rust --file org.freedesktop.portal.Request.xml`, see https://github.com/diwic/dbus-rs
// {
trait OrgFreedesktopPortalRequest {
    fn close(&self) -> Result<(), dbus::Error>;
}

#[derive(Debug)]
pub struct OrgFreedesktopPortalRequestResponse {
    pub response: u32,
    pub results: arg::PropMap,
}

impl arg::AppendAll for OrgFreedesktopPortalRequestResponse {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.response, i);
        arg::RefArg::append(&self.results, i);
    }
}

impl arg::ReadAll for OrgFreedesktopPortalRequestResponse {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopPortalRequestResponse {
            response: i.read()?,
            results: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopPortalRequestResponse {
    const NAME: &'static str = "Response";
    const INTERFACE: &'static str = "org.freedesktop.portal.Request";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>> OrgFreedesktopPortalRequest
    for blocking::Proxy<'a, C>
{
    fn close(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.portal.Request", "Close", ())
    }
}
// }

type Response = Option<OrgFreedesktopPortalRequestResponse>;

#[derive(Debug)]
#[allow(dead_code)]
pub struct StreamVardict {
    id: Option<String>,
    position: Option<(i32, i32)>,
    size: Option<(i32, i32)>,
    source_type: Option<u32>,
    mapping_id: Option<String>,
}

#[derive(Debug)]
pub struct Stream(u32, StreamVardict);

impl Stream {
    pub fn pw_node_id(&self) -> u32 {
        self.0
    }

    pub fn from_dbus(stream: &Variant<Box<dyn RefArg>>) -> Option<Self> {
        let mut stream = stream.as_iter()?.next()?.as_iter()?;
        let pipewire_node_id = stream.next()?.as_iter()?.next()?.as_u64()?;

        // TODO: Get the rest of the properties

        Some(Self(
            pipewire_node_id as u32,
            StreamVardict {
                id: None,
                position: None,
                size: None,
                source_type: None,
                mapping_id: None,
            },
        ))
    }
}

macro_rules! match_response {
    ( $code:expr ) => {
        match $code {
            0 => {}
            1 => {
                return Err(LinCapError::new(String::from(
                    "User cancelled the interaction",
                )));
            }
            2 => {
                return Err(LinCapError::new(String::from(
                    "The user interaction was ended in some other way",
                )));
            }
            _ => unreachable!(),
        }
    };
}

pub struct ScreenCastPortal<'a> {
    proxy: Proxy<'a, &'a Connection>,
    token: String,
    cursor_mode: u32,
}

impl<'a> ScreenCastPortal<'a> {
    pub fn new(connection: &'a Connection) -> Self {
        let proxy = connection.with_proxy(
            "org.freedesktop.portal.Desktop",
            "/org/freedesktop/portal/desktop",
            Duration::from_secs(4),
        );

        let token = format!("scap_{}", rand::random::<u16>());

        Self {
            proxy,
            token,
            cursor_mode: 1,
        }
    }

    fn create_session_args(&self) -> arg::PropMap {
        let mut map = arg::PropMap::new();
        map.insert(
            String::from("handle_token"),
            Variant(Box::new(self.token.clone())),
        );
        map.insert(
            String::from("session_handle_token"),
            Variant(Box::new(self.token.clone())),
        );
        map
    }

    fn select_sources_args(&self) -> Result<arg::PropMap, dbus::Error> {
        let mut map = arg::PropMap::new();
        map.insert(
            String::from("handle_token"),
            Variant(Box::new(self.token.clone())),
        );
        map.insert(
            String::from("types"),
            Variant(Box::new(self.proxy.available_source_types()?)),
        );
        map.insert(String::from("multiple"), Variant(Box::new(false)));
        map.insert(
            String::from("cursor_mode"),
            Variant(Box::new(self.cursor_mode)),
        );
        Ok(map)
    }

    fn handle_req_response(
        connection: &Connection,
        path: dbus::Path<'static>,
        iterations: usize,
        timeout: Duration,
        response: Arc<Mutex<Response>>,
    ) -> Result<(), dbus::Error> {
        let got_response = Arc::new(AtomicBool::new(false));
        let got_response_clone = Arc::clone(&got_response);

        let mut rule = MatchRule::new();
        rule.path = Some(dbus::Path::from(path));
        rule.msg_type = Some(dbus::MessageType::Signal);
        rule.sender = Some(BusName::from("org.freedesktop.portal.Desktop"));
        rule.interface = Some(Interface::from("org.freedesktop.portal.Request"));
        connection.add_match(
            rule,
            move |res: OrgFreedesktopPortalRequestResponse, _chuh, _msg| {
                let mut response = response.lock().expect("Failed to lock response mutex");
                *response = Some(res);
                got_response_clone.store(true, std::sync::atomic::Ordering::Relaxed);
                false
            },
        )?;

        for _ in 0..iterations {
            connection.process(timeout)?;

            if got_response.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }
        }

        Ok(())
    }

    fn create_session(&self) -> Result<dbus::Path, LinCapError> {
        let request_handle = self.proxy.create_session(self.create_session_args())?;

        let response = Arc::new(Mutex::new(None));
        let response_clone = Arc::clone(&response);
        Self::handle_req_response(
            self.proxy.connection,
            request_handle,
            100,
            Duration::from_millis(100),
            response_clone,
        )?;

        if let Some(res) = response.lock()?.take() {
            match_response!(res.response);
            match res
                .results
                .get("session_handle")
                .map(|h| h.0.as_str().map(String::from))
            {
                Some(h) => {
                    let p = dbus::Path::from(match h {
                        Some(p) => p,
                        None => {
                            return Err(LinCapError::new(String::from(
                                "Invalid session_handle received",
                            )))
                        }
                    });

                    return Ok(p);
                }
                None => return Err(LinCapError::new(String::from("Did not get session handle"))),
            }
        }

        Err(LinCapError::new(String::from("Did not get response")))
    }

    fn select_sources(&self, session_handle: dbus::Path) -> Result<(), LinCapError> {
        let request_handle = self
            .proxy
            .select_sources(session_handle, self.select_sources_args()?)?;

        let response = Arc::new(Mutex::new(None));
        let response_clone = Arc::clone(&response);
        Self::handle_req_response(
            self.proxy.connection,
            request_handle,
            1200, // Wait 2 min
            Duration::from_millis(100),
            response_clone,
        )?;

        if let Some(res) = response.lock()?.take() {
            match_response!(res.response);
            return Ok(());
        }

        Err(LinCapError::new(String::from("Did not get response")))
    }

    fn start(&self, session_handle: dbus::Path) -> Result<Stream, LinCapError> {
        let request_handle = self.proxy.start(session_handle, "", PropMap::new())?;

        let response = Arc::new(Mutex::new(None));
        let response_clone = Arc::clone(&response);
        Self::handle_req_response(
            self.proxy.connection,
            request_handle,
            100, // Wait 10 s
            Duration::from_millis(100),
            response_clone,
        )?;

        if let Some(res) = response.lock()?.take() {
            match_response!(res.response);
            match res.results.get("streams") {
                Some(s) => match Stream::from_dbus(s) {
                    Some(s) => return Ok(s),
                    None => {
                        return Err(LinCapError::new(String::from(
                            "Failed to extract stream properties",
                        )))
                    }
                },
                None => return Err(LinCapError::new(String::from("Did not get any streams"))),
            }
        }

        Err(LinCapError::new(String::from("Did not get response")))
    }

    pub fn create_stream(&self) -> Result<Stream, LinCapError> {
        let session_handle = self.create_session()?;
        self.select_sources(session_handle.clone())?;
        self.start(session_handle)
    }

    pub fn show_cursor(mut self, mode: bool) -> Result<Self, LinCapError> {
        let available_modes = self.proxy.available_cursor_modes()?;
        if mode && available_modes & 1 == 1 {
            self.cursor_mode = 1;
            return Ok(self);
        } else if !mode && available_modes & 2 == 1 {
            self.cursor_mode = 2;
            return Ok(self);
        }

        Err(LinCapError::new("Unsupported cursor mode".to_string()))
    }
}
