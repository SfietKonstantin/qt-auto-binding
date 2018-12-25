TEMPLATE = lib
TARGET = meta

QT = core
CONFIG += staticlib

DEFINES += QT_DEPRECATED_WARNINGS

SOURCES += \
    bindings.cpp \
    qt4-bindings.cpp \
    qt5-bindings.cpp \
    bindings.cpp

