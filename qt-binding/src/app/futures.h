#ifndef QT_BINDING_APP_FUTURES_H
#define QT_BINDING_APP_FUTURES_H

#include <QtCore/QObject>

namespace qt_binding {

using ExecTaskFunc = void (*)(const void *task);

class QtRuntime : public QObject
{
    Q_OBJECT
public:
    explicit QtRuntime(ExecTaskFunc execTask, QObject *parent);
    static QtRuntime *instance();

signals:
    void queueTask(const void *task);

private slots:
    void executeTask(const void *task);

private:
    ExecTaskFunc m_execTask{nullptr};
};

} // namespace qt_binding

#endif // QT_BINDING_APP_FUTURES_H
