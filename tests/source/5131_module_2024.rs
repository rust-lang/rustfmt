// rustfmt-imports_granularity: Module
// rustfmt-style_edition: 2024

#![allow(dead_code)]

mod a {
    pub mod b {
        pub struct Data {
            pub a: i32,
        }
    }

    use crate::a::b::Data;
    use crate::a::b::Data as Data2;

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
    use column_____________________________________________________________________________________102::{
        Foo,
        bar::Bar,
        bar::baz::Baz,
        Foo2,
        bar::Bar2,
    };

    use column_______________________________________________________________________________096::{
        Foo,
        bar::Bar,
        bar::baz::Baz,
        Foo2,
        bar::Bar2,
    };

    use column_________________________________________________________________________090::{
        Foo,
        bar::Bar,
        bar::baz::Baz,
        Foo2,
        bar::Bar2,
    };

    use c012::c018::c024::c030::c036::c042::c048::c054::c060::c066::c072::c078::c084::c090::c096::c102::{
        Foo,
        bar::Bar,
        bar::baz::Baz,
        Foo2,
        bar::Bar2,
    };

    use c012::c018::c024::c030::c036::c042::c048::c054::c060::c066::c072::c078::c084::c090::c096::{
        Foo,
        bar::Bar,
        bar::baz::Baz,
        Foo2,
        bar::Bar2,
    };

    use c012::c018::c024::c030::c036::c042::c048::c054::c060::c066::c072::c078::c084::c090::{
        Foo,
        bar::Bar,
        bar::baz::Baz,
        Foo2,
        bar::Bar2,
    };

    use c012::c018::c024::c030::c036::c042::c048::c054::c060::c066::c072::c078::c084::{
        Foo,
        bar::Bar,
        bar::baz::Baz,
        Foo2,
        bar::Bar2,
    };

    // Check that the behavior when "{" exceeds the max column.
    //
    // Note that `shape.offset_left(4)?.sub_width(1)?;` in
    // `rewrite_reorderable_or_regroupable_items()` replaces the max column 100 by 99.

    use x::column______________________________________________________________________________098::{
        Foo,
        bar::Bar,
        bar::baz::Baz,
        Foo2,
        bar::Bar2,
    };

    use x::column__Only_the_last_one_wraps_due_to_brace_______________________________________097::{
        Foo,
        bar::Bar,
        bar::baz::Baz,
        Foo2,
        bar::Bar2,
    };

    use x::column_____________________________________________________________________________096::{
        Foo,
        bar::Bar,
        bar::baz::Baz,
        Foo2,
        bar::Bar2,
    };

    // Test for top-level `UseSegmentKind::List`.
    use {
        a,
        column_____________________________________________________________________________________102,
    };
}

use smithay::{
    backend::renderer::element::{
        default_primary_scanout_output_compare, utils::select_dmabuf_feedback, RenderElementStates,
    },
    delegate_compositor, delegate_data_control, delegate_data_device, delegate_fractional_scale,
    delegate_input_method_manager, delegate_keyboard_shortcuts_inhibit, delegate_layer_shell,
    delegate_output, delegate_pointer_constraints, delegate_pointer_gestures,
    delegate_presentation, delegate_primary_selection, delegate_relative_pointer, delegate_seat,
    delegate_security_context, delegate_shm, delegate_tablet_manager, delegate_text_input_manager,
    delegate_viewporter, delegate_virtual_keyboard_manager, delegate_xdg_activation,
    delegate_xdg_decoration, delegate_xdg_shell,
    desktop::{
        space::SpaceElement,
        utils::{
            surface_presentation_feedback_flags_from_states, surface_primary_scanout_output,
            update_surface_primary_scanout_output, OutputPresentationFeedback,
        },
        PopupKind, PopupManager, Space,
    },
    input::{
        keyboard::{Keysym, LedState, XkbConfig},
        pointer::{CursorImageStatus, PointerHandle},
        Seat, SeatHandler, SeatState,
    },
    output::Output,
    reexports::{
        calloop::{generic::Generic, Interest, LoopHandle, Mode, PostAction},
        wayland_protocols::xdg::decoration::{
            self as xdg_decoration,
            zv1::server::zxdg_toplevel_decoration_v1::Mode as DecorationMode,
        },
        wayland_server::{
            backend::{ClientData, ClientId, DisconnectReason},
            protocol::{wl_data_source::WlDataSource, wl_surface::WlSurface},
            Display, DisplayHandle, Resource,
        },
    },
    utils::{Clock, Monotonic, Rectangle},
    wayland::{
        compositor::{get_parent, with_states, CompositorClientState, CompositorState},
        dmabuf::DmabufFeedback,
        fractional_scale::{
            with_fractional_scale, FractionalScaleHandler, FractionalScaleManagerState,
        },
        input_method::{InputMethodHandler, InputMethodManagerState, PopupSurface},
        keyboard_shortcuts_inhibit::{
            KeyboardShortcutsInhibitHandler, KeyboardShortcutsInhibitState,
            KeyboardShortcutsInhibitor,
        },
        output::{OutputHandler, OutputManagerState},
        pointer_constraints::{
            with_pointer_constraint, PointerConstraintsHandler, PointerConstraintsState,
        },
        pointer_gestures::PointerGesturesState,
        presentation::PresentationState,
        relative_pointer::RelativePointerManagerState,
        seat::WaylandFocus,
        security_context::{
            SecurityContext, SecurityContextHandler, SecurityContextListenerSource,
            SecurityContextState,
        },
        selection::data_device::{
            set_data_device_focus, ClientDndGrabHandler, DataDeviceHandler, DataDeviceState,
            ServerDndGrabHandler,
        },
        selection::{
            primary_selection::{
                set_primary_focus, PrimarySelectionHandler, PrimarySelectionState,
            },
            wlr_data_control::{DataControlHandler, DataControlState},
            SelectionHandler,
        },
        shell::{
            wlr_layer::WlrLayerShellState,
            xdg::{
                decoration::{XdgDecorationHandler, XdgDecorationState},
                ToplevelSurface, XdgShellState, XdgToplevelSurfaceData,
            },
        },
        shm::{ShmHandler, ShmState},
        socket::ListeningSocketSource,
        tablet_manager::{TabletManagerState, TabletSeatTrait},
        text_input::TextInputManagerState,
        viewporter::ViewporterState,
        virtual_keyboard::VirtualKeyboardManagerState,
        xdg_activation::{
            XdgActivationHandler, XdgActivationState, XdgActivationToken, XdgActivationTokenData,
        },
        xdg_foreign::{XdgForeignHandler, XdgForeignState},
    },
};
