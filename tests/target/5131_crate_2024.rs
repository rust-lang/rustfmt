// rustfmt-imports_granularity: Crate
// rustfmt-style_edition: 2024

use foo::{
    a, b, b as b2,
    b::{f, g, g as g2},
    c,
    d::e,
};
use qux::{h, h as h2, i};

mod indent4 {
    use column_____________________________________________________________________________________102::{
        Foo, Foo2,
        bar::{Bar, Bar2, baz::Baz},
    };

    use column_______________________________________________________________________________096::{
        Foo, Foo2,
        bar::{Bar, Bar2, baz::Baz},
    };

    use column_________________________________________________________________________090::{
        Foo, Foo2,
        bar::{Bar, Bar2, baz::Baz},
    };

    use c012::c018::c024::c030::c036::c042::c048::c054::c060::c066::c072::c078::c084::c090::c096::
        c102::{
        Foo, Foo2,
        bar::{Bar, Bar2, baz::Baz},
    };

    use c012::c018::c024::c030::c036::c042::c048::c054::c060::c066::c072::c078::c084::c090::c096::{
        Foo, Foo2,
        bar::{Bar, Bar2, baz::Baz},
    };

    use c012::c018::c024::c030::c036::c042::c048::c054::c060::c066::c072::c078::c084::c090::{
        Foo, Foo2,
        bar::{Bar, Bar2, baz::Baz},
    };

    use c012::c018::c024::c030::c036::c042::c048::c054::c060::c066::c072::c078::c084::{
        Foo, Foo2,
        bar::{Bar, Bar2, baz::Baz},
    };
}

use smithay::{
    backend::renderer::element::{
        RenderElementStates, default_primary_scanout_output_compare, utils::select_dmabuf_feedback,
    },
    delegate_compositor, delegate_data_control, delegate_data_device, delegate_fractional_scale,
    delegate_input_method_manager, delegate_keyboard_shortcuts_inhibit, delegate_layer_shell,
    delegate_output, delegate_pointer_constraints, delegate_pointer_gestures,
    delegate_presentation, delegate_primary_selection, delegate_relative_pointer, delegate_seat,
    delegate_security_context, delegate_shm, delegate_tablet_manager, delegate_text_input_manager,
    delegate_viewporter, delegate_virtual_keyboard_manager, delegate_xdg_activation,
    delegate_xdg_decoration, delegate_xdg_shell,
    desktop::{
        PopupKind, PopupManager, Space,
        space::SpaceElement,
        utils::{
            OutputPresentationFeedback, surface_presentation_feedback_flags_from_states,
            surface_primary_scanout_output, update_surface_primary_scanout_output,
        },
    },
    input::{
        Seat, SeatHandler, SeatState,
        keyboard::{Keysym, LedState, XkbConfig},
        pointer::{CursorImageStatus, PointerHandle},
    },
    output::Output,
    reexports::{
        calloop::{Interest, LoopHandle, Mode, PostAction, generic::Generic},
        wayland_protocols::xdg::decoration::{
            self as xdg_decoration,
            zv1::server::zxdg_toplevel_decoration_v1::Mode as DecorationMode,
        },
        wayland_server::{
            Display, DisplayHandle, Resource,
            backend::{ClientData, ClientId, DisconnectReason},
            protocol::{wl_data_source::WlDataSource, wl_surface::WlSurface},
        },
    },
    utils::{Clock, Monotonic, Rectangle},
    wayland::{
        compositor::{CompositorClientState, CompositorState, get_parent, with_states},
        dmabuf::DmabufFeedback,
        fractional_scale::{
            FractionalScaleHandler, FractionalScaleManagerState, with_fractional_scale,
        },
        input_method::{InputMethodHandler, InputMethodManagerState, PopupSurface},
        keyboard_shortcuts_inhibit::{
            KeyboardShortcutsInhibitHandler, KeyboardShortcutsInhibitState,
            KeyboardShortcutsInhibitor,
        },
        output::{OutputHandler, OutputManagerState},
        pointer_constraints::{
            PointerConstraintsHandler, PointerConstraintsState, with_pointer_constraint,
        },
        pointer_gestures::PointerGesturesState,
        presentation::PresentationState,
        relative_pointer::RelativePointerManagerState,
        seat::WaylandFocus,
        security_context::{
            SecurityContext, SecurityContextHandler, SecurityContextListenerSource,
            SecurityContextState,
        },
        selection::{
            SelectionHandler,
            data_device::{
                ClientDndGrabHandler, DataDeviceHandler, DataDeviceState, ServerDndGrabHandler,
                set_data_device_focus,
            },
            primary_selection::{
                PrimarySelectionHandler, PrimarySelectionState, set_primary_focus,
            },
            wlr_data_control::{DataControlHandler, DataControlState},
        },
        shell::{
            wlr_layer::WlrLayerShellState,
            xdg::{
                ToplevelSurface, XdgShellState, XdgToplevelSurfaceData,
                decoration::{XdgDecorationHandler, XdgDecorationState},
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
