use std::io;

#[derive(Debug)]
pub struct Magnet {
    xt:               String,        // xt (Exact Topic) – URN containing file hash
    xl:               i64,           // xl (Exact Length) – Size in bytes
    ar:               String,        // as (Acceptable Source) – Web link to the file online
    xs:               Vec<String>,   // xs (eXact Source) – P2P link.
    dn:               String,        // dn (Display Name) – Filename
    kt:               Vec<String>,   // kt (Keyword Topic) – Key words for search
    mt:               String,        // mt (Manifest Topic) – link to the metafile that contains a list of magneto (MAGMA – MAGnet MAnifest)
    tr:               Vec<String>,   // tr (Address Tracker) – Tracker URL for BitTorrent downloads
}

#[derive(Debug)]
pub enum LoadFileError {
    Io(io::Error),
}

#[derive(Debug)]
pub enum ParseMagnetError {
    ToStringError,
}

impl Magnet {
    pub fn new() -> Magnet {
        Magnet {
            xt:               String::new(),
            xl:               0,
            ar:               String::new(),
            xs:               Vec::new(),
            dn:               String::new(),
            kt:               Vec::new(),
            mt:               String::new(),
            tr:               Vec::new(),
        }
    }

    pub fn from_str(string: &str) -> Result<Magnet, ParseMagnetError> {
        match Self::parse_magnet(string.to_string()) {
            Ok(p) => Ok(p),
            Err(_) => Err(ParseMagnetError::ToStringError),
        }
    }

    pub fn parse_magnet(magnet: String) -> Result<Magnet, ParseMagnetError> {
        let contents = magnet[8..].split("&");
        let mut magnet = Magnet::new();

        for x in contents {
            let t: Vec<&str> = x.split("=").collect();
            match t[0] {
                "xt" => {
                    if &t[1][0..9] == "urn:btih:" {
                        magnet.xt = t[1][9..].to_lowercase();
                    }
                },
                "xl" => { magnet.xl = t[1].parse().unwrap(); },
                "as" => { magnet.ar = uri_decode(t[1]) },
                "xs" => { magnet.xs.push(uri_decode(t[1])) },
                "dn" => { magnet.dn = uri_decode(t[1]); },
                "kt" => { magnet.kt = t[1].split("+").map(|s| s.to_string()).collect(); },
                "mt" => { magnet.mt = uri_decode(t[1]) },
                "tr" => { magnet.tr.push(uri_decode(t[1])) },
                _    => {},
            }
        }

        Ok(magnet)
    }
}

fn uri_decode (url: &str) -> String {
    let mut url = url.to_string();
    url = url.replace("+", " ");
    url = url.replace("%3A", ":");
    url = url.replace("%2F", "/");
    url = url.replace("%21", "!");
    url = url.replace("%23", "#");
    url = url.replace("%24", "$");
    url = url.replace("%26", "&");
    url = url.replace("%27", "'");
    url = url.replace("%28", "(");
    url = url.replace("%29", ")");
    url = url.replace("%2A", "*");
    url = url.replace("%2B", "+");
    url = url.replace("%2C", ",");
    url = url.replace("%3A", ":");
    url = url.replace("%3B", ";");
    url = url.replace("%3D", "=");
    url = url.replace("%3F", "?");
    url = url.replace("%40", "@");
    url = url.replace("%5B", "[");
    url = url.replace("%5D", "]");
    url.to_string()
}

// magnet:?xt=urn:btih:945f2e8866dbe16761f034757c5629ba9b6c66f0&dn=Smolensk.2016.DVDRip.x264-AFO&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Fzer0day.ch%3A1337&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Fpublic.popcorn-tracker.org%3A6969
