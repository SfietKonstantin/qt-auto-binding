#include "object.h"

Object::Object()
    : m_value(0)
{
}

int Object::value() const
{
    return m_value;
}

void Object::setValue(int value)
{
    if (m_value != value) {
        m_value = value;
        emit valueChanged(m_value);
    }
}
