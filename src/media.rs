use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum MediaType {
    Photo,
    Video,
    Gif,
}

impl MediaType {
    pub fn exif_condition(&self) -> &str {
        match self {
            Self::Photo => {
                //Missing either CreateDate or DatetimeOriginal
                "((not $Exif:CreateDate or not $Exif:DatetimeOriginal) and $MIMEType=~/image/ and not $FileType eq \"GIF\")"
            }
            Self::Video => {
                //Missing CreateDate in both groups
                "(not $QuickTime:CreateDate and not $Xmp:CreateDate and $MIMEType=~/video/)"
            }
            Self::Gif => {
                //Like a photo but with a different tag group
                "((not $Xmp:CreateDate or not $Xmp:DatetimeOriginal) and $MIMEType=~/image/ and $FileType eq \"GIF\")"
            }
        }
    }
}
