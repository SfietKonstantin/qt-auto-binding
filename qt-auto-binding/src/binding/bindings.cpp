#include "object.h"

extern "C" {

void *new_object()
{
    return new Object();
}

void delete_object(const void *object)
{
    static_cast<const Object *>(object)->disconnect();
    delete static_cast<const Object *>(object);
}

int object_value(const void *object)
{
    return static_cast<const Object *>(object)->value();
}

void set_object_value(void *object, int value)
{
    static_cast<Object *>(object)->setValue(value);
}
}
