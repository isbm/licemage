use super::PkgLicence;

/// Common interface for a license parser.
/// This is not (!) used for the actual licence text determination,
/// but used only for metadata parsing, such files for machine readable formats
/// as /usr/share/doc/<package>/copyright (1.0) or RPM specfile
pub trait PkgLicenceItf {
    fn into(self) -> PkgLicence;
}
