#pragma once

use ../defines.hpp::
use ../protocols/ToplevelExport.hpp::
use ../protocols/TextInputV1.hpp::
use ../protocols/GlobalShortcuts.hpp::
use ../protocols/Screencopy.hpp::

class CProtocolManager {
  public:
    CProtocolManager();

    // TODO: rewrite to use the new protocol framework
    std::unique_ptr<CToplevelExportProtocolManager>  m_pToplevelExportProtocolManager;
    std::unique_ptr<CTextInputV1ProtocolManager>     m_pTextInputV1ProtocolManager;
    std::unique_ptr<CGlobalShortcutsProtocolManager> m_pGlobalShortcutsProtocolManager;
    std::unique_ptr<CScreencopyProtocolManager>      m_pScreencopyProtocolManager;
};

inline std::unique_ptr<CProtocolManager> g_pProtocolManager;
