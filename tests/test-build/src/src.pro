TEMPLATE = lib
TARGET = src

QT = core
CONFIG += staticlib

DEFINES += QT_DEPRECATED_WARNINGS

SOURCES += \
    object.cpp \
    bindings.cpp

HEADERS += \
    object.h
