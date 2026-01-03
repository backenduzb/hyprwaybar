use wayland_client::{
    protocol::{wl_compositor, wl_registry, wl_surface},
    Connection, Dispatch, QueueHandle, globals::{registry_queue_init, GlobalListContents, BindError},
};
use wayland_protocols_wlr::layer_shell::v1::client::{
    zwlr_layer_shell_v1::{self, ZwlrLayerShellV1},
    zwlr_layer_surface_v1::{self, Anchor, ZwlrLayerSurfaceV1},
};

#[derive(Default)]
struct State;

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
    fn event(_: &mut Self, _: &wl_compositor::WlCompositor, _: wl_compositor::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
}

impl Dispatch<wl_surface::WlSurface, ()> for State {
    fn event(_: &mut Self, _: &wl_surface::WlSurface, _: wl_surface::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
}

impl Dispatch<ZwlrLayerShellV1, ()> for State {
    fn event(_: &mut Self, _: &ZwlrLayerShellV1, _: zwlr_layer_shell_v1::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
}

impl Dispatch<ZwlrLayerSurfaceV1, ()> for State {
    fn event(
        state: &mut Self,
        layer_surface: &ZwlrLayerSurfaceV1,
        event: zwlr_layer_surface_v1::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let zwlr_layer_surface_v1::Event::Configure { serial, width, height } = event {
            layer_surface.ack_configure(serial);
        }
    }
}

pub fn run() {
    let conn = Connection::connect_to_env().expect("Wayland ga ulanish muvaffaqiyatsiz");

    let (globals, mut event_queue) = registry_queue_init::<State>(&conn)
        .expect("Registry ni ishga tushirib bo'lmadi");

    let qh = event_queue.handle();

    let compositor: wl_compositor::WlCompositor = globals
        .bind::<wl_compositor::WlCompositor, _, _>(&qh, 4..=6, ())
        .expect("wl_compositor topilmadi");

    let layer_shell: ZwlrLayerShellV1 = globals
        .bind::<ZwlrLayerShellV1, _, _>(&qh, 1..=4, ())
        .expect("zwlr_layer_shell_v1 topilmadi — kompozitor wlroots-based emasmi? (Sway/Hyprland va h.k.)");

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
    let height = 10u32;
    layer_surface.set_size(0, height);
    layer_surface.set_exclusive_zone(height as i32);
    
    surface.commit();

    loop {
        if let Err(e) = event_queue.blocking_dispatch(&mut State::default()) {
            eprintln!("Dispatch xatosi: {e:?}");
            break;
        }
    }
}