import Foundation

private func fileSystemPath(_ url: URL) -> UnsafeMutablePointer<CChar>? {
    url.withUnsafeFileSystemRepresentation { bytes in
        guard let bytes else {
            return nil
        }
        return strdup(bytes)
    }
}

@_cdecl("foundation_bundle_identifier")
public func foundation_bundle_identifier() -> UnsafeMutablePointer<CChar>? {
    guard let identifier = Bundle.main.bundleIdentifier else {
        return nil
    }
    return strdup(identifier)
}

@_cdecl("foundation_bundle_path")
public func foundation_bundle_path() -> UnsafeMutablePointer<CChar>? {
    fileSystemPath(Bundle.main.bundleURL)
}

@_cdecl("foundation_bundle_url_for_auxiliary_executable")
public func foundation_bundle_url_for_auxiliary_executable(
    _ name: UnsafePointer<CChar>
) -> UnsafeMutablePointer<CChar>? {
    guard let url = Bundle.main.url(forAuxiliaryExecutable: String(cString: name)) else {
        return nil
    }
    return fileSystemPath(url)
}
