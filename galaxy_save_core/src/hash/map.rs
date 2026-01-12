use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    sync::{Arc, LazyLock},
};

use bimap::BiHashMap;
use encoding_rs::SHIFT_JIS;
use parking_lot::Mutex;
use thiserror::Error;

use super::HashCode;

/// A container for associating hashes with their original label and vice versa.
#[derive(Debug, Default)]
pub struct HashCodeMap {
    /// The collection of bidirectional relations between hashes and labels.
    map: BiHashMap<HashCode, String>,

    /// The configured character encoding for labels.
    encoding: Option<Encoding>,

    /// Determines if foreign labels should be rejected when looking up the hash of a label.
    strict: bool,
}

impl HashCodeMap {
    /// Returns a thread-safe reference-counting pointer to the mutually exclusive `HashCodeMap`.
    pub fn get() -> Arc<Mutex<Self>> {
        static INSTANCE: LazyLock<Arc<Mutex<HashCodeMap>>> =
            LazyLock::new(|| Arc::new(Mutex::new(HashCodeMap::default())));

        INSTANCE.clone()
    }

    /// Returns a reference to the label corresponding to the hash.
    pub fn label_of(&self, hash: HashCode, width: Option<u32>) -> Option<&String> {
        match width {
            None => self.map.get_by_left(&hash),
            Some(width) => {
                let mask = u32::MAX >> (u32::BITS - width);
                let hash = hash.into_raw() & mask;

                self.map
                    .iter()
                    .find(|(k, _)| k.into_raw() & mask == hash)
                    .map(|(_, v)| v)
            }
        }
    }

    /// Returns the hash corresponding to the label.
    pub fn hash_of(&self, label: &str) -> Option<HashCode> {
        self.map
            .get_by_right(label)
            .copied()
            .or_else(|| (!self.strict).then(|| HashCode::from(label)))
    }

    /// Hashes and inserts a collection of labels from an iterator, converting to Shift JIS.
    pub fn extend_shift_jis<I: IntoIterator<Item = String>>(
        &mut self,
        iter: I,
    ) -> Result<(), ParseLabelError> {
        self.extend(iter, Encoding::ShiftJis, encode_shift_jis)
    }

    /// Hashes and inserts a collection of labels from an iterator, interpreting as UTF-8.
    pub fn extend_utf8<I: IntoIterator<Item = String>>(
        &mut self,
        iter: I,
    ) -> Result<(), ParseLabelError> {
        self.extend(iter, Encoding::Utf8, encode_utf8)
    }

    /// Hashes and inserts a collection of labels from an iterator.
    fn extend<I, F>(
        &mut self,
        iter: I,
        encoding: Encoding,
        encode: F,
    ) -> Result<(), ParseLabelError>
    where
        I: IntoIterator<Item = String>,
        F: Fn(String) -> Result<(HashCode, String), ParseLabelError>,
    {
        if *self.encoding.get_or_insert(encoding) != encoding {
            return Err(ParseLabelError::InconsistentEncoding);
        }

        for label in iter {
            let (hash, label) = encode(label)?;

            self.map.insert(hash, label);
        }

        Ok(())
    }

    /// Reads and hashes a newline-separated list of labels from a file, converting to Shift JIS.
    pub fn read_shift_jis<P: AsRef<Path>>(&mut self, path: P) -> Result<(), ParseLabelError> {
        self.read(path, Encoding::ShiftJis, encode_shift_jis)
    }

    /// Reads and hashes a newline-separated list of labels from a file, interpreting as UTF-8.
    pub fn read_utf8<P: AsRef<Path>>(&mut self, path: P) -> Result<(), ParseLabelError> {
        self.read(path, Encoding::Utf8, encode_utf8)
    }

    /// Reads and hashes a newline-separated list of labels from a file.
    fn read<P, F>(&mut self, path: P, encoding: Encoding, encode: F) -> Result<(), ParseLabelError>
    where
        P: AsRef<Path>,
        F: Fn(String) -> Result<(HashCode, String), ParseLabelError>,
    {
        if *self.encoding.get_or_insert(encoding) != encoding {
            return Err(ParseLabelError::InconsistentEncoding);
        }

        let reader = BufReader::new(File::open(path)?);
        let map = reader
            .lines()
            .map(|l| encode(l?))
            .collect::<Result<BiHashMap<_, _>, _>>()?;

        self.map.extend(map);

        Ok(())
    }

    /// Updates whether foreign labels should be rejected when looking up the hash of a label.
    pub fn set_strict(&mut self, strict: bool) {
        self.strict = strict;
    }

    /// Clears the map, removing all key-value pairs. Keeps the allocated memory for reuse.
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Clears the map, removing all key-value pairs and resets additional configuration. Keeps the allocated memory for reuse.
    pub fn reset(&mut self) {
        self.clear();
        self.encoding = None;
        self.strict = false;
    }
}

/// An error returned from converting a collection of labels.
#[derive(Debug, Error)]
pub enum ParseLabelError {
    /// An error occurred while inserting labels into the map using an incompatible character encoding.
    #[error("the requested character encoding does not match the current character encoding")]
    InconsistentEncoding,

    /// An error occurred while performing I/O operations.
    #[error(transparent)]
    Io(#[from] io::Error),

    /// An error occurred while encoding from UTF-8 to Shift JIS.
    #[error("label contains Shift JIS errors")]
    EncodeShiftJis,
}

/// A character encoding for labels prior to hashing.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Encoding {
    /// Shift JIS
    ShiftJis,

    /// UTF-8
    Utf8,
}

/// Creates a hash-label pair from a string converted to Shift JIS.
fn encode_shift_jis(label: String) -> Result<(HashCode, String), ParseLabelError> {
    let (encoded, _, is_error) = SHIFT_JIS.encode(&label);

    if is_error {
        return Err(ParseLabelError::EncodeShiftJis);
    }

    Ok((HashCode::from(encoded.as_ref()), label))
}

/// Creates a hash-label pair from a string.
fn encode_utf8(label: String) -> Result<(HashCode, String), ParseLabelError> {
    Ok((HashCode::from(&label), label))
}
