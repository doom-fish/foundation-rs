# Foundation coverage audit

Date: 2026-11-01. SDK: macOS 27.0.

This audit compares the file operation additions with the SDK symbol names.
It does not claim complete Foundation coverage.
The other domains remain in `COVERAGE.md`.

| SDK symbol | Rust surface | Difference |
| --- | --- | --- |
| `NSFileManager.trashItemAtURL:resultingItemURL:error:` | `FileManager::trash_item` | Returns the resulting path or a native error. |
| `NSURL.setResourceValues:error:`, `NSURLIsExcludedFromBackupKey` | `FileManager::set_excluded_from_backup` | Only the backup exclusion flag. |
| `NSURL.resourceValuesForKeys:error:`, `NSURLIsExcludedFromBackupKey` | `FileManager::is_excluded_from_backup` | Only the backup exclusion flag. |
| `NSURL.initFileURLWithFileSystemRepresentation:isDirectory:relativeToURL:`, `fileSystemRepresentation` | Internal path conversion | No general URL constructor or resource API. |
| `NSError.domain`, `code`, `localizedDescription` | `FoundationError::Native` | No user information or underlying error object. |
| `NSFileManager.copyItemAtURL:toURL:error:`, `moveItemAtURL:toURL:error:`, `removeItemAtURL:error:` | Not implemented | No file copy, move or permanent-delete wrapper. |
| `NSFileManager.enumeratorAtURL:includingPropertiesForKeys:options:errorHandler:` | Not implemented | No directory enumeration wrapper. |

## Verification

Clippy for all targets and the documentation build passed.
All 17 existing tests and three documentation examples passed.
`trash_item` returned a Trash path and removed its example file.
`backup_exclusion` verified both native flag values against `tmutil isexcluded`, with the parent exclusion included.
The examples created their source items in `target` and removed them.
The minimum supported macOS version requires a separate check.
