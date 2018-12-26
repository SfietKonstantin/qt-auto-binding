import QtQml 2.0
import test 1.0

QtObject {
    property Timer timer: Timer {
        interval: object.value
        onTriggered: Qt.quit()
    }

    property TestObject object : TestObject {
        value: 200
        Component.onCompleted: timer.start()
    }
}
