import Foundation

private func reportFileError(
    _ error: Error,
    _ domain: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ code: UnsafeMutablePointer<Int64>,
    _ message: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) {
    let error = error as NSError
    domain.pointee = strdup(error.domain)
    code.pointee = Int64(error.code)
    message.pointee = strdup(error.localizedDescription)
}

@_cdecl("foundation_file_manager_trash_item")
public func foundation_file_manager_trash_item(
    _ path: UnsafePointer<CChar>,
    _ domain: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ code: UnsafeMutablePointer<Int64>,
    _ message: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> UnsafeMutablePointer<CChar>? {
    autoreleasepool {
        do {
            let url = URL(fileURLWithFileSystemRepresentation: path, isDirectory: false, relativeTo: nil)
            var result: NSURL?
            try FileManager.default.trashItem(at: url, resultingItemURL: &result)
            guard let result else {
                throw NSError(domain: NSCocoaErrorDomain, code: NSFileWriteUnknownError)
            }
            return (result as URL).withUnsafeFileSystemRepresentation { bytes in
                guard let bytes else {
                    return nil
                }
                return strdup(bytes)
            }
        } catch {
            reportFileError(error, domain, code, message)
            return nil
        }
    }
}

@_cdecl("foundation_file_manager_set_excluded_from_backup")
public func foundation_file_manager_set_excluded_from_backup(
    _ path: UnsafePointer<CChar>,
    _ excluded: Bool,
    _ domain: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ code: UnsafeMutablePointer<Int64>,
    _ message: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Bool {
    autoreleasepool {
        do {
            var url = URL(fileURLWithFileSystemRepresentation: path, isDirectory: false, relativeTo: nil)
            var values = URLResourceValues()
            values.isExcludedFromBackup = excluded
            try url.setResourceValues(values)
            return true
        } catch {
            reportFileError(error, domain, code, message)
            return false
        }
    }
}

@_cdecl("foundation_file_manager_is_excluded_from_backup")
public func foundation_file_manager_is_excluded_from_backup(
    _ path: UnsafePointer<CChar>,
    _ domain: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ code: UnsafeMutablePointer<Int64>,
    _ message: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>
) -> Int32 {
    autoreleasepool {
        do {
            let url = URL(fileURLWithFileSystemRepresentation: path, isDirectory: false, relativeTo: nil)
            let values = try url.resourceValues(forKeys: [.isExcludedFromBackupKey])
            return values.isExcludedFromBackup == true ? 1 : 0
        } catch {
            reportFileError(error, domain, code, message)
            return -1
        }
    }
}
