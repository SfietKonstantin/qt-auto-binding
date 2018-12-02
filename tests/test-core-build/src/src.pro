TEMPLATE = lib
TARGET = test

QT = core
CONFIG += staticlib

DEFINES += QT_DEPRECATED_WARNINGS

SOURCES += \
    object.cpp \
    bindings.cpp

HEADERS += \
    object.h
