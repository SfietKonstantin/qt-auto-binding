#include <QtCore/QVariant>
#include <cstdint>

using CListAppendFunc = void (*)(void *output, const void *variant);
using CListFillFunc = void (*)(void *input, void *output, CListAppendFunc append);
using RsStringFillFunc = void (*)(void *output, const char *input, std::uint32_t inputSize);
using RsListFillFunc = void (*)(void *output, void *variant);


template<class T>
bool primitiveConvertTo(const QVariant &variant, T &value) {
    if (!variant.canConvert<T>()) {
        return false;
    }

    value = variant.value<T>();
    return true;
}

static void listAppend(void *output, const void *variant) {
    const auto qvariant = static_cast<const QVariant *>(variant);
    static_cast<QVariantList *>(output)->append(QVariant(*qvariant));
}

extern "C" {

QVariant *qt_binding_variant_create_bool(bool value) {
    return new QVariant(QVariant::fromValue<bool>(value));
}

QVariant *qt_binding_variant_create_i32(std::int32_t value) {
    return new QVariant(QVariant::fromValue<std::int32_t>(value));
}

QVariant *qt_binding_variant_create_u32(std::uint32_t value) {
    return new QVariant(QVariant::fromValue<std::uint32_t>(value));
}

QVariant *qt_binding_variant_create_i64(std::int64_t value) {
    return new QVariant(QVariant::fromValue<std::int64_t>(value));
}

QVariant *qt_binding_variant_create_u64(std::uint64_t value) {
    return new QVariant(QVariant::fromValue<std::uint64_t>(value));
}

QVariant *qt_binding_variant_create_f32(float value) {
    return new QVariant(QVariant::fromValue<float>(value));
}

QVariant *qt_binding_variant_create_f64(double value) {
    return new QVariant(QVariant::fromValue<double>(value));
}

QVariant *qt_binding_variant_create_string(const char *value, std::uint32_t size) {
    return new QVariant(QString::fromUtf8(value, size));
}

QVariant *qt_binding_variant_create_list(void *input, CListFillFunc fill) {
    auto list = QVariantList();
    fill(input, &list, listAppend);
    return new QVariant(list);
}


bool qt_binding_variant_fill_bool(const QVariant *variant, bool *value) {
    return primitiveConvertTo(*variant, *value);
}

bool qt_binding_variant_fill_i32(const QVariant *variant, std::int32_t *value) {
    return primitiveConvertTo(*variant, *value);
}

bool qt_binding_variant_fill_u32(const QVariant *variant, std::uint32_t *value) {
    return primitiveConvertTo(*variant, *value);
}

bool qt_binding_variant_fill_i64(const QVariant *variant, std::int64_t *value) {
    return primitiveConvertTo(*variant, *value);
}

bool qt_binding_variant_fill_u64(const QVariant *variant, std::uint64_t *value) {
    return primitiveConvertTo(*variant, *value);
}

bool qt_binding_variant_fill_f32(const QVariant *variant, float *value) {
    return primitiveConvertTo(*variant, *value);
}

bool qt_binding_variant_fill_f64(const QVariant *variant, double *value) {
    return primitiveConvertTo(*variant, *value);
}

bool qt_binding_variant_fill_string(const QVariant *variant, void *output, RsStringFillFunc fill) {
    if (!variant->canConvert<QString>()) {
        return false;
    }

    auto value = variant->value<QString>();
    const auto byteArray = value.toUtf8();
    fill(output, byteArray.constData(), byteArray.size());
    return true;
}

bool qt_binding_variant_fill_list(const QVariant *variant, void *output, RsListFillFunc fill) {
    if (!variant->canConvert<QVariantList>()) {
        return false;
    }

    auto values = variant->value<QVariantList>();
    for (const auto &value: values) {
        fill(output, new QVariant(value));
    }

    return true;
}

} // extern "C"
