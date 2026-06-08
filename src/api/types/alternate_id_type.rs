pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AlternateIDType {
    AltIDTypeInvalid,
    AltIDTypeTrackID2,
    AltIDTypeTrackID1,
    AltIDTypeSpiID,
    AltIDTypeNitfFileTitle,
    AltIDTypeTrackRepoAlertID,
    AltIDTypeAssetID,
    AltIDTypeLink16TrackNumber,
    AltIDTypeLink16Ju,
    AltIDTypeNcctMessageID,
    AltIDTypeCallsign,
    AltIDTypeMmsiID,
    AltIDTypeVmfUrn,
    AltIDTypeImoID,
    AltIDTypeVmfTargetNumber,
    AltIDTypeSerialNumber,
    AltIDTypeRegistrationID,
    AltIDTypeIbsGid,
    AltIDTypeDodaac,
    AltIDTypeUic,
    AltIDTypeNoradCatID,
    AltIDTypeUnoosaName,
    AltIDTypeUnoosaID,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AlternateIDType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AltIDTypeInvalid => serializer.serialize_str("ALT_ID_TYPE_INVALID"),
            Self::AltIDTypeTrackID2 => serializer.serialize_str("ALT_ID_TYPE_TRACK_ID_2"),
            Self::AltIDTypeTrackID1 => serializer.serialize_str("ALT_ID_TYPE_TRACK_ID_1"),
            Self::AltIDTypeSpiID => serializer.serialize_str("ALT_ID_TYPE_SPI_ID"),
            Self::AltIDTypeNitfFileTitle => serializer.serialize_str("ALT_ID_TYPE_NITF_FILE_TITLE"),
            Self::AltIDTypeTrackRepoAlertID => {
                serializer.serialize_str("ALT_ID_TYPE_TRACK_REPO_ALERT_ID")
            }
            Self::AltIDTypeAssetID => serializer.serialize_str("ALT_ID_TYPE_ASSET_ID"),
            Self::AltIDTypeLink16TrackNumber => {
                serializer.serialize_str("ALT_ID_TYPE_LINK16_TRACK_NUMBER")
            }
            Self::AltIDTypeLink16Ju => serializer.serialize_str("ALT_ID_TYPE_LINK16_JU"),
            Self::AltIDTypeNcctMessageID => serializer.serialize_str("ALT_ID_TYPE_NCCT_MESSAGE_ID"),
            Self::AltIDTypeCallsign => serializer.serialize_str("ALT_ID_TYPE_CALLSIGN"),
            Self::AltIDTypeMmsiID => serializer.serialize_str("ALT_ID_TYPE_MMSI_ID"),
            Self::AltIDTypeVmfUrn => serializer.serialize_str("ALT_ID_TYPE_VMF_URN"),
            Self::AltIDTypeImoID => serializer.serialize_str("ALT_ID_TYPE_IMO_ID"),
            Self::AltIDTypeVmfTargetNumber => {
                serializer.serialize_str("ALT_ID_TYPE_VMF_TARGET_NUMBER")
            }
            Self::AltIDTypeSerialNumber => serializer.serialize_str("ALT_ID_TYPE_SERIAL_NUMBER"),
            Self::AltIDTypeRegistrationID => {
                serializer.serialize_str("ALT_ID_TYPE_REGISTRATION_ID")
            }
            Self::AltIDTypeIbsGid => serializer.serialize_str("ALT_ID_TYPE_IBS_GID"),
            Self::AltIDTypeDodaac => serializer.serialize_str("ALT_ID_TYPE_DODAAC"),
            Self::AltIDTypeUic => serializer.serialize_str("ALT_ID_TYPE_UIC"),
            Self::AltIDTypeNoradCatID => serializer.serialize_str("ALT_ID_TYPE_NORAD_CAT_ID"),
            Self::AltIDTypeUnoosaName => serializer.serialize_str("ALT_ID_TYPE_UNOOSA_NAME"),
            Self::AltIDTypeUnoosaID => serializer.serialize_str("ALT_ID_TYPE_UNOOSA_ID"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AlternateIDType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ALT_ID_TYPE_INVALID" => Ok(Self::AltIDTypeInvalid),
            "ALT_ID_TYPE_TRACK_ID_2" => Ok(Self::AltIDTypeTrackID2),
            "ALT_ID_TYPE_TRACK_ID_1" => Ok(Self::AltIDTypeTrackID1),
            "ALT_ID_TYPE_SPI_ID" => Ok(Self::AltIDTypeSpiID),
            "ALT_ID_TYPE_NITF_FILE_TITLE" => Ok(Self::AltIDTypeNitfFileTitle),
            "ALT_ID_TYPE_TRACK_REPO_ALERT_ID" => Ok(Self::AltIDTypeTrackRepoAlertID),
            "ALT_ID_TYPE_ASSET_ID" => Ok(Self::AltIDTypeAssetID),
            "ALT_ID_TYPE_LINK16_TRACK_NUMBER" => Ok(Self::AltIDTypeLink16TrackNumber),
            "ALT_ID_TYPE_LINK16_JU" => Ok(Self::AltIDTypeLink16Ju),
            "ALT_ID_TYPE_NCCT_MESSAGE_ID" => Ok(Self::AltIDTypeNcctMessageID),
            "ALT_ID_TYPE_CALLSIGN" => Ok(Self::AltIDTypeCallsign),
            "ALT_ID_TYPE_MMSI_ID" => Ok(Self::AltIDTypeMmsiID),
            "ALT_ID_TYPE_VMF_URN" => Ok(Self::AltIDTypeVmfUrn),
            "ALT_ID_TYPE_IMO_ID" => Ok(Self::AltIDTypeImoID),
            "ALT_ID_TYPE_VMF_TARGET_NUMBER" => Ok(Self::AltIDTypeVmfTargetNumber),
            "ALT_ID_TYPE_SERIAL_NUMBER" => Ok(Self::AltIDTypeSerialNumber),
            "ALT_ID_TYPE_REGISTRATION_ID" => Ok(Self::AltIDTypeRegistrationID),
            "ALT_ID_TYPE_IBS_GID" => Ok(Self::AltIDTypeIbsGid),
            "ALT_ID_TYPE_DODAAC" => Ok(Self::AltIDTypeDodaac),
            "ALT_ID_TYPE_UIC" => Ok(Self::AltIDTypeUic),
            "ALT_ID_TYPE_NORAD_CAT_ID" => Ok(Self::AltIDTypeNoradCatID),
            "ALT_ID_TYPE_UNOOSA_NAME" => Ok(Self::AltIDTypeUnoosaName),
            "ALT_ID_TYPE_UNOOSA_ID" => Ok(Self::AltIDTypeUnoosaID),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AlternateIDType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AltIDTypeInvalid => write!(f, "ALT_ID_TYPE_INVALID"),
            Self::AltIDTypeTrackID2 => write!(f, "ALT_ID_TYPE_TRACK_ID_2"),
            Self::AltIDTypeTrackID1 => write!(f, "ALT_ID_TYPE_TRACK_ID_1"),
            Self::AltIDTypeSpiID => write!(f, "ALT_ID_TYPE_SPI_ID"),
            Self::AltIDTypeNitfFileTitle => write!(f, "ALT_ID_TYPE_NITF_FILE_TITLE"),
            Self::AltIDTypeTrackRepoAlertID => write!(f, "ALT_ID_TYPE_TRACK_REPO_ALERT_ID"),
            Self::AltIDTypeAssetID => write!(f, "ALT_ID_TYPE_ASSET_ID"),
            Self::AltIDTypeLink16TrackNumber => write!(f, "ALT_ID_TYPE_LINK16_TRACK_NUMBER"),
            Self::AltIDTypeLink16Ju => write!(f, "ALT_ID_TYPE_LINK16_JU"),
            Self::AltIDTypeNcctMessageID => write!(f, "ALT_ID_TYPE_NCCT_MESSAGE_ID"),
            Self::AltIDTypeCallsign => write!(f, "ALT_ID_TYPE_CALLSIGN"),
            Self::AltIDTypeMmsiID => write!(f, "ALT_ID_TYPE_MMSI_ID"),
            Self::AltIDTypeVmfUrn => write!(f, "ALT_ID_TYPE_VMF_URN"),
            Self::AltIDTypeImoID => write!(f, "ALT_ID_TYPE_IMO_ID"),
            Self::AltIDTypeVmfTargetNumber => write!(f, "ALT_ID_TYPE_VMF_TARGET_NUMBER"),
            Self::AltIDTypeSerialNumber => write!(f, "ALT_ID_TYPE_SERIAL_NUMBER"),
            Self::AltIDTypeRegistrationID => write!(f, "ALT_ID_TYPE_REGISTRATION_ID"),
            Self::AltIDTypeIbsGid => write!(f, "ALT_ID_TYPE_IBS_GID"),
            Self::AltIDTypeDodaac => write!(f, "ALT_ID_TYPE_DODAAC"),
            Self::AltIDTypeUic => write!(f, "ALT_ID_TYPE_UIC"),
            Self::AltIDTypeNoradCatID => write!(f, "ALT_ID_TYPE_NORAD_CAT_ID"),
            Self::AltIDTypeUnoosaName => write!(f, "ALT_ID_TYPE_UNOOSA_NAME"),
            Self::AltIDTypeUnoosaID => write!(f, "ALT_ID_TYPE_UNOOSA_ID"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
