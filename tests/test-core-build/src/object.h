#ifndef OBJECT_H
#define OBJECT_H

#include <QtCore/QObject>

class Object : public QObject
{
    Q_OBJECT
    Q_PROPERTY(int value READ value WRITE setValue NOTIFY valueChanged)
public:
    explicit Object();
    int value() const;
    void setValue(int value);
signals:
    void valueChanged(int value);

private:
    int m_value;
};

#endif // OBJECT_H
