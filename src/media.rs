use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum MediaType {
    Photo,
    Video,
}

impl MediaType {
    pub fn exif_condition(&self) -> &str {
        match self {
            Self::Photo => {
                //Missing either CreateDate or DatetimeOriginal
                "((not $Exif:CreateDate or not $Exif:DatetimeOriginal) and $MIMEType=~/image/)"
            }
            Self::Video => {
                //Missing CreateDate in either group
                "((not $QuickTime:CreateDate or not $Xmp:CreateDate) and $MIMEType=~/video/)"
            }
        }
    }
}
