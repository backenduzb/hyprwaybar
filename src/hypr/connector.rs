use crate::config::settings::BACKGROUND_COLOR;
use crate::config::settings::BAR_HEIGHT;
use crate::utils::label_drawer;
use image::{imageops::FilterType, GenericImageView};
use std::ffi::CString;
use std::os::fd::BorrowedFd;
use std::time::{Duration, Instant};
use wayland_client::protocol::wl_shm_pool;

use std::{
    fs::File,
    os::unix::io::{AsRawFd, FromRawFd},
    ptr,
};

use wayland_client::{
    globals::{registry_queue_init, GlobalListContents},
    protocol::{wl_buffer, wl_compositor, wl_registry, wl_shm, wl_surface},
    Connection, Dispatch, QueueHandle,
};

use wayland_protocols_wlr::layer_shell::v1::client::{
    zwlr_layer_shell_v1::{self, ZwlrLayerShellV1},
    zwlr_layer_surface_v1::{self, Anchor, ZwlrLayerSurfaceV1},
};

use cairo::{Context, Format, ImageSurface};

#[derive(Default)]
pub struct State {
    configured: bool,
    width: i32,
    height: i32,
}

impl Dispatch<wl_shm_pool::WlShmPool, ()> for State {
    fn event(
        _: &mut Self,
        _: &wl_shm_pool::WlShmPool,
        _: wl_shm_pool::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for State {
    fn event(
        _: &mut Self,
        _: &wl_registry::WlRegistry,
        _: wl_registry::Event,
        _: &GlobalListContents,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_compositor::WlCompositor, ()> for State {
    fn event(
        _: &mut Self,
        _: &wl_compositor::WlCompositor,
        _: wl_compositor::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_surface::WlSurface, ()> for State {
    fn event(
        _: &mut Self,
        _: &wl_surface::WlSurface,
        _: wl_surface::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_shm::WlShm, ()> for State {
    fn event(
        _: &mut Self,
        _: &wl_shm::WlShm,
        _: wl_shm::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_buffer::WlBuffer, ()> for State {
    fn event(
        _: &mut Self,
        _: &wl_buffer::WlBuffer,
        _: wl_buffer::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ZwlrLayerShellV1, ()> for State {
    fn event(
        _: &mut Self,
        _: &ZwlrLayerShellV1,
        _: zwlr_layer_shell_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ZwlrLayerSurfaceV1, ()> for State {
    fn event(
        state: &mut Self,
        layer_surface: &ZwlrLayerSurfaceV1,
        event: zwlr_layer_surface_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let zwlr_layer_surface_v1::Event::Configure {
            serial,
            width,
            height,
        } = event
        {
            layer_surface.ack_configure(serial);
            state.width = if width == 0 { 1920 } else { width as i32 };
            state.height = BAR_HEIGHT as i32;
            state.configured = true;
        }
    }
}

pub fn run_connector<F>(mut render: F)
where
    F: FnMut(&Context, &State),
{
    let conn = Connection::connect_to_env().unwrap();
    let (globals, mut event_queue) = registry_queue_init::<State>(&conn).unwrap();
    let qh = event_queue.handle();

    let compositor: wl_compositor::WlCompositor = globals.bind(&qh, 4..=6, ()).unwrap();

    let shm: wl_shm::WlShm = globals.bind(&qh, 1..=1, ()).unwrap();

    let layer_shell: ZwlrLayerShellV1 = globals.bind(&qh, 1..=4, ()).unwrap();

    let surface = compositor.create_surface(&qh, ());
    let layer_surface = layer_shell.get_layer_surface(
        &surface,
        None,
        zwlr_layer_shell_v1::Layer::Top,
        "rust-bar".into(),
        &qh,
        (),
    );
    layer_surface.set_anchor(Anchor::Top | Anchor::Left | Anchor::Right);
    layer_surface.set_size(0, BAR_HEIGHT);
    layer_surface.set_exclusive_zone(BAR_HEIGHT as i32);
    surface.commit();

    let mut state = State::default();

    while !state.configured {
        event_queue.blocking_dispatch(&mut state).unwrap();
    }

    let stride = state.width * 4;
    let size = stride * state.height;
    let name = CString::new("bar-buffer").unwrap();

    let memfd = unsafe { libc::syscall(libc::SYS_memfd_create, name.as_ptr(), 0) as i32 };
    assert!(memfd >= 0);

    unsafe {
        libc::ftruncate(memfd, size as i64);
    }

    let data = unsafe {
        libc::mmap(
            ptr::null_mut(),
            size as usize,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            memfd,
            0,
        )
    } as *mut u8;

    let file = unsafe { File::from_raw_fd(memfd) };
    let fd = unsafe { BorrowedFd::borrow_raw(file.as_raw_fd()) };

    let pool = shm.create_pool(fd, size, &qh, ());
    let buffer = pool.create_buffer(
        0,
        state.width,
        state.height,
        stride,
        wl_shm::Format::Argb8888,
        &qh,
        (),
    );

    let cairo_surface = unsafe {
        ImageSurface::create_for_data(
            std::slice::from_raw_parts_mut(data, size as usize),
            Format::ARgb32,
            state.width,
            state.height,
            stride,
        )
        .unwrap()
    };

    let cr = Context::new(&cairo_surface).unwrap();
    label_drawer::rounded_rect(
        &cr,
        5.0,
        0.0,
        (state.width - 10) as f64,
        (state.height - 2) as f64,
        8.0,
    );
    let (red, green, blue, alpha) = BACKGROUND_COLOR;
    cr.set_source_rgba(red, green, blue, alpha);
    cr.fill().unwrap();
    render(&cr, &state);
    cairo_surface.flush();
    surface.attach(Some(&buffer), 0, 0);
    surface.commit();
    let mut last_updated = Instant::now();

    loop {
        event_queue.blocking_dispatch(&mut state).unwrap();

        if last_updated.elapsed() >= Duration::from_secs(1) {
            unsafe {
                libc::memset(data as *mut _, 0, size as usize);
            }

            let cr = Context::new(&cairo_surface).unwrap();

            label_drawer::rounded_rect(
                &cr,
                5.0,
                0.0,
                (state.width - 10) as f64,
                (state.height - 2) as f64,
                8.0,
            );
            render(&cr, &state);

            cairo_surface.flush();
            surface.attach(Some(&buffer), 0, 0);
            surface.commit();

            last_updated = Instant::now();
        }
    }
}
