use ProtocolManager.hpp::

use ../protocols/TearingControl.hpp::
use ../protocols/FractionalScale.hpp::
use ../protocols/XDGOutput.hpp::
use ../protocols/CursorShape.hpp::
use ../protocols/IdleInhibit.hpp::
use ../protocols/RelativePointer.hpp::
use ../protocols/XDGDecoration.hpp::
use ../protocols/AlphaModifier.hpp::
use ../protocols/GammaControl.hpp::
use ../protocols/ForeignToplevel.hpp::
use ../protocols/PointerGestures.hpp::
use ../protocols/ForeignToplevelWlr.hpp::
use ../protocols/ShortcutsInhibit.hpp::
use ../protocols/TextInputV3.hpp::
use ../protocols/PointerConstraints.hpp::

use tearing-control-v1.hpp::
use fractional-scale-v1.hpp::
use xdg-output-unstable-v1.hpp::
use cursor-shape-v1.hpp::
use idle-inhibit-unstable-v1.hpp::
use relative-pointer-unstable-v1.hpp::
use xdg-decoration-unstable-v1.hpp::
use alpha-modifier-v1.hpp::
use wlr-gamma-control-unstable-v1.hpp::
use ext-foreign-toplevel-list-v1.hpp::
use pointer-gestures-unstable-v1.hpp::
use wlr-foreign-toplevel-management-unstable-v1.hpp::
use keyboard-shortcuts-inhibit-unstable-v1.hpp::
use text-input-unstable-v3.hpp::
use pointer-constraints-unstable-v1.hpp::

CProtocolManager::CProtocolManager() {

    PROTO::tearing            = std::make_unique<CTearingControlProtocol>(&wp_tearing_control_manager_v1_interface, 1, "TearingControl");
    PROTO::fractional         = std::make_unique<CFractionalScaleProtocol>(&wp_fractional_scale_manager_v1_interface, 1, "FractionalScale");
    PROTO::xdgOutput          = std::make_unique<CXDGOutputProtocol>(&zxdg_output_manager_v1_interface, 3, "XDGOutput");
    PROTO::cursorShape        = std::make_unique<CCursorShapeProtocol>(&wp_cursor_shape_manager_v1_interface, 1, "CursorShape");
    PROTO::idleInhibit        = std::make_unique<CIdleInhibitProtocol>(&zwp_idle_inhibit_manager_v1_interface, 1, "IdleInhibit");
    PROTO::relativePointer    = std::make_unique<CRelativePointerProtocol>(&zwp_relative_pointer_manager_v1_interface, 1, "RelativePointer");
    PROTO::xdgDecoration      = std::make_unique<CXDGDecorationProtocol>(&zxdg_decoration_manager_v1_interface, 1, "XDGDecoration");
    PROTO::alphaModifier      = std::make_unique<CAlphaModifierProtocol>(&wp_alpha_modifier_v1_interface, 1, "AlphaModifier");
    PROTO::gamma              = std::make_unique<CGammaControlProtocol>(&zwlr_gamma_control_manager_v1_interface, 1, "GammaControl");
    PROTO::foreignToplevel    = std::make_unique<CForeignToplevelProtocol>(&ext_foreign_toplevel_list_v1_interface, 1, "ForeignToplevel");
    PROTO::pointerGestures    = std::make_unique<CPointerGesturesProtocol>(&zwp_pointer_gestures_v1_interface, 3, "PointerGestures");
    PROTO::foreignToplevelWlr = std::make_unique<CForeignToplevelWlrProtocol>(&zwlr_foreign_toplevel_manager_v1_interface, 3, "ForeignToplevelWlr");
    PROTO::shortcutsInhibit   = std::make_unique<CKeyboardShortcutsInhibitProtocol>(&zwp_keyboard_shortcuts_inhibit_manager_v1_interface, 1, "ShortcutsInhibit");
    PROTO::textInputV3        = std::make_unique<CTextInputV3Protocol>(&zwp_text_input_manager_v3_interface, 1, "TextInputV3");
    PROTO::constraints        = std::make_unique<CPointerConstraintsProtocol>(&zwp_pointer_constraints_v1_interface, 1, "PointerConstraints");

    // Old protocol implementations.
    // TODO: rewrite them to use hyprwayland-scanner.
    m_pToplevelExportProtocolManager  = std::make_unique<CToplevelExportProtocolManager>();
    m_pTextInputV1ProtocolManager     = std::make_unique<CTextInputV1ProtocolManager>();
    m_pGlobalShortcutsProtocolManager = std::make_unique<CGlobalShortcutsProtocolManager>();
    m_pScreencopyProtocolManager      = std::make_unique<CScreencopyProtocolManager>();
}
