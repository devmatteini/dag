use std::fmt::{Display, Formatter};
use std::path::Path;
use std::{ffi::OsString, path::PathBuf};

use crate::installer::error::InstallError;

#[derive(Debug, Eq, PartialEq)]
pub enum Compression {
    Gz,
    Xz,
    Bz2,
}

#[derive(Debug, Eq, PartialEq)]
pub enum FileType {
    Debian,
    TarArchive(Compression),
    ZipArchive,
    CompressedFile(Compression),
}

#[derive(Debug)]
pub struct FileInfo {
    pub path: PathBuf,
    pub name: String,
    extension: Option<OsString>,
}

#[derive(Debug)]
pub struct SupportedFileInfo {
    pub path: PathBuf,
    pub file_type: FileType,
}

pub fn validate_file(file: FileInfo) -> Result<SupportedFileInfo, InstallError> {
    file.extension
        .and_then(file_type_for)
        .map(|file_type| SupportedFileInfo {
            path: PathBuf::from(&file.path),
            file_type,
        })
        .ok_or_else(|| InstallError::not_supported(&file.name))
}

fn file_type_for(extension: OsString) -> Option<FileType> {
    if extension == "deb" {
        return Some(FileType::Debian);
    }
    if extension == "gz" || extension == "tgz" {
        return Some(FileType::TarArchive(Compression::Gz));
    }
    if extension == "bz2" || extension == "tbz" {
        return Some(FileType::TarArchive(Compression::Bz2));
    }
    if extension == "xz" || extension == "txz" {
        return Some(FileType::TarArchive(Compression::Xz));
    }
    if extension == "zip" {
        return Some(FileType::ZipArchive);
    }

    None
}

impl Display for Compression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Compression::Gz => f.write_str("gz"),
            Compression::Xz => f.write_str("xz"),
            Compression::Bz2 => f.write_str("bz2"),
        }
    }
}

impl FileInfo {
    pub fn new(name: &str, path: &Path) -> Self {
        FileInfo {
            path: PathBuf::from(path),
            name: String::from(name),
            extension: Path::new(name).extension().map(OsString::from),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use test_case::test_case;

    use super::{validate_file, Compression, FileInfo, FileType, SupportedFileInfo};
    use crate::installer::error::InstallError;

    #[test_case("deb", FileType::Debian)]
    #[test_case("gz", FileType::TarArchive(Compression::Gz))]
    #[test_case("tgz", FileType::TarArchive(Compression::Gz))]
    #[test_case("bz2", FileType::TarArchive(Compression::Bz2))]
    #[test_case("tbz", FileType::TarArchive(Compression::Bz2))]
    #[test_case("xz", FileType::TarArchive(Compression::Xz))]
    #[test_case("txz", FileType::TarArchive(Compression::Xz))]
    #[test_case("zip", FileType::ZipArchive)]
    fn supported_file(file_extension: &str, expected_file_type: FileType) {
        let file_info = any_file_info(Some(file_extension));
        let result = validate_file(file_info);

        assert_ok_equal(expected_file_type, result);
    }

    #[test_case("txt")]
    fn not_supported(file_extension: &str) {
        let file_info = any_file_info(Some(file_extension));
        let result = validate_file(file_info);

        assert_not_supported(result);
    }

    #[test]
    fn no_file_extension() {
        let file_info = any_file_info(None);
        let result = validate_file(file_info);

        assert_not_supported(result);
    }

    fn any_file_info(extension: Option<&str>) -> FileInfo {
        FileInfo {
            path: PathBuf::new(),
            name: "ANY".into(),
            extension: extension.map(|x| x.into()),
        }
    }

    fn assert_ok_equal(expected: FileType, actual: Result<SupportedFileInfo, InstallError>) {
        if let Ok(x) = actual {
            assert_eq!(expected, x.file_type);
        } else {
            panic!("Result is Err: {:?}", actual);
        }
    }

    fn assert_not_supported(actual: Result<SupportedFileInfo, InstallError>) {
        if let Err(e) = actual {
            match e {
                InstallError::NotSupported(_) => {}
                _ => panic!("expected InstallError::NotSupported. Got {}", e),
            }
        } else {
            panic!("Result is ok: {:?}", actual);
        }
    }
}
