#include <QtCore/QObject>

extern "C" {

void qt_auto_binding_meta_delete_object(void *object)
{
    QObject *qobject = static_cast<QObject *>(object);
    qobject->deleteLater();
}

} // extern "C"
