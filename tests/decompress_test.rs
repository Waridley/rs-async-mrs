// Copyright (c) 2023 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use tokio_util::compat::TokioAsyncReadCompatExt;

mod common;

const TEST_FILE: &str = "tests/test_inputs/Bourree_annotated.mrs";

#[cfg(feature = "zstd")]
#[tokio::test]
async fn decompress_zstd_zip_seek() {
    common::check_decompress_seek(ZSTD_ZIP_FILE).await
}

#[cfg(feature = "deflate")]
#[tokio::test]
async fn decompress_deflate_zip_seek() {
    common::check_decompress_seek(TEST_FILE).await
}

#[tokio::test]
async fn check_empty_zip_seek() {
    let mut data: Vec<u8> = Vec::new();
    async_mrs::base::write::ZipFileWriter::new(futures::io::Cursor::new(&mut data)).close().await.unwrap();
    async_mrs::base::read::seek::ZipFileReader::new(futures::io::Cursor::new(&data)).await.unwrap();
}

#[cfg(feature = "deflate")]
#[tokio::test]
async fn decompress_deflate_zip_mem() {
    let content = tokio::fs::read(TEST_FILE).await.unwrap();
    common::check_decompress_mem(content).await
}

#[cfg(feature = "deflate")]
#[cfg(feature = "tokio-fs")]
#[tokio::test]
async fn decompress_deflate_zip_fs() {
    common::check_decompress_fs(TEST_FILE).await
}

// TODO: Not sure if .mrs files have extra fields
// #[tokio::test]
// async fn decompress_zip_with_utf8_extra() {
//     let file = tokio::fs::File::open(TEST_FILE).await.unwrap();
//     let mut file_compat = file.compat();
//     let zip = async_mrs::base::read::seek::ZipFileReader::new(&mut file_compat).await.unwrap();
//     let zip_entries: Vec<_> = zip.file().entries().to_vec();
//     assert_eq!(zip_entries.len(), 1);
//     assert_eq!(zip_entries[0].header_size(), 93);
//     assert_eq!(zip_entries[0].filename().as_str().unwrap(), "\u{4E2D}\u{6587}.txt");
//     assert_eq!(zip_entries[0].filename().alternative(), Some(b"\xD6\xD0\xCe\xC4.txt".as_ref()));
// }
