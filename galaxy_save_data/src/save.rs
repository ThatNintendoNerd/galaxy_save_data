//! Essential save file format utilities.

use std::{
    fs,
    io::{Cursor, Seek, SeekFrom, Write},
    path::Path,
};

use binrw::{BinReaderExt, BinResult, BinWrite, Endian, binread};
use galaxy_save_core::{
    mem::Checksum,
    save::{CheckSaveFileError, SaveFileHeader},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod user_file;

pub use user_file::{SaveDataUserFile, SaveDataUserFileInfo};

/// The container for the save data.
#[binread]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct SaveDataFile {
    /// The supplementary information of the save file.
    #[br(temp)]
    header: SaveDataFileHeader,

    /// The collection of user file descriptors.
    #[br(count = header.user_file_info_num)]
    pub user_file_info: Vec<SaveDataUserFileInfo>,
}

impl SaveDataFile {
    /// Reads the data from the given file path in big-endian.
    pub fn read_be_file<P: AsRef<Path>>(path: P) -> BinResult<Self> {
        let mut reader = Cursor::new(fs::read(path)?);

        reader.read_be()
    }

    /// Reads the data from the given file path in little-endian.
    pub fn read_le_file<P: AsRef<Path>>(path: P) -> BinResult<Self> {
        let mut reader = Cursor::new(fs::read(path)?);

        reader.read_le()
    }

    /// Writes the data to the given file path in big-endian.
    pub fn write_be_file<P: AsRef<Path>>(&self, path: P) -> BinResult<()> {
        let mut writer = Cursor::new(Vec::with_capacity(
            SaveDataFileHeader::FILE_SIZE_MAX as usize,
        ));

        self.write_be(&mut writer)?;
        writer.rewind()?;

        let buf = &writer.get_ref()[size_of::<Checksum>()..];
        let checksum = Checksum::from_be_bytes(buf);

        checksum.write_be(&mut writer)?;
        fs::write(path, writer.get_mut())?;

        Ok(())
    }

    /// Writes the data to the given file path in little-endian.
    pub fn write_le_file<P: AsRef<Path>>(&self, path: P) -> BinResult<()> {
        let mut writer = Cursor::new(Vec::with_capacity(
            SaveDataFileHeader::FILE_SIZE_MAX as usize,
        ));

        self.write_le(&mut writer)?;
        writer.rewind()?;

        let buf = &writer.get_ref()[size_of::<Checksum>()..];
        let checksum = Checksum::from_le_bytes(buf);

        checksum.write_le(&mut writer)?;
        fs::write(path, writer.get_mut())?;

        Ok(())
    }

    /// Validates the data from the given file path in big-endian.
    pub fn check_be_file<P: AsRef<Path>>(path: P) -> Result<(), CheckSaveFileError> {
        let mut reader = Cursor::new(fs::read(path)?);
        let header = reader.read_be::<SaveDataFileHeader>()?;

        header.check_be(reader.into_inner().as_slice())
    }

    /// Validates the data from the given file path in little-endian.
    pub fn check_le_file<P: AsRef<Path>>(path: P) -> Result<(), CheckSaveFileError> {
        let mut reader = Cursor::new(fs::read(path)?);
        let header = reader.read_le::<SaveDataFileHeader>()?;

        header.check_le(reader.into_inner().as_slice())
    }
}

impl BinWrite for SaveDataFile {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        writer.seek(SeekFrom::Start(size_of::<SaveDataFileHeader>() as u64))?;

        let mut data_offset = writer.stream_position()? as u32
            + self.user_file_info.len() as u32 * SaveDataUserFileInfo::data_size() as u32;

        for user_file_info in &self.user_file_info {
            user_file_info.write_options(writer, endian, (data_offset,))?;

            let pos = writer.stream_position()?;
            data_offset = writer.seek(SeekFrom::End(0))? as u32;
            writer.seek(SeekFrom::Start(pos))?;
        }

        let version = SaveDataFileHeader::VERSION;
        let file_info_num = self.user_file_info.len() as u32;
        let file_size = writer.seek(SeekFrom::End(0))? as u32;

        writer.seek(SeekFrom::Start(size_of::<Checksum>() as u64))?;

        version.write_options(writer, endian, ())?;
        file_info_num.write_options(writer, endian, ())?;
        file_size.write_options(writer, endian, ())?;

        Ok(())
    }
}

/// The supplementary information of the save file.
#[binread]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SaveDataFileHeader {
    /// The precomputed checksum derived from the file data, excluding the checksum itself.
    pub checksum: Checksum,

    /// The version number of the save data.
    pub version: u32,

    /// The number of stored user file descriptors.
    pub user_file_info_num: u32,

    /// The size of the file, in bytes.
    pub file_size: u32,
}

impl SaveFileHeader for SaveDataFileHeader {
    const VERSION: u32 = 2;
    const USER_FILE_INFO_MAX: u32 = 23;
    const FILE_SIZE_MAX: u32 = 0xFFFF;

    fn checksum(&self) -> Checksum {
        self.checksum
    }

    fn version(&self) -> u32 {
        self.version
    }

    fn user_file_info_num(&self) -> u32 {
        self.user_file_info_num
    }

    fn file_size(&self) -> u32 {
        self.file_size
    }
}
