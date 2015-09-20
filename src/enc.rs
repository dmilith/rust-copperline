use encoding::all::*;
use encoding::types::EncodingRef;

pub enum Encoding {
    Ascii,
    Big52003,
    EucJp,
    Gb18030,
    Gbk,
    Hz,
    Ibm866,
    Iso2022Jp,
    Iso88591,
    Iso885910,
    Iso885913,
    Iso885914,
    Iso885915,
    Iso885916,
    Iso88592,
    Iso88593,
    Iso88594,
    Iso88595,
    Iso88596,
    Iso88597,
    Iso88598,
    Koi8R,
    Koi8U,
    MacCyrillic,
    MacRoman,
    Utf16BE,
    Utf16LE,
    Utf8,
    Windows1250,
    Windows1251,
    Windows1252,
    Windows1253,
    Windows1254,
    Windows1255,
    Windows1256,
    Windows1257,
    Windows1258,
    Windows31J,
    Windows874,
    Windows949
}

pub fn to_encoding_ref(enc: Encoding) -> EncodingRef {
    match enc {
        Encoding::Ascii => ASCII,
        Encoding::Big52003 => BIG5_2003,
        Encoding::EucJp => EUC_JP,
        Encoding::Gb18030 => GB18030,
        Encoding::Gbk => GBK,
        Encoding::Hz => HZ,
        Encoding::Ibm866 => IBM866,
        Encoding::Iso2022Jp => ISO_2022_JP,
        Encoding::Iso88591 => ISO_8859_1,
        Encoding::Iso885910 => ISO_8859_10,
        Encoding::Iso885913 => ISO_8859_13,
        Encoding::Iso885914 => ISO_8859_14,
        Encoding::Iso885915 => ISO_8859_15,
        Encoding::Iso885916 => ISO_8859_16,
        Encoding::Iso88592 => ISO_8859_2,
        Encoding::Iso88593 => ISO_8859_3,
        Encoding::Iso88594 => ISO_8859_4,
        Encoding::Iso88595 => ISO_8859_5,
        Encoding::Iso88596 => ISO_8859_6,
        Encoding::Iso88597 => ISO_8859_7,
        Encoding::Iso88598 => ISO_8859_8,
        Encoding::Koi8R => KOI8_R,
        Encoding::Koi8U => KOI8_U,
        Encoding::MacCyrillic => MAC_CYRILLIC,
        Encoding::MacRoman => MAC_ROMAN,
        Encoding::Utf16BE => UTF_16BE,
        Encoding::Utf16LE => UTF_16LE,
        Encoding::Utf8 => UTF_8,
        Encoding::Windows1250 => WINDOWS_1250,
        Encoding::Windows1251 => WINDOWS_1251,
        Encoding::Windows1252 => WINDOWS_1252,
        Encoding::Windows1253 => WINDOWS_1253,
        Encoding::Windows1254 => WINDOWS_1254,
        Encoding::Windows1255 => WINDOWS_1255,
        Encoding::Windows1256 => WINDOWS_1256,
        Encoding::Windows1257 => WINDOWS_1257,
        Encoding::Windows1258 => WINDOWS_1258,
        Encoding::Windows31J => WINDOWS_31J,
        Encoding::Windows874 => WINDOWS_874,
        Encoding::Windows949 => WINDOWS_949
    }
}
