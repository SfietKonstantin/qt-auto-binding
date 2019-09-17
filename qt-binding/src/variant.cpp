#include <QtCore/QVariant>

extern "C" {

QVariant *qt_binding_variant_create_invalid()
{
    return new QVariant();
}

QVariant *qt_binding_variant_clone(const QVariant *qvariant)
{
    return new QVariant(*qvariant);
}

bool qt_binding_variant_compare(const QVariant *first, const QVariant *second)
{
    return *first == *second;
}

void qt_binding_variant_delete(QVariant *qvariant)
{
    delete qvariant;
}

const char *qt_binding_variant_get_type_name(const QVariant *qvariant)
{
    const auto *type = qvariant->typeName();
    return type != nullptr ? type : "Unknown";
}

} // extern "C"
