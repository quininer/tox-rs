use std::io;
use std::thread;
use std::mem::transmute;
use libc::c_void;
use mio::{
    EventLoop,
    Handler,
    Sender
};
use super::{ ffi, vars, Tox, Network };


pub enum Events {
    SelfConnection(vars::Connection)
}

pub trait Listener<H: Handler>: Network {
    fn listen(&mut self) -> io::Result<EventLoop<H>>;
}

impl<H> Listener<H> for Tox where H: Handler {
    fn listen(&mut self) -> io::Result<EventLoop<H>> {
        let event_loop = try!(EventLoop::new());
        let mut sender = Box::new(event_loop.channel());

        unsafe {
            let void: *mut c_void = transmute(&mut sender);

            ffi::tox_callback_self_connection_status(self.core, on_self_connection_status, void);
        }

        let mut tox = self.clone();
        thread::spawn(move || {
            tox.iterate();
            thread::sleep(tox.interval());
        });

        Ok(event_loop)
    }
}

extern "C" fn on_self_connection_status(_: *mut ffi::Tox, connection_status: vars::Connection, void: *mut c_void) {
    unsafe {
        let sender: &mut Sender<Events> = transmute(void);
        sender.send(Events::SelfConnection(connection_status)).ok();
    }
}
