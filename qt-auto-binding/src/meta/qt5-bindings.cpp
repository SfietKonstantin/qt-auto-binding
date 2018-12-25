#include <QtCore/QMetaType>
#include <QtCore/QObject>

extern "C" {

void *qt_auto_binding_meta_new_object(const char *typeName)
{
    int type = QMetaType::type(typeName);
    if (type == QMetaType::UnknownType) {
        return 0;
    }

    const QMetaObject *metaObject = QMetaType::metaObjectForType(type);
    return metaObject->newInstance();
}

} // extern "C"
