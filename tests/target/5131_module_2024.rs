// rustfmt-imports_granularity: Module
// rustfmt-style_edition: 2024

#![allow(dead_code)]

mod a {
    pub mod b {
        pub struct Data {
            pub a: i32,
        }
    }

    use crate::a::b::{Data, Data as Data2};

    pub fn data(a: i32) -> Data {
        Data { a }
    }

    pub fn data2(a: i32) -> Data2 {
        Data2 { a }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        pub fn test() {
            data(1);
            data2(1);
        }
    }
}

mod indent4 {
    use column_____________________________________________________________________________________102::
        bar::baz::Baz;
    use column_____________________________________________________________________________________102::
        bar::{Bar, Bar2};
    use column_____________________________________________________________________________________102::{
        Foo, Foo2,
    };

    use column_______________________________________________________________________________096::
        bar::baz::Baz;
    use column_______________________________________________________________________________096::
        bar::{Bar, Bar2};
    use column_______________________________________________________________________________096::{
        Foo, Foo2,
    };

    use column_________________________________________________________________________090::bar::
        baz::Baz;
    use column_________________________________________________________________________090::bar::{
        Bar, Bar2,
    };
    use column_________________________________________________________________________090::{
        Foo, Foo2,
    };

    use c012::c018::c024::c030::c036::c042::c048::c054::c060::c066::c072::c078::c084::c090::c096::
        c102::bar::baz::Baz;
    use c012::c018::c024::c030::c036::c042::c048::c054::c060::c066::c072::c078::c084::c090::c096::
        c102::bar::{Bar, Bar2};
    use c012::c018::c024::c030::c036::c042::c048::c054::c060::c066::c072::c078::c084::c090::c096::
        c102::{Foo, Foo2};

    use c012::c018::c024::c030::c036::c042::c048::c054::c060::c066::c072::c078::c084::c090::c096::
        bar::baz::Baz;
    use c012::c018::c024::c030::c036::c042::c048::c054::c060::c066::c072::c078::c084::c090::c096::
        bar::{Bar, Bar2};
    use c012::c018::c024::c030::c036::c042::c048::c054::c060::c066::c072::c078::c084::c090::c096::{
        Foo, Foo2,
    };

    use c012::c018::c024::c030::c036::c042::c048::c054::c060::c066::c072::c078::c084::c090::bar::
        baz::Baz;
    use c012::c018::c024::c030::c036::c042::c048::c054::c060::c066::c072::c078::c084::c090::bar::{
        Bar, Bar2,
    };
    use c012::c018::c024::c030::c036::c042::c048::c054::c060::c066::c072::c078::c084::c090::{
        Foo, Foo2,
    };

    use c012::c018::c024::c030::c036::c042::c048::c054::c060::c066::c072::c078::c084::bar::baz::Baz;
    use c012::c018::c024::c030::c036::c042::c048::c054::c060::c066::c072::c078::c084::bar::{
        Bar, Bar2,
    };
    use c012::c018::c024::c030::c036::c042::c048::c054::c060::c066::c072::c078::c084::{Foo, Foo2};

    // Check that the behavior when "{" exceeds the max column.
    //
    // Note that `shape.offset_left(4)?.sub_width(1)?;` in
    // `rewrite_reorderable_or_regroupable_items()` replaces the max column 100 by 99.

    use x::
        column______________________________________________________________________________098::
        bar::baz::Baz;
    use x::
        column______________________________________________________________________________098::
        bar::{Bar, Bar2};
    use x::
        column______________________________________________________________________________098::{
        Foo, Foo2,
    };

    use x::column__Only_the_last_one_wraps_due_to_brace_______________________________________097::
        bar::baz::Baz;
    use x::column__Only_the_last_one_wraps_due_to_brace_______________________________________097::
        bar::{Bar, Bar2};
    use x::
        column__Only_the_last_one_wraps_due_to_brace_______________________________________097::{
        Foo, Foo2,
    };

    use x::column_____________________________________________________________________________096::
        bar::baz::Baz;
    use x::column_____________________________________________________________________________096::
        bar::{Bar, Bar2};
    use x::
        column_____________________________________________________________________________096::{
        Foo, Foo2,
    };

    // Test for top-level `UseSegmentKind::List`.
    use {
        a,
        column_____________________________________________________________________________________102,
    };
}

use smithay::backend::renderer::element::utils::select_dmabuf_feedback;
use smithay::backend::renderer::element::{
    default_primary_scanout_output_compare, RenderElementStates,
};
use smithay::desktop::space::SpaceElement;
use smithay::desktop::utils::{
    surface_presentation_feedback_flags_from_states, surface_primary_scanout_output,
    update_surface_primary_scanout_output, OutputPresentationFeedback,
};
use smithay::desktop::{PopupKind, PopupManager, Space};
use smithay::input::keyboard::{Keysym, LedState, XkbConfig};
use smithay::input::pointer::{CursorImageStatus, PointerHandle};
use smithay::input::{Seat, SeatHandler, SeatState};
use smithay::output::Output;
use smithay::reexports::calloop::generic::Generic;
use smithay::reexports::calloop::{Interest, LoopHandle, Mode, PostAction};
use smithay::reexports::wayland_protocols::xdg::decoration::zv1::server::
    zxdg_toplevel_decoration_v1::Mode as DecorationMode;
use smithay::reexports::wayland_protocols::xdg::decoration::{self as xdg_decoration};
use smithay::reexports::wayland_server::backend::{ClientData, ClientId, DisconnectReason};
use smithay::reexports::wayland_server::protocol::wl_data_source::WlDataSource;
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::reexports::wayland_server::{Display, DisplayHandle, Resource};
use smithay::utils::{Clock, Monotonic, Rectangle};
use smithay::wayland::compositor::{
    get_parent, with_states, CompositorClientState, CompositorState,
};
use smithay::wayland::dmabuf::DmabufFeedback;
use smithay::wayland::fractional_scale::{
    with_fractional_scale, FractionalScaleHandler, FractionalScaleManagerState,
};
use smithay::wayland::input_method::{InputMethodHandler, InputMethodManagerState, PopupSurface};
use smithay::wayland::keyboard_shortcuts_inhibit::{
    KeyboardShortcutsInhibitHandler, KeyboardShortcutsInhibitState, KeyboardShortcutsInhibitor,
};
use smithay::wayland::output::{OutputHandler, OutputManagerState};
use smithay::wayland::pointer_constraints::{
    with_pointer_constraint, PointerConstraintsHandler, PointerConstraintsState,
};
use smithay::wayland::pointer_gestures::PointerGesturesState;
use smithay::wayland::presentation::PresentationState;
use smithay::wayland::relative_pointer::RelativePointerManagerState;
use smithay::wayland::seat::WaylandFocus;
use smithay::wayland::security_context::{
    SecurityContext, SecurityContextHandler, SecurityContextListenerSource, SecurityContextState,
};
use smithay::wayland::selection::data_device::{
    set_data_device_focus, ClientDndGrabHandler, DataDeviceHandler, DataDeviceState,
    ServerDndGrabHandler,
};
use smithay::wayland::selection::primary_selection::{
    set_primary_focus, PrimarySelectionHandler, PrimarySelectionState,
};
use smithay::wayland::selection::wlr_data_control::{DataControlHandler, DataControlState};
use smithay::wayland::selection::SelectionHandler;
use smithay::wayland::shell::wlr_layer::WlrLayerShellState;
use smithay::wayland::shell::xdg::decoration::{XdgDecorationHandler, XdgDecorationState};
use smithay::wayland::shell::xdg::{ToplevelSurface, XdgShellState, XdgToplevelSurfaceData};
use smithay::wayland::shm::{ShmHandler, ShmState};
use smithay::wayland::socket::ListeningSocketSource;
use smithay::wayland::tablet_manager::{TabletManagerState, TabletSeatTrait};
use smithay::wayland::text_input::TextInputManagerState;
use smithay::wayland::viewporter::ViewporterState;
use smithay::wayland::virtual_keyboard::VirtualKeyboardManagerState;
use smithay::wayland::xdg_activation::{
    XdgActivationHandler, XdgActivationState, XdgActivationToken, XdgActivationTokenData,
};
use smithay::wayland::xdg_foreign::{XdgForeignHandler, XdgForeignState};
use smithay::{
    delegate_compositor, delegate_data_control, delegate_data_device, delegate_fractional_scale,
    delegate_input_method_manager, delegate_keyboard_shortcuts_inhibit, delegate_layer_shell,
    delegate_output, delegate_pointer_constraints, delegate_pointer_gestures,
    delegate_presentation, delegate_primary_selection, delegate_relative_pointer, delegate_seat,
    delegate_security_context, delegate_shm, delegate_tablet_manager, delegate_text_input_manager,
    delegate_viewporter, delegate_virtual_keyboard_manager, delegate_xdg_activation,
    delegate_xdg_decoration, delegate_xdg_shell,
};
