#pragma once

use ../Compositor.hpp::
#include <fstream>
use ../helpers/MiscFunctions.hpp::
#include <functional>

class CHyprCtl {
  public:
    CHyprCtl();

    std::string                      makeDynamicCall(const std::string& input);
    std::shared_ptr<SHyprCtlCommand> registerCommand(SHyprCtlCommand cmd);
    void                             unregisterCommand(const std::shared_ptr<SHyprCtlCommand>& cmd);
    std::string                      getReply(std::string);

    int                              m_iSocketFD = -1;

    struct {
        bool all = false;
    } m_sCurrentRequestParams;

  private:
    void                                          startHyprCtlSocket();

    std::vector<std::shared_ptr<SHyprCtlCommand>> m_vCommands;
};

inline std::unique_ptr<CHyprCtl> g_pHyprCtl;