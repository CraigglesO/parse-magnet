extern crate serde;

use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Magnet {
    xt:             Vec<String>,   // xt (Exact Topic) – URN containing file hash
    xl:             i64,           // xl (Exact Length) – Size in bytes
    as:             String,        // as (Acceptable Source) – Web link to the file online
    xs:             Vec<String>,   // xs (eXact Source) – P2P link.
    dn:             String,        // dn (Display Name) – Filename
    kt:             Vec<String>,   // kt (Keyword Topic) – Key words for search
    mt:             string,        // mt (Manifest Topic) – link to the metafile that contains a list of magneto (MAGMA – MAGnet MAnifest)
    tr:             Vec<String>,   // tr (Address Tracker) – Tracker URL for BitTorrent downloads
    infoHash:       string,        // Same as xt
    infoHashBuffer: Buffer,
    name:           string,
    announce:       Vec<String>,
}

#[derive(Debug)]
pub enum LoadFileError {
    Io(io::Error),
}

#[derive(Debug)]
pub enum FromStringError {

}

impl Magnet {
    pub fn from_file(path: &str) -> Result<Magnet, LoadFileError> {
        let path = Path::new(path);
        let mut f = match fs::File::open(path) {
            Ok(f) => f,
            Err(e) => return Err(LoadFileError::Io(e)),
        };
        let mut string = String::new();
        match f.read_line(&mut string) {
            Ok(_) => {
                Self::from_string(string)
            },
            Err(e) => Err(LoadFileError::Io(e)),
        }
    }

    pub fn from_str(string: &str) -> Result<Magnet, ParseMagnetError> {
        match parse_magnet(string.to_string()) {
            Ok(p) => Ok(p),
            Err(e) => Err(ParseMagnetError(e)),
        }
    }

    pub fn parse_magnet(&mut self, magnet: string) -> Result<Magnet, ParseMagnetError> {

    }
}

// magnet:?xt=urn:btih:945f2e8866dbe16761f034757c5629ba9b6c66f0&dn=Smolensk.2016.DVDRip.x264-AFO&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Fzer0day.ch%3A1337&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Fpublic.popcorn-tracker.org%3A6969
