use std::fs;
use std::io::Write;

use roozle as rz;

#[test]
fn test_buffer_from_file() {
    let file_path = "/tmp/roozle_test.txt";
    let content = "Roozle is a fast string searching library.\nIt is currently in development mode.\n";

    let mut file = fs::File::create(file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();

    let buffer = rz::Buffer::from_file(file_path).unwrap();

    assert_eq!(*buffer, content.to_string());
    assert_eq!(buffer.len(), content.len());
    assert_eq!(buffer.word_count(), content.split_whitespace().count());
    assert_eq!(buffer.line_count(), content.lines().count());

    fs::remove_file(file_path).unwrap();
}

#[test]
fn test_buffer_from_string() {
    let content = "Roozle is a fast string searching library.\nIt is currently in development mode.\n";

    let buffer = rz::Buffer::from_string(content);

    assert_eq!(*buffer, content.to_string());
    assert_eq!(buffer.len(), content.len());
    assert_eq!(buffer.word_count(), content.split_whitespace().count());
    assert_eq!(buffer.line_count(), content.lines().count());
}

#[test]
fn test_buffer_push_string() {
    let content = "Roozle is a fast string searching library.";

    let mut buffer = rz::Buffer::new();

    let words = content.split_whitespace().collect::<Vec<&str>>();
    let mut words_iter = words.iter();

    buffer.push_string(words_iter.next().unwrap());
    for word in words_iter {
        buffer.push_string(" ");
        buffer.push_string(word);
    }

    assert_eq!(*buffer, content.to_string());
    assert_eq!(buffer.len(), content.len());
    assert_eq!(buffer.word_count(), content.split_whitespace().count());
}