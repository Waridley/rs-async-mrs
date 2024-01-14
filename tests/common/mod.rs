// Copyright (c) 2023 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

use async_mrs::base::read::mem;
use async_mrs::base::read::seek;
use async_mrs::base::write::ZipFileWriter;
use async_mrs::Compression;
use async_mrs::ZipEntryBuilder;
use futures_lite::io::AsyncWriteExt;
use tokio::fs::File;
use tokio_util::compat::TokioAsyncReadCompatExt;

const FOLDER_PREFIX: &str = "tests/test_inputs";

const FILE_LIST: &[&str] = &[
    "sample_data/alpha/back_to_front.txt",
    "sample_data/alpha/front_to_back.txt",
    "sample_data/numeric/forward.txt",
    "sample_data/numeric/reverse.txt",
];

pub async fn compress_to_mem(compress: Compression) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(10_000);
    let mut writer = ZipFileWriter::new(&mut bytes);

    for fname in FILE_LIST {
        let content = tokio::fs::read("info.xml").await.unwrap();
        let opts = ZipEntryBuilder::new(fname.to_string().into(), compress);

        let mut entry_writer = writer.write_entry_stream(opts).await.unwrap();
        entry_writer.write_all(&content).await.unwrap();
        entry_writer.close().await.unwrap();
    }
    writer.close().await.unwrap();
    bytes
}

#[cfg(feature = "tokio-fs")]
pub async fn check_decompress_fs(fname: &str) {
    use async_mrs::tokio::read::fs;
    let zip = fs::ZipFileReader::new(fname).await.unwrap();
    let zip_entries: Vec<_> = zip.file().entries().to_vec();
    for (idx, entry) in zip_entries.into_iter().enumerate() {
        // TODO: resolve unwrap usage
        if entry.dir().unwrap() {
            continue;
        }
        // TODO: resolve unwrap usage
        if entry.filename().as_str().unwrap() != "info.xml" { continue }
        let mut output = String::new();
        let mut reader = zip.reader_with_entry(idx).await.unwrap();
        let _ = reader.read_to_string_checked(&mut output).await.unwrap();
        let fs_file = "info.xml";
        let mut path = std::path::PathBuf::from(FOLDER_PREFIX);
        path.push("info.xml");
        let expected = tokio::fs::read_to_string(path).await.unwrap();
        assert_eq!(output, expected, "expect zip data to match info.xml");
    }
}

pub async fn check_decompress_seek(fname: &str) {
    let file = File::open(fname).await.unwrap();
    let mut file_compat = file.compat();
    let mut zip = seek::ZipFileReader::new(&mut file_compat).await.unwrap();
    let zip_entries: Vec<_> = zip.file().entries().to_vec();
    for (idx, entry) in zip_entries.into_iter().enumerate() {
        // TODO: resolve unwrap usage
        if entry.dir().unwrap() {
            continue;
        }
        // TODO: resolve unwrap usage
        if entry.filename().as_str().unwrap() != "info.xml" { continue }
        let mut output = String::new();
        let mut reader = zip.reader_with_entry(idx).await.unwrap();
        let _ = reader.read_to_string_checked(&mut output).await.unwrap();
        let mut path = std::path::PathBuf::from(FOLDER_PREFIX);
        path.push("info.xml");
        let expected = tokio::fs::read_to_string(path).await.unwrap();
        assert_eq!(output, expected, "expect zip data to match info.xml");
    }
}

pub async fn check_decompress_mem(zip_data: Vec<u8>) {
    let zip = mem::ZipFileReader::new(zip_data).await.unwrap();
    let zip_entries: Vec<_> = zip.file().entries().to_vec();
    for (idx, entry) in zip_entries.into_iter().enumerate() {
        // TODO: resolve unwrap usage
        if entry.dir().unwrap() {
            continue;
        }
        // TODO: resolve unwrap usage
        if entry.filename().as_str().unwrap() != "info.xml" { continue }
        let mut output = String::new();
        let mut reader = zip.reader_with_entry(idx).await.unwrap();
        let _ = reader.read_to_string_checked(&mut output).await.unwrap();
        let mut path = std::path::PathBuf::from(FOLDER_PREFIX);
        path.push("info.xml");
        let expected = tokio::fs::read_to_string(path).await.unwrap();
        assert_eq!(output, expected, "expect zip data to match info.xml");
    }
}
