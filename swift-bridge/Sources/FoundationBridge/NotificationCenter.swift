import Foundation

public typealias NotificationContextFunction = @convention(c) (UnsafeMutableRawPointer?) -> Void

private final class NotificationCallback {
    private let context: UnsafeMutableRawPointer
    private let callback: NotificationContextFunction
    private let retainContext: NotificationContextFunction
    private let releaseContext: NotificationContextFunction

    init(
        context: UnsafeMutableRawPointer,
        callback: @escaping NotificationContextFunction,
        retainContext: @escaping NotificationContextFunction,
        releaseContext: @escaping NotificationContextFunction
    ) {
        self.context = context
        self.callback = callback
        self.retainContext = retainContext
        self.releaseContext = releaseContext
        retainContext(context)
    }

    deinit {
        releaseContext(context)
    }

    func call() {
        retainContext(context)
        defer { releaseContext(context) }
        callback(context)
    }
}

private final class RegisteredNotificationObserver {
    private let center: NotificationCenter
    private let token: NSObjectProtocol

    init(center: NotificationCenter, token: NSObjectProtocol) {
        self.center = center
        self.token = token
    }

    deinit {
        center.removeObserver(token)
    }
}

private func notificationCenter(_ handle: UnsafeMutableRawPointer) -> NotificationCenter {
    Unmanaged<NotificationCenter>.fromOpaque(handle).takeUnretainedValue()
}

@_cdecl("foundation_notification_center_default")
public func foundation_notification_center_default() -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(NotificationCenter.default).toOpaque()
}

@_cdecl("foundation_notification_center_new")
public func foundation_notification_center_new() -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(NotificationCenter()).toOpaque()
}

@_cdecl("foundation_notification_center_retain")
public func foundation_notification_center_retain(_ handle: UnsafeMutableRawPointer) {
    _ = Unmanaged<NotificationCenter>.fromOpaque(handle).retain()
}

@_cdecl("foundation_notification_center_release")
public func foundation_notification_center_release(_ handle: UnsafeMutableRawPointer) {
    Unmanaged<NotificationCenter>.fromOpaque(handle).release()
}

@_cdecl("foundation_notification_center_add_observer")
public func foundation_notification_center_add_observer(
    _ handle: UnsafeMutableRawPointer,
    _ name: UnsafePointer<CChar>,
    _ callback: @escaping NotificationContextFunction,
    _ context: UnsafeMutableRawPointer,
    _ retainContext: @escaping NotificationContextFunction,
    _ releaseContext: @escaping NotificationContextFunction
) -> UnsafeMutableRawPointer {
    let center = notificationCenter(handle)
    let retainedCallback = NotificationCallback(
        context: context,
        callback: callback,
        retainContext: retainContext,
        releaseContext: releaseContext
    )
    let token = center.addObserver(
        forName: Notification.Name(String(cString: name)),
        object: nil,
        queue: nil
    ) { _ in
        retainedCallback.call()
    }
    return Unmanaged.passRetained(RegisteredNotificationObserver(center: center, token: token)).toOpaque()
}

@_cdecl("foundation_notification_observer_release")
public func foundation_notification_observer_release(_ handle: UnsafeMutableRawPointer) {
    Unmanaged<RegisteredNotificationObserver>.fromOpaque(handle).release()
}

@_cdecl("foundation_notification_center_post")
public func foundation_notification_center_post(_ handle: UnsafeMutableRawPointer, _ name: UnsafePointer<CChar>) {
    notificationCenter(handle).post(name: Notification.Name(String(cString: name)), object: nil)
}
