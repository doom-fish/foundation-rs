import Foundation

@_cdecl("foundation_string_free")
public func foundation_string_free(_ string: UnsafeMutablePointer<CChar>?) {
    free(string)
}
