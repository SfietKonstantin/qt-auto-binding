#include "object.h"

#include <QtCore/QCoreApplication>
#include <QtQml/QQmlComponent>
#include <QtQml/QQmlEngine>

static void init()
{
    Q_INIT_RESOURCE(bindings);
    qmlRegisterType<Object>("test", 1, 0, "TestObject");
}

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

int run_test()
{
    init();

    int argc = 0;
    char *argv = {};
    QCoreApplication app(argc, &argv);

    QQmlEngine engine;
    QObject::connect(&engine, &QQmlEngine::quit, &app, QCoreApplication::quit);

    QQmlComponent component(&engine, QUrl("qrc:/test.qml"));
    auto *item = component.create();
    if (item == nullptr) {
        return 1;
    }

    return app.exec();
}

} // extern "C"
