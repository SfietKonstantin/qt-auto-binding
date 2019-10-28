#ifndef QT_BINDING_WITH_GUI
#include <QtCore/QCoreApplication>
#else // QT_BINDING_WITH_GUI
#ifndef QT_BINDING_WITH_WIDGETS
#include <QtGui/QGuiApplication>
#else // QT_BINDING_WITH_WIDGETS
#include <QtWidgets/QApplication>
#endif // QT_BINDING_WITH_WIDGETS
#endif // QT_BINDING_WITH_GUI

#include <memory>

namespace {

template <typename T>
struct Array
{
    Array(const T *first, std::ptrdiff_t size)
        : m_begin(first)
        , m_end(first + size)
    {
    }
    const T *begin() const noexcept
    {
        return m_begin;
    }
    const T *end() const noexcept
    {
        return m_end;
    }

    const T *m_begin;
    const T *m_end;
};

} // namespace

namespace qt_binding {

#ifndef QT_BINDING_WITH_GUI
using Application = QCoreApplication;
#else // QT_BINDING_WITH_GUI
#ifndef QT_BINDING_WITH_WIDGETS
using Application = QGuiApplication;
#else // QT_BINDING_WITH_WIDGETS
using Application = QApplication;
#endif
#endif // QT_BINDING_WITH_GUI

class AppContainer
{
public:
    AppContainer(int argc, const char *const *argv)
        : m_argc(argc)
    {
        auto array = Array<const char *>(argv, static_cast<std::ptrdiff_t>(argc));
        std::transform(array.begin(), array.end(), std::back_inserter(m_arguments),
                       [](const char *arg) { return QByteArray(arg); });
        std::transform(m_arguments.begin(), m_arguments.end(), std::back_inserter(m_argv),
                       [](QByteArray &arg) { return arg.data(); });

        m_app.reset(new Application(m_argc, m_argv.data()));
    }
    std::unique_ptr<Application> m_app;

private:
    std::vector<QByteArray> m_arguments;
    int m_argc{0};
    std::vector<char *> m_argv;
};

} // namespace qt_binding

extern "C" {

qt_binding::AppContainer *qt_binding_application_create(int argc, const char *const *argv)
{
    return new qt_binding::AppContainer(argc, argv);
}

void qt_binding_application_delete(qt_binding::AppContainer *app)
{
    delete app;
}

int qt_binding_application_exec(qt_binding::AppContainer *app)
{
    return app->m_app->exec();
}

void qt_binding_application_exit(int code)
{
    qt_binding::Application::exit(code);
}

} // extern "C"
