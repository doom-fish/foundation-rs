#if FOUNDATION_NETWORK
import Foundation
import Security

@_cdecl("foundation_network_queue_new")
public func foundation_network_queue_new(_ name: UnsafePointer<CChar>) -> UnsafeMutableRawPointer {
    autoreleasepool {
        let queue = OperationQueue()
        queue.name = String(cString: name)
        queue.maxConcurrentOperationCount = 1
        return Unmanaged.passRetained(queue).toOpaque()
    }
}

@_cdecl("foundation_network_queue_release")
public func foundation_network_queue_release(_ handle: UnsafeMutableRawPointer) {
    autoreleasepool { Unmanaged<OperationQueue>.fromOpaque(handle).release() }
}

@_cdecl("foundation_network_queue_is_current")
public func foundation_network_queue_is_current(_ handle: UnsafeMutableRawPointer) -> Bool {
    OperationQueue.current === Unmanaged<OperationQueue>.fromOpaque(handle).takeUnretainedValue()
}

@_cdecl("foundation_network_session_new")
public func foundation_network_session_new(
    _ queue: UnsafeMutableRawPointer,
    _ idle: Double,
    _ resource: Double,
    _ connections: Int32,
    _ proxy: Int32
) -> UnsafeMutableRawPointer {
    autoreleasepool {
        let session = NetworkSession(
            queue: Unmanaged<OperationQueue>.fromOpaque(queue).takeUnretainedValue(),
            idle: idle,
            resource: resource,
            connections: Int(connections),
            directOnly: proxy == 1
        )
        return Unmanaged.passRetained(session).toOpaque()
    }
}

@_cdecl("foundation_network_session_release")
public func foundation_network_session_release(_ handle: UnsafeMutableRawPointer) {
    autoreleasepool {
        let owner = Unmanaged<NetworkSession>.fromOpaque(handle)
        owner.takeUnretainedValue().invalidate()
        owner.release()
    }
}

@_cdecl("foundation_network_session_cancel")
public func foundation_network_session_cancel(_ handle: UnsafeMutableRawPointer) {
    Unmanaged<NetworkSession>.fromOpaque(handle).takeUnretainedValue().invalidate()
}

@_cdecl("foundation_network_request_new")
public func foundation_network_request_new(
    _ url: UnsafePointer<CChar>,
    _ method: UnsafePointer<CChar>,
    _ timeout: Double
) -> UnsafeMutableRawPointer? {
    autoreleasepool {
        guard let url = URL(string: String(cString: url)), url.host?.isEmpty == false,
              let scheme = url.scheme?.lowercased(), ["http", "https", "ws", "wss"].contains(scheme) else {
            return nil
        }
        var request = URLRequest(url: url)
        request.httpMethod = String(cString: method)
        if timeout > 0 { request.timeoutInterval = timeout }
        return Unmanaged.passRetained(NetworkRequest(request, explicitTimeout: timeout > 0)).toOpaque()
    }
}

@_cdecl("foundation_network_request_release")
public func foundation_network_request_release(_ handle: UnsafeMutableRawPointer) {
    autoreleasepool { Unmanaged<NetworkRequest>.fromOpaque(handle).release() }
}

@_cdecl("foundation_network_request_add_header")
public func foundation_network_request_add_header(
    _ handle: UnsafeMutableRawPointer,
    _ name: UnsafePointer<CChar>,
    _ value: UnsafePointer<CChar>
) {
    autoreleasepool {
        let request = Unmanaged<NetworkRequest>.fromOpaque(handle).takeUnretainedValue()
        let name = String(cString: name)
        let value = String(cString: value)
        request.headers.append((name, value))
        request.value.addValue(value, forHTTPHeaderField: name)
    }
}

@_cdecl("foundation_network_request_set_body")
public func foundation_network_request_set_body(
    _ handle: UnsafeMutableRawPointer,
    _ bytes: UnsafePointer<UInt8>,
    _ count: Int
) {
    autoreleasepool {
        let request = Unmanaged<NetworkRequest>.fromOpaque(handle).takeUnretainedValue()
        request.value.httpBody = Data(bytes: bytes, count: count)
    }
}

@_cdecl("foundation_network_request_url")
public func foundation_network_request_url(_ handle: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let request = Unmanaged<NetworkRequest>.fromOpaque(handle).takeUnretainedValue()
    return request.value.url?.absoluteString.withCString { strdup($0) }
}

@_cdecl("foundation_network_request_method")
public func foundation_network_request_method(_ handle: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let request = Unmanaged<NetworkRequest>.fromOpaque(handle).takeUnretainedValue()
    return request.value.httpMethod?.withCString { strdup($0) }
}

@_cdecl("foundation_network_request_timeout")
public func foundation_network_request_timeout(_ handle: UnsafeMutableRawPointer) -> Double {
    Unmanaged<NetworkRequest>.fromOpaque(handle).takeUnretainedValue().value.timeoutInterval
}

@_cdecl("foundation_network_request_header_count")
public func foundation_network_request_header_count(_ handle: UnsafeMutableRawPointer) -> Int {
    Unmanaged<NetworkRequest>.fromOpaque(handle).takeUnretainedValue().headers.count
}

@_cdecl("foundation_network_request_header_name")
public func foundation_network_request_header_name(_ handle: UnsafeMutableRawPointer, _ index: Int) -> UnsafeMutablePointer<CChar>? {
    let request = Unmanaged<NetworkRequest>.fromOpaque(handle).takeUnretainedValue()
    return request.headers[index].0.withCString { strdup($0) }
}

@_cdecl("foundation_network_request_header_value")
public func foundation_network_request_header_value(_ handle: UnsafeMutableRawPointer, _ index: Int) -> UnsafeMutablePointer<CChar>? {
    let request = Unmanaged<NetworkRequest>.fromOpaque(handle).takeUnretainedValue()
    return request.headers[index].1.withCString { strdup($0) }
}

@_cdecl("foundation_network_task_new")
public func foundation_network_task_new(
    _ session: UnsafeMutableRawPointer,
    _ request: UnsafeMutableRawPointer,
    _ kind: Int32,
    _ deadline: Double,
    _ limit: Int,
    _ context: UnsafeMutableRawPointer,
    _ callback: @escaping NetworkEventFunction,
    _ retain: @escaping NetworkContextFunction,
    _ release: @escaping NetworkContextFunction
) -> UnsafeMutableRawPointer? {
    autoreleasepool {
        let owner = Unmanaged<NetworkSession>.fromOpaque(session).takeUnretainedValue()
        let request = Unmanaged<NetworkRequest>.fromOpaque(request).takeUnretainedValue()
        var value = request.value
        if !request.explicitTimeout { value.timeoutInterval = owner.idleTimeout }
        let task = NetworkTask(
            owner: owner,
            request: value,
            callback: NetworkCallback(context: context, callback: callback, retain: retain, release: release),
            webSocket: kind == 1,
            messageLimit: limit,
            deadline: deadline
        )
        guard owner.register(task) else { return nil }
        return Unmanaged.passRetained(task).toOpaque()
    }
}

@_cdecl("foundation_network_task_release")
public func foundation_network_task_release(_ handle: UnsafeMutableRawPointer) {
    autoreleasepool {
        let task = Unmanaged<NetworkTask>.fromOpaque(handle)
        task.takeUnretainedValue().cancel(NSError(domain: NSURLErrorDomain, code: NSURLErrorCancelled))
        task.release()
    }
}

@_cdecl("foundation_network_task_resume")
public func foundation_network_task_resume(_ handle: UnsafeMutableRawPointer) {
    Unmanaged<NetworkTask>.fromOpaque(handle).takeUnretainedValue().resume()
}

@_cdecl("foundation_network_task_suspend")
public func foundation_network_task_suspend(_ handle: UnsafeMutableRawPointer) {
    let task = Unmanaged<NetworkTask>.fromOpaque(handle).takeUnretainedValue()
    task.owner.enqueue {
        guard task.active && !task.paused else { return }
        task.paused = true
        task.current?.suspend()
    }
}

@_cdecl("foundation_network_task_cancel")
public func foundation_network_task_cancel(_ handle: UnsafeMutableRawPointer) {
    Unmanaged<NetworkTask>.fromOpaque(handle).takeUnretainedValue()
        .cancel(NSError(domain: NSURLErrorDomain, code: NSURLErrorCancelled))
}

@_cdecl("foundation_network_task_upload")
public func foundation_network_task_upload(_ handle: UnsafeMutableRawPointer, _ length: Int64) -> UnsafeMutableRawPointer? {
    let task = Unmanaged<NetworkTask>.fromOpaque(handle).takeUnretainedValue()
    return task.lock.withLock {
        guard !task.webSocket && !task.started && task.upload == nil && task.termination == nil,
              let upload = NetworkUpload(length: length) else { return nil }
        task.upload = upload
        return Unmanaged.passRetained(upload).toOpaque()
    }
}

@_cdecl("foundation_network_upload_write")
public func foundation_network_upload_write(
    _ handle: UnsafeMutableRawPointer,
    _ bytes: UnsafePointer<UInt8>,
    _ count: Int
) -> Int {
    Unmanaged<NetworkUpload>.fromOpaque(handle).takeUnretainedValue().write(bytes, count: count)
}

@_cdecl("foundation_network_upload_finish")
public func foundation_network_upload_finish(_ handle: UnsafeMutableRawPointer) -> Bool {
    Unmanaged<NetworkUpload>.fromOpaque(handle).takeUnretainedValue().finish()
}

@_cdecl("foundation_network_upload_release")
public func foundation_network_upload_release(_ handle: UnsafeMutableRawPointer) {
    let upload = Unmanaged<NetworkUpload>.fromOpaque(handle)
    upload.takeUnretainedValue().releaseWriter()
    upload.release()
}

@_cdecl("foundation_network_websocket_receive")
public func foundation_network_websocket_receive(_ handle: UnsafeMutableRawPointer) -> Bool {
    Unmanaged<NetworkTask>.fromOpaque(handle).takeUnretainedValue().receive()
}

@_cdecl("foundation_network_websocket_send")
public func foundation_network_websocket_send(
    _ handle: UnsafeMutableRawPointer,
    _ operation: Int64,
    _ text: Bool,
    _ bytes: UnsafePointer<UInt8>,
    _ count: Int
) -> Bool {
    let task = Unmanaged<NetworkTask>.fromOpaque(handle).takeUnretainedValue()
    guard task.reserve(0) else { return false }
    task.send(operation: operation, text: text, data: Data(bytes: bytes, count: count))
    return true
}

@_cdecl("foundation_network_websocket_ping")
public func foundation_network_websocket_ping(_ handle: UnsafeMutableRawPointer, _ operation: Int64) -> Bool {
    let task = Unmanaged<NetworkTask>.fromOpaque(handle).takeUnretainedValue()
    guard task.reserve(2) else { return false }
    task.owner.enqueue {
        guard task.active, let socket = task.current as? URLSessionWebSocketTask else { return }
        socket.sendPing { error in
            task.owner.enqueue {
                task.lock.withLock { task.pinging = false }
                let event = NetworkEvent(10, error: error as NSError?)
                event.first = operation
                task.emit(event)
            }
        }
    }
    return true
}

@_cdecl("foundation_network_websocket_close")
public func foundation_network_websocket_close(
    _ handle: UnsafeMutableRawPointer,
    _ code: Int64,
    _ reason: UnsafePointer<UInt8>,
    _ count: Int
) -> Bool {
    let task = Unmanaged<NetworkTask>.fromOpaque(handle).takeUnretainedValue()
    let accepted = task.lock.withLock {
        guard task.opened && !task.closing && !task.finished && task.termination == nil else { return false }
        task.closing = true
        return true
    }
    guard accepted else { return false }
    let data = Data(bytes: reason, count: count)
    task.owner.enqueue {
        guard task.active, let socket = task.current as? URLSessionWebSocketTask,
              let code = URLSessionWebSocketTask.CloseCode(rawValue: Int(code)) else { return }
        socket.cancel(with: code, reason: data)
    }
    return true
}

@_cdecl("foundation_network_event_kind")
public func foundation_network_event_kind(_ handle: UnsafeMutableRawPointer) -> Int32 {
    Unmanaged<NetworkEvent>.fromOpaque(handle).takeUnretainedValue().kind
}

@_cdecl("foundation_network_event_response")
public func foundation_network_event_response(_ handle: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    let event = Unmanaged<NetworkEvent>.fromOpaque(handle).takeUnretainedValue()
    return event.response.map { Unmanaged.passUnretained($0).toOpaque() }
}

@_cdecl("foundation_network_event_request")
public func foundation_network_event_request(_ handle: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    let event = Unmanaged<NetworkEvent>.fromOpaque(handle).takeUnretainedValue()
    return event.request.map { Unmanaged.passUnretained($0).toOpaque() }
}

@_cdecl("foundation_network_event_redirect")
public func foundation_network_event_redirect(_ handle: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    let event = Unmanaged<NetworkEvent>.fromOpaque(handle).takeUnretainedValue()
    return event.redirect.map { Unmanaged.passRetained($0).toOpaque() }
}

@_cdecl("foundation_network_event_error")
public func foundation_network_event_error(_ handle: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    let event = Unmanaged<NetworkEvent>.fromOpaque(handle).takeUnretainedValue()
    return event.error.map { Unmanaged.passUnretained($0).toOpaque() }
}

@_cdecl("foundation_network_event_text")
public func foundation_network_event_text(_ handle: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let event = Unmanaged<NetworkEvent>.fromOpaque(handle).takeUnretainedValue()
    return event.text?.withCString { strdup($0) }
}

@_cdecl("foundation_network_event_bytes")
public func foundation_network_event_bytes(_ handle: UnsafeMutableRawPointer, _ count: UnsafeMutablePointer<Int>) -> UnsafePointer<UInt8>? {
    let event = Unmanaged<NetworkEvent>.fromOpaque(handle).takeUnretainedValue()
    count.pointee = event.data?.length ?? 0
    return event.data?.bytes.assumingMemoryBound(to: UInt8.self)
}

@_cdecl("foundation_network_event_number")
public func foundation_network_event_number(_ handle: UnsafeMutableRawPointer, _ index: Int32) -> Int64 {
    let event = Unmanaged<NetworkEvent>.fromOpaque(handle).takeUnretainedValue()
    return index == 0 ? event.first : event.second
}

@_cdecl("foundation_network_event_flags")
public func foundation_network_event_flags(_ handle: UnsafeMutableRawPointer) -> Int32 {
    Unmanaged<NetworkEvent>.fromOpaque(handle).takeUnretainedValue().flags
}

@_cdecl("foundation_network_response_url")
public func foundation_network_response_url(_ handle: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let response = Unmanaged<NetworkResponse>.fromOpaque(handle).takeUnretainedValue()
    return response.value.url?.absoluteString.withCString { strdup($0) }
}

@_cdecl("foundation_network_response_status")
public func foundation_network_response_status(_ handle: UnsafeMutableRawPointer) -> Int64 {
    Int64(Unmanaged<NetworkResponse>.fromOpaque(handle).takeUnretainedValue().value.statusCode)
}

@_cdecl("foundation_network_response_length")
public func foundation_network_response_length(_ handle: UnsafeMutableRawPointer) -> Int64 {
    Unmanaged<NetworkResponse>.fromOpaque(handle).takeUnretainedValue().value.expectedContentLength
}

@_cdecl("foundation_network_response_header_count")
public func foundation_network_response_header_count(_ handle: UnsafeMutableRawPointer) -> Int {
    Unmanaged<NetworkResponse>.fromOpaque(handle).takeUnretainedValue().headers.count
}

@_cdecl("foundation_network_response_header_name")
public func foundation_network_response_header_name(_ handle: UnsafeMutableRawPointer, _ index: Int) -> UnsafeMutablePointer<CChar>? {
    let response = Unmanaged<NetworkResponse>.fromOpaque(handle).takeUnretainedValue()
    return response.headers[index].0.withCString { strdup($0) }
}

@_cdecl("foundation_network_response_header_value")
public func foundation_network_response_header_value(_ handle: UnsafeMutableRawPointer, _ index: Int) -> UnsafeMutablePointer<CChar>? {
    let response = Unmanaged<NetworkResponse>.fromOpaque(handle).takeUnretainedValue()
    return response.headers[index].1.withCString { strdup($0) }
}

@_cdecl("foundation_network_redirect_resolve")
public func foundation_network_redirect_resolve(_ handle: UnsafeMutableRawPointer, _ request: UnsafeMutableRawPointer?) -> Bool {
    let redirect = Unmanaged<NetworkRedirect>.fromOpaque(handle).takeUnretainedValue()
    let request = request.map { Unmanaged<NetworkRequest>.fromOpaque($0).takeUnretainedValue() }
    return redirect.resolve(request)
}

@_cdecl("foundation_network_redirect_release")
public func foundation_network_redirect_release(_ handle: UnsafeMutableRawPointer) {
    Unmanaged<NetworkRedirect>.fromOpaque(handle).release()
}

@_cdecl("foundation_network_error_domain")
public func foundation_network_error_domain(_ handle: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let error = Unmanaged<NSError>.fromOpaque(handle).takeUnretainedValue()
    return error.domain.withCString { strdup($0) }
}

@_cdecl("foundation_network_error_code")
public func foundation_network_error_code(_ handle: UnsafeMutableRawPointer) -> Int64 {
    Int64(Unmanaged<NSError>.fromOpaque(handle).takeUnretainedValue().code)
}

@_cdecl("foundation_network_error_description")
public func foundation_network_error_description(_ handle: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let error = Unmanaged<NSError>.fromOpaque(handle).takeUnretainedValue()
    return error.localizedDescription.withCString { strdup($0) }
}

@_cdecl("foundation_network_error_details")
public func foundation_network_error_details(_ handle: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let error = Unmanaged<NSError>.fromOpaque(handle).takeUnretainedValue()
    return String(describing: error.userInfo).withCString { strdup($0) }
}

@_cdecl("foundation_network_error_trust")
public func foundation_network_error_trust(_ handle: UnsafeMutableRawPointer) -> UnsafeMutablePointer<CChar>? {
    let error = Unmanaged<NSError>.fromOpaque(handle).takeUnretainedValue()
    guard let value = error.userInfo[NSURLErrorFailingURLPeerTrustErrorKey],
          CFGetTypeID(value as CFTypeRef) == SecTrustGetTypeID() else { return nil }
    let trust = value as! SecTrust
    guard let result = SecTrustCopyResult(trust) else { return nil }
    return String(describing: result).withCString { strdup($0) }
}

@_cdecl("foundation_network_error_underlying")
public func foundation_network_error_underlying(_ handle: UnsafeMutableRawPointer) -> UnsafeMutableRawPointer? {
    let error = Unmanaged<NSError>.fromOpaque(handle).takeUnretainedValue()
    return (error.userInfo[NSUnderlyingErrorKey] as? NSError).map { Unmanaged.passUnretained($0).toOpaque() }
}

@_cdecl("foundation_network_error_socket_code")
public func foundation_network_error_socket_code(_ handle: UnsafeMutableRawPointer, _ code: UnsafeMutablePointer<Int32>) -> Bool {
    let error = Unmanaged<NSError>.fromOpaque(handle).takeUnretainedValue()
    if error.domain == NSPOSIXErrorDomain {
        code.pointee = Int32(clamping: error.code)
        return true
    }
    guard let domain = error.userInfo["_kCFStreamErrorDomainKey"] as? NSNumber, domain.intValue == 1,
          let value = error.userInfo["_kCFStreamErrorCodeKey"] as? NSNumber else { return false }
    code.pointee = value.int32Value
    return true
}
#endif
