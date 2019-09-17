#include <QtGui/QGuiApplication>

extern "C" {

bool qt_has_gui_app() {
    return dynamic_cast<QGuiApplication *>(QCoreApplication::instance()) != nullptr;
}

} // extern "C"
