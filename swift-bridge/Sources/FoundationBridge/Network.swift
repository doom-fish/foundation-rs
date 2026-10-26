#if FOUNDATION_NETWORK
import Foundation
import CoreFoundation
import Security
import Darwin

public typealias NetworkContextFunction = @convention(c) (UnsafeMutableRawPointer?) -> Void
public typealias NetworkEventFunction = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void

final class NetworkCallback {
    let context: UnsafeMutableRawPointer
    let callback: NetworkEventFunction
    let retainContext: NetworkContextFunction
    let releaseContext: NetworkContextFunction

    init(
        context: UnsafeMutableRawPointer,
        callback: @escaping NetworkEventFunction,
        retain: @escaping NetworkContextFunction,
        release: @escaping NetworkContextFunction
    ) {
        self.context = context
        self.callback = callback
        retainContext = retain
        releaseContext = release
        retain(context)
    }

    deinit {
        releaseContext(context)
    }

    func call(_ event: NetworkEvent) {
        autoreleasepool {
            retainContext(context)
            defer { releaseContext(context) }
            callback(context, Unmanaged.passUnretained(event).toOpaque())
        }
    }
}

final class NetworkRequest {
    var value: URLRequest
    var headers: [(String, String)]
    let explicitTimeout: Bool

    init(_ value: URLRequest, explicitTimeout: Bool = true) {
        self.value = value
        self.explicitTimeout = explicitTimeout
        headers = (value.allHTTPHeaderFields ?? [:]).sorted { $0.key < $1.key }
    }
}

final class NetworkResponse {
    let value: HTTPURLResponse
    let headers: [(String, String)]

    init(_ value: HTTPURLResponse) {
        self.value = value
        headers = value.allHeaderFields.map { (String(describing: $0.key), String(describing: $0.value)) }
            .sorted { $0.0 < $1.0 }
    }

    var valid: Bool {
        headers.count <= 100 && headers.allSatisfy { $0.0.utf8.count + $0.1.utf8.count <= 100 * 1024 }
    }
}

final class NetworkEvent {
    let kind: Int32
    var response: NetworkResponse?
    var request: NetworkRequest?
    var redirect: NetworkRedirect?
    var error: NSError?
    var text: String?
    var data: NSData?
    var first: Int64 = 0
    var second: Int64 = 0
    var flags: Int32 = 0

    init(_ kind: Int32, error: NSError? = nil) {
        self.kind = kind
        self.error = error
    }
}

final class NetworkRedirect {
    let queue: OperationQueue
    let lock = NSLock()
    var completion: ((NetworkRequest?) -> Void)?

    init(queue: OperationQueue, completion: @escaping (NetworkRequest?) -> Void) {
        self.queue = queue
        self.completion = completion
    }

    @discardableResult
    func resolve(_ request: NetworkRequest?) -> Bool {
        let handler = lock.withLock { () -> ((NetworkRequest?) -> Void)? in
            let handler = completion
            completion = nil
            return handler
        }
        guard let handler else { return false }
        autoreleasepool { queue.addOperation { autoreleasepool { handler(request) } } }
        return true
    }

    deinit {
        if let handler = completion {
            autoreleasepool { queue.addOperation { autoreleasepool { handler(nil) } } }
        }
    }
}

final class NetworkUpload {
    let input: CFReadStream
    let output: CFWriteStream
    let lock = NSLock()
    let length: Int64
    var written: Int64 = 0
    var ended = false
    var cancelled = false

    init?(length: Int64) {
        var read: Unmanaged<CFReadStream>?
        var write: Unmanaged<CFWriteStream>?
        CFStreamCreateBoundPair(nil, &read, &write, 256 * 1024)
        guard let read, let write else {
            read?.release()
            write?.release()
            return nil
        }
        input = read.takeRetainedValue()
        output = write.takeRetainedValue()
        self.length = length
        guard CFWriteStreamOpen(output) else { return nil }
    }

    var stream: InputStream {
        unsafeBitCast(input, to: InputStream.self)
    }

    func write(_ bytes: UnsafePointer<UInt8>, count: Int) -> Int {
        while true {
            lock.lock()
            if cancelled || ended || Int64(count) > length - written {
                lock.unlock()
                return -1
            }
            let status = CFWriteStreamGetStatus(output)
            if status == .error || status == .closed || status == .atEnd {
                lock.unlock()
                return -1
            }
            if CFWriteStreamCanAcceptBytes(output) {
                let count = CFWriteStreamWrite(output, bytes, count)
                if count > 0 { written += Int64(count) }
                lock.unlock()
                return count
            }
            lock.unlock()
            Thread.sleep(forTimeInterval: 0.005)
        }
    }

    func finish() -> Bool {
        lock.withLock {
            guard !cancelled && written == length else { return false }
            if !ended {
                ended = true
                CFWriteStreamClose(output)
            }
            return true
        }
    }

    func abort() {
        lock.withLock {
            cancelled = true
            CFWriteStreamClose(output)
            CFReadStreamClose(input)
        }
    }

    func releaseWriter() {
        lock.withLock {
            if !ended {
                cancelled = true
                CFWriteStreamClose(output)
                CFReadStreamClose(input)
            }
        }
    }

    deinit {
        CFWriteStreamClose(output)
        CFReadStreamClose(input)
    }
}

func localIPv4(_ address: UInt32) -> Bool {
    address >> 24 == 127
        || address >> 24 == 0
        || address & 0xff000000 == 0x0a000000
        || address & 0xfff00000 == 0xac100000
        || address & 0xffff0000 == 0xc0a80000
        || address & 0xffff0000 == 0xa9fe0000
        || address & 0xffc00000 == 0x64400000
}

func localIPv6(_ address: in6_addr) -> Bool {
    withUnsafeBytes(of: address) { bytes in
        if bytes.prefix(15).allSatisfy({ $0 == 0 }) && bytes[15] <= 1 { return true }
        if bytes[0] & 0xfe == 0xfc { return true }
        if bytes[0] == 0xfe && bytes[1] & 0x80 != 0 { return true }
        if bytes.prefix(10).allSatisfy({ $0 == 0 }) && bytes[10] == 255 && bytes[11] == 255 {
            let value = UInt32(bytes[12]) << 24 | UInt32(bytes[13]) << 16
                | UInt32(bytes[14]) << 8 | UInt32(bytes[15])
            return localIPv4(value)
        }
        return false
    }
}

func localAddress(_ address: UnsafePointer<sockaddr>) -> Bool {
    if Int32(address.pointee.sa_family) == AF_INET {
        return address.withMemoryRebound(to: sockaddr_in.self, capacity: 1) {
            localIPv4(UInt32(bigEndian: $0.pointee.sin_addr.s_addr))
        }
    }
    if Int32(address.pointee.sa_family) == AF_INET6 {
        return address.withMemoryRebound(to: sockaddr_in6.self, capacity: 1) {
            localIPv6($0.pointee.sin6_addr)
        }
    }
    return false
}

func localHost(_ url: URL) -> Bool {
    guard var host = url.host?.lowercased() else { return false }
    host = host.trimmingCharacters(in: CharacterSet(charactersIn: "[]"))
    if host.hasSuffix(".") { host.removeLast() }
    if host == "localhost" || host.hasSuffix(".localhost") || host.hasSuffix(".local") {
        return true
    }
    var ipv4 = in_addr()
    if inet_pton(AF_INET, host, &ipv4) == 1 {
        return localIPv4(UInt32(bigEndian: ipv4.s_addr))
    }
    var ipv6 = in6_addr()
    let literal = String(host.split(separator: "%", maxSplits: 1).first ?? "")
    if inet_pton(AF_INET6, literal, &ipv6) == 1 { return localIPv6(ipv6) }
    if !host.contains(".") && !host.contains(":") { return true }
    var hints = addrinfo()
    hints.ai_family = AF_UNSPEC
    hints.ai_socktype = SOCK_STREAM
    var addresses: UnsafeMutablePointer<addrinfo>?
    guard getaddrinfo(host, nil, &hints, &addresses) == 0, let first = addresses else { return false }
    defer { freeaddrinfo(first) }
    var next: UnsafeMutablePointer<addrinfo>? = first
    while let entry = next {
        if let address = entry.pointee.ai_addr, localAddress(address) { return true }
        next = entry.pointee.ai_next
    }
    return false
}

final class NetworkTask {
    let owner: NetworkSession
    let identifier = UUID()
    let callback: NetworkCallback
    let webSocket: Bool
    let messageLimit: Int
    let deadline: Double
    let lock = NSLock()
    var request: URLRequest
    var current: URLSessionTask?
    var upload: NetworkUpload?
    var suppliedBody = false
    var timer: DispatchSourceTimer?
    var started = false
    var paused = false
    var generation: UInt64 = 0
    var direct = false
    var opened = false
    var closing = false
    var sending = false
    var receiving = false
    var pinging = false
    var finished = false
    var termination: NSError?
    var closeEvent: NetworkEvent?
    weak var pendingRedirect: NetworkRedirect?

    init(
        owner: NetworkSession,
        request: URLRequest,
        callback: NetworkCallback,
        webSocket: Bool,
        messageLimit: Int,
        deadline: Double
    ) {
        self.owner = owner
        self.request = request
        self.callback = callback
        self.webSocket = webSocket
        self.messageLimit = messageLimit
        let limit = webSocket ? request.timeoutInterval : owner.resourceTimeout
        self.deadline = deadline > 0 ? min(deadline, limit) : limit
    }

    var active: Bool {
        lock.withLock { !finished && termination == nil }
    }

    func emit(_ event: NetworkEvent) {
        if active { callback.call(event) }
    }

    func resume() {
        owner.enqueue { [self] in
            guard self.active else { return }
            let wasPaused = self.paused
            self.paused = false
            if self.started {
                if wasPaused { self.current?.resume() }
            } else {
                self.lock.withLock { self.started = true }
                if self.deadline > 0 {
                    let timer = DispatchSource.makeTimerSource(queue: DispatchQueue.global(qos: .utility))
                    timer.schedule(deadline: .now() + self.deadline)
                    timer.setEventHandler { [weak self] in
                        self?.cancel(NSError(domain: NSURLErrorDomain, code: NSURLErrorTimedOut))
                    }
                    self.timer = timer
                    timer.resume()
                }
                self.route(self.request)
            }
        }
    }

    func route(_ request: URLRequest) {
        guard active else { return }
        generation &+= 1
        let generation = generation
        DispatchQueue.global(qos: .utility).async {
            let direct = self.owner.directOnly || request.url.map(localHost) == true
            self.owner.enqueue {
                guard self.active && self.generation == generation else { return }
                self.direct = direct
                let session: URLSession?
                if self.webSocket {
                    session = direct ? self.owner.directWebSocketSession : self.owner.systemWebSocketSession
                } else {
                    session = direct ? self.owner.directSession : self.owner.systemSession
                }
                guard let session else {
                    self.cancel(NSError(domain: NSURLErrorDomain, code: NSURLErrorCancelled))
                    return
                }
                self.request = request
                let task: URLSessionTask
                if self.webSocket {
                    var nativeRequest = request
                    nativeRequest.timeoutInterval = Double(UInt32.max)
                    let socket = session.webSocketTask(with: nativeRequest)
                    socket.maximumMessageSize = self.messageLimit
                    task = socket
                } else if let upload = self.upload {
                    var request = request
                    request.httpBody = nil
                    request.httpBodyStream = nil
                    request.setValue(String(upload.length), forHTTPHeaderField: "Content-Length")
                    task = session.uploadTask(withStreamedRequest: request)
                } else {
                    task = session.dataTask(with: request)
                }
                self.current = task
                self.owner.attach(task, to: self)
                if !self.paused { task.resume() }
            }
        }
    }

    func cancel(_ error: NSError) {
        let result = lock.withLock { () -> (Bool, NetworkUpload?) in
            if finished || termination != nil { return (false, nil) }
            termination = error
            return (true, upload)
        }
        guard result.0 else { return }
        result.1?.abort()
        owner.enqueue {
            self.current?.cancel()
            self.pendingRedirect?.resolve(nil)
            self.finish(error)
        }
    }

    func finish(_ error: NSError?) {
        let result = lock.withLock { () -> (Bool, NSError?) in
            if finished { return (false, nil) }
            finished = true
            return (true, termination ?? error)
        }
        guard result.0 else { return }
        timer?.cancel()
        timer = nil
        if let upload, result.1 != nil || !upload.finish() { upload.abort() }
        pendingRedirect?.resolve(nil)
        owner.remove(self)
        if lock.withLock({ termination == nil }), let event = closeEvent {
            closeEvent = nil
            callback.call(event)
        }
        callback.call(NetworkEvent(5, error: result.1))
    }

    func reserve(_ kind: Int) -> Bool {
        lock.withLock {
            guard opened && !closing && !finished && termination == nil else { return false }
            switch kind {
            case 0:
                guard !sending else { return false }
                sending = true
            case 1:
                guard !receiving else { return false }
                receiving = true
            default:
                guard !pinging else { return false }
                pinging = true
            }
            return true
        }
    }

    func receive() -> Bool {
        guard reserve(1) else { return false }
        owner.enqueue {
            guard self.active, let socket = self.current as? URLSessionWebSocketTask else { return }
            socket.receive { result in
                self.owner.enqueue {
                    self.lock.withLock { self.receiving = false }
                    let event = NetworkEvent(7)
                    switch result {
                    case .success(.string(let text)):
                        event.data = Data(text.utf8) as NSData
                        event.flags = 1
                    case .success(.data(let data)):
                        event.data = data as NSData
                    case .failure(let error):
                        event.error = error as NSError
                    @unknown default:
                        event.error = NSError(domain: NSURLErrorDomain, code: NSURLErrorCannotDecodeContentData)
                    }
                    self.emit(event)
                }
            }
        }
        return true
    }

    func send(operation: Int64, text: Bool, data: Data) {
        owner.enqueue {
            guard self.active, let socket = self.current as? URLSessionWebSocketTask else { return }
            let message: URLSessionWebSocketTask.Message = text
                ? .string(String(decoding: data, as: UTF8.self)) : .data(data)
            socket.send(message) { error in
                self.owner.enqueue {
                    self.lock.withLock { self.sending = false }
                    let event = NetworkEvent(9, error: error as NSError?)
                    event.first = operation
                    self.emit(event)
                }
            }
        }
    }
}

final class NetworkSession: NSObject, URLSessionDataDelegate, URLSessionWebSocketDelegate {
    let queue: OperationQueue
    let directOnly: Bool
    let idleTimeout: Double
    let resourceTimeout: Double
    let lock = NSLock()
    var valid = true
    var tasks: [UUID: NetworkTask] = [:]
    var nativeTasks: [ObjectIdentifier: NetworkTask] = [:]
    var directSession: URLSession?
    var systemSession: URLSession?
    var directWebSocketSession: URLSession?
    var systemWebSocketSession: URLSession?

    init(queue: OperationQueue, idle: Double, resource: Double, connections: Int, directOnly: Bool) {
        self.queue = queue
        self.directOnly = directOnly
        idleTimeout = idle
        resourceTimeout = resource
        super.init()
        func configuration(direct: Bool, webSocket: Bool = false) -> URLSessionConfiguration {
            let config = URLSessionConfiguration.ephemeral
            config.urlCache = nil
            config.httpCookieStorage = nil
            config.httpShouldSetCookies = false
            config.urlCredentialStorage = nil
            config.requestCachePolicy = .reloadIgnoringLocalCacheData
            config.timeoutIntervalForRequest = webSocket ? Double(UInt32.max) : idle
            config.timeoutIntervalForResource = webSocket ? Double(UInt32.max) : resource
            config.httpMaximumConnectionsPerHost = connections
            config.tlsMinimumSupportedProtocolVersion = .TLSv12
            config.waitsForConnectivity = false
            if direct { config.connectionProxyDictionary = [:] }
            return config
        }
        directSession = URLSession(configuration: configuration(direct: true), delegate: self, delegateQueue: queue)
        systemSession = URLSession(configuration: configuration(direct: false), delegate: self, delegateQueue: queue)
        directWebSocketSession = URLSession(configuration: configuration(direct: true, webSocket: true), delegate: self, delegateQueue: queue)
        systemWebSocketSession = URLSession(configuration: configuration(direct: false, webSocket: true), delegate: self, delegateQueue: queue)
    }

    func enqueue(_ body: @escaping () -> Void) {
        autoreleasepool {
            if OperationQueue.current === queue {
                body()
            } else {
                queue.addOperation { autoreleasepool(invoking: body) }
            }
        }
    }

    func register(_ task: NetworkTask) -> Bool {
        lock.withLock {
            guard valid else { return false }
            tasks[task.identifier] = task
            return true
        }
    }

    func attach(_ native: URLSessionTask, to task: NetworkTask) {
        lock.withLock { nativeTasks[ObjectIdentifier(native)] = task }
    }

    func find(_ native: URLSessionTask) -> NetworkTask? {
        lock.withLock { nativeTasks[ObjectIdentifier(native)] }
    }

    func remove(_ task: NetworkTask) {
        lock.withLock {
            tasks.removeValue(forKey: task.identifier)
            if let native = task.current {
                nativeTasks.removeValue(forKey: ObjectIdentifier(native))
            }
        }
    }

    func invalidate() {
        let children = lock.withLock { () -> [NetworkTask] in
            valid = false
            return Array(tasks.values)
        }
        for task in children {
            task.cancel(NSError(domain: NSURLErrorDomain, code: NSURLErrorCancelled))
        }
        enqueue {
            self.directSession?.invalidateAndCancel()
            self.systemSession?.invalidateAndCancel()
            self.directWebSocketSession?.invalidateAndCancel()
            self.systemWebSocketSession?.invalidateAndCancel()
        }
    }

    func urlSession(_ session: URLSession, didBecomeInvalidWithError error: Error?) {
        if session === directSession { directSession = nil }
        if session === systemSession { systemSession = nil }
        if session === directWebSocketSession { directWebSocketSession = nil }
        if session === systemWebSocketSession { systemWebSocketSession = nil }
    }

    func urlSession(
        _ session: URLSession,
        dataTask: URLSessionDataTask,
        didReceive response: URLResponse,
        completionHandler: @escaping (URLSession.ResponseDisposition) -> Void
    ) {
        guard let task = find(dataTask), task.active, let http = response as? HTTPURLResponse else {
            completionHandler(.cancel)
            return
        }
        let response = NetworkResponse(http)
        guard response.valid else {
            completionHandler(.cancel)
            task.cancel(NSError(domain: NSURLErrorDomain, code: NSURLErrorCannotParseResponse))
            return
        }
        let event = NetworkEvent(1)
        event.response = response
        task.emit(event)
        completionHandler(task.active ? .allow : .cancel)
    }

    func urlSession(_ session: URLSession, dataTask: URLSessionDataTask, didReceive data: Data) {
        guard let task = find(dataTask) else { return }
        let event = NetworkEvent(2)
        event.data = data as NSData
        task.emit(event)
    }

    func urlSession(
        _ session: URLSession,
        task native: URLSessionTask,
        didSendBodyData bytesSent: Int64,
        totalBytesSent: Int64,
        totalBytesExpectedToSend: Int64
    ) {
        guard let task = find(native) else { return }
        let event = NetworkEvent(3)
        event.first = totalBytesSent
        event.second = totalBytesExpectedToSend
        task.emit(event)
    }

    func urlSession(
        _ session: URLSession,
        task native: URLSessionTask,
        needNewBodyStream completionHandler: @escaping (InputStream?) -> Void
    ) {
        guard let task = find(native), task.active, let upload = task.upload, !task.suppliedBody else {
            completionHandler(nil)
            return
        }
        task.suppliedBody = true
        completionHandler(upload.stream)
    }

    func urlSession(
        _ session: URLSession,
        task native: URLSessionTask,
        willPerformHTTPRedirection response: HTTPURLResponse,
        newRequest request: URLRequest,
        completionHandler: @escaping (URLRequest?) -> Void
    ) {
        guard let task = find(native), task.active else {
            completionHandler(nil)
            return
        }
        let response = NetworkResponse(response)
        guard response.valid else {
            completionHandler(nil)
            task.cancel(NSError(domain: NSURLErrorDomain, code: NSURLErrorCannotParseResponse))
            return
        }
        let decision = NetworkRedirect(queue: queue) { [weak task] proposed in
            guard let task, task.active else {
                completionHandler(nil)
                return
            }
            task.pendingRedirect = nil
            guard let proposed else {
                completionHandler(nil)
                return
            }
            var request = proposed.value
            if !proposed.explicitTimeout { request.timeoutInterval = task.request.timeoutInterval }
            if task.webSocket, let url = request.url, var components = URLComponents(url: url, resolvingAgainstBaseURL: true) {
                if components.scheme == "http" { components.scheme = "ws" }
                if components.scheme == "https" { components.scheme = "wss" }
                request.url = components.url
            }
            guard let scheme = request.url?.scheme?.lowercased(),
                  task.webSocket ? ["ws", "wss"].contains(scheme) : ["http", "https"].contains(scheme) else {
                completionHandler(nil)
                task.cancel(NSError(domain: NSURLErrorDomain, code: NSURLErrorUnsupportedURL))
                return
            }
            self.lock.withLock { _ = self.nativeTasks.removeValue(forKey: ObjectIdentifier(native)) }
            completionHandler(nil)
            native.cancel()
            task.current = nil
            let upload = task.lock.withLock { () -> NetworkUpload? in
                let upload = task.upload
                task.upload = nil
                return upload
            }
            upload?.abort()
            task.route(request)
        }
        task.pendingRedirect = decision
        let event = NetworkEvent(4)
        event.response = response
        var proposed = request
        proposed.timeoutInterval = task.request.timeoutInterval
        event.request = NetworkRequest(proposed)
        event.redirect = decision
        task.emit(event)
    }

    func urlSession(
        _ session: URLSession,
        task native: URLSessionTask,
        didFinishCollecting metrics: URLSessionTaskMetrics
    ) {
        guard let task = find(native) else { return }
        for metric in metrics.transactionMetrics {
            let event = NetworkEvent(11)
            event.text = metric.networkProtocolName
            event.first = metric.negotiatedTLSProtocolVersion.map { Int64($0.rawValue) } ?? 0
            event.flags = (metric.isReusedConnection ? 1 : 0)
                | (metric.isProxyConnection ? 2 : 0) | (task.direct ? 4 : 0)
            task.closeEvent = event
        }
    }

    func urlSession(_ session: URLSession, task native: URLSessionTask, didCompleteWithError error: Error?) {
        find(native)?.finish(error as NSError?)
    }

    func urlSession(
        _ session: URLSession,
        webSocketTask native: URLSessionWebSocketTask,
        didOpenWithProtocol protocolName: String?
    ) {
        guard let task = find(native), task.active else { return }
        let offered = task.request.value(forHTTPHeaderField: "Sec-WebSocket-Protocol")?
            .split(separator: ",").map { $0.trimmingCharacters(in: .whitespaces) } ?? []
        if let protocolName {
            guard offered.contains(protocolName) else {
                task.cancel(NSError(domain: NSURLErrorDomain, code: NSURLErrorBadServerResponse))
                return
            }
        } else if !offered.isEmpty {
            task.cancel(NSError(domain: NSURLErrorDomain, code: NSURLErrorBadServerResponse))
            return
        }
        task.timer?.cancel()
        task.timer = nil
        task.lock.withLock { task.opened = true }
        let event = NetworkEvent(6)
        event.text = protocolName
        task.emit(event)
    }

    func urlSession(
        _ session: URLSession,
        webSocketTask native: URLSessionWebSocketTask,
        didCloseWith closeCode: URLSessionWebSocketTask.CloseCode,
        reason: Data?
    ) {
        guard let task = find(native) else { return }
        task.lock.withLock { task.closing = true }
        let event = NetworkEvent(8)
        event.first = closeCode.rawValue == 0 ? 1005 : Int64(closeCode.rawValue)
        event.data = reason as NSData?
        task.emit(event)
    }
}
#endif
