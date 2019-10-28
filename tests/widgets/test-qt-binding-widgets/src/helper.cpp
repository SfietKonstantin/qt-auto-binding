#include <QtWidgets/QApplication>

extern "C" {

bool qt_has_app() {
    return dynamic_cast<QApplication *>(QCoreApplication::instance()) != nullptr;
}

} // extern "C"
