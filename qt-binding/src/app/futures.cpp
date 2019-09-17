#include "futures.h"

#include <QtCore/QCoreApplication>

namespace qt_binding {

QtRuntime::QtRuntime(ExecTaskFunc execTask, QObject *parent)
    : QObject(parent)
    , m_execTask(execTask)
{
    connect(this, &QtRuntime::queueTask, this, &QtRuntime::executeTask, Qt::QueuedConnection);
}

QtRuntime *QtRuntime::instance()
{
    auto app = QCoreApplication::instance();
    if (app == nullptr) {
        return nullptr;
    }

    return app->findChild<QtRuntime *>(QString(), Qt::FindDirectChildrenOnly);
}

void QtRuntime::executeTask(const void *task)
{
    m_execTask(task);
}

} // namespace qt_binding

extern "C" {

void qt_binding_futures_runtime_init(qt_binding::ExecTaskFunc execTask)
{
    auto app = QCoreApplication::instance();
    new qt_binding::QtRuntime(execTask, app);
}

bool qt_binding_futures_task_queue(const void *task)
{
    auto runtime = qt_binding::QtRuntime::instance();
    if (runtime == nullptr) {
        return false;
    }

    emit runtime->queueTask(task);
    return true;
}

} // extern "C"
