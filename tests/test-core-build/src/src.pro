TEMPLATE = lib
TARGET = test

QT = core qml
CONFIG += staticlib

DEFINES += QT_DEPRECATED_WARNINGS

SOURCES += \
    object.cpp \
    bindings.cpp

HEADERS += \
    object.h

RESOURCES += \
    res.qrc

OTHER_FILES += \
    test.qml
