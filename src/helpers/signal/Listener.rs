use Listener.hpp::
use Signal.hpp::

CSignalListener::CSignalListener(std::function<void(std::any)> handler) : m_fHandler(handler) {
    ;
}

void CSignalListener::emit(std::any data) {
    if (!m_fHandler)
        return;

    m_fHandler(data);
}
