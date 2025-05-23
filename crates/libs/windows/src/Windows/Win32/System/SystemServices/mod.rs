pub const ACCESS_ALLOWED_ACE_TYPE: u32 = 0u32;
pub const ACCESS_ALLOWED_CALLBACK_ACE_TYPE: u32 = 9u32;
pub const ACCESS_ALLOWED_CALLBACK_OBJECT_ACE_TYPE: u32 = 11u32;
pub const ACCESS_ALLOWED_COMPOUND_ACE_TYPE: u32 = 4u32;
pub const ACCESS_ALLOWED_OBJECT_ACE_TYPE: u32 = 5u32;
pub const ACCESS_DENIED_ACE_TYPE: u32 = 1u32;
pub const ACCESS_DENIED_CALLBACK_ACE_TYPE: u32 = 10u32;
pub const ACCESS_DENIED_CALLBACK_OBJECT_ACE_TYPE: u32 = 12u32;
pub const ACCESS_DENIED_OBJECT_ACE_TYPE: u32 = 6u32;
pub const ACCESS_DS_OBJECT_TYPE_NAME_A: windows_core::PCSTR = windows_core::s!("Directory Service Object");
pub const ACCESS_DS_OBJECT_TYPE_NAME_W: windows_core::PCWSTR = windows_core::w!("Directory Service Object");
pub const ACCESS_DS_SOURCE_A: windows_core::PCSTR = windows_core::s!("DS");
pub const ACCESS_DS_SOURCE_W: windows_core::PCWSTR = windows_core::w!("DS");
pub const ACCESS_FILTER_SECURITY_INFORMATION: i32 = 256i32;
pub const ACCESS_MAX_LEVEL: u32 = 4u32;
pub const ACCESS_MAX_MS_ACE_TYPE: u32 = 8u32;
pub const ACCESS_MAX_MS_OBJECT_ACE_TYPE: u32 = 8u32;
pub const ACCESS_MAX_MS_V2_ACE_TYPE: u32 = 3u32;
pub const ACCESS_MAX_MS_V3_ACE_TYPE: u32 = 4u32;
pub const ACCESS_MAX_MS_V4_ACE_TYPE: u32 = 8u32;
pub const ACCESS_MAX_MS_V5_ACE_TYPE: u32 = 21u32;
pub const ACCESS_MIN_MS_ACE_TYPE: u32 = 0u32;
pub const ACCESS_MIN_MS_OBJECT_ACE_TYPE: u32 = 5u32;
pub const ACCESS_OBJECT_GUID: u32 = 0u32;
pub const ACCESS_PROPERTY_GUID: u32 = 2u32;
pub const ACCESS_PROPERTY_SET_GUID: u32 = 1u32;
pub const ACCESS_REASON_DATA_MASK: u32 = 65535u32;
pub const ACCESS_REASON_EXDATA_MASK: u32 = 2130706432u32;
pub const ACCESS_REASON_STAGING_MASK: u32 = 2147483648u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ACCESS_REASON_TYPE(pub i32);
pub const ACCESS_REASON_TYPE_MASK: u32 = 16711680u32;
pub const ACCESS_SYSTEM_SECURITY: u32 = 16777216u32;
pub const ACL_REVISION1: u32 = 1u32;
pub const ACL_REVISION2: u32 = 2u32;
pub const ACL_REVISION3: u32 = 3u32;
pub const ACL_REVISION4: u32 = 4u32;
pub const ACPI_PPM_HARDWARE_ALL: u32 = 254u32;
pub const ACPI_PPM_SOFTWARE_ALL: u32 = 252u32;
pub const ACPI_PPM_SOFTWARE_ANY: u32 = 253u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ACTIVATION_CONTEXT_INFO_CLASS(pub i32);
pub const ACTIVATION_CONTEXT_PATH_TYPE_ASSEMBLYREF: u32 = 4u32;
pub const ACTIVATION_CONTEXT_PATH_TYPE_NONE: u32 = 1u32;
pub const ACTIVATION_CONTEXT_PATH_TYPE_URL: u32 = 3u32;
pub const ACTIVATION_CONTEXT_PATH_TYPE_WIN32_FILE: u32 = 2u32;
pub const ACTIVATION_CONTEXT_SECTION_APPLICATION_SETTINGS: u32 = 10u32;
pub const ACTIVATION_CONTEXT_SECTION_ASSEMBLY_INFORMATION: u32 = 1u32;
pub const ACTIVATION_CONTEXT_SECTION_CLR_SURROGATES: u32 = 9u32;
pub const ACTIVATION_CONTEXT_SECTION_COMPATIBILITY_INFO: u32 = 11u32;
pub const ACTIVATION_CONTEXT_SECTION_COM_INTERFACE_REDIRECTION: u32 = 5u32;
pub const ACTIVATION_CONTEXT_SECTION_COM_PROGID_REDIRECTION: u32 = 7u32;
pub const ACTIVATION_CONTEXT_SECTION_COM_SERVER_REDIRECTION: u32 = 4u32;
pub const ACTIVATION_CONTEXT_SECTION_COM_TYPE_LIBRARY_REDIRECTION: u32 = 6u32;
pub const ACTIVATION_CONTEXT_SECTION_DLL_REDIRECTION: u32 = 2u32;
pub const ACTIVATION_CONTEXT_SECTION_GLOBAL_OBJECT_RENAME_TABLE: u32 = 8u32;
pub const ACTIVATION_CONTEXT_SECTION_WINDOW_CLASS_REDIRECTION: u32 = 3u32;
pub const ACTIVATION_CONTEXT_SECTION_WINRT_ACTIVATABLE_CLASSES: u32 = 12u32;
pub const ALERT_SYSTEM_CRITICAL: ALERT_SYSTEM_SEV = ALERT_SYSTEM_SEV(5u32);
pub const ALERT_SYSTEM_ERROR: ALERT_SYSTEM_SEV = ALERT_SYSTEM_SEV(3u32);
pub const ALERT_SYSTEM_INFORMATIONAL: ALERT_SYSTEM_SEV = ALERT_SYSTEM_SEV(1u32);
pub const ALERT_SYSTEM_QUERY: ALERT_SYSTEM_SEV = ALERT_SYSTEM_SEV(4u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ALERT_SYSTEM_SEV(pub u32);
pub const ALERT_SYSTEM_WARNING: ALERT_SYSTEM_SEV = ALERT_SYSTEM_SEV(2u32);
pub const ALL_POWERSCHEMES_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x68a1e95e_13ea_41e1_8011_0c496ca490b0);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ANON_OBJECT_HEADER {
    pub Sig1: u16,
    pub Sig2: u16,
    pub Version: u16,
    pub Machine: u16,
    pub TimeDateStamp: u32,
    pub ClassID: windows_core::GUID,
    pub SizeOfData: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ANON_OBJECT_HEADER_BIGOBJ {
    pub Sig1: u16,
    pub Sig2: u16,
    pub Version: u16,
    pub Machine: u16,
    pub TimeDateStamp: u32,
    pub ClassID: windows_core::GUID,
    pub SizeOfData: u32,
    pub Flags: u32,
    pub MetaDataSize: u32,
    pub MetaDataOffset: u32,
    pub NumberOfSections: u32,
    pub PointerToSymbolTable: u32,
    pub NumberOfSymbols: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ANON_OBJECT_HEADER_V2 {
    pub Sig1: u16,
    pub Sig2: u16,
    pub Version: u16,
    pub Machine: u16,
    pub TimeDateStamp: u32,
    pub ClassID: windows_core::GUID,
    pub SizeOfData: u32,
    pub Flags: u32,
    pub MetaDataSize: u32,
    pub MetaDataOffset: u32,
}
pub const ANYSIZE_ARRAY: u32 = 1u32;
pub const APPCOMMAND_BASS_BOOST: APPCOMMAND_ID = APPCOMMAND_ID(20u32);
pub const APPCOMMAND_BASS_DOWN: APPCOMMAND_ID = APPCOMMAND_ID(19u32);
pub const APPCOMMAND_BASS_UP: APPCOMMAND_ID = APPCOMMAND_ID(21u32);
pub const APPCOMMAND_BROWSER_BACKWARD: APPCOMMAND_ID = APPCOMMAND_ID(1u32);
pub const APPCOMMAND_BROWSER_FAVORITES: APPCOMMAND_ID = APPCOMMAND_ID(6u32);
pub const APPCOMMAND_BROWSER_FORWARD: APPCOMMAND_ID = APPCOMMAND_ID(2u32);
pub const APPCOMMAND_BROWSER_HOME: APPCOMMAND_ID = APPCOMMAND_ID(7u32);
pub const APPCOMMAND_BROWSER_REFRESH: APPCOMMAND_ID = APPCOMMAND_ID(3u32);
pub const APPCOMMAND_BROWSER_SEARCH: APPCOMMAND_ID = APPCOMMAND_ID(5u32);
pub const APPCOMMAND_BROWSER_STOP: APPCOMMAND_ID = APPCOMMAND_ID(4u32);
pub const APPCOMMAND_CLOSE: APPCOMMAND_ID = APPCOMMAND_ID(31u32);
pub const APPCOMMAND_COPY: APPCOMMAND_ID = APPCOMMAND_ID(36u32);
pub const APPCOMMAND_CORRECTION_LIST: APPCOMMAND_ID = APPCOMMAND_ID(45u32);
pub const APPCOMMAND_CUT: APPCOMMAND_ID = APPCOMMAND_ID(37u32);
pub const APPCOMMAND_DELETE: APPCOMMAND_ID = APPCOMMAND_ID(53u32);
pub const APPCOMMAND_DICTATE_OR_COMMAND_CONTROL_TOGGLE: APPCOMMAND_ID = APPCOMMAND_ID(43u32);
pub const APPCOMMAND_DWM_FLIP3D: APPCOMMAND_ID = APPCOMMAND_ID(54u32);
pub const APPCOMMAND_FIND: APPCOMMAND_ID = APPCOMMAND_ID(28u32);
pub const APPCOMMAND_FORWARD_MAIL: APPCOMMAND_ID = APPCOMMAND_ID(40u32);
pub const APPCOMMAND_HELP: APPCOMMAND_ID = APPCOMMAND_ID(27u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct APPCOMMAND_ID(pub u32);
pub const APPCOMMAND_LAUNCH_APP1: APPCOMMAND_ID = APPCOMMAND_ID(17u32);
pub const APPCOMMAND_LAUNCH_APP2: APPCOMMAND_ID = APPCOMMAND_ID(18u32);
pub const APPCOMMAND_LAUNCH_MAIL: APPCOMMAND_ID = APPCOMMAND_ID(15u32);
pub const APPCOMMAND_LAUNCH_MEDIA_SELECT: APPCOMMAND_ID = APPCOMMAND_ID(16u32);
pub const APPCOMMAND_MEDIA_CHANNEL_DOWN: APPCOMMAND_ID = APPCOMMAND_ID(52u32);
pub const APPCOMMAND_MEDIA_CHANNEL_UP: APPCOMMAND_ID = APPCOMMAND_ID(51u32);
pub const APPCOMMAND_MEDIA_FAST_FORWARD: APPCOMMAND_ID = APPCOMMAND_ID(49u32);
pub const APPCOMMAND_MEDIA_NEXTTRACK: APPCOMMAND_ID = APPCOMMAND_ID(11u32);
pub const APPCOMMAND_MEDIA_PAUSE: APPCOMMAND_ID = APPCOMMAND_ID(47u32);
pub const APPCOMMAND_MEDIA_PLAY: APPCOMMAND_ID = APPCOMMAND_ID(46u32);
pub const APPCOMMAND_MEDIA_PLAY_PAUSE: APPCOMMAND_ID = APPCOMMAND_ID(14u32);
pub const APPCOMMAND_MEDIA_PREVIOUSTRACK: APPCOMMAND_ID = APPCOMMAND_ID(12u32);
pub const APPCOMMAND_MEDIA_RECORD: APPCOMMAND_ID = APPCOMMAND_ID(48u32);
pub const APPCOMMAND_MEDIA_REWIND: APPCOMMAND_ID = APPCOMMAND_ID(50u32);
pub const APPCOMMAND_MEDIA_STOP: APPCOMMAND_ID = APPCOMMAND_ID(13u32);
pub const APPCOMMAND_MICROPHONE_VOLUME_DOWN: APPCOMMAND_ID = APPCOMMAND_ID(25u32);
pub const APPCOMMAND_MICROPHONE_VOLUME_MUTE: APPCOMMAND_ID = APPCOMMAND_ID(24u32);
pub const APPCOMMAND_MICROPHONE_VOLUME_UP: APPCOMMAND_ID = APPCOMMAND_ID(26u32);
pub const APPCOMMAND_MIC_ON_OFF_TOGGLE: APPCOMMAND_ID = APPCOMMAND_ID(44u32);
pub const APPCOMMAND_NEW: APPCOMMAND_ID = APPCOMMAND_ID(29u32);
pub const APPCOMMAND_OPEN: APPCOMMAND_ID = APPCOMMAND_ID(30u32);
pub const APPCOMMAND_PASTE: APPCOMMAND_ID = APPCOMMAND_ID(38u32);
pub const APPCOMMAND_PRINT: APPCOMMAND_ID = APPCOMMAND_ID(33u32);
pub const APPCOMMAND_REDO: APPCOMMAND_ID = APPCOMMAND_ID(35u32);
pub const APPCOMMAND_REPLY_TO_MAIL: APPCOMMAND_ID = APPCOMMAND_ID(39u32);
pub const APPCOMMAND_SAVE: APPCOMMAND_ID = APPCOMMAND_ID(32u32);
pub const APPCOMMAND_SEND_MAIL: APPCOMMAND_ID = APPCOMMAND_ID(41u32);
pub const APPCOMMAND_SPELL_CHECK: APPCOMMAND_ID = APPCOMMAND_ID(42u32);
pub const APPCOMMAND_TREBLE_DOWN: APPCOMMAND_ID = APPCOMMAND_ID(22u32);
pub const APPCOMMAND_TREBLE_UP: APPCOMMAND_ID = APPCOMMAND_ID(23u32);
pub const APPCOMMAND_UNDO: APPCOMMAND_ID = APPCOMMAND_ID(34u32);
pub const APPCOMMAND_VOLUME_DOWN: APPCOMMAND_ID = APPCOMMAND_ID(9u32);
pub const APPCOMMAND_VOLUME_MUTE: APPCOMMAND_ID = APPCOMMAND_ID(8u32);
pub const APPCOMMAND_VOLUME_UP: APPCOMMAND_ID = APPCOMMAND_ID(10u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct APPLICATIONLAUNCH_SETTING_VALUE {
    pub ActivationTime: i64,
    pub Flags: u32,
    pub ButtonInstanceID: u32,
}
pub const APPLICATION_ERROR_MASK: u32 = 536870912u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ARM64_FNPDATA_CR(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ARM64_FNPDATA_FLAGS(pub i32);
pub const ARM64_MAX_BREAKPOINTS: u32 = 8u32;
pub const ARM64_MAX_WATCHPOINTS: u32 = 2u32;
pub const ARM64_MULT_INTRINSICS_SUPPORTED: u32 = 1u32;
pub const ARM64_PREFETCH_KEEP: u32 = 0u32;
pub const ARM64_PREFETCH_L1: u32 = 0u32;
pub const ARM64_PREFETCH_L2: u32 = 2u32;
pub const ARM64_PREFETCH_L3: u32 = 4u32;
pub const ARM64_PREFETCH_PLD: u32 = 0u32;
pub const ARM64_PREFETCH_PLI: u32 = 8u32;
pub const ARM64_PREFETCH_PST: u32 = 16u32;
pub const ARM64_PREFETCH_STRM: u32 = 1u32;
pub const ARM_CACHE_ALIGNMENT_SIZE: u32 = 128u32;
pub const ARM_MAX_BREAKPOINTS: u32 = 8u32;
pub const ARM_MAX_WATCHPOINTS: u32 = 1u32;
pub const ASSERT_BREAKPOINT: u32 = 524291u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ATF_FLAGS(pub u32);
impl ATF_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ATF_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ATF_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ATF_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for ATF_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for ATF_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const ATF_ONOFFFEEDBACK: ATF_FLAGS = ATF_FLAGS(2u32);
pub const ATF_TIMEOUTON: ATF_FLAGS = ATF_FLAGS(1u32);
pub const AUDIT_ALLOW_NO_PRIVILEGE: u32 = 1u32;
pub const AccessReasonAllowedAce: ACCESS_REASON_TYPE = ACCESS_REASON_TYPE(65536i32);
pub const AccessReasonAllowedParentAce: ACCESS_REASON_TYPE = ACCESS_REASON_TYPE(196608i32);
pub const AccessReasonDeniedAce: ACCESS_REASON_TYPE = ACCESS_REASON_TYPE(131072i32);
pub const AccessReasonDeniedParentAce: ACCESS_REASON_TYPE = ACCESS_REASON_TYPE(262144i32);
pub const AccessReasonEmptyDacl: ACCESS_REASON_TYPE = ACCESS_REASON_TYPE(6291456i32);
pub const AccessReasonFilterAce: ACCESS_REASON_TYPE = ACCESS_REASON_TYPE(10485760i32);
pub const AccessReasonFromPrivilege: ACCESS_REASON_TYPE = ACCESS_REASON_TYPE(2097152i32);
pub const AccessReasonIntegrityLevel: ACCESS_REASON_TYPE = ACCESS_REASON_TYPE(3145728i32);
pub const AccessReasonMissingPrivilege: ACCESS_REASON_TYPE = ACCESS_REASON_TYPE(1048576i32);
pub const AccessReasonNoGrant: ACCESS_REASON_TYPE = ACCESS_REASON_TYPE(8388608i32);
pub const AccessReasonNoSD: ACCESS_REASON_TYPE = ACCESS_REASON_TYPE(7340032i32);
pub const AccessReasonNone: ACCESS_REASON_TYPE = ACCESS_REASON_TYPE(0i32);
pub const AccessReasonNotGrantedByCape: ACCESS_REASON_TYPE = ACCESS_REASON_TYPE(327680i32);
pub const AccessReasonNotGrantedByParentCape: ACCESS_REASON_TYPE = ACCESS_REASON_TYPE(393216i32);
pub const AccessReasonNotGrantedToAppContainer: ACCESS_REASON_TYPE = ACCESS_REASON_TYPE(458752i32);
pub const AccessReasonNullDacl: ACCESS_REASON_TYPE = ACCESS_REASON_TYPE(5242880i32);
pub const AccessReasonOwnership: ACCESS_REASON_TYPE = ACCESS_REASON_TYPE(4194304i32);
pub const AccessReasonTrustLabel: ACCESS_REASON_TYPE = ACCESS_REASON_TYPE(9437184i32);
pub const ActivationContextBasicInformation: ACTIVATION_CONTEXT_INFO_CLASS = ACTIVATION_CONTEXT_INFO_CLASS(1i32);
pub const ActivationContextDetailedInformation: ACTIVATION_CONTEXT_INFO_CLASS = ACTIVATION_CONTEXT_INFO_CLASS(2i32);
pub const ActivationContextManifestResourceName: ACTIVATION_CONTEXT_INFO_CLASS = ACTIVATION_CONTEXT_INFO_CLASS(7i32);
pub const AdapterType: SERVICE_NODE_TYPE = SERVICE_NODE_TYPE(4i32);
pub const AssemblyDetailedInformationInActivationContext: ACTIVATION_CONTEXT_INFO_CLASS = ACTIVATION_CONTEXT_INFO_CLASS(3i32);
pub const AssemblyDetailedInformationInActivationContxt: ACTIVATION_CONTEXT_INFO_CLASS = ACTIVATION_CONTEXT_INFO_CLASS(3i32);
pub const AutoLoad: SERVICE_LOAD_TYPE = SERVICE_LOAD_TYPE(2i32);
pub const BATTERY_DISCHARGE_FLAGS_ENABLE: u32 = 2147483648u32;
pub const BATTERY_DISCHARGE_FLAGS_EVENTCODE_MASK: u32 = 7u32;
pub const BREAK_DEBUG_BASE: u32 = 524288u32;
pub const BootLoad: SERVICE_LOAD_TYPE = SERVICE_LOAD_TYPE(0i32);
pub const CACHE_FULLY_ASSOCIATIVE: u32 = 255u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CFE_UNDERLINE(pub u32);
impl CFE_UNDERLINE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CFE_UNDERLINE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CFE_UNDERLINE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CFE_UNDERLINE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CFE_UNDERLINE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CFE_UNDERLINE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CFG_CALL_TARGET_CONVERT_EXPORT_SUPPRESSED_TO_VALID: u32 = 4u32;
pub const CFG_CALL_TARGET_CONVERT_XFG_TO_CFG: u32 = 16u32;
pub const CFG_CALL_TARGET_PROCESSED: u32 = 2u32;
pub const CFG_CALL_TARGET_VALID: u32 = 1u32;
pub const CFG_CALL_TARGET_VALID_XFG: u32 = 8u32;
pub const CFU_CF1UNDERLINE: CFE_UNDERLINE = CFE_UNDERLINE(255u32);
pub const CFU_INVERT: CFE_UNDERLINE = CFE_UNDERLINE(254u32);
pub const CFU_UNDERLINE: CFE_UNDERLINE = CFE_UNDERLINE(1u32);
pub const CFU_UNDERLINEDASH: CFE_UNDERLINE = CFE_UNDERLINE(5u32);
pub const CFU_UNDERLINEDASHDOT: CFE_UNDERLINE = CFE_UNDERLINE(6u32);
pub const CFU_UNDERLINEDASHDOTDOT: CFE_UNDERLINE = CFE_UNDERLINE(7u32);
pub const CFU_UNDERLINEDOTTED: CFE_UNDERLINE = CFE_UNDERLINE(4u32);
pub const CFU_UNDERLINEDOUBLE: CFE_UNDERLINE = CFE_UNDERLINE(3u32);
pub const CFU_UNDERLINEDOUBLEWAVE: CFE_UNDERLINE = CFE_UNDERLINE(11u32);
pub const CFU_UNDERLINEHAIRLINE: CFE_UNDERLINE = CFE_UNDERLINE(10u32);
pub const CFU_UNDERLINEHEAVYWAVE: CFE_UNDERLINE = CFE_UNDERLINE(12u32);
pub const CFU_UNDERLINELONGDASH: CFE_UNDERLINE = CFE_UNDERLINE(13u32);
pub const CFU_UNDERLINENONE: CFE_UNDERLINE = CFE_UNDERLINE(0u32);
pub const CFU_UNDERLINETHICK: CFE_UNDERLINE = CFE_UNDERLINE(9u32);
pub const CFU_UNDERLINETHICKDASH: CFE_UNDERLINE = CFE_UNDERLINE(14u32);
pub const CFU_UNDERLINETHICKDASHDOT: CFE_UNDERLINE = CFE_UNDERLINE(15u32);
pub const CFU_UNDERLINETHICKDASHDOTDOT: CFE_UNDERLINE = CFE_UNDERLINE(16u32);
pub const CFU_UNDERLINETHICKDOTTED: CFE_UNDERLINE = CFE_UNDERLINE(17u32);
pub const CFU_UNDERLINETHICKLONGDASH: CFE_UNDERLINE = CFE_UNDERLINE(18u32);
pub const CFU_UNDERLINEWAVE: CFE_UNDERLINE = CFE_UNDERLINE(8u32);
pub const CFU_UNDERLINEWORD: CFE_UNDERLINE = CFE_UNDERLINE(2u32);
pub const CLAIM_SECURITY_ATTRIBUTES_INFORMATION_VERSION: u32 = 1u32;
pub const CLAIM_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1: u32 = 1u32;
pub const CLAIM_SECURITY_ATTRIBUTE_CUSTOM_FLAGS: u32 = 4294901760u32;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_INVALID: u32 = 0u32;
pub const CM_SERVICE_MEASURED_BOOT_LOAD: u32 = 32u32;
pub const CM_SERVICE_NETWORK_BOOT_LOAD: u32 = 1u32;
pub const CM_SERVICE_RAM_DISK_BOOT_LOAD: u32 = 256u32;
pub const CM_SERVICE_SD_DISK_BOOT_LOAD: u32 = 8u32;
pub const CM_SERVICE_USB3_DISK_BOOT_LOAD: u32 = 16u32;
pub const CM_SERVICE_USB_DISK_BOOT_LOAD: u32 = 4u32;
pub const CM_SERVICE_VERIFIER_BOOT_LOAD: u32 = 64u32;
pub const CM_SERVICE_VIRTUAL_DISK_BOOT_LOAD: u32 = 2u32;
pub const CM_SERVICE_WINPE_BOOT_LOAD: u32 = 128u32;
pub const COMIMAGE_FLAGS_32BITPREFERRED: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(131072i32);
pub const COMIMAGE_FLAGS_32BITREQUIRED: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(2i32);
pub const COMIMAGE_FLAGS_ILONLY: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(1i32);
pub const COMIMAGE_FLAGS_IL_LIBRARY: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(4i32);
pub const COMIMAGE_FLAGS_NATIVE_ENTRYPOINT: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(16i32);
pub const COMIMAGE_FLAGS_STRONGNAMESIGNED: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(8i32);
pub const COMIMAGE_FLAGS_TRACKDEBUGDATA: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(65536i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COMPONENT_FILTER {
    pub ComponentFlags: u32,
}
pub const COMPONENT_KTM: u32 = 1u32;
pub const COMPONENT_VALID_FLAGS: u32 = 1u32;
pub const COMPRESSION_ENGINE_HIBER: u32 = 512u32;
pub const COMPRESSION_ENGINE_MAXIMUM: u32 = 256u32;
pub const COMPRESSION_ENGINE_STANDARD: u32 = 0u32;
pub const CORE_PARKING_POLICY_CHANGE_IDEAL: u32 = 0u32;
pub const CORE_PARKING_POLICY_CHANGE_MAX: u32 = 3u32;
pub const CORE_PARKING_POLICY_CHANGE_MULTISTEP: u32 = 3u32;
pub const CORE_PARKING_POLICY_CHANGE_ROCKET: u32 = 2u32;
pub const CORE_PARKING_POLICY_CHANGE_SINGLE: u32 = 1u32;
pub const COR_DELETED_NAME_LENGTH: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(8i32);
pub const COR_ILMETHOD_SECT_SMALL_MAX_DATASIZE: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(255i32);
pub const COR_VERSION_MAJOR: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(2i32);
pub const COR_VERSION_MAJOR_V2: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(2i32);
pub const COR_VERSION_MINOR: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(5i32);
pub const COR_VTABLEGAP_NAME_LENGTH: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(8i32);
pub const COR_VTABLE_32BIT: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(1i32);
pub const COR_VTABLE_64BIT: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(2i32);
pub const COR_VTABLE_CALL_MOST_DERIVED: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(16i32);
pub const COR_VTABLE_FROM_UNMANAGED: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(4i32);
pub const COR_VTABLE_FROM_UNMANAGED_RETAIN_APPDOMAIN: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(8i32);
pub const CREATE_BOUNDARY_DESCRIPTOR_ADD_APPCONTAINER_SID: u32 = 1u32;
pub const CRITICAL_ACE_FLAG: u32 = 32u32;
pub const CTMF_INCLUDE_APPCONTAINER: u32 = 1u32;
pub const CTMF_INCLUDE_LPAC: u32 = 2u32;
pub const CompatibilityInformationInActivationContext: ACTIVATION_CONTEXT_INFO_CLASS = ACTIVATION_CONTEXT_INFO_CLASS(6i32);
pub const CriticalError: SERVICE_ERROR_TYPE = SERVICE_ERROR_TYPE(3i32);
pub const DECIMAL_NEG: u8 = 128u8;
pub const DEDICATED_MEMORY_CACHE_ELIGIBLE: u32 = 1u32;
pub const DEVICEFAMILYDEVICEFORM_KEY: windows_core::PCWSTR = windows_core::w!("\\Registry\\Machine\\Software\\Microsoft\\Windows NT\\CurrentVersion\\OEM");
pub const DEVICEFAMILYDEVICEFORM_VALUE: windows_core::PCWSTR = windows_core::w!("DeviceForm");
pub const DIAGNOSTIC_REASON_DETAILED_STRING: u32 = 2u32;
pub const DIAGNOSTIC_REASON_NOT_SPECIFIED: u32 = 2147483648u32;
pub const DIAGNOSTIC_REASON_SIMPLE_STRING: u32 = 1u32;
pub const DIAGNOSTIC_REASON_VERSION: u32 = 0u32;
pub const DISCHARGE_POLICY_CRITICAL: u32 = 0u32;
pub const DISCHARGE_POLICY_LOW: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union DISPATCHER_CONTEXT_NONVOLREG_ARM64 {
    pub Buffer: [u8; 152],
    pub Anonymous: DISPATCHER_CONTEXT_NONVOLREG_ARM64_0,
}
impl Default for DISPATCHER_CONTEXT_NONVOLREG_ARM64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DISPATCHER_CONTEXT_NONVOLREG_ARM64_0 {
    pub GpNvRegs: [u64; 11],
    pub FpNvRegs: [f64; 8],
}
impl Default for DISPATCHER_CONTEXT_NONVOLREG_ARM64_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DLL_PROCESS_ATTACH: u32 = 1u32;
pub const DLL_PROCESS_DETACH: u32 = 0u32;
pub const DLL_THREAD_ATTACH: u32 = 2u32;
pub const DLL_THREAD_DETACH: u32 = 3u32;
pub const DOMAIN_ALIAS_RID_ACCESS_CONTROL_ASSISTANCE_OPS: i32 = 579i32;
pub const DOMAIN_ALIAS_RID_ACCOUNT_OPS: i32 = 548i32;
pub const DOMAIN_ALIAS_RID_ADMINS: i32 = 544i32;
pub const DOMAIN_ALIAS_RID_AUTHORIZATIONACCESS: i32 = 560i32;
pub const DOMAIN_ALIAS_RID_BACKUP_OPS: i32 = 551i32;
pub const DOMAIN_ALIAS_RID_CACHEABLE_PRINCIPALS_GROUP: i32 = 571i32;
pub const DOMAIN_ALIAS_RID_CERTSVC_DCOM_ACCESS_GROUP: i32 = 574i32;
pub const DOMAIN_ALIAS_RID_CRYPTO_OPERATORS: i32 = 569i32;
pub const DOMAIN_ALIAS_RID_DCOM_USERS: i32 = 562i32;
pub const DOMAIN_ALIAS_RID_DEFAULT_ACCOUNT: i32 = 581i32;
pub const DOMAIN_ALIAS_RID_DEVICE_OWNERS: i32 = 583i32;
pub const DOMAIN_ALIAS_RID_EVENT_LOG_READERS_GROUP: i32 = 573i32;
pub const DOMAIN_ALIAS_RID_GUESTS: i32 = 546i32;
pub const DOMAIN_ALIAS_RID_HYPER_V_ADMINS: i32 = 578i32;
pub const DOMAIN_ALIAS_RID_INCOMING_FOREST_TRUST_BUILDERS: i32 = 557i32;
pub const DOMAIN_ALIAS_RID_IUSERS: i32 = 568i32;
pub const DOMAIN_ALIAS_RID_LOGGING_USERS: i32 = 559i32;
pub const DOMAIN_ALIAS_RID_MONITORING_USERS: i32 = 558i32;
pub const DOMAIN_ALIAS_RID_NETWORK_CONFIGURATION_OPS: i32 = 556i32;
pub const DOMAIN_ALIAS_RID_NON_CACHEABLE_PRINCIPALS_GROUP: i32 = 572i32;
pub const DOMAIN_ALIAS_RID_POWER_USERS: i32 = 547i32;
pub const DOMAIN_ALIAS_RID_PREW2KCOMPACCESS: i32 = 554i32;
pub const DOMAIN_ALIAS_RID_PRINT_OPS: i32 = 550i32;
pub const DOMAIN_ALIAS_RID_RAS_SERVERS: i32 = 553i32;
pub const DOMAIN_ALIAS_RID_RDS_ENDPOINT_SERVERS: i32 = 576i32;
pub const DOMAIN_ALIAS_RID_RDS_MANAGEMENT_SERVERS: i32 = 577i32;
pub const DOMAIN_ALIAS_RID_RDS_REMOTE_ACCESS_SERVERS: i32 = 575i32;
pub const DOMAIN_ALIAS_RID_REMOTE_DESKTOP_USERS: i32 = 555i32;
pub const DOMAIN_ALIAS_RID_REMOTE_MANAGEMENT_USERS: i32 = 580i32;
pub const DOMAIN_ALIAS_RID_REPLICATOR: i32 = 552i32;
pub const DOMAIN_ALIAS_RID_STORAGE_REPLICA_ADMINS: i32 = 582i32;
pub const DOMAIN_ALIAS_RID_SYSTEM_OPS: i32 = 549i32;
pub const DOMAIN_ALIAS_RID_TS_LICENSE_SERVERS: i32 = 561i32;
pub const DOMAIN_ALIAS_RID_USERS: i32 = 545i32;
pub const DOMAIN_GROUP_RID_ADMINS: i32 = 512i32;
pub const DOMAIN_GROUP_RID_AUTHORIZATION_DATA_CONTAINS_CLAIMS: i32 = 497i32;
pub const DOMAIN_GROUP_RID_AUTHORIZATION_DATA_IS_COMPOUNDED: i32 = 496i32;
pub const DOMAIN_GROUP_RID_CDC_RESERVED: i32 = 524i32;
pub const DOMAIN_GROUP_RID_CERT_ADMINS: i32 = 517i32;
pub const DOMAIN_GROUP_RID_CLONEABLE_CONTROLLERS: i32 = 522i32;
pub const DOMAIN_GROUP_RID_COMPUTERS: i32 = 515i32;
pub const DOMAIN_GROUP_RID_CONTROLLERS: i32 = 516i32;
pub const DOMAIN_GROUP_RID_ENTERPRISE_ADMINS: i32 = 519i32;
pub const DOMAIN_GROUP_RID_ENTERPRISE_KEY_ADMINS: i32 = 527i32;
pub const DOMAIN_GROUP_RID_ENTERPRISE_READONLY_DOMAIN_CONTROLLERS: i32 = 498i32;
pub const DOMAIN_GROUP_RID_GUESTS: i32 = 514i32;
pub const DOMAIN_GROUP_RID_KEY_ADMINS: i32 = 526i32;
pub const DOMAIN_GROUP_RID_POLICY_ADMINS: i32 = 520i32;
pub const DOMAIN_GROUP_RID_PROTECTED_USERS: i32 = 525i32;
pub const DOMAIN_GROUP_RID_READONLY_CONTROLLERS: i32 = 521i32;
pub const DOMAIN_GROUP_RID_SCHEMA_ADMINS: i32 = 518i32;
pub const DOMAIN_GROUP_RID_USERS: i32 = 513i32;
pub const DOMAIN_USER_RID_ADMIN: i32 = 500i32;
pub const DOMAIN_USER_RID_DEFAULT_ACCOUNT: i32 = 503i32;
pub const DOMAIN_USER_RID_GUEST: i32 = 501i32;
pub const DOMAIN_USER_RID_KRBTGT: i32 = 502i32;
pub const DOMAIN_USER_RID_MAX: i32 = 999i32;
pub const DOMAIN_USER_RID_WDAG_ACCOUNT: i32 = 504i32;
pub const DYNAMIC_EH_CONTINUATION_TARGET_ADD: u32 = 1u32;
pub const DYNAMIC_EH_CONTINUATION_TARGET_PROCESSED: u32 = 2u32;
pub const DYNAMIC_ENFORCED_ADDRESS_RANGE_ADD: u32 = 1u32;
pub const DYNAMIC_ENFORCED_ADDRESS_RANGE_PROCESSED: u32 = 2u32;
pub const DemandLoad: SERVICE_LOAD_TYPE = SERVICE_LOAD_TYPE(3i32);
pub const DisableLoad: SERVICE_LOAD_TYPE = SERVICE_LOAD_TYPE(4i32);
pub const DriverType: SERVICE_NODE_TYPE = SERVICE_NODE_TYPE(1i32);
pub const EMARCH_ENC_I17_IC_INST_WORD_POS_X: u32 = 12u32;
pub const EMARCH_ENC_I17_IC_INST_WORD_X: u32 = 3u32;
pub const EMARCH_ENC_I17_IC_SIZE_X: u32 = 1u32;
pub const EMARCH_ENC_I17_IC_VAL_POS_X: u32 = 21u32;
pub const EMARCH_ENC_I17_IMM41a_INST_WORD_POS_X: u32 = 14u32;
pub const EMARCH_ENC_I17_IMM41a_INST_WORD_X: u32 = 1u32;
pub const EMARCH_ENC_I17_IMM41a_SIZE_X: u32 = 10u32;
pub const EMARCH_ENC_I17_IMM41a_VAL_POS_X: u32 = 22u32;
pub const EMARCH_ENC_I17_IMM41b_INST_WORD_POS_X: u32 = 24u32;
pub const EMARCH_ENC_I17_IMM41b_INST_WORD_X: u32 = 1u32;
pub const EMARCH_ENC_I17_IMM41b_SIZE_X: u32 = 8u32;
pub const EMARCH_ENC_I17_IMM41b_VAL_POS_X: u32 = 32u32;
pub const EMARCH_ENC_I17_IMM41c_INST_WORD_POS_X: u32 = 0u32;
pub const EMARCH_ENC_I17_IMM41c_INST_WORD_X: u32 = 2u32;
pub const EMARCH_ENC_I17_IMM41c_SIZE_X: u32 = 23u32;
pub const EMARCH_ENC_I17_IMM41c_VAL_POS_X: u32 = 40u32;
pub const EMARCH_ENC_I17_IMM5C_INST_WORD_POS_X: u32 = 13u32;
pub const EMARCH_ENC_I17_IMM5C_INST_WORD_X: u32 = 3u32;
pub const EMARCH_ENC_I17_IMM5C_SIZE_X: u32 = 5u32;
pub const EMARCH_ENC_I17_IMM5C_VAL_POS_X: u32 = 16u32;
pub const EMARCH_ENC_I17_IMM7B_INST_WORD_POS_X: u32 = 4u32;
pub const EMARCH_ENC_I17_IMM7B_INST_WORD_X: u32 = 3u32;
pub const EMARCH_ENC_I17_IMM7B_SIZE_X: u32 = 7u32;
pub const EMARCH_ENC_I17_IMM7B_VAL_POS_X: u32 = 0u32;
pub const EMARCH_ENC_I17_IMM9D_INST_WORD_POS_X: u32 = 18u32;
pub const EMARCH_ENC_I17_IMM9D_INST_WORD_X: u32 = 3u32;
pub const EMARCH_ENC_I17_IMM9D_SIZE_X: u32 = 9u32;
pub const EMARCH_ENC_I17_IMM9D_VAL_POS_X: u32 = 7u32;
pub const EMARCH_ENC_I17_SIGN_INST_WORD_POS_X: u32 = 27u32;
pub const EMARCH_ENC_I17_SIGN_INST_WORD_X: u32 = 3u32;
pub const EMARCH_ENC_I17_SIGN_SIZE_X: u32 = 1u32;
pub const EMARCH_ENC_I17_SIGN_VAL_POS_X: u32 = 63u32;
pub const ENCLAVE_LONG_ID_LENGTH: u32 = 32u32;
pub const ENCLAVE_SHORT_ID_LENGTH: u32 = 16u32;
pub const ENCLAVE_TYPE_SGX: u32 = 1u32;
pub const ENCLAVE_TYPE_SGX2: u32 = 2u32;
pub const ENCLAVE_TYPE_VBS: u32 = 16u32;
pub const ENCLAVE_TYPE_VBS_BASIC: u32 = 17u32;
pub const ENCLAVE_VBS_FLAG_DEBUG: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENLISTMENT_BASIC_INFORMATION {
    pub EnlistmentId: windows_core::GUID,
    pub TransactionId: windows_core::GUID,
    pub ResourceManagerId: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENLISTMENT_CRM_INFORMATION {
    pub CrmTransactionManagerId: windows_core::GUID,
    pub CrmResourceManagerId: windows_core::GUID,
    pub CrmEnlistmentId: windows_core::GUID,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ENLISTMENT_INFORMATION_CLASS(pub i32);
pub const ENLISTMENT_QUERY_INFORMATION: u32 = 1u32;
pub const ENLISTMENT_RECOVER: u32 = 4u32;
pub const ENLISTMENT_SET_INFORMATION: u32 = 2u32;
pub const ENLISTMENT_SUBORDINATE_RIGHTS: u32 = 8u32;
pub const ENLISTMENT_SUPERIOR_RIGHTS: u32 = 16u32;
pub const ERROR_SEVERITY_ERROR: u32 = 3221225472u32;
pub const ERROR_SEVERITY_INFORMATIONAL: u32 = 1073741824u32;
pub const ERROR_SEVERITY_SUCCESS: u32 = 0u32;
pub const ERROR_SEVERITY_WARNING: u32 = 2147483648u32;
pub const EVENTLOG_BACKWARDS_READ: u32 = 8u32;
pub const EVENTLOG_END_ALL_PAIRED_EVENTS: u32 = 4u32;
pub const EVENTLOG_END_PAIRED_EVENT: u32 = 2u32;
pub const EVENTLOG_FORWARDS_READ: u32 = 4u32;
pub const EVENTLOG_PAIRED_EVENT_ACTIVE: u32 = 8u32;
pub const EVENTLOG_PAIRED_EVENT_INACTIVE: u32 = 16u32;
pub const EVENTLOG_START_PAIRED_EVENT: u32 = 1u32;
pub const EXCEPTION_COLLIDED_UNWIND: u32 = 64u32;
pub const EXCEPTION_EXECUTE_FAULT: u32 = 8u32;
pub const EXCEPTION_EXIT_UNWIND: u32 = 4u32;
pub const EXCEPTION_MAXIMUM_PARAMETERS: u32 = 15u32;
pub const EXCEPTION_NESTED_CALL: u32 = 16u32;
pub const EXCEPTION_NONCONTINUABLE: u32 = 1u32;
pub const EXCEPTION_READ_FAULT: u32 = 0u32;
pub const EXCEPTION_SOFTWARE_ORIGINATE: u32 = 128u32;
pub const EXCEPTION_STACK_INVALID: u32 = 8u32;
pub const EXCEPTION_TARGET_UNWIND: u32 = 32u32;
pub const EXCEPTION_UNWINDING: u32 = 2u32;
pub const EXCEPTION_WRITE_FAULT: u32 = 1u32;
pub const EnlistmentBasicInformation: ENLISTMENT_INFORMATION_CLASS = ENLISTMENT_INFORMATION_CLASS(0i32);
pub const EnlistmentCrmInformation: ENLISTMENT_INFORMATION_CLASS = ENLISTMENT_INFORMATION_CLASS(2i32);
pub const EnlistmentRecoveryInformation: ENLISTMENT_INFORMATION_CLASS = ENLISTMENT_INFORMATION_CLASS(1i32);
pub const FAST_FAIL_ADMINLESS_ACCESS_DENIED: u32 = 55u32;
pub const FAST_FAIL_APCS_DISABLED: u32 = 32u32;
pub const FAST_FAIL_CAST_GUARD: u32 = 65u32;
pub const FAST_FAIL_CERTIFICATION_FAILURE: u32 = 20u32;
pub const FAST_FAIL_CONTROL_INVALID_RETURN_ADDRESS: u32 = 57u32;
pub const FAST_FAIL_CORRUPT_LIST_ENTRY: u32 = 3u32;
pub const FAST_FAIL_CRYPTO_LIBRARY: u32 = 22u32;
pub const FAST_FAIL_DEPRECATED_SERVICE_INVOKED: u32 = 27u32;
pub const FAST_FAIL_DLOAD_PROTECTION_FAILURE: u32 = 25u32;
pub const FAST_FAIL_ENCLAVE_CALL_FAILURE: u32 = 53u32;
pub const FAST_FAIL_ETW_CORRUPTION: u32 = 61u32;
pub const FAST_FAIL_FATAL_APP_EXIT: u32 = 7u32;
pub const FAST_FAIL_FLAGS_CORRUPTION: u32 = 59u32;
pub const FAST_FAIL_GS_COOKIE_INIT: u32 = 6u32;
pub const FAST_FAIL_GUARD_EXPORT_SUPPRESSION_FAILURE: u32 = 46u32;
pub const FAST_FAIL_GUARD_ICALL_CHECK_FAILURE: u32 = 10u32;
pub const FAST_FAIL_GUARD_ICALL_CHECK_FAILURE_XFG: u32 = 64u32;
pub const FAST_FAIL_GUARD_ICALL_CHECK_SUPPRESSED: u32 = 31u32;
pub const FAST_FAIL_GUARD_JUMPTABLE: u32 = 37u32;
pub const FAST_FAIL_GUARD_SS_FAILURE: u32 = 44u32;
pub const FAST_FAIL_GUARD_WRITE_CHECK_FAILURE: u32 = 11u32;
pub const FAST_FAIL_HEAP_METADATA_CORRUPTION: u32 = 50u32;
pub const FAST_FAIL_HOST_VISIBILITY_CHANGE: u32 = 66u32;
pub const FAST_FAIL_INCORRECT_STACK: u32 = 4u32;
pub const FAST_FAIL_INVALID_ARG: u32 = 5u32;
pub const FAST_FAIL_INVALID_BALANCED_TREE: u32 = 29u32;
pub const FAST_FAIL_INVALID_BUFFER_ACCESS: u32 = 28u32;
pub const FAST_FAIL_INVALID_CALL_IN_DLL_CALLOUT: u32 = 23u32;
pub const FAST_FAIL_INVALID_CONTROL_STACK: u32 = 47u32;
pub const FAST_FAIL_INVALID_DISPATCH_CONTEXT: u32 = 39u32;
pub const FAST_FAIL_INVALID_EXCEPTION_CHAIN: u32 = 21u32;
pub const FAST_FAIL_INVALID_FAST_FAIL_CODE: u32 = 4294967295u32;
pub const FAST_FAIL_INVALID_FIBER_SWITCH: u32 = 12u32;
pub const FAST_FAIL_INVALID_FILE_OPERATION: u32 = 42u32;
pub const FAST_FAIL_INVALID_FLS_DATA: u32 = 70u32;
pub const FAST_FAIL_INVALID_IAT: u32 = 49u32;
pub const FAST_FAIL_INVALID_IDLE_STATE: u32 = 33u32;
pub const FAST_FAIL_INVALID_IMAGE_BASE: u32 = 24u32;
pub const FAST_FAIL_INVALID_JUMP_BUFFER: u32 = 18u32;
pub const FAST_FAIL_INVALID_LOCK_STATE: u32 = 36u32;
pub const FAST_FAIL_INVALID_LONGJUMP_TARGET: u32 = 38u32;
pub const FAST_FAIL_INVALID_NEXT_THREAD: u32 = 30u32;
pub const FAST_FAIL_INVALID_PFN: u32 = 63u32;
pub const FAST_FAIL_INVALID_REFERENCE_COUNT: u32 = 14u32;
pub const FAST_FAIL_INVALID_SET_OF_CONTEXT: u32 = 13u32;
pub const FAST_FAIL_INVALID_SYSCALL_NUMBER: u32 = 41u32;
pub const FAST_FAIL_INVALID_THREAD: u32 = 40u32;
pub const FAST_FAIL_KERNEL_CET_SHADOW_STACK_ASSIST: u32 = 67u32;
pub const FAST_FAIL_LEGACY_GS_VIOLATION: u32 = 0u32;
pub const FAST_FAIL_LOADER_CONTINUITY_FAILURE: u32 = 45u32;
pub const FAST_FAIL_LOW_LABEL_ACCESS_DENIED: u32 = 52u32;
pub const FAST_FAIL_LPAC_ACCESS_DENIED: u32 = 43u32;
pub const FAST_FAIL_MRDATA_MODIFIED: u32 = 19u32;
pub const FAST_FAIL_MRDATA_PROTECTION_FAILURE: u32 = 34u32;
pub const FAST_FAIL_NTDLL_PATCH_FAILED: u32 = 69u32;
pub const FAST_FAIL_PATCH_CALLBACK_FAILED: u32 = 68u32;
pub const FAST_FAIL_PAYLOAD_RESTRICTION_VIOLATION: u32 = 51u32;
pub const FAST_FAIL_RANGE_CHECK_FAILURE: u32 = 8u32;
pub const FAST_FAIL_RIO_ABORT: u32 = 62u32;
pub const FAST_FAIL_SET_CONTEXT_DENIED: u32 = 48u32;
pub const FAST_FAIL_STACK_COOKIE_CHECK_FAILURE: u32 = 2u32;
pub const FAST_FAIL_UNEXPECTED_CALL: u32 = 56u32;
pub const FAST_FAIL_UNEXPECTED_HEAP_EXCEPTION: u32 = 35u32;
pub const FAST_FAIL_UNEXPECTED_HOST_BEHAVIOR: u32 = 58u32;
pub const FAST_FAIL_UNHANDLED_LSS_EXCEPTON: u32 = 54u32;
pub const FAST_FAIL_UNSAFE_EXTENSION_CALL: u32 = 26u32;
pub const FAST_FAIL_UNSAFE_REGISTRY_ACCESS: u32 = 9u32;
pub const FAST_FAIL_VEH_CORRUPTION: u32 = 60u32;
pub const FAST_FAIL_VTGUARD_CHECK_FAILURE: u32 = 1u32;
pub const FILE_ATTRIBUTE_STRICTLY_SEQUENTIAL: u32 = 536870912u32;
pub const FILE_CASE_PRESERVED_NAMES: u32 = 2u32;
pub const FILE_CASE_SENSITIVE_SEARCH: u32 = 1u32;
pub const FILE_CS_FLAG_CASE_SENSITIVE_DIR: u32 = 1u32;
pub const FILE_DAX_VOLUME: u32 = 536870912u32;
pub const FILE_FILE_COMPRESSION: u32 = 16u32;
pub const FILE_NAMED_STREAMS: u32 = 262144u32;
pub const FILE_NAME_FLAGS_UNSPECIFIED: u32 = 128u32;
pub const FILE_NAME_FLAG_BOTH: u32 = 3u32;
pub const FILE_NAME_FLAG_DOS: u32 = 2u32;
pub const FILE_NAME_FLAG_HARDLINK: u32 = 0u32;
pub const FILE_NAME_FLAG_NTFS: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_NOTIFY_FULL_INFORMATION {
    pub NextEntryOffset: u32,
    pub Action: u32,
    pub CreationTime: i64,
    pub LastModificationTime: i64,
    pub LastChangeTime: i64,
    pub LastAccessTime: i64,
    pub AllocatedLength: i64,
    pub FileSize: i64,
    pub FileAttributes: u32,
    pub Anonymous: FILE_NOTIFY_FULL_INFORMATION_0,
    pub FileId: i64,
    pub ParentFileId: i64,
    pub FileNameLength: u16,
    pub FileNameFlags: u8,
    pub Reserved: u8,
    pub FileName: [u16; 1],
}
impl Default for FILE_NOTIFY_FULL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FILE_NOTIFY_FULL_INFORMATION_0 {
    pub ReparsePointTag: u32,
    pub EaSize: u32,
}
impl Default for FILE_NOTIFY_FULL_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_PERSISTENT_ACLS: u32 = 8u32;
pub const FILE_READ_ONLY_VOLUME: u32 = 524288u32;
pub const FILE_RETURNS_CLEANUP_RESULT_INFO: u32 = 512u32;
pub const FILE_SEQUENTIAL_WRITE_ONCE: u32 = 1048576u32;
pub const FILE_SUPPORTS_BLOCK_REFCOUNTING: u32 = 134217728u32;
pub const FILE_SUPPORTS_BYPASS_IO: u32 = 2048u32;
pub const FILE_SUPPORTS_CASE_SENSITIVE_DIRS: u32 = 8192u32;
pub const FILE_SUPPORTS_ENCRYPTION: u32 = 131072u32;
pub const FILE_SUPPORTS_EXTENDED_ATTRIBUTES: u32 = 8388608u32;
pub const FILE_SUPPORTS_GHOSTING: u32 = 1073741824u32;
pub const FILE_SUPPORTS_HARD_LINKS: u32 = 4194304u32;
pub const FILE_SUPPORTS_INTEGRITY_STREAMS: u32 = 67108864u32;
pub const FILE_SUPPORTS_OBJECT_IDS: u32 = 65536u32;
pub const FILE_SUPPORTS_OPEN_BY_FILE_ID: u32 = 16777216u32;
pub const FILE_SUPPORTS_POSIX_UNLINK_RENAME: u32 = 1024u32;
pub const FILE_SUPPORTS_REMOTE_STORAGE: u32 = 256u32;
pub const FILE_SUPPORTS_REPARSE_POINTS: u32 = 128u32;
pub const FILE_SUPPORTS_SPARSE_FILES: u32 = 64u32;
pub const FILE_SUPPORTS_SPARSE_VDL: u32 = 268435456u32;
pub const FILE_SUPPORTS_STREAM_SNAPSHOTS: u32 = 4096u32;
pub const FILE_SUPPORTS_TRANSACTIONS: u32 = 2097152u32;
pub const FILE_SUPPORTS_USN_JOURNAL: u32 = 33554432u32;
pub const FILE_UNICODE_ON_DISK: u32 = 4u32;
pub const FILE_VOLUME_IS_COMPRESSED: u32 = 32768u32;
pub const FILE_VOLUME_QUOTAS: u32 = 32u32;
pub const FILL_NV_MEMORY_FLAG_FLUSH: u32 = 1u32;
pub const FILL_NV_MEMORY_FLAG_NON_TEMPORAL: u32 = 2u32;
pub const FILL_NV_MEMORY_FLAG_NO_DRAIN: u32 = 256u32;
pub const FLS_MAXIMUM_AVAILABLE: u32 = 4080u32;
pub const FLUSH_FLAGS_FILE_DATA_ONLY: u32 = 1u32;
pub const FLUSH_FLAGS_FILE_DATA_SYNC_ONLY: u32 = 4u32;
pub const FLUSH_FLAGS_NO_SYNC: u32 = 2u32;
pub const FLUSH_NV_MEMORY_IN_FLAG_NO_DRAIN: u32 = 1u32;
pub const FOREST_USER_RID_MAX: i32 = 499i32;
pub const FRAME_FPO: u32 = 0u32;
pub const FRAME_NONFPO: u32 = 3u32;
pub const FRAME_TRAP: u32 = 1u32;
pub const FRAME_TSS: u32 = 2u32;
pub const FileInformationInAssemblyOfAssemblyInActivationContext: ACTIVATION_CONTEXT_INFO_CLASS = ACTIVATION_CONTEXT_INFO_CLASS(4i32);
pub const FileInformationInAssemblyOfAssemblyInActivationContxt: ACTIVATION_CONTEXT_INFO_CLASS = ACTIVATION_CONTEXT_INFO_CLASS(4i32);
pub const FileSystemType: SERVICE_NODE_TYPE = SERVICE_NODE_TYPE(2i32);
pub const GC_ALLGESTURES: GESTURECONFIG_FLAGS = GESTURECONFIG_FLAGS(1u32);
pub const GC_PAN: GESTURECONFIG_FLAGS = GESTURECONFIG_FLAGS(1u32);
pub const GC_PAN_WITH_GUTTER: GESTURECONFIG_FLAGS = GESTURECONFIG_FLAGS(8u32);
pub const GC_PAN_WITH_INERTIA: GESTURECONFIG_FLAGS = GESTURECONFIG_FLAGS(16u32);
pub const GC_PAN_WITH_SINGLE_FINGER_HORIZONTALLY: GESTURECONFIG_FLAGS = GESTURECONFIG_FLAGS(4u32);
pub const GC_PAN_WITH_SINGLE_FINGER_VERTICALLY: GESTURECONFIG_FLAGS = GESTURECONFIG_FLAGS(2u32);
pub const GC_PRESSANDTAP: GESTURECONFIG_FLAGS = GESTURECONFIG_FLAGS(1u32);
pub const GC_ROLLOVER: GESTURECONFIG_FLAGS = GESTURECONFIG_FLAGS(1u32);
pub const GC_ROTATE: GESTURECONFIG_FLAGS = GESTURECONFIG_FLAGS(1u32);
pub const GC_TWOFINGERTAP: GESTURECONFIG_FLAGS = GESTURECONFIG_FLAGS(1u32);
pub const GC_ZOOM: GESTURECONFIG_FLAGS = GESTURECONFIG_FLAGS(1u32);
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct GDI_NONREMOTE {
    pub fContext: i32,
    pub u: GDI_NONREMOTE_0,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for GDI_NONREMOTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub union GDI_NONREMOTE_0 {
    pub hInproc: i32,
    pub hRemote: *mut super::Com::DWORD_BLOB,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for GDI_NONREMOTE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GESTURECONFIG_FLAGS(pub u32);
impl GESTURECONFIG_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GESTURECONFIG_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GESTURECONFIG_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GESTURECONFIG_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GESTURECONFIG_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GESTURECONFIG_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const GUID_ACDC_POWER_SOURCE: windows_core::GUID = windows_core::GUID::from_u128(0x5d3e9a59_e9d5_4b00_a6bd_ff34ff516548);
pub const GUID_ACTIVE_POWERSCHEME: windows_core::GUID = windows_core::GUID::from_u128(0x31f9f286_5084_42fe_b720_2b0264993763);
pub const GUID_ADAPTIVE_INPUT_CONTROLLER_STATE: windows_core::GUID = windows_core::GUID::from_u128(0x0e98fae9_f45a_4de1_a757_6031f197f6ea);
pub const GUID_ADAPTIVE_POWER_BEHAVIOR_SUBGROUP: windows_core::GUID = windows_core::GUID::from_u128(0x8619b916_e004_4dd8_9b66_dae86f806698);
pub const GUID_ADVANCED_COLOR_QUALITY_BIAS: windows_core::GUID = windows_core::GUID::from_u128(0x684c3e69_a4f7_4014_8754_d45179a56167);
pub const GUID_ALLOW_AWAYMODE: windows_core::GUID = windows_core::GUID::from_u128(0x25dfa149_5dd1_4736_b5ab_e8a37b5b8187);
pub const GUID_ALLOW_DISPLAY_REQUIRED: windows_core::GUID = windows_core::GUID::from_u128(0xa9ceb8da_cd46_44fb_a98b_02af69de4623);
pub const GUID_ALLOW_RTC_WAKE: windows_core::GUID = windows_core::GUID::from_u128(0xbd3b718a_0680_4d9d_8ab2_e1d2b4ac806d);
pub const GUID_ALLOW_STANDBY_STATES: windows_core::GUID = windows_core::GUID::from_u128(0xabfc2519_3608_4c2a_94ea_171b0ed546ab);
pub const GUID_ALLOW_SYSTEM_REQUIRED: windows_core::GUID = windows_core::GUID::from_u128(0xa4b195f5_8225_47d8_8012_9d41369786e2);
pub const GUID_APPLAUNCH_BUTTON: windows_core::GUID = windows_core::GUID::from_u128(0x1a689231_7399_4e9a_8f99_b71f999db3fa);
pub const GUID_BACKGROUND_TASK_NOTIFICATION: windows_core::GUID = windows_core::GUID::from_u128(0xcf23f240_2a54_48d8_b114_de1518ff052e);
pub const GUID_BATTERY_COUNT: windows_core::GUID = windows_core::GUID::from_u128(0x7d263f15_fca4_49e5_854b_a9f2bfbd5c24);
pub const GUID_BATTERY_DISCHARGE_ACTION_0: windows_core::GUID = windows_core::GUID::from_u128(0x637ea02f_bbcb_4015_8e2c_a1c7b9c0b546);
pub const GUID_BATTERY_DISCHARGE_ACTION_1: windows_core::GUID = windows_core::GUID::from_u128(0xd8742dcb_3e6a_4b3c_b3fe_374623cdcf06);
pub const GUID_BATTERY_DISCHARGE_ACTION_2: windows_core::GUID = windows_core::GUID::from_u128(0x421cba38_1a8e_4881_ac89_e33a8b04ece4);
pub const GUID_BATTERY_DISCHARGE_ACTION_3: windows_core::GUID = windows_core::GUID::from_u128(0x80472613_9780_455e_b308_72d3003cf2f8);
pub const GUID_BATTERY_DISCHARGE_FLAGS_0: windows_core::GUID = windows_core::GUID::from_u128(0x5dbb7c9f_38e9_40d2_9749_4f8a0e9f640f);
pub const GUID_BATTERY_DISCHARGE_FLAGS_1: windows_core::GUID = windows_core::GUID::from_u128(0xbcded951_187b_4d05_bccc_f7e51960c258);
pub const GUID_BATTERY_DISCHARGE_FLAGS_2: windows_core::GUID = windows_core::GUID::from_u128(0x7fd2f0c4_feb7_4da3_8117_e3fbedc46582);
pub const GUID_BATTERY_DISCHARGE_FLAGS_3: windows_core::GUID = windows_core::GUID::from_u128(0x73613ccf_dbfa_4279_8356_4935f6bf62f3);
pub const GUID_BATTERY_DISCHARGE_LEVEL_0: windows_core::GUID = windows_core::GUID::from_u128(0x9a66d8d7_4ff7_4ef9_b5a2_5a326ca2a469);
pub const GUID_BATTERY_DISCHARGE_LEVEL_1: windows_core::GUID = windows_core::GUID::from_u128(0x8183ba9a_e910_48da_8769_14ae6dc1170a);
pub const GUID_BATTERY_DISCHARGE_LEVEL_2: windows_core::GUID = windows_core::GUID::from_u128(0x07a07ca2_adaf_40d7_b077_533aaded1bfa);
pub const GUID_BATTERY_DISCHARGE_LEVEL_3: windows_core::GUID = windows_core::GUID::from_u128(0x58afd5a6_c2dd_47d2_9fbf_ef70cc5c5965);
pub const GUID_BATTERY_PERCENTAGE_REMAINING: windows_core::GUID = windows_core::GUID::from_u128(0xa7ad8041_b45a_4cae_87a3_eecbb468a9e1);
pub const GUID_BATTERY_SUBGROUP: windows_core::GUID = windows_core::GUID::from_u128(0xe73a048d_bf27_4f12_9731_8b2076e8891f);
pub const GUID_CONNECTIVITY_IN_STANDBY: windows_core::GUID = windows_core::GUID::from_u128(0xf15576e8_98b7_4186_b944_eafa664402d9);
pub const GUID_CONSOLE_DISPLAY_STATE: windows_core::GUID = windows_core::GUID::from_u128(0x6fe69556_704a_47a0_8f24_c28d936fda47);
pub const GUID_CRITICAL_POWER_TRANSITION: windows_core::GUID = windows_core::GUID::from_u128(0xb7a27025_e569_46c2_a504_2b96cad225a1);
pub const GUID_DEEP_SLEEP_ENABLED: windows_core::GUID = windows_core::GUID::from_u128(0xd502f7ee_1dc7_4efd_a55d_f04b6f5c0545);
pub const GUID_DEEP_SLEEP_PLATFORM_STATE: windows_core::GUID = windows_core::GUID::from_u128(0xd23f2fb8_9536_4038_9c94_1ce02e5c2152);
pub const GUID_DEVICE_IDLE_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x4faab71a_92e5_4726_b531_224559672d19);
pub const GUID_DEVICE_POWER_POLICY_VIDEO_BRIGHTNESS: windows_core::GUID = windows_core::GUID::from_u128(0xaded5e82_b909_4619_9949_f5d71dac0bcb);
pub const GUID_DEVICE_POWER_POLICY_VIDEO_DIM_BRIGHTNESS: windows_core::GUID = windows_core::GUID::from_u128(0xf1fbfde2_a960_4165_9f88_50667911ce96);
pub const GUID_DISCONNECTED_STANDBY_MODE: windows_core::GUID = windows_core::GUID::from_u128(0x68afb2d9_ee95_47a8_8f50_4115088073b1);
pub const GUID_DISK_ADAPTIVE_POWERDOWN: windows_core::GUID = windows_core::GUID::from_u128(0x396a32e1_499a_40b2_9124_a96afe707667);
pub const GUID_DISK_BURST_IGNORE_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x80e3c60e_bb94_4ad8_bbe0_0d3195efc663);
pub const GUID_DISK_COALESCING_POWERDOWN_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0xc36f0eb4_2988_4a70_8eee_0884fc2c2433);
pub const GUID_DISK_IDLE_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0x58e39ba8_b8e6_4ef6_90d0_89ae32b258d6);
pub const GUID_DISK_MAX_POWER: windows_core::GUID = windows_core::GUID::from_u128(0x51dea550_bb38_4bc4_991b_eacf37be5ec8);
pub const GUID_DISK_NVME_NOPPME: windows_core::GUID = windows_core::GUID::from_u128(0xfc7372b6_ab2d_43ee_8797_15e9841f2cca);
pub const GUID_DISK_POWERDOWN_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0x6738e2c4_e8a5_4a42_b16a_e040e769756e);
pub const GUID_DISK_SUBGROUP: windows_core::GUID = windows_core::GUID::from_u128(0x0012ee47_9041_4b5d_9b77_535fba8b1442);
pub const GUID_ENABLE_SWITCH_FORCED_SHUTDOWN: windows_core::GUID = windows_core::GUID::from_u128(0x833a6b62_dfa4_46d1_82f8_e09e34d029d6);
pub const GUID_ENERGY_SAVER_BATTERY_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0xe69653ca_cf7f_4f05_aa73_cb833fa90ad4);
pub const GUID_ENERGY_SAVER_BRIGHTNESS: windows_core::GUID = windows_core::GUID::from_u128(0x13d09884_f74e_474a_a852_b6bde8ad03a8);
pub const GUID_ENERGY_SAVER_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x5c5bb349_ad29_4ee2_9d0b_2b25270f7a81);
pub const GUID_ENERGY_SAVER_SUBGROUP: windows_core::GUID = windows_core::GUID::from_u128(0xde830923_a562_41af_a086_e3a2c6bad2da);
pub const GUID_EXECUTION_REQUIRED_REQUEST_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0x3166bc41_7e98_4e03_b34e_ec0f5f2b218e);
pub const GUID_GLOBAL_USER_PRESENCE: windows_core::GUID = windows_core::GUID::from_u128(0x786e8a1d_b427_4344_9207_09e70bdcbea9);
pub const GUID_GPU_PREFERENCE_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0xdd848b2a_8a5d_4451_9ae2_39cd41658f6c);
pub const GUID_GRAPHICS_SUBGROUP: windows_core::GUID = windows_core::GUID::from_u128(0x5fb4938d_1ee8_4b0f_9a3c_5036b0ab995c);
pub const GUID_HIBERNATE_FASTS4_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x94ac6d29_73ce_41a6_809f_6363ba21b47e);
pub const GUID_HIBERNATE_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0x9d7815a6_7ee4_497e_8888_515a05f02364);
pub const GUID_HUPR_ADAPTIVE_AWAY_DIM_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0xa79c8e0e_f271_482d_8f8a_5db9a18312de);
pub const GUID_HUPR_ADAPTIVE_AWAY_DISPLAY_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0x0a7d6ab6_ac83_4ad1_8282_eca5b58308f3);
pub const GUID_HUPR_ADAPTIVE_INATTENTIVE_DIM_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0xcf8c6097_12b8_4279_bbdd_44601ee5209d);
pub const GUID_HUPR_ADAPTIVE_INATTENTIVE_DISPLAY_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0xee16691e_6ab3_4619_bb48_1c77c9357e5a);
pub const GUID_IDLE_BACKGROUND_TASK: windows_core::GUID = windows_core::GUID::from_u128(0x515c31d8_f734_163d_a0fd_11a08c91e8f1);
pub const GUID_IDLE_RESILIENCY_PERIOD: windows_core::GUID = windows_core::GUID::from_u128(0xc42b79aa_aa3a_484b_a98f_2cf32aa90a28);
pub const GUID_IDLE_RESILIENCY_SUBGROUP: windows_core::GUID = windows_core::GUID::from_u128(0x2e601130_5351_4d9d_8e04_252966bad054);
pub const GUID_INTSTEER_LOAD_PER_PROC_TRIGGER: windows_core::GUID = windows_core::GUID::from_u128(0x73cde64d_d720_4bb2_a860_c755afe77ef2);
pub const GUID_INTSTEER_MODE: windows_core::GUID = windows_core::GUID::from_u128(0x2bfc24f9_5ea2_4801_8213_3dbae01aa39d);
pub const GUID_INTSTEER_SUBGROUP: windows_core::GUID = windows_core::GUID::from_u128(0x48672f38_7a9a_4bb2_8bf8_3d85be19de4e);
pub const GUID_INTSTEER_TIME_UNPARK_TRIGGER: windows_core::GUID = windows_core::GUID::from_u128(0xd6ba4903_386f_4c2c_8adb_5c21b3328d25);
pub const GUID_LEGACY_RTC_MITIGATION: windows_core::GUID = windows_core::GUID::from_u128(0x1a34bdc3_7e6b_442e_a9d0_64b6ef378e84);
pub const GUID_LIDCLOSE_ACTION: windows_core::GUID = windows_core::GUID::from_u128(0x5ca83367_6e45_459f_a27b_476b1d01c936);
pub const GUID_LIDOPEN_POWERSTATE: windows_core::GUID = windows_core::GUID::from_u128(0x99ff10e7_23b1_4c07_a9d1_5c3206d741b4);
pub const GUID_LIDSWITCH_STATE_CHANGE: windows_core::GUID = windows_core::GUID::from_u128(0xba3e0f4d_b817_4094_a2d1_d56379e6a0f3);
pub const GUID_LIDSWITCH_STATE_RELIABILITY: windows_core::GUID = windows_core::GUID::from_u128(0xae4c4ff1_d361_43f4_80aa_bbb6eb03de94);
pub const GUID_LOCK_CONSOLE_ON_WAKE: windows_core::GUID = windows_core::GUID::from_u128(0x0e796bdb_100d_47d6_a2d5_f7d2daa51f51);
pub const GUID_MAX_POWER_SAVINGS: windows_core::GUID = windows_core::GUID::from_u128(0xa1841308_3541_4fab_bc81_f71556f20b4a);
pub const GUID_MIN_POWER_SAVINGS: windows_core::GUID = windows_core::GUID::from_u128(0x8c5e7fda_e8bf_4a96_9a85_a6e23a8c635c);
pub const GUID_MIXED_REALITY_MODE: windows_core::GUID = windows_core::GUID::from_u128(0x1e626b4e_cf04_4f8d_9cc7_c97c5b0f2391);
pub const GUID_MONITOR_POWER_ON: windows_core::GUID = windows_core::GUID::from_u128(0x02731015_4510_4526_99e6_e5a17ebd1aea);
pub const GUID_NON_ADAPTIVE_INPUT_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0x5adbbfbc_074e_4da1_ba38_db8b36b2c8f3);
pub const GUID_PCIEXPRESS_ASPM_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0xee12f906_d277_404b_b6da_e5fa1a576df5);
pub const GUID_PCIEXPRESS_SETTINGS_SUBGROUP: windows_core::GUID = windows_core::GUID::from_u128(0x501a4d13_42af_4429_9fd1_a8218c268e20);
pub const GUID_POWERBUTTON_ACTION: windows_core::GUID = windows_core::GUID::from_u128(0x7648efa3_dd9c_4e3e_b566_50f929386280);
pub const GUID_POWERSCHEME_PERSONALITY: windows_core::GUID = windows_core::GUID::from_u128(0x245d8541_3943_4422_b025_13a784f679b7);
pub const GUID_POWER_SAVING_STATUS: windows_core::GUID = windows_core::GUID::from_u128(0xe00958c0_c213_4ace_ac77_fecced2eeea5);
pub const GUID_PROCESSOR_ALLOW_THROTTLING: windows_core::GUID = windows_core::GUID::from_u128(0x3b04d4fd_1cc7_4f23_ab1c_d1337819c4bb);
pub const GUID_PROCESSOR_CLASS0_FLOOR_PERF: windows_core::GUID = windows_core::GUID::from_u128(0xfddc842b_8364_4edc_94cf_c17f60de1c80);
pub const GUID_PROCESSOR_CLASS1_INITIAL_PERF: windows_core::GUID = windows_core::GUID::from_u128(0x1facfc65_a930_4bc5_9f38_504ec097bbc0);
pub const GUID_PROCESSOR_COMPLEX_PARKING_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0xb669a5e9_7b1d_4132_baaa_49190abcfeb6);
pub const GUID_PROCESSOR_CORE_PARKING_AFFINITY_HISTORY_DECREASE_FACTOR: windows_core::GUID = windows_core::GUID::from_u128(0x8f7b45e3_c393_480a_878c_f67ac3d07082);
pub const GUID_PROCESSOR_CORE_PARKING_AFFINITY_HISTORY_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x5b33697b_e89d_4d38_aa46_9e7dfb7cd2f9);
pub const GUID_PROCESSOR_CORE_PARKING_AFFINITY_WEIGHTING: windows_core::GUID = windows_core::GUID::from_u128(0xe70867f1_fa2f_4f4e_aea1_4d8a0ba23b20);
pub const GUID_PROCESSOR_CORE_PARKING_DECREASE_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x71021b41_c749_4d21_be74_a00f335d582b);
pub const GUID_PROCESSOR_CORE_PARKING_DECREASE_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x68dd2f27_a4ce_4e11_8487_3794e4135dfa);
pub const GUID_PROCESSOR_CORE_PARKING_DECREASE_TIME: windows_core::GUID = windows_core::GUID::from_u128(0xdfd10d17_d5eb_45dd_877a_9a34ddd15c82);
pub const GUID_PROCESSOR_CORE_PARKING_INCREASE_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0xc7be0679_2817_4d69_9d02_519a537ed0c6);
pub const GUID_PROCESSOR_CORE_PARKING_INCREASE_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0xdf142941_20f3_4edf_9a4a_9c83d3d717d1);
pub const GUID_PROCESSOR_CORE_PARKING_INCREASE_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x2ddd5a84_5a71_437e_912a_db0b8c788732);
pub const GUID_PROCESSOR_CORE_PARKING_MAX_CORES: windows_core::GUID = windows_core::GUID::from_u128(0xea062031_0e34_4ff1_9b6d_eb1059334028);
pub const GUID_PROCESSOR_CORE_PARKING_MAX_CORES_1: windows_core::GUID = windows_core::GUID::from_u128(0xea062031_0e34_4ff1_9b6d_eb1059334029);
pub const GUID_PROCESSOR_CORE_PARKING_MIN_CORES: windows_core::GUID = windows_core::GUID::from_u128(0x0cc5b647_c1df_4637_891a_dec35c318583);
pub const GUID_PROCESSOR_CORE_PARKING_MIN_CORES_1: windows_core::GUID = windows_core::GUID::from_u128(0x0cc5b647_c1df_4637_891a_dec35c318584);
pub const GUID_PROCESSOR_CORE_PARKING_OVER_UTILIZATION_HISTORY_DECREASE_FACTOR: windows_core::GUID = windows_core::GUID::from_u128(0x1299023c_bc28_4f0a_81ec_d3295a8d815d);
pub const GUID_PROCESSOR_CORE_PARKING_OVER_UTILIZATION_HISTORY_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x9ac18e92_aa3c_4e27_b307_01ae37307129);
pub const GUID_PROCESSOR_CORE_PARKING_OVER_UTILIZATION_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x943c8cb6_6f93_4227_ad87_e9a3feec08d1);
pub const GUID_PROCESSOR_CORE_PARKING_OVER_UTILIZATION_WEIGHTING: windows_core::GUID = windows_core::GUID::from_u128(0x8809c2d8_b155_42d4_bcda_0d345651b1db);
pub const GUID_PROCESSOR_DISTRIBUTE_UTILITY: windows_core::GUID = windows_core::GUID::from_u128(0xe0007330_f589_42ed_a401_5ddb10e785d3);
pub const GUID_PROCESSOR_DUTY_CYCLING: windows_core::GUID = windows_core::GUID::from_u128(0x4e4450b3_6179_4e91_b8f1_5bb9938f81a1);
pub const GUID_PROCESSOR_FREQUENCY_LIMIT: windows_core::GUID = windows_core::GUID::from_u128(0x75b0ae3f_bce0_45a7_8c89_c9611c25e100);
pub const GUID_PROCESSOR_FREQUENCY_LIMIT_1: windows_core::GUID = windows_core::GUID::from_u128(0x75b0ae3f_bce0_45a7_8c89_c9611c25e101);
pub const GUID_PROCESSOR_HETEROGENEOUS_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x7f2f5cfa_f10c_4823_b5e1_e93ae85f46b5);
pub const GUID_PROCESSOR_HETERO_DECREASE_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0xf8861c27_95e7_475c_865b_13c0cb3f9d6b);
pub const GUID_PROCESSOR_HETERO_DECREASE_THRESHOLD_1: windows_core::GUID = windows_core::GUID::from_u128(0xf8861c27_95e7_475c_865b_13c0cb3f9d6c);
pub const GUID_PROCESSOR_HETERO_DECREASE_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x7f2492b6_60b1_45e5_ae55_773f8cd5caec);
pub const GUID_PROCESSOR_HETERO_INCREASE_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0xb000397d_9b0b_483d_98c9_692a6060cfbf);
pub const GUID_PROCESSOR_HETERO_INCREASE_THRESHOLD_1: windows_core::GUID = windows_core::GUID::from_u128(0xb000397d_9b0b_483d_98c9_692a6060cfc0);
pub const GUID_PROCESSOR_HETERO_INCREASE_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x4009efa7_e72d_4cba_9edf_91084ea8cbc3);
pub const GUID_PROCESSOR_IDLESTATE_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x68f262a7_f621_4069_b9a5_4874169be23c);
pub const GUID_PROCESSOR_IDLE_ALLOW_SCALING: windows_core::GUID = windows_core::GUID::from_u128(0x6c2993b0_8f48_481f_bcc6_00dd2742aa06);
pub const GUID_PROCESSOR_IDLE_DEMOTE_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x4b92d758_5a24_4851_a470_815d78aee119);
pub const GUID_PROCESSOR_IDLE_DISABLE: windows_core::GUID = windows_core::GUID::from_u128(0x5d76a2ca_e8c0_402f_a133_2158492d58ad);
pub const GUID_PROCESSOR_IDLE_PROMOTE_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x7b224883_b3cc_4d79_819f_8374152cbe7c);
pub const GUID_PROCESSOR_IDLE_STATE_MAXIMUM: windows_core::GUID = windows_core::GUID::from_u128(0x9943e905_9a30_4ec1_9b99_44dd3b76f7a2);
pub const GUID_PROCESSOR_IDLE_TIME_CHECK: windows_core::GUID = windows_core::GUID::from_u128(0xc4581c31_89ab_4597_8e2b_9c9cab440e6b);
pub const GUID_PROCESSOR_LATENCY_HINT_MIN_UNPARK: windows_core::GUID = windows_core::GUID::from_u128(0x616cdaa5_695e_4545_97ad_97dc2d1bdd88);
pub const GUID_PROCESSOR_LATENCY_HINT_MIN_UNPARK_1: windows_core::GUID = windows_core::GUID::from_u128(0x616cdaa5_695e_4545_97ad_97dc2d1bdd89);
pub const GUID_PROCESSOR_LONG_THREAD_ARCH_CLASS_LOWER_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x43f278bc_0f8a_46d0_8b31_9a23e615d713);
pub const GUID_PROCESSOR_LONG_THREAD_ARCH_CLASS_UPPER_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0xbf903d33_9d24_49d3_a468_e65e0325046a);
pub const GUID_PROCESSOR_MODULE_PARKING_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0xb0deaf6b_59c0_4523_8a45_ca7f40244114);
pub const GUID_PROCESSOR_PARKING_CONCURRENCY_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x2430ab6f_a520_44a2_9601_f7f23b5134b1);
pub const GUID_PROCESSOR_PARKING_CORE_OVERRIDE: windows_core::GUID = windows_core::GUID::from_u128(0xa55612aa_f624_42c6_a443_7397d064c04f);
pub const GUID_PROCESSOR_PARKING_DISTRIBUTION_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x4bdaf4e9_d103_46d7_a5f0_6280121616ef);
pub const GUID_PROCESSOR_PARKING_HEADROOM_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0xf735a673_2066_4f80_a0c5_ddee0cf1bf5d);
pub const GUID_PROCESSOR_PARKING_PERF_STATE: windows_core::GUID = windows_core::GUID::from_u128(0x447235c7_6a8d_4cc0_8e24_9eaf70b96e2b);
pub const GUID_PROCESSOR_PARKING_PERF_STATE_1: windows_core::GUID = windows_core::GUID::from_u128(0x447235c7_6a8d_4cc0_8e24_9eaf70b96e2c);
pub const GUID_PROCESSOR_PERFSTATE_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0xbbdc3814_18e9_4463_8a55_d197327c45c0);
pub const GUID_PROCESSOR_PERF_AUTONOMOUS_ACTIVITY_WINDOW: windows_core::GUID = windows_core::GUID::from_u128(0xcfeda3d0_7697_4566_a922_a9086cd49dfa);
pub const GUID_PROCESSOR_PERF_AUTONOMOUS_MODE: windows_core::GUID = windows_core::GUID::from_u128(0x8baa4a8a_14c6_4451_8e8b_14bdbd197537);
pub const GUID_PROCESSOR_PERF_BOOST_MODE: windows_core::GUID = windows_core::GUID::from_u128(0xbe337238_0d82_4146_a960_4f3749d470c7);
pub const GUID_PROCESSOR_PERF_BOOST_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x45bcc044_d885_43e2_8605_ee0ec6e96b59);
pub const GUID_PROCESSOR_PERF_CORE_PARKING_HISTORY: windows_core::GUID = windows_core::GUID::from_u128(0x77d7f282_8f1a_42cd_8537_45450a839be8);
pub const GUID_PROCESSOR_PERF_DECREASE_HISTORY: windows_core::GUID = windows_core::GUID::from_u128(0x0300f6f8_abd6_45a9_b74f_4908691a40b5);
pub const GUID_PROCESSOR_PERF_DECREASE_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x40fbefc7_2e9d_4d25_a185_0cfd8574bac6);
pub const GUID_PROCESSOR_PERF_DECREASE_POLICY_1: windows_core::GUID = windows_core::GUID::from_u128(0x40fbefc7_2e9d_4d25_a185_0cfd8574bac7);
pub const GUID_PROCESSOR_PERF_DECREASE_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x12a0ab44_fe28_4fa9_b3bd_4b64f44960a6);
pub const GUID_PROCESSOR_PERF_DECREASE_THRESHOLD_1: windows_core::GUID = windows_core::GUID::from_u128(0x12a0ab44_fe28_4fa9_b3bd_4b64f44960a7);
pub const GUID_PROCESSOR_PERF_DECREASE_TIME: windows_core::GUID = windows_core::GUID::from_u128(0xd8edeb9b_95cf_4f95_a73c_b061973693c8);
pub const GUID_PROCESSOR_PERF_DECREASE_TIME_1: windows_core::GUID = windows_core::GUID::from_u128(0xd8edeb9b_95cf_4f95_a73c_b061973693c9);
pub const GUID_PROCESSOR_PERF_ENERGY_PERFORMANCE_PREFERENCE: windows_core::GUID = windows_core::GUID::from_u128(0x36687f9e_e3a5_4dbf_b1dc_15eb381c6863);
pub const GUID_PROCESSOR_PERF_ENERGY_PERFORMANCE_PREFERENCE_1: windows_core::GUID = windows_core::GUID::from_u128(0x36687f9e_e3a5_4dbf_b1dc_15eb381c6864);
pub const GUID_PROCESSOR_PERF_HISTORY: windows_core::GUID = windows_core::GUID::from_u128(0x7d24baa7_0b84_480f_840c_1b0743c00f5f);
pub const GUID_PROCESSOR_PERF_HISTORY_1: windows_core::GUID = windows_core::GUID::from_u128(0x7d24baa7_0b84_480f_840c_1b0743c00f60);
pub const GUID_PROCESSOR_PERF_INCREASE_HISTORY: windows_core::GUID = windows_core::GUID::from_u128(0x99b3ef01_752f_46a1_80fb_7730011f2354);
pub const GUID_PROCESSOR_PERF_INCREASE_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x465e1f50_b610_473a_ab58_00d1077dc418);
pub const GUID_PROCESSOR_PERF_INCREASE_POLICY_1: windows_core::GUID = windows_core::GUID::from_u128(0x465e1f50_b610_473a_ab58_00d1077dc419);
pub const GUID_PROCESSOR_PERF_INCREASE_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x06cadf0e_64ed_448a_8927_ce7bf90eb35d);
pub const GUID_PROCESSOR_PERF_INCREASE_THRESHOLD_1: windows_core::GUID = windows_core::GUID::from_u128(0x06cadf0e_64ed_448a_8927_ce7bf90eb35e);
pub const GUID_PROCESSOR_PERF_INCREASE_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x984cf492_3bed_4488_a8f9_4286c97bf5aa);
pub const GUID_PROCESSOR_PERF_INCREASE_TIME_1: windows_core::GUID = windows_core::GUID::from_u128(0x984cf492_3bed_4488_a8f9_4286c97bf5ab);
pub const GUID_PROCESSOR_PERF_LATENCY_HINT: windows_core::GUID = windows_core::GUID::from_u128(0x0822df31_9c83_441c_a079_0de4cf009c7b);
pub const GUID_PROCESSOR_PERF_LATENCY_HINT_PERF: windows_core::GUID = windows_core::GUID::from_u128(0x619b7505_003b_4e82_b7a6_4dd29c300971);
pub const GUID_PROCESSOR_PERF_LATENCY_HINT_PERF_1: windows_core::GUID = windows_core::GUID::from_u128(0x619b7505_003b_4e82_b7a6_4dd29c300972);
pub const GUID_PROCESSOR_PERF_TIME_CHECK: windows_core::GUID = windows_core::GUID::from_u128(0x4d2b0152_7d5c_498b_88e2_34345392a2c5);
pub const GUID_PROCESSOR_RESPONSIVENESS_DISABLE_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x38b8383d_cce0_4c79_9e3e_56a4f17cc480);
pub const GUID_PROCESSOR_RESPONSIVENESS_DISABLE_THRESHOLD_1: windows_core::GUID = windows_core::GUID::from_u128(0x38b8383d_cce0_4c79_9e3e_56a4f17cc481);
pub const GUID_PROCESSOR_RESPONSIVENESS_DISABLE_TIME: windows_core::GUID = windows_core::GUID::from_u128(0xf565999f_3fb0_411a_a226_3f0198dec130);
pub const GUID_PROCESSOR_RESPONSIVENESS_DISABLE_TIME_1: windows_core::GUID = windows_core::GUID::from_u128(0xf565999f_3fb0_411a_a226_3f0198dec131);
pub const GUID_PROCESSOR_RESPONSIVENESS_ENABLE_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x3d44e256_7222_4415_a9ed_9c45fa3dd830);
pub const GUID_PROCESSOR_RESPONSIVENESS_ENABLE_THRESHOLD_1: windows_core::GUID = windows_core::GUID::from_u128(0x3d44e256_7222_4415_a9ed_9c45fa3dd831);
pub const GUID_PROCESSOR_RESPONSIVENESS_ENABLE_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x3d915188_7830_49ae_a79a_0fb0a1e5a200);
pub const GUID_PROCESSOR_RESPONSIVENESS_ENABLE_TIME_1: windows_core::GUID = windows_core::GUID::from_u128(0x3d915188_7830_49ae_a79a_0fb0a1e5a201);
pub const GUID_PROCESSOR_RESPONSIVENESS_EPP_CEILING: windows_core::GUID = windows_core::GUID::from_u128(0x4427c73b_9756_4a5c_b84b_c7bda79c7320);
pub const GUID_PROCESSOR_RESPONSIVENESS_EPP_CEILING_1: windows_core::GUID = windows_core::GUID::from_u128(0x4427c73b_9756_4a5c_b84b_c7bda79c7321);
pub const GUID_PROCESSOR_RESPONSIVENESS_PERF_FLOOR: windows_core::GUID = windows_core::GUID::from_u128(0xce8e92ee_6a86_4572_bfe0_20c21d03cd40);
pub const GUID_PROCESSOR_RESPONSIVENESS_PERF_FLOOR_1: windows_core::GUID = windows_core::GUID::from_u128(0xce8e92ee_6a86_4572_bfe0_20c21d03cd41);
pub const GUID_PROCESSOR_SETTINGS_SUBGROUP: windows_core::GUID = windows_core::GUID::from_u128(0x54533251_82be_4824_96c1_47b60b740d00);
pub const GUID_PROCESSOR_SHORT_THREAD_ARCH_CLASS_LOWER_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x53824d46_87bd_4739_aa1b_aa793fac36d6);
pub const GUID_PROCESSOR_SHORT_THREAD_ARCH_CLASS_UPPER_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x828423eb_8662_4344_90f7_52bf15870f5a);
pub const GUID_PROCESSOR_SHORT_THREAD_RUNTIME_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0xd92998c2_6a48_49ca_85d4_8cceec294570);
pub const GUID_PROCESSOR_SHORT_THREAD_SCHEDULING_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0xbae08b81_2d5e_4688_ad6a_13243356654b);
pub const GUID_PROCESSOR_SMT_UNPARKING_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0xb28a6829_c5f7_444e_8f61_10e24e85c532);
pub const GUID_PROCESSOR_SOFT_PARKING_LATENCY: windows_core::GUID = windows_core::GUID::from_u128(0x97cfac41_2217_47eb_992d_618b1977c907);
pub const GUID_PROCESSOR_THREAD_SCHEDULING_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x93b8b6dc_0698_4d1c_9ee4_0644e900c85d);
pub const GUID_PROCESSOR_THROTTLE_MAXIMUM: windows_core::GUID = windows_core::GUID::from_u128(0xbc5038f7_23e0_4960_96da_33abaf5935ec);
pub const GUID_PROCESSOR_THROTTLE_MAXIMUM_1: windows_core::GUID = windows_core::GUID::from_u128(0xbc5038f7_23e0_4960_96da_33abaf5935ed);
pub const GUID_PROCESSOR_THROTTLE_MINIMUM: windows_core::GUID = windows_core::GUID::from_u128(0x893dee8e_2bef_41e0_89c6_b55d0929964c);
pub const GUID_PROCESSOR_THROTTLE_MINIMUM_1: windows_core::GUID = windows_core::GUID::from_u128(0x893dee8e_2bef_41e0_89c6_b55d0929964d);
pub const GUID_PROCESSOR_THROTTLE_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x57027304_4af6_4104_9260_e3d95248fc36);
pub const GUID_SESSION_DISPLAY_STATUS: windows_core::GUID = windows_core::GUID::from_u128(0x2b84c20e_ad23_4ddf_93db_05ffbd7efca5);
pub const GUID_SESSION_USER_PRESENCE: windows_core::GUID = windows_core::GUID::from_u128(0x3c0f4548_c03f_4c4d_b9f2_237ede686376);
pub const GUID_SLEEPBUTTON_ACTION: windows_core::GUID = windows_core::GUID::from_u128(0x96996bc0_ad50_47ec_923b_6f41874dd9eb);
pub const GUID_SLEEP_IDLE_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x81cd32e0_7833_44f3_8737_7081f38d1f70);
pub const GUID_SLEEP_SUBGROUP: windows_core::GUID = windows_core::GUID::from_u128(0x238c9fa8_0aad_41ed_83f4_97be242c8f20);
pub const GUID_SPR_ACTIVE_SESSION_CHANGE: windows_core::GUID = windows_core::GUID::from_u128(0x0e24ce38_c393_4742_bdb1_744f4b9ee08e);
pub const GUID_STANDBY_BUDGET_GRACE_PERIOD: windows_core::GUID = windows_core::GUID::from_u128(0x60c07fe1_0556_45cf_9903_d56e32210242);
pub const GUID_STANDBY_BUDGET_PERCENT: windows_core::GUID = windows_core::GUID::from_u128(0x9fe527be_1b70_48da_930d_7bcf17b44990);
pub const GUID_STANDBY_RESERVE_GRACE_PERIOD: windows_core::GUID = windows_core::GUID::from_u128(0xc763ee92_71e8_4127_84eb_f6ed043a3e3d);
pub const GUID_STANDBY_RESERVE_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x468fe7e5_1158_46ec_88bc_5b96c9e44fd0);
pub const GUID_STANDBY_RESET_PERCENT: windows_core::GUID = windows_core::GUID::from_u128(0x49cb11a5_56e2_4afb_9d38_3df47872e21b);
pub const GUID_STANDBY_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0x29f6c1db_86da_48c5_9fdb_f2b67b1f44da);
pub const GUID_SYSTEM_AWAYMODE: windows_core::GUID = windows_core::GUID::from_u128(0x98a7f580_01f7_48aa_9c0f_44352c29e5c0);
pub const GUID_SYSTEM_BUTTON_SUBGROUP: windows_core::GUID = windows_core::GUID::from_u128(0x4f971e89_eebd_4455_a8de_9e59040e7347);
pub const GUID_SYSTEM_COOLING_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x94d3a615_a899_4ac5_ae2b_e4d8f634367f);
pub const GUID_TYPICAL_POWER_SAVINGS: windows_core::GUID = windows_core::GUID::from_u128(0x381b4222_f694_41f0_9685_ff5bb260df2e);
pub const GUID_UNATTEND_SLEEP_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0x7bc4a2f9_d8fc_4469_b07b_33eb785aaca0);
pub const GUID_USERINTERFACEBUTTON_ACTION: windows_core::GUID = windows_core::GUID::from_u128(0xa7066653_8d6c_40a8_910e_a1f54b84c7e5);
pub const GUID_USER_PRESENCE_PREDICTION: windows_core::GUID = windows_core::GUID::from_u128(0x82011705_fb95_4d46_8d35_4042b1d20def);
pub const GUID_VIDEO_ADAPTIVE_DISPLAY_BRIGHTNESS: windows_core::GUID = windows_core::GUID::from_u128(0xfbd9aa66_9553_4097_ba44_ed6e9d65eab8);
pub const GUID_VIDEO_ADAPTIVE_PERCENT_INCREASE: windows_core::GUID = windows_core::GUID::from_u128(0xeed904df_b142_4183_b10b_5a1197a37864);
pub const GUID_VIDEO_ADAPTIVE_POWERDOWN: windows_core::GUID = windows_core::GUID::from_u128(0x90959d22_d6a1_49b9_af93_bce885ad335b);
pub const GUID_VIDEO_ANNOYANCE_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0x82dbcf2d_cd67_40c5_bfdc_9f1a5ccd4663);
pub const GUID_VIDEO_CONSOLE_LOCK_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0x8ec4b3a5_6868_48c2_be75_4f3044be88a7);
pub const GUID_VIDEO_CURRENT_MONITOR_BRIGHTNESS: windows_core::GUID = windows_core::GUID::from_u128(0x8ffee2c6_2d01_46be_adb9_398addc5b4ff);
pub const GUID_VIDEO_DIM_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0x17aaa29b_8b43_4b94_aafe_35f64daaf1ee);
pub const GUID_VIDEO_POWERDOWN_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0x3c0bc021_c8a8_4e07_a973_6b14cbcb2b7e);
pub const GUID_VIDEO_SUBGROUP: windows_core::GUID = windows_core::GUID::from_u128(0x7516b95f_f776_4464_8c53_06167f40cc99);
pub const HEAP_OPTIMIZE_RESOURCES_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HEAP_OPTIMIZE_RESOURCES_INFORMATION {
    pub Version: u32,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HIBERFILE_BUCKET {
    pub MaxPhysicalMemory: u64,
    pub PhysicalMemoryPercent: [u32; 3],
}
impl Default for HIBERFILE_BUCKET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HIBERFILE_BUCKET_SIZE(pub i32);
pub const HIBERFILE_TYPE_FULL: u32 = 2u32;
pub const HIBERFILE_TYPE_MAX: u32 = 3u32;
pub const HIBERFILE_TYPE_NONE: u32 = 0u32;
pub const HIBERFILE_TYPE_REDUCED: u32 = 1u32;
pub const HiberFileBucket16GB: HIBERFILE_BUCKET_SIZE = HIBERFILE_BUCKET_SIZE(4i32);
pub const HiberFileBucket1GB: HIBERFILE_BUCKET_SIZE = HIBERFILE_BUCKET_SIZE(0i32);
pub const HiberFileBucket2GB: HIBERFILE_BUCKET_SIZE = HIBERFILE_BUCKET_SIZE(1i32);
pub const HiberFileBucket32GB: HIBERFILE_BUCKET_SIZE = HIBERFILE_BUCKET_SIZE(5i32);
pub const HiberFileBucket4GB: HIBERFILE_BUCKET_SIZE = HIBERFILE_BUCKET_SIZE(2i32);
pub const HiberFileBucket8GB: HIBERFILE_BUCKET_SIZE = HIBERFILE_BUCKET_SIZE(3i32);
pub const HiberFileBucketMax: HIBERFILE_BUCKET_SIZE = HIBERFILE_BUCKET_SIZE(7i32);
pub const HiberFileBucketUnlimited: HIBERFILE_BUCKET_SIZE = HIBERFILE_BUCKET_SIZE(6i32);
pub const IGP_CONVERSION: IGP_ID = IGP_ID(8u32);
pub const IGP_GETIMEVERSION: IGP_ID = IGP_ID(4294967292u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IGP_ID(pub u32);
pub const IGP_PROPERTY: IGP_ID = IGP_ID(4u32);
pub const IGP_SELECT: IGP_ID = IGP_ID(24u32);
pub const IGP_SENTENCE: IGP_ID = IGP_ID(12u32);
pub const IGP_SETCOMPSTR: IGP_ID = IGP_ID(20u32);
pub const IGP_UI: IGP_ID = IGP_ID(16u32);
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_ALPHA64_RUNTIME_FUNCTION_ENTRY {
    pub BeginAddress: u64,
    pub EndAddress: u64,
    pub ExceptionHandler: u64,
    pub HandlerData: u64,
    pub PrologEndAddress: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_ALPHA_RUNTIME_FUNCTION_ENTRY {
    pub BeginAddress: u32,
    pub EndAddress: u32,
    pub ExceptionHandler: u32,
    pub HandlerData: u32,
    pub PrologEndAddress: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_ARCHITECTURE_ENTRY {
    pub FixupInstRVA: u32,
    pub NewInst: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_ARCHITECTURE_HEADER {
    pub _bitfield: u32,
    pub FirstEntryRVA: u32,
}
pub const IMAGE_ARCHIVE_END: windows_core::PCSTR = windows_core::s!("`\n");
pub const IMAGE_ARCHIVE_HYBRIDMAP_MEMBER: windows_core::PCSTR = windows_core::s!("/<HYBRIDMAP>/   ");
pub const IMAGE_ARCHIVE_LINKER_MEMBER: windows_core::PCSTR = windows_core::s!("/               ");
pub const IMAGE_ARCHIVE_LONGNAMES_MEMBER: windows_core::PCSTR = windows_core::s!("//              ");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_ARCHIVE_MEMBER_HEADER {
    pub Name: [u8; 16],
    pub Date: [u8; 12],
    pub UserID: [u8; 6],
    pub GroupID: [u8; 6],
    pub Mode: [u8; 8],
    pub Size: [u8; 10],
    pub EndHeader: [u8; 2],
}
impl Default for IMAGE_ARCHIVE_MEMBER_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_ARCHIVE_PAD: windows_core::PCSTR = windows_core::s!("\n");
pub const IMAGE_ARCHIVE_START: windows_core::PCSTR = windows_core::s!("!<arch>\n");
pub const IMAGE_ARCHIVE_START_SIZE: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA {
    pub HeaderData: u32,
    pub Anonymous: IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA_0,
}
impl Default for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_ARM_RUNTIME_FUNCTION_ENTRY {
    pub BeginAddress: u32,
    pub Anonymous: IMAGE_ARM_RUNTIME_FUNCTION_ENTRY_0,
}
impl Default for IMAGE_ARM_RUNTIME_FUNCTION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_ARM_RUNTIME_FUNCTION_ENTRY_0 {
    pub UnwindData: u32,
    pub Anonymous: IMAGE_ARM_RUNTIME_FUNCTION_ENTRY_0_0,
}
impl Default for IMAGE_ARM_RUNTIME_FUNCTION_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_ARM_RUNTIME_FUNCTION_ENTRY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_AUX_SYMBOL {
    pub Sym: IMAGE_AUX_SYMBOL_0,
    pub File: IMAGE_AUX_SYMBOL_1,
    pub Section: IMAGE_AUX_SYMBOL_2,
    pub TokenDef: IMAGE_AUX_SYMBOL_TOKEN_DEF,
    pub CRC: IMAGE_AUX_SYMBOL_3,
}
impl Default for IMAGE_AUX_SYMBOL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct IMAGE_AUX_SYMBOL_3 {
    pub crc: u32,
    pub rgbReserved: [u8; 14],
}
impl Default for IMAGE_AUX_SYMBOL_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_AUX_SYMBOL_1 {
    pub Name: [u8; 18],
}
impl Default for IMAGE_AUX_SYMBOL_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_AUX_SYMBOL_2 {
    pub Length: u32,
    pub NumberOfRelocations: u16,
    pub NumberOfLinenumbers: u16,
    pub CheckSum: u32,
    pub Number: i16,
    pub Selection: u8,
    pub bReserved: u8,
    pub HighNumber: i16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct IMAGE_AUX_SYMBOL_0 {
    pub TagIndex: u32,
    pub Misc: IMAGE_AUX_SYMBOL_0_0,
    pub FcnAry: IMAGE_AUX_SYMBOL_0_1,
    pub TvIndex: u16,
}
impl Default for IMAGE_AUX_SYMBOL_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_AUX_SYMBOL_0_1 {
    pub Function: IMAGE_AUX_SYMBOL_0_1_0,
    pub Array: IMAGE_AUX_SYMBOL_0_1_1,
}
impl Default for IMAGE_AUX_SYMBOL_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_AUX_SYMBOL_0_1_1 {
    pub Dimension: [u16; 4],
}
impl Default for IMAGE_AUX_SYMBOL_0_1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_AUX_SYMBOL_0_1_0 {
    pub PointerToLinenumber: u32,
    pub PointerToNextFunction: u32,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub union IMAGE_AUX_SYMBOL_0_0 {
    pub LnSz: IMAGE_AUX_SYMBOL_0_0_0,
    pub TotalSize: u32,
}
impl Default for IMAGE_AUX_SYMBOL_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_AUX_SYMBOL_0_0_0 {
    pub Linenumber: u16,
    pub Size: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_AUX_SYMBOL_EX {
    pub Sym: IMAGE_AUX_SYMBOL_EX_0,
    pub File: IMAGE_AUX_SYMBOL_EX_1,
    pub Section: IMAGE_AUX_SYMBOL_EX_2,
    pub Anonymous: IMAGE_AUX_SYMBOL_EX_3,
    pub CRC: IMAGE_AUX_SYMBOL_EX_4,
}
impl Default for IMAGE_AUX_SYMBOL_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_AUX_SYMBOL_EX_3 {
    pub TokenDef: IMAGE_AUX_SYMBOL_TOKEN_DEF,
    pub rgbReserved: [u8; 2],
}
impl Default for IMAGE_AUX_SYMBOL_EX_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct IMAGE_AUX_SYMBOL_EX_4 {
    pub crc: u32,
    pub rgbReserved: [u8; 16],
}
impl Default for IMAGE_AUX_SYMBOL_EX_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_AUX_SYMBOL_EX_1 {
    pub Name: [u8; 20],
}
impl Default for IMAGE_AUX_SYMBOL_EX_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct IMAGE_AUX_SYMBOL_EX_2 {
    pub Length: u32,
    pub NumberOfRelocations: u16,
    pub NumberOfLinenumbers: u16,
    pub CheckSum: u32,
    pub Number: i16,
    pub Selection: u8,
    pub bReserved: u8,
    pub HighNumber: i16,
    pub rgbReserved: [u8; 2],
}
impl Default for IMAGE_AUX_SYMBOL_EX_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct IMAGE_AUX_SYMBOL_EX_0 {
    pub WeakDefaultSymIndex: u32,
    pub WeakSearchType: u32,
    pub rgbReserved: [u8; 12],
}
impl Default for IMAGE_AUX_SYMBOL_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct IMAGE_AUX_SYMBOL_TOKEN_DEF {
    pub bAuxType: u8,
    pub bReserved: u8,
    pub SymbolTableIndex: u32,
    pub rgbReserved: [u8; 12],
}
impl Default for IMAGE_AUX_SYMBOL_TOKEN_DEF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAGE_AUX_SYMBOL_TYPE(pub i32);
pub const IMAGE_AUX_SYMBOL_TYPE_TOKEN_DEF: IMAGE_AUX_SYMBOL_TYPE = IMAGE_AUX_SYMBOL_TYPE(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_BASE_RELOCATION {
    pub VirtualAddress: u32,
    pub SizeOfBlock: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_BDD_DYNAMIC_RELOCATION {
    pub Left: u16,
    pub Right: u16,
    pub Value: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_BDD_INFO {
    pub Version: u32,
    pub BDDSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_BOUND_FORWARDER_REF {
    pub TimeDateStamp: u32,
    pub OffsetModuleName: u16,
    pub Reserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_BOUND_IMPORT_DESCRIPTOR {
    pub TimeDateStamp: u32,
    pub OffsetModuleName: u16,
    pub NumberOfModuleForwarderRefs: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_CE_RUNTIME_FUNCTION_ENTRY {
    pub FuncStart: u32,
    pub _bitfield: u32,
}
pub const IMAGE_COMDAT_SELECT_ANY: u32 = 2u32;
pub const IMAGE_COMDAT_SELECT_ASSOCIATIVE: u32 = 5u32;
pub const IMAGE_COMDAT_SELECT_EXACT_MATCH: u32 = 4u32;
pub const IMAGE_COMDAT_SELECT_LARGEST: u32 = 6u32;
pub const IMAGE_COMDAT_SELECT_NEWEST: u32 = 7u32;
pub const IMAGE_COMDAT_SELECT_NODUPLICATES: u32 = 1u32;
pub const IMAGE_COMDAT_SELECT_SAME_SIZE: u32 = 3u32;
pub const IMAGE_COR_EATJ_THUNK_SIZE: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(32i32);
pub const IMAGE_COR_MIH_BASICBLOCK: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(8i32);
pub const IMAGE_COR_MIH_EHRVA: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(2i32);
pub const IMAGE_COR_MIH_METHODRVA: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_DEBUG_MISC {
    pub DataType: u32,
    pub Length: u32,
    pub Unicode: bool,
    pub Reserved: [u8; 3],
    pub Data: [u8; 1],
}
impl Default for IMAGE_DEBUG_MISC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_DEBUG_MISC_EXENAME: u32 = 1u32;
pub const IMAGE_DEBUG_TYPE_BBT: u32 = 10u32;
pub const IMAGE_DEBUG_TYPE_CLSID: u32 = 11u32;
pub const IMAGE_DEBUG_TYPE_EX_DLLCHARACTERISTICS: u32 = 20u32;
pub const IMAGE_DEBUG_TYPE_ILTCG: u32 = 14u32;
pub const IMAGE_DEBUG_TYPE_MPX: u32 = 15u32;
pub const IMAGE_DEBUG_TYPE_OMAP_FROM_SRC: u32 = 8u32;
pub const IMAGE_DEBUG_TYPE_OMAP_TO_SRC: u32 = 7u32;
pub const IMAGE_DEBUG_TYPE_POGO: u32 = 13u32;
pub const IMAGE_DEBUG_TYPE_REPRO: u32 = 16u32;
pub const IMAGE_DEBUG_TYPE_RESERVED10: u32 = 10u32;
pub const IMAGE_DEBUG_TYPE_SPGO: u32 = 18u32;
pub const IMAGE_DEBUG_TYPE_VC_FEATURE: u32 = 12u32;
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct IMAGE_DOS_HEADER {
    pub e_magic: u16,
    pub e_cblp: u16,
    pub e_cp: u16,
    pub e_crlc: u16,
    pub e_cparhdr: u16,
    pub e_minalloc: u16,
    pub e_maxalloc: u16,
    pub e_ss: u16,
    pub e_sp: u16,
    pub e_csum: u16,
    pub e_ip: u16,
    pub e_cs: u16,
    pub e_lfarlc: u16,
    pub e_ovno: u16,
    pub e_res: [u16; 4],
    pub e_oemid: u16,
    pub e_oeminfo: u16,
    pub e_res2: [u16; 10],
    pub e_lfanew: i32,
}
impl Default for IMAGE_DOS_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_DOS_SIGNATURE: u16 = 23117u16;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_DYNAMIC_RELOCATION32 {
    pub Symbol: u32,
    pub BaseRelocSize: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_DYNAMIC_RELOCATION32_V2 {
    pub HeaderSize: u32,
    pub FixupInfoSize: u32,
    pub Symbol: u32,
    pub SymbolGroup: u32,
    pub Flags: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_DYNAMIC_RELOCATION64 {
    pub Symbol: u64,
    pub BaseRelocSize: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_DYNAMIC_RELOCATION64_V2 {
    pub HeaderSize: u32,
    pub FixupInfoSize: u32,
    pub Symbol: u64,
    pub SymbolGroup: u32,
    pub Flags: u32,
}
pub const IMAGE_DYNAMIC_RELOCATION_FUNCTION_OVERRIDE: u32 = 7u32;
pub const IMAGE_DYNAMIC_RELOCATION_GUARD_IMPORT_CONTROL_TRANSFER: u32 = 3u32;
pub const IMAGE_DYNAMIC_RELOCATION_GUARD_INDIR_CONTROL_TRANSFER: u32 = 4u32;
pub const IMAGE_DYNAMIC_RELOCATION_GUARD_RF_EPILOGUE: u32 = 2u32;
pub const IMAGE_DYNAMIC_RELOCATION_GUARD_RF_PROLOGUE: u32 = 1u32;
pub const IMAGE_DYNAMIC_RELOCATION_GUARD_SWITCHTABLE_BRANCH: u32 = 5u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_DYNAMIC_RELOCATION_TABLE {
    pub Version: u32,
    pub Size: u32,
}
pub const IMAGE_ENCLAVE_FLAG_PRIMARY_IMAGE: u32 = 1u32;
pub const IMAGE_ENCLAVE_IMPORT_MATCH_AUTHOR_ID: u32 = 2u32;
pub const IMAGE_ENCLAVE_IMPORT_MATCH_FAMILY_ID: u32 = 3u32;
pub const IMAGE_ENCLAVE_IMPORT_MATCH_IMAGE_ID: u32 = 4u32;
pub const IMAGE_ENCLAVE_IMPORT_MATCH_NONE: u32 = 0u32;
pub const IMAGE_ENCLAVE_IMPORT_MATCH_UNIQUE_ID: u32 = 1u32;
pub const IMAGE_ENCLAVE_LONG_ID_LENGTH: u32 = 32u32;
pub const IMAGE_ENCLAVE_POLICY_DEBUGGABLE: u32 = 1u32;
pub const IMAGE_ENCLAVE_SHORT_ID_LENGTH: u32 = 16u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_EPILOGUE_DYNAMIC_RELOCATION_HEADER {
    pub EpilogueCount: u32,
    pub EpilogueByteCount: u8,
    pub BranchDescriptorElementSize: u8,
    pub BranchDescriptorCount: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_EXPORT_DIRECTORY {
    pub Characteristics: u32,
    pub TimeDateStamp: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub Name: u32,
    pub Base: u32,
    pub NumberOfFunctions: u32,
    pub NumberOfNames: u32,
    pub AddressOfFunctions: u32,
    pub AddressOfNames: u32,
    pub AddressOfNameOrdinals: u32,
}
pub const IMAGE_FUNCTION_OVERRIDE_ARM64_BRANCH26: u32 = 2u32;
pub const IMAGE_FUNCTION_OVERRIDE_ARM64_THUNK: u32 = 3u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_FUNCTION_OVERRIDE_DYNAMIC_RELOCATION {
    pub OriginalRva: u32,
    pub BDDOffset: u32,
    pub RvaSize: u32,
    pub BaseRelocSize: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_FUNCTION_OVERRIDE_HEADER {
    pub FuncOverrideSize: u32,
}
pub const IMAGE_FUNCTION_OVERRIDE_INVALID: u32 = 0u32;
pub const IMAGE_FUNCTION_OVERRIDE_X64_REL32: u32 = 1u32;
pub const IMAGE_GUARD_CASTGUARD_PRESENT: u32 = 16777216u32;
pub const IMAGE_GUARD_CFW_INSTRUMENTED: u32 = 512u32;
pub const IMAGE_GUARD_CF_ENABLE_EXPORT_SUPPRESSION: u32 = 32768u32;
pub const IMAGE_GUARD_CF_EXPORT_SUPPRESSION_INFO_PRESENT: u32 = 16384u32;
pub const IMAGE_GUARD_CF_FUNCTION_TABLE_PRESENT: u32 = 1024u32;
pub const IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_MASK: u32 = 4026531840u32;
pub const IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_SHIFT: u32 = 28u32;
pub const IMAGE_GUARD_CF_INSTRUMENTED: u32 = 256u32;
pub const IMAGE_GUARD_CF_LONGJUMP_TABLE_PRESENT: u32 = 65536u32;
pub const IMAGE_GUARD_DELAYLOAD_IAT_IN_ITS_OWN_SECTION: u32 = 8192u32;
pub const IMAGE_GUARD_EH_CONTINUATION_TABLE_PRESENT: u32 = 4194304u32;
pub const IMAGE_GUARD_FLAG_EXPORT_SUPPRESSED: u32 = 2u32;
pub const IMAGE_GUARD_FLAG_FID_LANGEXCPTHANDLER: u32 = 4u32;
pub const IMAGE_GUARD_FLAG_FID_SUPPRESSED: u32 = 1u32;
pub const IMAGE_GUARD_FLAG_FID_XFG: u32 = 8u32;
pub const IMAGE_GUARD_MEMCPY_PRESENT: u32 = 33554432u32;
pub const IMAGE_GUARD_PROTECT_DELAYLOAD_IAT: u32 = 4096u32;
pub const IMAGE_GUARD_RETPOLINE_PRESENT: u32 = 1048576u32;
pub const IMAGE_GUARD_RF_ENABLE: u32 = 262144u32;
pub const IMAGE_GUARD_RF_INSTRUMENTED: u32 = 131072u32;
pub const IMAGE_GUARD_RF_STRICT: u32 = 524288u32;
pub const IMAGE_GUARD_SECURITY_COOKIE_UNUSED: u32 = 2048u32;
pub const IMAGE_GUARD_XFG_ENABLED: u32 = 8388608u32;
pub const IMAGE_HOT_PATCH_ABSOLUTE: u32 = 180224u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_HOT_PATCH_BASE {
    pub SequenceNumber: u32,
    pub Flags: u32,
    pub OriginalTimeDateStamp: u32,
    pub OriginalCheckSum: u32,
    pub CodeIntegrityInfo: u32,
    pub CodeIntegritySize: u32,
    pub PatchTable: u32,
    pub BufferOffset: u32,
}
pub const IMAGE_HOT_PATCH_BASE_CAN_ROLL_BACK: u32 = 2u32;
pub const IMAGE_HOT_PATCH_BASE_OBLIGATORY: u32 = 1u32;
pub const IMAGE_HOT_PATCH_CALL_TARGET: u32 = 278528u32;
pub const IMAGE_HOT_PATCH_CHUNK_INVERSE: u32 = 2147483648u32;
pub const IMAGE_HOT_PATCH_CHUNK_OBLIGATORY: u32 = 1073741824u32;
pub const IMAGE_HOT_PATCH_CHUNK_RESERVED: u32 = 1072705536u32;
pub const IMAGE_HOT_PATCH_CHUNK_SIZE: u32 = 4095u32;
pub const IMAGE_HOT_PATCH_CHUNK_SOURCE_RVA: u32 = 32768u32;
pub const IMAGE_HOT_PATCH_CHUNK_TARGET_RVA: u32 = 16384u32;
pub const IMAGE_HOT_PATCH_CHUNK_TYPE: u32 = 1032192u32;
pub const IMAGE_HOT_PATCH_DYNAMIC_VALUE: u32 = 491520u32;
pub const IMAGE_HOT_PATCH_FUNCTION: u32 = 114688u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_HOT_PATCH_HASHES {
    pub SHA256: [u8; 32],
    pub SHA1: [u8; 20],
}
impl Default for IMAGE_HOT_PATCH_HASHES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_HOT_PATCH_INDIRECT: u32 = 376832u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_HOT_PATCH_INFO {
    pub Version: u32,
    pub Size: u32,
    pub SequenceNumber: u32,
    pub BaseImageList: u32,
    pub BaseImageCount: u32,
    pub BufferOffset: u32,
    pub ExtraPatchSize: u32,
}
pub const IMAGE_HOT_PATCH_NONE: u32 = 0u32;
pub const IMAGE_HOT_PATCH_NO_CALL_TARGET: u32 = 409600u32;
pub const IMAGE_HOT_PATCH_REL32: u32 = 245760u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_IMPORT_BY_NAME {
    pub Hint: u16,
    pub Name: [i8; 1],
}
impl Default for IMAGE_IMPORT_BY_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_IMPORT_CONTROL_TRANSFER_DYNAMIC_RELOCATION {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_IMPORT_DESCRIPTOR {
    pub Anonymous: IMAGE_IMPORT_DESCRIPTOR_0,
    pub TimeDateStamp: u32,
    pub ForwarderChain: u32,
    pub Name: u32,
    pub FirstThunk: u32,
}
impl Default for IMAGE_IMPORT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_IMPORT_DESCRIPTOR_0 {
    pub Characteristics: u32,
    pub OriginalFirstThunk: u32,
}
impl Default for IMAGE_IMPORT_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_INDIR_CONTROL_TRANSFER_DYNAMIC_RELOCATION {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_LINENUMBER {
    pub Type: IMAGE_LINENUMBER_0,
    pub Linenumber: u16,
}
impl Default for IMAGE_LINENUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub union IMAGE_LINENUMBER_0 {
    pub SymbolTableIndex: u32,
    pub VirtualAddress: u32,
}
impl Default for IMAGE_LINENUMBER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_NT_SIGNATURE: u32 = 17744u32;
pub const IMAGE_NUMBEROF_DIRECTORY_ENTRIES: u32 = 16u32;
pub const IMAGE_ORDINAL_FLAG32: u32 = 2147483648u32;
pub const IMAGE_ORDINAL_FLAG64: u64 = 9223372036854775808u64;
#[repr(C, packed(2))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_OS2_HEADER {
    pub ne_magic: u16,
    pub ne_ver: i8,
    pub ne_rev: i8,
    pub ne_enttab: u16,
    pub ne_cbenttab: u16,
    pub ne_crc: i32,
    pub ne_flags: u16,
    pub ne_autodata: u16,
    pub ne_heap: u16,
    pub ne_stack: u16,
    pub ne_csip: i32,
    pub ne_sssp: i32,
    pub ne_cseg: u16,
    pub ne_cmod: u16,
    pub ne_cbnrestab: u16,
    pub ne_segtab: u16,
    pub ne_rsrctab: u16,
    pub ne_restab: u16,
    pub ne_modtab: u16,
    pub ne_imptab: u16,
    pub ne_nrestab: i32,
    pub ne_cmovent: u16,
    pub ne_align: u16,
    pub ne_cres: u16,
    pub ne_exetyp: u8,
    pub ne_flagsothers: u8,
    pub ne_pretthunks: u16,
    pub ne_psegrefbytes: u16,
    pub ne_swaparea: u16,
    pub ne_expver: u16,
}
pub const IMAGE_OS2_SIGNATURE: u16 = 17742u16;
pub const IMAGE_OS2_SIGNATURE_LE: u16 = 17740u16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_POLICY_ENTRY {
    pub Type: IMAGE_POLICY_ENTRY_TYPE,
    pub PolicyId: IMAGE_POLICY_ID,
    pub u: IMAGE_POLICY_ENTRY_0,
}
impl Default for IMAGE_POLICY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_POLICY_ENTRY_0 {
    pub None: *const core::ffi::c_void,
    pub BoolValue: bool,
    pub Int8Value: i8,
    pub UInt8Value: u8,
    pub Int16Value: i16,
    pub UInt16Value: u16,
    pub Int32Value: i32,
    pub UInt32Value: u32,
    pub Int64Value: i64,
    pub UInt64Value: u64,
    pub AnsiStringValue: windows_core::PCSTR,
    pub UnicodeStringValue: windows_core::PCWSTR,
}
impl Default for IMAGE_POLICY_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAGE_POLICY_ENTRY_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAGE_POLICY_ID(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_POLICY_METADATA {
    pub Version: u8,
    pub Reserved0: [u8; 7],
    pub ApplicationId: u64,
    pub Policies: [IMAGE_POLICY_ENTRY; 1],
}
impl Default for IMAGE_POLICY_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_POLICY_METADATA_VERSION: u32 = 1u32;
pub const IMAGE_POLICY_SECTION_NAME: windows_core::PCSTR = windows_core::s!(".tPolicy");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_PROLOGUE_DYNAMIC_RELOCATION_HEADER {
    pub PrologueByteCount: u8,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct IMAGE_RELOCATION {
    pub Anonymous: IMAGE_RELOCATION_0,
    pub SymbolTableIndex: u32,
    pub Type: u16,
}
impl Default for IMAGE_RELOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub union IMAGE_RELOCATION_0 {
    pub VirtualAddress: u32,
    pub RelocCount: u32,
}
impl Default for IMAGE_RELOCATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_REL_ALPHA_ABSOLUTE: u32 = 0u32;
pub const IMAGE_REL_ALPHA_BRADDR: u32 = 7u32;
pub const IMAGE_REL_ALPHA_GPDISP: u32 = 6u32;
pub const IMAGE_REL_ALPHA_GPREL32: u32 = 3u32;
pub const IMAGE_REL_ALPHA_GPRELHI: u32 = 23u32;
pub const IMAGE_REL_ALPHA_GPRELLO: u32 = 22u32;
pub const IMAGE_REL_ALPHA_HINT: u32 = 8u32;
pub const IMAGE_REL_ALPHA_INLINE_REFLONG: u32 = 9u32;
pub const IMAGE_REL_ALPHA_LITERAL: u32 = 4u32;
pub const IMAGE_REL_ALPHA_LITUSE: u32 = 5u32;
pub const IMAGE_REL_ALPHA_MATCH: u32 = 13u32;
pub const IMAGE_REL_ALPHA_PAIR: u32 = 12u32;
pub const IMAGE_REL_ALPHA_REFHI: u32 = 10u32;
pub const IMAGE_REL_ALPHA_REFLO: u32 = 11u32;
pub const IMAGE_REL_ALPHA_REFLONG: u32 = 1u32;
pub const IMAGE_REL_ALPHA_REFLONGNB: u32 = 16u32;
pub const IMAGE_REL_ALPHA_REFQ1: u32 = 21u32;
pub const IMAGE_REL_ALPHA_REFQ2: u32 = 20u32;
pub const IMAGE_REL_ALPHA_REFQ3: u32 = 19u32;
pub const IMAGE_REL_ALPHA_REFQUAD: u32 = 2u32;
pub const IMAGE_REL_ALPHA_SECREL: u32 = 15u32;
pub const IMAGE_REL_ALPHA_SECRELHI: u32 = 18u32;
pub const IMAGE_REL_ALPHA_SECRELLO: u32 = 17u32;
pub const IMAGE_REL_ALPHA_SECTION: u32 = 14u32;
pub const IMAGE_REL_AMD64_ABSOLUTE: u32 = 0u32;
pub const IMAGE_REL_AMD64_ADDR32: u32 = 2u32;
pub const IMAGE_REL_AMD64_ADDR32NB: u32 = 3u32;
pub const IMAGE_REL_AMD64_ADDR64: u32 = 1u32;
pub const IMAGE_REL_AMD64_CFG_BR: u32 = 20u32;
pub const IMAGE_REL_AMD64_CFG_BR_REX: u32 = 21u32;
pub const IMAGE_REL_AMD64_CFG_CALL: u32 = 22u32;
pub const IMAGE_REL_AMD64_EHANDLER: u32 = 17u32;
pub const IMAGE_REL_AMD64_IMPORT_BR: u32 = 18u32;
pub const IMAGE_REL_AMD64_IMPORT_CALL: u32 = 19u32;
pub const IMAGE_REL_AMD64_INDIR_BR: u32 = 23u32;
pub const IMAGE_REL_AMD64_INDIR_BR_REX: u32 = 24u32;
pub const IMAGE_REL_AMD64_INDIR_BR_SWITCHTABLE_FIRST: u32 = 32u32;
pub const IMAGE_REL_AMD64_INDIR_BR_SWITCHTABLE_LAST: u32 = 47u32;
pub const IMAGE_REL_AMD64_INDIR_CALL: u32 = 25u32;
pub const IMAGE_REL_AMD64_PAIR: u32 = 15u32;
pub const IMAGE_REL_AMD64_REL32: u32 = 4u32;
pub const IMAGE_REL_AMD64_REL32_1: u32 = 5u32;
pub const IMAGE_REL_AMD64_REL32_2: u32 = 6u32;
pub const IMAGE_REL_AMD64_REL32_3: u32 = 7u32;
pub const IMAGE_REL_AMD64_REL32_4: u32 = 8u32;
pub const IMAGE_REL_AMD64_REL32_5: u32 = 9u32;
pub const IMAGE_REL_AMD64_SECREL: u32 = 11u32;
pub const IMAGE_REL_AMD64_SECREL7: u32 = 12u32;
pub const IMAGE_REL_AMD64_SECTION: u32 = 10u32;
pub const IMAGE_REL_AMD64_SREL32: u32 = 14u32;
pub const IMAGE_REL_AMD64_SSPAN32: u32 = 16u32;
pub const IMAGE_REL_AMD64_TOKEN: u32 = 13u32;
pub const IMAGE_REL_AM_ABSOLUTE: u32 = 0u32;
pub const IMAGE_REL_AM_ADDR32: u32 = 1u32;
pub const IMAGE_REL_AM_ADDR32NB: u32 = 2u32;
pub const IMAGE_REL_AM_CALL32: u32 = 3u32;
pub const IMAGE_REL_AM_FUNCINFO: u32 = 4u32;
pub const IMAGE_REL_AM_REL32_1: u32 = 5u32;
pub const IMAGE_REL_AM_REL32_2: u32 = 6u32;
pub const IMAGE_REL_AM_SECREL: u32 = 7u32;
pub const IMAGE_REL_AM_SECTION: u32 = 8u32;
pub const IMAGE_REL_AM_TOKEN: u32 = 9u32;
pub const IMAGE_REL_ARM64_ABSOLUTE: u32 = 0u32;
pub const IMAGE_REL_ARM64_ADDR32: u32 = 1u32;
pub const IMAGE_REL_ARM64_ADDR32NB: u32 = 2u32;
pub const IMAGE_REL_ARM64_ADDR64: u32 = 14u32;
pub const IMAGE_REL_ARM64_BRANCH19: u32 = 15u32;
pub const IMAGE_REL_ARM64_BRANCH26: u32 = 3u32;
pub const IMAGE_REL_ARM64_PAGEBASE_REL21: u32 = 4u32;
pub const IMAGE_REL_ARM64_PAGEOFFSET_12A: u32 = 6u32;
pub const IMAGE_REL_ARM64_PAGEOFFSET_12L: u32 = 7u32;
pub const IMAGE_REL_ARM64_REL21: u32 = 5u32;
pub const IMAGE_REL_ARM64_SECREL: u32 = 8u32;
pub const IMAGE_REL_ARM64_SECREL_HIGH12A: u32 = 10u32;
pub const IMAGE_REL_ARM64_SECREL_LOW12A: u32 = 9u32;
pub const IMAGE_REL_ARM64_SECREL_LOW12L: u32 = 11u32;
pub const IMAGE_REL_ARM64_SECTION: u32 = 13u32;
pub const IMAGE_REL_ARM64_TOKEN: u32 = 12u32;
pub const IMAGE_REL_ARM_ABSOLUTE: u32 = 0u32;
pub const IMAGE_REL_ARM_ADDR32: u32 = 1u32;
pub const IMAGE_REL_ARM_ADDR32NB: u32 = 2u32;
pub const IMAGE_REL_ARM_BLX11: u32 = 9u32;
pub const IMAGE_REL_ARM_BLX23T: u32 = 21u32;
pub const IMAGE_REL_ARM_BLX24: u32 = 8u32;
pub const IMAGE_REL_ARM_BRANCH11: u32 = 4u32;
pub const IMAGE_REL_ARM_BRANCH20T: u32 = 18u32;
pub const IMAGE_REL_ARM_BRANCH24: u32 = 3u32;
pub const IMAGE_REL_ARM_BRANCH24T: u32 = 20u32;
pub const IMAGE_REL_ARM_GPREL12: u32 = 6u32;
pub const IMAGE_REL_ARM_GPREL7: u32 = 7u32;
pub const IMAGE_REL_ARM_MOV32: u32 = 16u32;
pub const IMAGE_REL_ARM_MOV32A: u32 = 16u32;
pub const IMAGE_REL_ARM_MOV32T: u32 = 17u32;
pub const IMAGE_REL_ARM_SECREL: u32 = 15u32;
pub const IMAGE_REL_ARM_SECTION: u32 = 14u32;
pub const IMAGE_REL_ARM_TOKEN: u32 = 5u32;
pub const IMAGE_REL_BASED_ABSOLUTE: u32 = 0u32;
pub const IMAGE_REL_BASED_ARM_MOV32: u32 = 5u32;
pub const IMAGE_REL_BASED_DIR64: u32 = 10u32;
pub const IMAGE_REL_BASED_HIGH: u32 = 1u32;
pub const IMAGE_REL_BASED_HIGHADJ: u32 = 4u32;
pub const IMAGE_REL_BASED_HIGHLOW: u32 = 3u32;
pub const IMAGE_REL_BASED_IA64_IMM64: u32 = 9u32;
pub const IMAGE_REL_BASED_LOW: u32 = 2u32;
pub const IMAGE_REL_BASED_MACHINE_SPECIFIC_5: u32 = 5u32;
pub const IMAGE_REL_BASED_MACHINE_SPECIFIC_7: u32 = 7u32;
pub const IMAGE_REL_BASED_MACHINE_SPECIFIC_8: u32 = 8u32;
pub const IMAGE_REL_BASED_MACHINE_SPECIFIC_9: u32 = 9u32;
pub const IMAGE_REL_BASED_MIPS_JMPADDR: u32 = 5u32;
pub const IMAGE_REL_BASED_MIPS_JMPADDR16: u32 = 9u32;
pub const IMAGE_REL_BASED_RESERVED: u32 = 6u32;
pub const IMAGE_REL_BASED_THUMB_MOV32: u32 = 7u32;
pub const IMAGE_REL_CEE_ABSOLUTE: u32 = 0u32;
pub const IMAGE_REL_CEE_ADDR32: u32 = 1u32;
pub const IMAGE_REL_CEE_ADDR32NB: u32 = 3u32;
pub const IMAGE_REL_CEE_ADDR64: u32 = 2u32;
pub const IMAGE_REL_CEE_SECREL: u32 = 5u32;
pub const IMAGE_REL_CEE_SECTION: u32 = 4u32;
pub const IMAGE_REL_CEE_TOKEN: u32 = 6u32;
pub const IMAGE_REL_CEF_ABSOLUTE: u32 = 0u32;
pub const IMAGE_REL_CEF_ADDR32: u32 = 1u32;
pub const IMAGE_REL_CEF_ADDR32NB: u32 = 3u32;
pub const IMAGE_REL_CEF_ADDR64: u32 = 2u32;
pub const IMAGE_REL_CEF_SECREL: u32 = 5u32;
pub const IMAGE_REL_CEF_SECTION: u32 = 4u32;
pub const IMAGE_REL_CEF_TOKEN: u32 = 6u32;
pub const IMAGE_REL_EBC_ABSOLUTE: u32 = 0u32;
pub const IMAGE_REL_EBC_ADDR32NB: u32 = 1u32;
pub const IMAGE_REL_EBC_REL32: u32 = 2u32;
pub const IMAGE_REL_EBC_SECREL: u32 = 4u32;
pub const IMAGE_REL_EBC_SECTION: u32 = 3u32;
pub const IMAGE_REL_I386_ABSOLUTE: u32 = 0u32;
pub const IMAGE_REL_I386_DIR16: u32 = 1u32;
pub const IMAGE_REL_I386_DIR32: u32 = 6u32;
pub const IMAGE_REL_I386_DIR32NB: u32 = 7u32;
pub const IMAGE_REL_I386_REL16: u32 = 2u32;
pub const IMAGE_REL_I386_REL32: u32 = 20u32;
pub const IMAGE_REL_I386_SECREL: u32 = 11u32;
pub const IMAGE_REL_I386_SECREL7: u32 = 13u32;
pub const IMAGE_REL_I386_SECTION: u32 = 10u32;
pub const IMAGE_REL_I386_SEG12: u32 = 9u32;
pub const IMAGE_REL_I386_TOKEN: u32 = 12u32;
pub const IMAGE_REL_IA64_ABSOLUTE: u32 = 0u32;
pub const IMAGE_REL_IA64_ADDEND: u32 = 31u32;
pub const IMAGE_REL_IA64_DIR32: u32 = 4u32;
pub const IMAGE_REL_IA64_DIR32NB: u32 = 16u32;
pub const IMAGE_REL_IA64_DIR64: u32 = 5u32;
pub const IMAGE_REL_IA64_GPREL22: u32 = 9u32;
pub const IMAGE_REL_IA64_GPREL32: u32 = 28u32;
pub const IMAGE_REL_IA64_IMM14: u32 = 1u32;
pub const IMAGE_REL_IA64_IMM22: u32 = 2u32;
pub const IMAGE_REL_IA64_IMM64: u32 = 3u32;
pub const IMAGE_REL_IA64_IMMGPREL64: u32 = 26u32;
pub const IMAGE_REL_IA64_LTOFF22: u32 = 10u32;
pub const IMAGE_REL_IA64_PCREL21B: u32 = 6u32;
pub const IMAGE_REL_IA64_PCREL21F: u32 = 8u32;
pub const IMAGE_REL_IA64_PCREL21M: u32 = 7u32;
pub const IMAGE_REL_IA64_PCREL60B: u32 = 22u32;
pub const IMAGE_REL_IA64_PCREL60F: u32 = 23u32;
pub const IMAGE_REL_IA64_PCREL60I: u32 = 24u32;
pub const IMAGE_REL_IA64_PCREL60M: u32 = 25u32;
pub const IMAGE_REL_IA64_PCREL60X: u32 = 21u32;
pub const IMAGE_REL_IA64_SECREL22: u32 = 12u32;
pub const IMAGE_REL_IA64_SECREL32: u32 = 14u32;
pub const IMAGE_REL_IA64_SECREL64I: u32 = 13u32;
pub const IMAGE_REL_IA64_SECTION: u32 = 11u32;
pub const IMAGE_REL_IA64_SREL14: u32 = 17u32;
pub const IMAGE_REL_IA64_SREL22: u32 = 18u32;
pub const IMAGE_REL_IA64_SREL32: u32 = 19u32;
pub const IMAGE_REL_IA64_TOKEN: u32 = 27u32;
pub const IMAGE_REL_IA64_UREL32: u32 = 20u32;
pub const IMAGE_REL_M32R_ABSOLUTE: u32 = 0u32;
pub const IMAGE_REL_M32R_ADDR24: u32 = 3u32;
pub const IMAGE_REL_M32R_ADDR32: u32 = 1u32;
pub const IMAGE_REL_M32R_ADDR32NB: u32 = 2u32;
pub const IMAGE_REL_M32R_GPREL16: u32 = 4u32;
pub const IMAGE_REL_M32R_PAIR: u32 = 11u32;
pub const IMAGE_REL_M32R_PCREL16: u32 = 6u32;
pub const IMAGE_REL_M32R_PCREL24: u32 = 5u32;
pub const IMAGE_REL_M32R_PCREL8: u32 = 7u32;
pub const IMAGE_REL_M32R_REFHALF: u32 = 8u32;
pub const IMAGE_REL_M32R_REFHI: u32 = 9u32;
pub const IMAGE_REL_M32R_REFLO: u32 = 10u32;
pub const IMAGE_REL_M32R_SECREL32: u32 = 13u32;
pub const IMAGE_REL_M32R_SECTION: u32 = 12u32;
pub const IMAGE_REL_M32R_TOKEN: u32 = 14u32;
pub const IMAGE_REL_MIPS_ABSOLUTE: u32 = 0u32;
pub const IMAGE_REL_MIPS_GPREL: u32 = 6u32;
pub const IMAGE_REL_MIPS_JMPADDR: u32 = 3u32;
pub const IMAGE_REL_MIPS_JMPADDR16: u32 = 16u32;
pub const IMAGE_REL_MIPS_LITERAL: u32 = 7u32;
pub const IMAGE_REL_MIPS_PAIR: u32 = 37u32;
pub const IMAGE_REL_MIPS_REFHALF: u32 = 1u32;
pub const IMAGE_REL_MIPS_REFHI: u32 = 4u32;
pub const IMAGE_REL_MIPS_REFLO: u32 = 5u32;
pub const IMAGE_REL_MIPS_REFWORD: u32 = 2u32;
pub const IMAGE_REL_MIPS_REFWORDNB: u32 = 34u32;
pub const IMAGE_REL_MIPS_SECREL: u32 = 11u32;
pub const IMAGE_REL_MIPS_SECRELHI: u32 = 13u32;
pub const IMAGE_REL_MIPS_SECRELLO: u32 = 12u32;
pub const IMAGE_REL_MIPS_SECTION: u32 = 10u32;
pub const IMAGE_REL_MIPS_TOKEN: u32 = 14u32;
pub const IMAGE_REL_PPC_ABSOLUTE: u32 = 0u32;
pub const IMAGE_REL_PPC_ADDR14: u32 = 5u32;
pub const IMAGE_REL_PPC_ADDR16: u32 = 4u32;
pub const IMAGE_REL_PPC_ADDR24: u32 = 3u32;
pub const IMAGE_REL_PPC_ADDR32: u32 = 2u32;
pub const IMAGE_REL_PPC_ADDR32NB: u32 = 10u32;
pub const IMAGE_REL_PPC_ADDR64: u32 = 1u32;
pub const IMAGE_REL_PPC_BRNTAKEN: u32 = 1024u32;
pub const IMAGE_REL_PPC_BRTAKEN: u32 = 512u32;
pub const IMAGE_REL_PPC_GPREL: u32 = 21u32;
pub const IMAGE_REL_PPC_IFGLUE: u32 = 13u32;
pub const IMAGE_REL_PPC_IMGLUE: u32 = 14u32;
pub const IMAGE_REL_PPC_NEG: u32 = 256u32;
pub const IMAGE_REL_PPC_PAIR: u32 = 18u32;
pub const IMAGE_REL_PPC_REFHI: u32 = 16u32;
pub const IMAGE_REL_PPC_REFLO: u32 = 17u32;
pub const IMAGE_REL_PPC_REL14: u32 = 7u32;
pub const IMAGE_REL_PPC_REL24: u32 = 6u32;
pub const IMAGE_REL_PPC_SECREL: u32 = 11u32;
pub const IMAGE_REL_PPC_SECREL16: u32 = 15u32;
pub const IMAGE_REL_PPC_SECRELHI: u32 = 20u32;
pub const IMAGE_REL_PPC_SECRELLO: u32 = 19u32;
pub const IMAGE_REL_PPC_SECTION: u32 = 12u32;
pub const IMAGE_REL_PPC_TOCDEFN: u32 = 2048u32;
pub const IMAGE_REL_PPC_TOCREL14: u32 = 9u32;
pub const IMAGE_REL_PPC_TOCREL16: u32 = 8u32;
pub const IMAGE_REL_PPC_TOKEN: u32 = 22u32;
pub const IMAGE_REL_PPC_TYPEMASK: u32 = 255u32;
pub const IMAGE_REL_SH3_ABSOLUTE: u32 = 0u32;
pub const IMAGE_REL_SH3_DIRECT16: u32 = 1u32;
pub const IMAGE_REL_SH3_DIRECT32: u32 = 2u32;
pub const IMAGE_REL_SH3_DIRECT32_NB: u32 = 16u32;
pub const IMAGE_REL_SH3_DIRECT4: u32 = 6u32;
pub const IMAGE_REL_SH3_DIRECT4_LONG: u32 = 8u32;
pub const IMAGE_REL_SH3_DIRECT4_WORD: u32 = 7u32;
pub const IMAGE_REL_SH3_DIRECT8: u32 = 3u32;
pub const IMAGE_REL_SH3_DIRECT8_LONG: u32 = 5u32;
pub const IMAGE_REL_SH3_DIRECT8_WORD: u32 = 4u32;
pub const IMAGE_REL_SH3_GPREL4_LONG: u32 = 17u32;
pub const IMAGE_REL_SH3_PCREL12_WORD: u32 = 11u32;
pub const IMAGE_REL_SH3_PCREL8_LONG: u32 = 10u32;
pub const IMAGE_REL_SH3_PCREL8_WORD: u32 = 9u32;
pub const IMAGE_REL_SH3_SECREL: u32 = 15u32;
pub const IMAGE_REL_SH3_SECTION: u32 = 14u32;
pub const IMAGE_REL_SH3_SIZEOF_SECTION: u32 = 13u32;
pub const IMAGE_REL_SH3_STARTOF_SECTION: u32 = 12u32;
pub const IMAGE_REL_SH3_TOKEN: u32 = 18u32;
pub const IMAGE_REL_SHM_PAIR: u32 = 24u32;
pub const IMAGE_REL_SHM_PCRELPT: u32 = 19u32;
pub const IMAGE_REL_SHM_REFHALF: u32 = 21u32;
pub const IMAGE_REL_SHM_REFLO: u32 = 20u32;
pub const IMAGE_REL_SHM_RELHALF: u32 = 23u32;
pub const IMAGE_REL_SHM_RELLO: u32 = 22u32;
pub const IMAGE_REL_SH_NOMODE: u32 = 32768u32;
pub const IMAGE_REL_THUMB_BLX23: u32 = 21u32;
pub const IMAGE_REL_THUMB_BRANCH20: u32 = 18u32;
pub const IMAGE_REL_THUMB_BRANCH24: u32 = 20u32;
pub const IMAGE_REL_THUMB_MOV32: u32 = 17u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_RESOURCE_DATA_ENTRY {
    pub OffsetToData: u32,
    pub Size: u32,
    pub CodePage: u32,
    pub Reserved: u32,
}
pub const IMAGE_RESOURCE_DATA_IS_DIRECTORY: u32 = 2147483648u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_RESOURCE_DIRECTORY {
    pub Characteristics: u32,
    pub TimeDateStamp: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub NumberOfNamedEntries: u16,
    pub NumberOfIdEntries: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_RESOURCE_DIRECTORY_ENTRY {
    pub Anonymous1: IMAGE_RESOURCE_DIRECTORY_ENTRY_0,
    pub Anonymous2: IMAGE_RESOURCE_DIRECTORY_ENTRY_1,
}
impl Default for IMAGE_RESOURCE_DIRECTORY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_RESOURCE_DIRECTORY_ENTRY_0 {
    pub Anonymous: IMAGE_RESOURCE_DIRECTORY_ENTRY_0_0,
    pub Name: u32,
    pub Id: u16,
}
impl Default for IMAGE_RESOURCE_DIRECTORY_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_RESOURCE_DIRECTORY_ENTRY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_RESOURCE_DIRECTORY_ENTRY_1 {
    pub OffsetToData: u32,
    pub Anonymous: IMAGE_RESOURCE_DIRECTORY_ENTRY_1_0,
}
impl Default for IMAGE_RESOURCE_DIRECTORY_ENTRY_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_RESOURCE_DIRECTORY_ENTRY_1_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_RESOURCE_DIRECTORY_STRING {
    pub Length: u16,
    pub NameString: [i8; 1],
}
impl Default for IMAGE_RESOURCE_DIRECTORY_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_RESOURCE_DIR_STRING_U {
    pub Length: u16,
    pub NameString: [u16; 1],
}
impl Default for IMAGE_RESOURCE_DIR_STRING_U {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_RESOURCE_NAME_IS_STRING: u32 = 2147483648u32;
pub const IMAGE_SEPARATE_DEBUG_FLAGS_MASK: u32 = 32768u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_SEPARATE_DEBUG_HEADER {
    pub Signature: u16,
    pub Flags: u16,
    pub Machine: u16,
    pub Characteristics: u16,
    pub TimeDateStamp: u32,
    pub CheckSum: u32,
    pub ImageBase: u32,
    pub SizeOfImage: u32,
    pub NumberOfSections: u32,
    pub ExportedNamesSize: u32,
    pub DebugDirectorySize: u32,
    pub SectionAlignment: u32,
    pub Reserved: [u32; 2],
}
impl Default for IMAGE_SEPARATE_DEBUG_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_SEPARATE_DEBUG_MISMATCH: u32 = 32768u32;
pub const IMAGE_SEPARATE_DEBUG_SIGNATURE: u32 = 18756u32;
pub const IMAGE_SIZEOF_ARCHIVE_MEMBER_HDR: u32 = 60u32;
pub const IMAGE_SIZEOF_FILE_HEADER: u32 = 20u32;
pub const IMAGE_SIZEOF_SECTION_HEADER: u32 = 40u32;
pub const IMAGE_SIZEOF_SHORT_NAME: u32 = 8u32;
pub const IMAGE_SIZEOF_SYMBOL: u32 = 18u32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_SWITCHTABLE_BRANCH_DYNAMIC_RELOCATION {
    pub _bitfield: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct IMAGE_SYMBOL {
    pub N: IMAGE_SYMBOL_0,
    pub Value: u32,
    pub SectionNumber: i16,
    pub Type: u16,
    pub StorageClass: u8,
    pub NumberOfAuxSymbols: u8,
}
impl Default for IMAGE_SYMBOL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub union IMAGE_SYMBOL_0 {
    pub ShortName: [u8; 8],
    pub Name: IMAGE_SYMBOL_0_0,
    pub LongName: [u32; 2],
}
impl Default for IMAGE_SYMBOL_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_SYMBOL_0_0 {
    pub Short: u32,
    pub Long: u32,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct IMAGE_SYMBOL_EX {
    pub N: IMAGE_SYMBOL_EX_0,
    pub Value: u32,
    pub SectionNumber: i32,
    pub Type: u16,
    pub StorageClass: u8,
    pub NumberOfAuxSymbols: u8,
}
impl Default for IMAGE_SYMBOL_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub union IMAGE_SYMBOL_EX_0 {
    pub ShortName: [u8; 8],
    pub Name: IMAGE_SYMBOL_EX_0_0,
    pub LongName: [u32; 2],
}
impl Default for IMAGE_SYMBOL_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_SYMBOL_EX_0_0 {
    pub Short: u32,
    pub Long: u32,
}
pub const IMAGE_SYM_CLASS_ARGUMENT: u32 = 9u32;
pub const IMAGE_SYM_CLASS_AUTOMATIC: u32 = 1u32;
pub const IMAGE_SYM_CLASS_BIT_FIELD: u32 = 18u32;
pub const IMAGE_SYM_CLASS_BLOCK: u32 = 100u32;
pub const IMAGE_SYM_CLASS_CLR_TOKEN: u32 = 107u32;
pub const IMAGE_SYM_CLASS_END_OF_STRUCT: u32 = 102u32;
pub const IMAGE_SYM_CLASS_ENUM_TAG: u32 = 15u32;
pub const IMAGE_SYM_CLASS_EXTERNAL: u32 = 2u32;
pub const IMAGE_SYM_CLASS_EXTERNAL_DEF: u32 = 5u32;
pub const IMAGE_SYM_CLASS_FAR_EXTERNAL: u32 = 68u32;
pub const IMAGE_SYM_CLASS_FILE: u32 = 103u32;
pub const IMAGE_SYM_CLASS_FUNCTION: u32 = 101u32;
pub const IMAGE_SYM_CLASS_LABEL: u32 = 6u32;
pub const IMAGE_SYM_CLASS_MEMBER_OF_ENUM: u32 = 16u32;
pub const IMAGE_SYM_CLASS_MEMBER_OF_STRUCT: u32 = 8u32;
pub const IMAGE_SYM_CLASS_MEMBER_OF_UNION: u32 = 11u32;
pub const IMAGE_SYM_CLASS_NULL: u32 = 0u32;
pub const IMAGE_SYM_CLASS_REGISTER: u32 = 4u32;
pub const IMAGE_SYM_CLASS_REGISTER_PARAM: u32 = 17u32;
pub const IMAGE_SYM_CLASS_SECTION: u32 = 104u32;
pub const IMAGE_SYM_CLASS_STATIC: u32 = 3u32;
pub const IMAGE_SYM_CLASS_STRUCT_TAG: u32 = 10u32;
pub const IMAGE_SYM_CLASS_TYPE_DEFINITION: u32 = 13u32;
pub const IMAGE_SYM_CLASS_UNDEFINED_LABEL: u32 = 7u32;
pub const IMAGE_SYM_CLASS_UNDEFINED_STATIC: u32 = 14u32;
pub const IMAGE_SYM_CLASS_UNION_TAG: u32 = 12u32;
pub const IMAGE_SYM_CLASS_WEAK_EXTERNAL: u32 = 105u32;
pub const IMAGE_SYM_DTYPE_ARRAY: u32 = 3u32;
pub const IMAGE_SYM_DTYPE_FUNCTION: u32 = 2u32;
pub const IMAGE_SYM_DTYPE_NULL: u32 = 0u32;
pub const IMAGE_SYM_DTYPE_POINTER: u32 = 1u32;
pub const IMAGE_SYM_SECTION_MAX: u32 = 65279u32;
pub const IMAGE_SYM_SECTION_MAX_EX: u32 = 2147483647u32;
pub const IMAGE_SYM_TYPE_BYTE: u32 = 12u32;
pub const IMAGE_SYM_TYPE_CHAR: u32 = 2u32;
pub const IMAGE_SYM_TYPE_DOUBLE: u32 = 7u32;
pub const IMAGE_SYM_TYPE_DWORD: u32 = 15u32;
pub const IMAGE_SYM_TYPE_ENUM: u32 = 10u32;
pub const IMAGE_SYM_TYPE_FLOAT: u32 = 6u32;
pub const IMAGE_SYM_TYPE_INT: u32 = 4u32;
pub const IMAGE_SYM_TYPE_LONG: u32 = 5u32;
pub const IMAGE_SYM_TYPE_MOE: u32 = 11u32;
pub const IMAGE_SYM_TYPE_NULL: u32 = 0u32;
pub const IMAGE_SYM_TYPE_PCODE: u32 = 32768u32;
pub const IMAGE_SYM_TYPE_SHORT: u32 = 3u32;
pub const IMAGE_SYM_TYPE_STRUCT: u32 = 8u32;
pub const IMAGE_SYM_TYPE_UINT: u32 = 14u32;
pub const IMAGE_SYM_TYPE_UNION: u32 = 9u32;
pub const IMAGE_SYM_TYPE_VOID: u32 = 1u32;
pub const IMAGE_SYM_TYPE_WORD: u32 = 13u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_TLS_DIRECTORY32 {
    pub StartAddressOfRawData: u32,
    pub EndAddressOfRawData: u32,
    pub AddressOfIndex: u32,
    pub AddressOfCallBacks: u32,
    pub SizeOfZeroFill: u32,
    pub Anonymous: IMAGE_TLS_DIRECTORY32_0,
}
impl Default for IMAGE_TLS_DIRECTORY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_TLS_DIRECTORY32_0 {
    pub Characteristics: u32,
    pub Anonymous: IMAGE_TLS_DIRECTORY32_0_0,
}
impl Default for IMAGE_TLS_DIRECTORY32_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_TLS_DIRECTORY32_0_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct IMAGE_TLS_DIRECTORY64 {
    pub StartAddressOfRawData: u64,
    pub EndAddressOfRawData: u64,
    pub AddressOfIndex: u64,
    pub AddressOfCallBacks: u64,
    pub SizeOfZeroFill: u32,
    pub Anonymous: IMAGE_TLS_DIRECTORY64_0,
}
impl Default for IMAGE_TLS_DIRECTORY64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_TLS_DIRECTORY64_0 {
    pub Characteristics: u32,
    pub Anonymous: IMAGE_TLS_DIRECTORY64_0_0,
}
impl Default for IMAGE_TLS_DIRECTORY64_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_TLS_DIRECTORY64_0_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct IMAGE_VXD_HEADER {
    pub e32_magic: u16,
    pub e32_border: u8,
    pub e32_worder: u8,
    pub e32_level: u32,
    pub e32_cpu: u16,
    pub e32_os: u16,
    pub e32_ver: u32,
    pub e32_mflags: u32,
    pub e32_mpages: u32,
    pub e32_startobj: u32,
    pub e32_eip: u32,
    pub e32_stackobj: u32,
    pub e32_esp: u32,
    pub e32_pagesize: u32,
    pub e32_lastpagesize: u32,
    pub e32_fixupsize: u32,
    pub e32_fixupsum: u32,
    pub e32_ldrsize: u32,
    pub e32_ldrsum: u32,
    pub e32_objtab: u32,
    pub e32_objcnt: u32,
    pub e32_objmap: u32,
    pub e32_itermap: u32,
    pub e32_rsrctab: u32,
    pub e32_rsrccnt: u32,
    pub e32_restab: u32,
    pub e32_enttab: u32,
    pub e32_dirtab: u32,
    pub e32_dircnt: u32,
    pub e32_fpagetab: u32,
    pub e32_frectab: u32,
    pub e32_impmod: u32,
    pub e32_impmodcnt: u32,
    pub e32_impproc: u32,
    pub e32_pagesum: u32,
    pub e32_datapage: u32,
    pub e32_preload: u32,
    pub e32_nrestab: u32,
    pub e32_cbnrestab: u32,
    pub e32_nressum: u32,
    pub e32_autodata: u32,
    pub e32_debuginfo: u32,
    pub e32_debuglen: u32,
    pub e32_instpreload: u32,
    pub e32_instdemand: u32,
    pub e32_heapsize: u32,
    pub e32_res3: [u8; 12],
    pub e32_winresoff: u32,
    pub e32_winreslen: u32,
    pub e32_devid: u16,
    pub e32_ddkver: u16,
}
impl Default for IMAGE_VXD_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_VXD_SIGNATURE: u16 = 17740u16;
pub const IMAGE_WEAK_EXTERN_ANTI_DEPENDENCY: u32 = 4u32;
pub const IMAGE_WEAK_EXTERN_SEARCH_ALIAS: u32 = 3u32;
pub const IMAGE_WEAK_EXTERN_SEARCH_LIBRARY: u32 = 2u32;
pub const IMAGE_WEAK_EXTERN_SEARCH_NOLIBRARY: u32 = 1u32;
pub const IMPORT_OBJECT_CODE: IMPORT_OBJECT_TYPE = IMPORT_OBJECT_TYPE(0i32);
pub const IMPORT_OBJECT_CONST: IMPORT_OBJECT_TYPE = IMPORT_OBJECT_TYPE(2i32);
pub const IMPORT_OBJECT_DATA: IMPORT_OBJECT_TYPE = IMPORT_OBJECT_TYPE(1i32);
pub const IMPORT_OBJECT_HDR_SIG2: u32 = 65535u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMPORT_OBJECT_HEADER {
    pub Sig1: u16,
    pub Sig2: u16,
    pub Version: u16,
    pub Machine: u16,
    pub TimeDateStamp: u32,
    pub SizeOfData: u32,
    pub Anonymous: IMPORT_OBJECT_HEADER_0,
    pub _bitfield: u16,
}
impl Default for IMPORT_OBJECT_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMPORT_OBJECT_HEADER_0 {
    pub Ordinal: u16,
    pub Hint: u16,
}
impl Default for IMPORT_OBJECT_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMPORT_OBJECT_NAME: IMPORT_OBJECT_NAME_TYPE = IMPORT_OBJECT_NAME_TYPE(1i32);
pub const IMPORT_OBJECT_NAME_EXPORTAS: IMPORT_OBJECT_NAME_TYPE = IMPORT_OBJECT_NAME_TYPE(4i32);
pub const IMPORT_OBJECT_NAME_NO_PREFIX: IMPORT_OBJECT_NAME_TYPE = IMPORT_OBJECT_NAME_TYPE(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMPORT_OBJECT_NAME_TYPE(pub i32);
pub const IMPORT_OBJECT_NAME_UNDECORATE: IMPORT_OBJECT_NAME_TYPE = IMPORT_OBJECT_NAME_TYPE(3i32);
pub const IMPORT_OBJECT_ORDINAL: IMPORT_OBJECT_NAME_TYPE = IMPORT_OBJECT_NAME_TYPE(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMPORT_OBJECT_TYPE(pub i32);
pub const INITIAL_CPSR: u32 = 16u32;
pub const INITIAL_FPCSR: u32 = 639u32;
pub const INITIAL_FPSCR: u32 = 0u32;
pub const INITIAL_MXCSR: u32 = 8064u32;
pub const IO_COMPLETION_MODIFY_STATE: u32 = 2u32;
pub const IO_REPARSE_TAG_AF_UNIX: u32 = 2147483683u32;
pub const IO_REPARSE_TAG_APPEXECLINK: u32 = 2147483675u32;
pub const IO_REPARSE_TAG_CLOUD: u32 = 2415919130u32;
pub const IO_REPARSE_TAG_CLOUD_1: u32 = 2415923226u32;
pub const IO_REPARSE_TAG_CLOUD_2: u32 = 2415927322u32;
pub const IO_REPARSE_TAG_CLOUD_3: u32 = 2415931418u32;
pub const IO_REPARSE_TAG_CLOUD_4: u32 = 2415935514u32;
pub const IO_REPARSE_TAG_CLOUD_5: u32 = 2415939610u32;
pub const IO_REPARSE_TAG_CLOUD_6: u32 = 2415943706u32;
pub const IO_REPARSE_TAG_CLOUD_7: u32 = 2415947802u32;
pub const IO_REPARSE_TAG_CLOUD_8: u32 = 2415951898u32;
pub const IO_REPARSE_TAG_CLOUD_9: u32 = 2415955994u32;
pub const IO_REPARSE_TAG_CLOUD_A: u32 = 2415960090u32;
pub const IO_REPARSE_TAG_CLOUD_B: u32 = 2415964186u32;
pub const IO_REPARSE_TAG_CLOUD_C: u32 = 2415968282u32;
pub const IO_REPARSE_TAG_CLOUD_D: u32 = 2415972378u32;
pub const IO_REPARSE_TAG_CLOUD_E: u32 = 2415976474u32;
pub const IO_REPARSE_TAG_CLOUD_F: u32 = 2415980570u32;
pub const IO_REPARSE_TAG_CLOUD_MASK: u32 = 61440u32;
pub const IO_REPARSE_TAG_CSV: u32 = 2147483657u32;
pub const IO_REPARSE_TAG_DATALESS_CIM: u32 = 2684354600u32;
pub const IO_REPARSE_TAG_DEDUP: u32 = 2147483667u32;
pub const IO_REPARSE_TAG_DFS: u32 = 2147483658u32;
pub const IO_REPARSE_TAG_DFSR: u32 = 2147483666u32;
pub const IO_REPARSE_TAG_FILE_PLACEHOLDER: u32 = 2147483669u32;
pub const IO_REPARSE_TAG_GLOBAL_REPARSE: u32 = 2684354585u32;
pub const IO_REPARSE_TAG_HSM: u32 = 3221225476u32;
pub const IO_REPARSE_TAG_HSM2: u32 = 2147483654u32;
pub const IO_REPARSE_TAG_MOUNT_POINT: u32 = 2684354563u32;
pub const IO_REPARSE_TAG_NFS: u32 = 2147483668u32;
pub const IO_REPARSE_TAG_ONEDRIVE: u32 = 2147483681u32;
pub const IO_REPARSE_TAG_PROJFS: u32 = 2415919132u32;
pub const IO_REPARSE_TAG_PROJFS_TOMBSTONE: u32 = 2684354594u32;
pub const IO_REPARSE_TAG_RESERVED_INVALID: i32 = -1073709056i32;
pub const IO_REPARSE_TAG_RESERVED_ONE: u32 = 1u32;
pub const IO_REPARSE_TAG_RESERVED_RANGE: u32 = 2u32;
pub const IO_REPARSE_TAG_RESERVED_TWO: u32 = 2u32;
pub const IO_REPARSE_TAG_RESERVED_ZERO: u32 = 0u32;
pub const IO_REPARSE_TAG_SIS: u32 = 2147483655u32;
pub const IO_REPARSE_TAG_STORAGE_SYNC: u32 = 2147483678u32;
pub const IO_REPARSE_TAG_SYMLINK: u32 = 2684354572u32;
pub const IO_REPARSE_TAG_UNHANDLED: u32 = 2147483680u32;
pub const IO_REPARSE_TAG_WCI: u32 = 2147483672u32;
pub const IO_REPARSE_TAG_WCI_1: u32 = 2415923224u32;
pub const IO_REPARSE_TAG_WCI_LINK: u32 = 2684354599u32;
pub const IO_REPARSE_TAG_WCI_LINK_1: u32 = 2684358695u32;
pub const IO_REPARSE_TAG_WCI_TOMBSTONE: u32 = 2684354591u32;
pub const IO_REPARSE_TAG_WIM: u32 = 2147483656u32;
pub const IO_REPARSE_TAG_WOF: u32 = 2147483671u32;
pub const IS_TEXT_UNICODE_DBCS_LEADBYTE: u32 = 1024u32;
pub const IS_TEXT_UNICODE_UTF8: u32 = 2048u32;
pub const ITWW_OPEN_CONNECT: WORD_WHEEL_OPEN_FLAGS = WORD_WHEEL_OPEN_FLAGS(0u32);
pub const IgnoreError: SERVICE_ERROR_TYPE = SERVICE_ERROR_TYPE(0i32);
pub const ImagePolicyEntryTypeAnsiString: IMAGE_POLICY_ENTRY_TYPE = IMAGE_POLICY_ENTRY_TYPE(10i32);
pub const ImagePolicyEntryTypeBool: IMAGE_POLICY_ENTRY_TYPE = IMAGE_POLICY_ENTRY_TYPE(1i32);
pub const ImagePolicyEntryTypeInt16: IMAGE_POLICY_ENTRY_TYPE = IMAGE_POLICY_ENTRY_TYPE(4i32);
pub const ImagePolicyEntryTypeInt32: IMAGE_POLICY_ENTRY_TYPE = IMAGE_POLICY_ENTRY_TYPE(6i32);
pub const ImagePolicyEntryTypeInt64: IMAGE_POLICY_ENTRY_TYPE = IMAGE_POLICY_ENTRY_TYPE(8i32);
pub const ImagePolicyEntryTypeInt8: IMAGE_POLICY_ENTRY_TYPE = IMAGE_POLICY_ENTRY_TYPE(2i32);
pub const ImagePolicyEntryTypeMaximum: IMAGE_POLICY_ENTRY_TYPE = IMAGE_POLICY_ENTRY_TYPE(13i32);
pub const ImagePolicyEntryTypeNone: IMAGE_POLICY_ENTRY_TYPE = IMAGE_POLICY_ENTRY_TYPE(0i32);
pub const ImagePolicyEntryTypeOverride: IMAGE_POLICY_ENTRY_TYPE = IMAGE_POLICY_ENTRY_TYPE(12i32);
pub const ImagePolicyEntryTypeUInt16: IMAGE_POLICY_ENTRY_TYPE = IMAGE_POLICY_ENTRY_TYPE(5i32);
pub const ImagePolicyEntryTypeUInt32: IMAGE_POLICY_ENTRY_TYPE = IMAGE_POLICY_ENTRY_TYPE(7i32);
pub const ImagePolicyEntryTypeUInt64: IMAGE_POLICY_ENTRY_TYPE = IMAGE_POLICY_ENTRY_TYPE(9i32);
pub const ImagePolicyEntryTypeUInt8: IMAGE_POLICY_ENTRY_TYPE = IMAGE_POLICY_ENTRY_TYPE(3i32);
pub const ImagePolicyEntryTypeUnicodeString: IMAGE_POLICY_ENTRY_TYPE = IMAGE_POLICY_ENTRY_TYPE(11i32);
pub const ImagePolicyIdCapability: IMAGE_POLICY_ID = IMAGE_POLICY_ID(10i32);
pub const ImagePolicyIdCrashDump: IMAGE_POLICY_ID = IMAGE_POLICY_ID(3i32);
pub const ImagePolicyIdCrashDumpKey: IMAGE_POLICY_ID = IMAGE_POLICY_ID(4i32);
pub const ImagePolicyIdCrashDumpKeyGuid: IMAGE_POLICY_ID = IMAGE_POLICY_ID(5i32);
pub const ImagePolicyIdDebug: IMAGE_POLICY_ID = IMAGE_POLICY_ID(2i32);
pub const ImagePolicyIdDeviceId: IMAGE_POLICY_ID = IMAGE_POLICY_ID(9i32);
pub const ImagePolicyIdEtw: IMAGE_POLICY_ID = IMAGE_POLICY_ID(1i32);
pub const ImagePolicyIdMaximum: IMAGE_POLICY_ID = IMAGE_POLICY_ID(12i32);
pub const ImagePolicyIdNone: IMAGE_POLICY_ID = IMAGE_POLICY_ID(0i32);
pub const ImagePolicyIdParentSd: IMAGE_POLICY_ID = IMAGE_POLICY_ID(6i32);
pub const ImagePolicyIdParentSdRev: IMAGE_POLICY_ID = IMAGE_POLICY_ID(7i32);
pub const ImagePolicyIdScenarioId: IMAGE_POLICY_ID = IMAGE_POLICY_ID(11i32);
pub const ImagePolicyIdSvn: IMAGE_POLICY_ID = IMAGE_POLICY_ID(8i32);
pub const JOB_OBJECT_ASSIGN_PROCESS: u32 = 1u32;
pub const JOB_OBJECT_IMPERSONATE: u32 = 32u32;
pub const JOB_OBJECT_MSG_ABNORMAL_EXIT_PROCESS: u32 = 8u32;
pub const JOB_OBJECT_MSG_ACTIVE_PROCESS_LIMIT: u32 = 3u32;
pub const JOB_OBJECT_MSG_ACTIVE_PROCESS_ZERO: u32 = 4u32;
pub const JOB_OBJECT_MSG_END_OF_JOB_TIME: u32 = 1u32;
pub const JOB_OBJECT_MSG_END_OF_PROCESS_TIME: u32 = 2u32;
pub const JOB_OBJECT_MSG_EXIT_PROCESS: u32 = 7u32;
pub const JOB_OBJECT_MSG_JOB_CYCLE_TIME_LIMIT: u32 = 12u32;
pub const JOB_OBJECT_MSG_JOB_MEMORY_LIMIT: u32 = 10u32;
pub const JOB_OBJECT_MSG_MAXIMUM: u32 = 13u32;
pub const JOB_OBJECT_MSG_MINIMUM: u32 = 1u32;
pub const JOB_OBJECT_MSG_NEW_PROCESS: u32 = 6u32;
pub const JOB_OBJECT_MSG_NOTIFICATION_LIMIT: u32 = 11u32;
pub const JOB_OBJECT_MSG_PROCESS_MEMORY_LIMIT: u32 = 9u32;
pub const JOB_OBJECT_MSG_SILO_TERMINATED: u32 = 13u32;
pub const JOB_OBJECT_NET_RATE_CONTROL_MAX_DSCP_TAG: u32 = 64u32;
pub const JOB_OBJECT_QUERY: u32 = 4u32;
pub const JOB_OBJECT_SET_ATTRIBUTES: u32 = 2u32;
pub const JOB_OBJECT_SET_SECURITY_ATTRIBUTES: u32 = 16u32;
pub const JOB_OBJECT_TERMINATE: u32 = 8u32;
pub const JOB_OBJECT_UILIMIT_ALL: u32 = 511u32;
pub const JOB_OBJECT_UILIMIT_IME: u32 = 256u32;
pub const JOB_OBJECT_UI_VALID_FLAGS: u32 = 511u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KERNEL_CET_CONTEXT {
    pub Ssp: u64,
    pub Rip: u64,
    pub SegCs: u16,
    pub Anonymous: KERNEL_CET_CONTEXT_0,
    pub Fill: [u16; 2],
}
impl Default for KERNEL_CET_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KERNEL_CET_CONTEXT_0 {
    pub AllFlags: u16,
    pub Anonymous: KERNEL_CET_CONTEXT_0_0,
}
impl Default for KERNEL_CET_CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERNEL_CET_CONTEXT_0_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KTMOBJECT_CURSOR {
    pub LastQuery: windows_core::GUID,
    pub ObjectIdCount: u32,
    pub ObjectIds: [windows_core::GUID; 1],
}
impl Default for KTMOBJECT_CURSOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KTMOBJECT_ENLISTMENT: KTMOBJECT_TYPE = KTMOBJECT_TYPE(3i32);
pub const KTMOBJECT_INVALID: KTMOBJECT_TYPE = KTMOBJECT_TYPE(4i32);
pub const KTMOBJECT_RESOURCE_MANAGER: KTMOBJECT_TYPE = KTMOBJECT_TYPE(2i32);
pub const KTMOBJECT_TRANSACTION: KTMOBJECT_TYPE = KTMOBJECT_TYPE(0i32);
pub const KTMOBJECT_TRANSACTION_MANAGER: KTMOBJECT_TYPE = KTMOBJECT_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KTMOBJECT_TYPE(pub i32);
pub const LANG_AFRIKAANS: u32 = 54u32;
pub const LANG_ALBANIAN: u32 = 28u32;
pub const LANG_ALSATIAN: u32 = 132u32;
pub const LANG_AMHARIC: u32 = 94u32;
pub const LANG_ARABIC: u32 = 1u32;
pub const LANG_ARMENIAN: u32 = 43u32;
pub const LANG_ASSAMESE: u32 = 77u32;
pub const LANG_AZERBAIJANI: u32 = 44u32;
pub const LANG_AZERI: u32 = 44u32;
pub const LANG_BANGLA: u32 = 69u32;
pub const LANG_BASHKIR: u32 = 109u32;
pub const LANG_BASQUE: u32 = 45u32;
pub const LANG_BELARUSIAN: u32 = 35u32;
pub const LANG_BENGALI: u32 = 69u32;
pub const LANG_BOSNIAN: u32 = 26u32;
pub const LANG_BOSNIAN_NEUTRAL: u32 = 30746u32;
pub const LANG_BRETON: u32 = 126u32;
pub const LANG_BULGARIAN: u32 = 2u32;
pub const LANG_CATALAN: u32 = 3u32;
pub const LANG_CENTRAL_KURDISH: u32 = 146u32;
pub const LANG_CHEROKEE: u32 = 92u32;
pub const LANG_CHINESE: u32 = 4u32;
pub const LANG_CHINESE_SIMPLIFIED: u32 = 4u32;
pub const LANG_CHINESE_TRADITIONAL: u32 = 31748u32;
pub const LANG_CORSICAN: u32 = 131u32;
pub const LANG_CROATIAN: u32 = 26u32;
pub const LANG_CZECH: u32 = 5u32;
pub const LANG_DANISH: u32 = 6u32;
pub const LANG_DARI: u32 = 140u32;
pub const LANG_DIVEHI: u32 = 101u32;
pub const LANG_DUTCH: u32 = 19u32;
pub const LANG_ENGLISH: u32 = 9u32;
pub const LANG_ESTONIAN: u32 = 37u32;
pub const LANG_FAEROESE: u32 = 56u32;
pub const LANG_FARSI: u32 = 41u32;
pub const LANG_FILIPINO: u32 = 100u32;
pub const LANG_FINNISH: u32 = 11u32;
pub const LANG_FRENCH: u32 = 12u32;
pub const LANG_FRISIAN: u32 = 98u32;
pub const LANG_FULAH: u32 = 103u32;
pub const LANG_GALICIAN: u32 = 86u32;
pub const LANG_GEORGIAN: u32 = 55u32;
pub const LANG_GERMAN: u32 = 7u32;
pub const LANG_GREEK: u32 = 8u32;
pub const LANG_GREENLANDIC: u32 = 111u32;
pub const LANG_GUJARATI: u32 = 71u32;
pub const LANG_HAUSA: u32 = 104u32;
pub const LANG_HAWAIIAN: u32 = 117u32;
pub const LANG_HEBREW: u32 = 13u32;
pub const LANG_HINDI: u32 = 57u32;
pub const LANG_HUNGARIAN: u32 = 14u32;
pub const LANG_ICELANDIC: u32 = 15u32;
pub const LANG_IGBO: u32 = 112u32;
pub const LANG_INDONESIAN: u32 = 33u32;
pub const LANG_INUKTITUT: u32 = 93u32;
pub const LANG_INVARIANT: u32 = 127u32;
pub const LANG_IRISH: u32 = 60u32;
pub const LANG_ITALIAN: u32 = 16u32;
pub const LANG_JAPANESE: u32 = 17u32;
pub const LANG_KANNADA: u32 = 75u32;
pub const LANG_KASHMIRI: u32 = 96u32;
pub const LANG_KAZAK: u32 = 63u32;
pub const LANG_KHMER: u32 = 83u32;
pub const LANG_KICHE: u32 = 134u32;
pub const LANG_KINYARWANDA: u32 = 135u32;
pub const LANG_KONKANI: u32 = 87u32;
pub const LANG_KOREAN: u32 = 18u32;
pub const LANG_KYRGYZ: u32 = 64u32;
pub const LANG_LAO: u32 = 84u32;
pub const LANG_LATVIAN: u32 = 38u32;
pub const LANG_LITHUANIAN: u32 = 39u32;
pub const LANG_LOWER_SORBIAN: u32 = 46u32;
pub const LANG_LUXEMBOURGISH: u32 = 110u32;
pub const LANG_MACEDONIAN: u32 = 47u32;
pub const LANG_MALAY: u32 = 62u32;
pub const LANG_MALAYALAM: u32 = 76u32;
pub const LANG_MALTESE: u32 = 58u32;
pub const LANG_MANIPURI: u32 = 88u32;
pub const LANG_MAORI: u32 = 129u32;
pub const LANG_MAPUDUNGUN: u32 = 122u32;
pub const LANG_MARATHI: u32 = 78u32;
pub const LANG_MOHAWK: u32 = 124u32;
pub const LANG_MONGOLIAN: u32 = 80u32;
pub const LANG_NEPALI: u32 = 97u32;
pub const LANG_NEUTRAL: u32 = 0u32;
pub const LANG_NORWEGIAN: u32 = 20u32;
pub const LANG_OCCITAN: u32 = 130u32;
pub const LANG_ODIA: u32 = 72u32;
pub const LANG_ORIYA: u32 = 72u32;
pub const LANG_PASHTO: u32 = 99u32;
pub const LANG_PERSIAN: u32 = 41u32;
pub const LANG_POLISH: u32 = 21u32;
pub const LANG_PORTUGUESE: u32 = 22u32;
pub const LANG_PULAR: u32 = 103u32;
pub const LANG_PUNJABI: u32 = 70u32;
pub const LANG_QUECHUA: u32 = 107u32;
pub const LANG_ROMANIAN: u32 = 24u32;
pub const LANG_ROMANSH: u32 = 23u32;
pub const LANG_RUSSIAN: u32 = 25u32;
pub const LANG_SAKHA: u32 = 133u32;
pub const LANG_SAMI: u32 = 59u32;
pub const LANG_SANSKRIT: u32 = 79u32;
pub const LANG_SCOTTISH_GAELIC: u32 = 145u32;
pub const LANG_SERBIAN: u32 = 26u32;
pub const LANG_SERBIAN_NEUTRAL: u32 = 31770u32;
pub const LANG_SINDHI: u32 = 89u32;
pub const LANG_SINHALESE: u32 = 91u32;
pub const LANG_SLOVAK: u32 = 27u32;
pub const LANG_SLOVENIAN: u32 = 36u32;
pub const LANG_SOTHO: u32 = 108u32;
pub const LANG_SPANISH: u32 = 10u32;
pub const LANG_SWAHILI: u32 = 65u32;
pub const LANG_SWEDISH: u32 = 29u32;
pub const LANG_SYRIAC: u32 = 90u32;
pub const LANG_TAJIK: u32 = 40u32;
pub const LANG_TAMAZIGHT: u32 = 95u32;
pub const LANG_TAMIL: u32 = 73u32;
pub const LANG_TATAR: u32 = 68u32;
pub const LANG_TELUGU: u32 = 74u32;
pub const LANG_THAI: u32 = 30u32;
pub const LANG_TIBETAN: u32 = 81u32;
pub const LANG_TIGRIGNA: u32 = 115u32;
pub const LANG_TIGRINYA: u32 = 115u32;
pub const LANG_TSWANA: u32 = 50u32;
pub const LANG_TURKISH: u32 = 31u32;
pub const LANG_TURKMEN: u32 = 66u32;
pub const LANG_UIGHUR: u32 = 128u32;
pub const LANG_UKRAINIAN: u32 = 34u32;
pub const LANG_UPPER_SORBIAN: u32 = 46u32;
pub const LANG_URDU: u32 = 32u32;
pub const LANG_UZBEK: u32 = 67u32;
pub const LANG_VALENCIAN: u32 = 3u32;
pub const LANG_VIETNAMESE: u32 = 42u32;
pub const LANG_WELSH: u32 = 82u32;
pub const LANG_WOLOF: u32 = 136u32;
pub const LANG_XHOSA: u32 = 52u32;
pub const LANG_YAKUT: u32 = 133u32;
pub const LANG_YI: u32 = 120u32;
pub const LANG_YORUBA: u32 = 106u32;
pub const LANG_ZULU: u32 = 53u32;
pub const LMEM_DISCARDABLE: u32 = 3840u32;
pub const LMEM_DISCARDED: u32 = 16384u32;
pub const LMEM_INVALID_HANDLE: u32 = 32768u32;
pub const LMEM_LOCKCOUNT: u32 = 255u32;
pub const LMEM_MODIFY: u32 = 128u32;
pub const LMEM_NOCOMPACT: u32 = 16u32;
pub const LMEM_NODISCARD: u32 = 32u32;
pub const LMEM_VALID_FLAGS: u32 = 3954u32;
pub const LOCALE_NAME_MAX_LENGTH: u32 = 85u32;
pub const LOCALE_TRANSIENT_KEYBOARD1: u32 = 8192u32;
pub const LOCALE_TRANSIENT_KEYBOARD2: u32 = 9216u32;
pub const LOCALE_TRANSIENT_KEYBOARD3: u32 = 10240u32;
pub const LOCALE_TRANSIENT_KEYBOARD4: u32 = 11264u32;
pub const LTP_PC_SMT: u32 = 1u32;
pub const MAILSLOT_NO_MESSAGE: u32 = 4294967295u32;
pub const MAILSLOT_WAIT_FOREVER: u32 = 4294967295u32;
pub const MAXBYTE: u32 = 255u32;
pub const MAXCHAR: u32 = 127u32;
pub const MAXDWORD: u32 = 4294967295u32;
pub const MAXIMUM_ALLOWED: u32 = 33554432u32;
pub const MAXIMUM_PROCESSORS: u32 = 64u32;
pub const MAXIMUM_PROC_PER_GROUP: u32 = 64u32;
pub const MAXIMUM_SUPPORTED_EXTENSION: u32 = 512u32;
pub const MAXIMUM_SUSPEND_COUNT: u32 = 127u32;
pub const MAXIMUM_WAIT_OBJECTS: u32 = 64u32;
pub const MAXIMUM_XSTATE_FEATURES: u32 = 64u32;
pub const MAXLOGICALLOGNAMESIZE: u32 = 256u32;
pub const MAXLONG: u32 = 2147483647u32;
pub const MAXLONGLONG: u64 = 9223372036854775807u64;
pub const MAXSHORT: u32 = 32767u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MAXVERSIONTESTED_INFO {
    pub MaxVersionTested: u64,
}
pub const MAXWORD: u32 = 65535u32;
pub const MAX_ACL_REVISION: u32 = 4u32;
pub const MAX_CLASS_NAME: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(1024i32);
pub const MAX_HW_COUNTERS: u32 = 16u32;
pub const MAX_PACKAGE_NAME: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(1024i32);
pub const MAX_UCSCHAR: u32 = 1114111u32;
pub const MEMORY_ALLOCATION_ALIGNMENT: u32 = 16u32;
pub const MEMORY_PARTITION_MODIFY_ACCESS: u32 = 2u32;
pub const MEMORY_PARTITION_QUERY_ACCESS: u32 = 1u32;
pub const MEMORY_PRIORITY_LOWEST: u32 = 0u32;
pub const MEM_4MB_PAGES: u32 = 2147483648u32;
pub const MEM_COALESCE_PLACEHOLDERS: u32 = 1u32;
pub const MEM_DIFFERENT_IMAGE_BASE_OK: u32 = 8388608u32;
pub const MEM_EXTENDED_PARAMETER_EC_CODE: u32 = 64u32;
pub const MEM_EXTENDED_PARAMETER_GRAPHICS: u32 = 1u32;
pub const MEM_EXTENDED_PARAMETER_IMAGE_NO_HPAT: u32 = 128u32;
pub const MEM_EXTENDED_PARAMETER_NONPAGED: u32 = 2u32;
pub const MEM_EXTENDED_PARAMETER_NONPAGED_HUGE: u32 = 16u32;
pub const MEM_EXTENDED_PARAMETER_NONPAGED_LARGE: u32 = 8u32;
pub const MEM_EXTENDED_PARAMETER_SOFT_FAULT_PAGES: u32 = 32u32;
pub const MEM_EXTENDED_PARAMETER_TYPE_BITS: u32 = 8u32;
pub const MEM_EXTENDED_PARAMETER_ZERO_PAGES_OPTIONAL: u32 = 4u32;
pub const MEM_PHYSICAL: u32 = 4194304u32;
pub const MEM_ROTATE: u32 = 8388608u32;
pub const MEM_TOP_DOWN: u32 = 1048576u32;
pub const MEM_WRITE_WATCH: u32 = 2097152u32;
pub const MESSAGE_RESOURCE_UNICODE: u32 = 1u32;
pub const MESSAGE_RESOURCE_UTF8: u32 = 2u32;
pub const MINCHAR: u32 = 128u32;
pub const MINLONG: u32 = 2147483648u32;
pub const MINSHORT: u32 = 32768u32;
pub const MIN_UCSCHAR: u32 = 0u32;
pub const MK_CONTROL: MODIFIERKEYS_FLAGS = MODIFIERKEYS_FLAGS(8u32);
pub const MK_LBUTTON: MODIFIERKEYS_FLAGS = MODIFIERKEYS_FLAGS(1u32);
pub const MK_MBUTTON: MODIFIERKEYS_FLAGS = MODIFIERKEYS_FLAGS(16u32);
pub const MK_RBUTTON: MODIFIERKEYS_FLAGS = MODIFIERKEYS_FLAGS(2u32);
pub const MK_SHIFT: MODIFIERKEYS_FLAGS = MODIFIERKEYS_FLAGS(4u32);
pub const MK_XBUTTON1: MODIFIERKEYS_FLAGS = MODIFIERKEYS_FLAGS(32u32);
pub const MK_XBUTTON2: MODIFIERKEYS_FLAGS = MODIFIERKEYS_FLAGS(64u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MODIFIERKEYS_FLAGS(pub u32);
impl MODIFIERKEYS_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MODIFIERKEYS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MODIFIERKEYS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MODIFIERKEYS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MODIFIERKEYS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MODIFIERKEYS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MONITOR_DISPLAY_STATE(pub i32);
pub const MS_PPM_SOFTWARE_ALL: u32 = 1u32;
pub const MUTANT_QUERY_STATE: u32 = 1u32;
pub const MaxActivationContextInfoClass: ACTIVATION_CONTEXT_INFO_CLASS = ACTIVATION_CONTEXT_INFO_CLASS(8i32);
pub const NATIVE_TYPE_MAX_CB: ReplacesCorHdrNumericDefines = ReplacesCorHdrNumericDefines(1i32);
pub const NETWORK_APP_INSTANCE_CSV_FLAGS_VALID_ONLY_IF_CSV_COORDINATOR: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NETWORK_APP_INSTANCE_EA {
    pub AppInstanceID: windows_core::GUID,
    pub CsvFlags: u32,
}
pub const NLS_VALID_LOCALE_MASK: u32 = 1048575u32;
pub const NONVOL_FP_NUMREG_ARM64: u32 = 8u32;
pub const NONVOL_INT_NUMREG_ARM64: u32 = 11u32;
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct NON_PAGED_DEBUG_INFO {
    pub Signature: u16,
    pub Flags: u16,
    pub Size: u32,
    pub Machine: u16,
    pub Characteristics: u16,
    pub TimeDateStamp: u32,
    pub CheckSum: u32,
    pub SizeOfImage: u32,
    pub ImageBase: u64,
}
pub const NON_PAGED_DEBUG_SIGNATURE: u32 = 18766u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NOTIFY_USER_POWER_SETTING {
    pub Guid: windows_core::GUID,
}
pub const NO_SUBGROUP_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xfea3413e_7e05_4911_9a71_700331f1c294);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NT_TIB32 {
    pub ExceptionList: u32,
    pub StackBase: u32,
    pub StackLimit: u32,
    pub SubSystemTib: u32,
    pub Anonymous: NT_TIB32_0,
    pub ArbitraryUserPointer: u32,
    pub Self_: u32,
}
impl Default for NT_TIB32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NT_TIB32_0 {
    pub FiberData: u32,
    pub Version: u32,
}
impl Default for NT_TIB32_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NT_TIB64 {
    pub ExceptionList: u64,
    pub StackBase: u64,
    pub StackLimit: u64,
    pub SubSystemTib: u64,
    pub Anonymous: NT_TIB64_0,
    pub ArbitraryUserPointer: u64,
    pub Self_: u64,
}
impl Default for NT_TIB64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NT_TIB64_0 {
    pub FiberData: u64,
    pub Version: u32,
}
impl Default for NT_TIB64_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NUMA_NO_PREFERRED_NODE: u32 = 4294967295u32;
pub const NUM_DISCHARGE_POLICIES: u32 = 4u32;
pub const N_BTMASK: u32 = 15u32;
pub const N_BTSHFT: u32 = 4u32;
pub const N_TMASK: u32 = 48u32;
pub const N_TMASK1: u32 = 192u32;
pub const N_TMASK2: u32 = 240u32;
pub const N_TSHIFT: u32 = 2u32;
pub const NormalError: SERVICE_ERROR_TYPE = SERVICE_ERROR_TYPE(1i32);
pub const OUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK_EXPORT_NAME: windows_core::PCSTR = windows_core::s!("OutOfProcessFunctionTableCallback");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PACKEDEVENTINFO {
    pub ulSize: u32,
    pub ulNumEventsForLogFile: u32,
    pub ulOffsets: [u32; 1],
}
impl Default for PACKEDEVENTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PARKING_TOPOLOGY_POLICY_DISABLED: u32 = 0u32;
pub const PARKING_TOPOLOGY_POLICY_ROUNDROBIN: u32 = 1u32;
pub const PARKING_TOPOLOGY_POLICY_SEQUENTIAL: u32 = 2u32;
pub const PERFORMANCE_DATA_VERSION: u32 = 1u32;
pub const PERFSTATE_POLICY_CHANGE_DECREASE_MAX: u32 = 2u32;
pub const PERFSTATE_POLICY_CHANGE_IDEAL: u32 = 0u32;
pub const PERFSTATE_POLICY_CHANGE_IDEAL_AGGRESSIVE: u32 = 3u32;
pub const PERFSTATE_POLICY_CHANGE_INCREASE_MAX: u32 = 3u32;
pub const PERFSTATE_POLICY_CHANGE_ROCKET: u32 = 2u32;
pub const PERFSTATE_POLICY_CHANGE_SINGLE: u32 = 1u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub type PEXCEPTION_FILTER = Option<unsafe extern "system" fn(exceptionpointers: *mut super::Diagnostics::Debug::EXCEPTION_POINTERS, establisherframe: *const core::ffi::c_void) -> i32>;
pub const PF_NON_TEMPORAL_LEVEL_ALL: u32 = 0u32;
pub const PF_TEMPORAL_LEVEL_1: u32 = 1u32;
pub const PF_TEMPORAL_LEVEL_2: u32 = 2u32;
pub const PF_TEMPORAL_LEVEL_3: u32 = 3u32;
pub type PIMAGE_TLS_CALLBACK = Option<unsafe extern "system" fn(dllhandle: *mut core::ffi::c_void, reason: u32, reserved: *mut core::ffi::c_void)>;
pub const POLICY_AUDIT_SUBCATEGORY_COUNT: u32 = 59u32;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub type POUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK = Option<unsafe extern "system" fn(process: super::super::Foundation::HANDLE, tableaddress: *const core::ffi::c_void, entries: *mut u32, functions: *mut *mut super::Diagnostics::Debug::IMAGE_RUNTIME_FUNCTION_ENTRY) -> u32>;
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub type POUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK = Option<unsafe extern "system" fn(process: super::super::Foundation::HANDLE, tableaddress: *const core::ffi::c_void, entries: *mut u32, functions: *mut *mut super::Diagnostics::Debug::IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY) -> u32>;
pub const POWERBUTTON_ACTION_INDEX_HIBERNATE: u32 = 2u32;
pub const POWERBUTTON_ACTION_INDEX_NOTHING: u32 = 0u32;
pub const POWERBUTTON_ACTION_INDEX_SHUTDOWN: u32 = 3u32;
pub const POWERBUTTON_ACTION_INDEX_SLEEP: u32 = 1u32;
pub const POWERBUTTON_ACTION_INDEX_TURN_OFF_THE_DISPLAY: u32 = 4u32;
pub const POWERBUTTON_ACTION_VALUE_HIBERNATE: u32 = 3u32;
pub const POWERBUTTON_ACTION_VALUE_NOTHING: u32 = 0u32;
pub const POWERBUTTON_ACTION_VALUE_SHUTDOWN: u32 = 6u32;
pub const POWERBUTTON_ACTION_VALUE_SLEEP: u32 = 2u32;
pub const POWERBUTTON_ACTION_VALUE_TURN_OFF_THE_DISPLAY: u32 = 8u32;
pub const POWER_ACTION_ACPI_CRITICAL: u32 = 16777216u32;
pub const POWER_ACTION_ACPI_USER_NOTIFY: u32 = 33554432u32;
pub const POWER_ACTION_CRITICAL: u32 = 2147483648u32;
pub const POWER_ACTION_DIRECTED_DRIPS: u32 = 67108864u32;
pub const POWER_ACTION_DISABLE_WAKES: u32 = 1073741824u32;
pub const POWER_ACTION_DOZE_TO_HIBERNATE: u32 = 32u32;
pub const POWER_ACTION_HIBERBOOT: u32 = 8u32;
pub const POWER_ACTION_LIGHTEST_FIRST: u32 = 268435456u32;
pub const POWER_ACTION_LOCK_CONSOLE: u32 = 536870912u32;
pub const POWER_ACTION_OVERRIDE_APPS: u32 = 4u32;
pub const POWER_ACTION_PSEUDO_TRANSITION: u32 = 134217728u32;
pub const POWER_ACTION_QUERY_ALLOWED: u32 = 1u32;
pub const POWER_ACTION_UI_ALLOWED: u32 = 2u32;
pub const POWER_ACTION_USER_NOTIFY: u32 = 16u32;
pub const POWER_CONNECTIVITY_IN_STANDBY_DISABLED: u32 = 0u32;
pub const POWER_CONNECTIVITY_IN_STANDBY_ENABLED: u32 = 1u32;
pub const POWER_CONNECTIVITY_IN_STANDBY_SYSTEM_MANAGED: u32 = 2u32;
pub const POWER_DEVICE_IDLE_POLICY_CONSERVATIVE: u32 = 1u32;
pub const POWER_DEVICE_IDLE_POLICY_PERFORMANCE: u32 = 0u32;
pub const POWER_DISCONNECTED_STANDBY_MODE_AGGRESSIVE: u32 = 1u32;
pub const POWER_DISCONNECTED_STANDBY_MODE_NORMAL: u32 = 0u32;
pub const POWER_REQUEST_CONTEXT_VERSION: u32 = 0u32;
pub const POWER_SETTING_VALUE_VERSION: u32 = 1u32;
pub const POWER_SYSTEM_MAXIMUM: u32 = 7u32;
pub const POWER_USER_NOTIFY_FORCED_SHUTDOWN: u32 = 32u32;
pub const PO_THROTTLE_ADAPTIVE: u32 = 3u32;
pub const PO_THROTTLE_CONSTANT: u32 = 1u32;
pub const PO_THROTTLE_DEGRADE: u32 = 2u32;
pub const PO_THROTTLE_MAXIMUM: u32 = 4u32;
pub const PO_THROTTLE_NONE: u32 = 0u32;
pub const PRAGMA_DEPRECATED_DDK: u32 = 1u32;
pub const PRIVILEGE_SET_ALL_NECESSARY: u32 = 1u32;
pub const PROCESSOR_ALPHA_21064: u32 = 21064u32;
pub const PROCESSOR_AMD_X8664: u32 = 8664u32;
pub const PROCESSOR_ARM720: u32 = 1824u32;
pub const PROCESSOR_ARM820: u32 = 2080u32;
pub const PROCESSOR_ARM920: u32 = 2336u32;
pub const PROCESSOR_ARM_7TDMI: u32 = 70001u32;
pub const PROCESSOR_DUTY_CYCLING_DISABLED: u32 = 0u32;
pub const PROCESSOR_DUTY_CYCLING_ENABLED: u32 = 1u32;
pub const PROCESSOR_HITACHI_SH3: u32 = 10003u32;
pub const PROCESSOR_HITACHI_SH3E: u32 = 10004u32;
pub const PROCESSOR_HITACHI_SH4: u32 = 10005u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESSOR_IDLESTATE_INFO {
    pub TimeCheck: u32,
    pub DemotePercent: u8,
    pub PromotePercent: u8,
    pub Spare: [u8; 2],
}
impl Default for PROCESSOR_IDLESTATE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESSOR_IDLESTATE_POLICY {
    pub Revision: u16,
    pub Flags: PROCESSOR_IDLESTATE_POLICY_0,
    pub PolicyCount: u32,
    pub Policy: [PROCESSOR_IDLESTATE_INFO; 3],
}
impl Default for PROCESSOR_IDLESTATE_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESSOR_IDLESTATE_POLICY_0 {
    pub AsWORD: u16,
    pub Anonymous: PROCESSOR_IDLESTATE_POLICY_0_0,
}
impl Default for PROCESSOR_IDLESTATE_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESSOR_IDLESTATE_POLICY_0_0 {
    pub _bitfield: u16,
}
pub const PROCESSOR_IDLESTATE_POLICY_COUNT: u32 = 3u32;
pub const PROCESSOR_INTEL_386: u32 = 386u32;
pub const PROCESSOR_INTEL_486: u32 = 486u32;
pub const PROCESSOR_INTEL_IA64: u32 = 2200u32;
pub const PROCESSOR_INTEL_PENTIUM: u32 = 586u32;
pub const PROCESSOR_MIPS_R4000: u32 = 4000u32;
pub const PROCESSOR_MOTOROLA_821: u32 = 821u32;
pub const PROCESSOR_OPTIL: u32 = 18767u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESSOR_PERFSTATE_POLICY {
    pub Revision: u32,
    pub MaxThrottle: u8,
    pub MinThrottle: u8,
    pub BusyAdjThreshold: u8,
    pub Anonymous: PROCESSOR_PERFSTATE_POLICY_0,
    pub TimeCheck: u32,
    pub IncreaseTime: u32,
    pub DecreaseTime: u32,
    pub IncreasePercent: u32,
    pub DecreasePercent: u32,
}
impl Default for PROCESSOR_PERFSTATE_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESSOR_PERFSTATE_POLICY_0 {
    pub Spare: u8,
    pub Flags: PROCESSOR_PERFSTATE_POLICY_0_0,
}
impl Default for PROCESSOR_PERFSTATE_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESSOR_PERFSTATE_POLICY_0_0 {
    pub AsBYTE: u8,
    pub Anonymous: PROCESSOR_PERFSTATE_POLICY_0_0_0,
}
impl Default for PROCESSOR_PERFSTATE_POLICY_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESSOR_PERFSTATE_POLICY_0_0_0 {
    pub _bitfield: u8,
}
pub const PROCESSOR_PERF_AUTONOMOUS_MODE_DISABLED: u32 = 0u32;
pub const PROCESSOR_PERF_AUTONOMOUS_MODE_ENABLED: u32 = 1u32;
pub const PROCESSOR_PERF_BOOST_MODE_AGGRESSIVE: u32 = 2u32;
pub const PROCESSOR_PERF_BOOST_MODE_AGGRESSIVE_AT_GUARANTEED: u32 = 5u32;
pub const PROCESSOR_PERF_BOOST_MODE_DISABLED: u32 = 0u32;
pub const PROCESSOR_PERF_BOOST_MODE_EFFICIENT_AGGRESSIVE: u32 = 4u32;
pub const PROCESSOR_PERF_BOOST_MODE_EFFICIENT_AGGRESSIVE_AT_GUARANTEED: u32 = 6u32;
pub const PROCESSOR_PERF_BOOST_MODE_EFFICIENT_ENABLED: u32 = 3u32;
pub const PROCESSOR_PERF_BOOST_MODE_ENABLED: u32 = 1u32;
pub const PROCESSOR_PERF_BOOST_MODE_MAX: u32 = 6u32;
pub const PROCESSOR_PERF_BOOST_POLICY_DISABLED: u32 = 0u32;
pub const PROCESSOR_PERF_BOOST_POLICY_MAX: u32 = 100u32;
pub const PROCESSOR_PERF_ENERGY_PREFERENCE: u32 = 0u32;
pub const PROCESSOR_PERF_MAXIMUM_ACTIVITY_WINDOW: u32 = 1270000000u32;
pub const PROCESSOR_PERF_MINIMUM_ACTIVITY_WINDOW: u32 = 0u32;
pub const PROCESSOR_PERF_PERFORMANCE_PREFERENCE: u32 = 255u32;
pub const PROCESSOR_PPC_601: u32 = 601u32;
pub const PROCESSOR_PPC_603: u32 = 603u32;
pub const PROCESSOR_PPC_604: u32 = 604u32;
pub const PROCESSOR_PPC_620: u32 = 620u32;
pub const PROCESSOR_SHx_SH3: u32 = 103u32;
pub const PROCESSOR_SHx_SH4: u32 = 104u32;
pub const PROCESSOR_STRONGARM: u32 = 2577u32;
pub const PROCESSOR_THROTTLE_AUTOMATIC: u32 = 2u32;
pub const PROCESSOR_THROTTLE_DISABLED: u32 = 0u32;
pub const PROCESSOR_THROTTLE_ENABLED: u32 = 1u32;
pub const PROCESS_HEAP_ENTRY_BUSY: u32 = 4u32;
pub const PROCESS_HEAP_ENTRY_DDESHARE: u32 = 32u32;
pub const PROCESS_HEAP_ENTRY_MOVEABLE: u32 = 16u32;
pub const PROCESS_HEAP_REGION: u32 = 1u32;
pub const PROCESS_HEAP_SEG_ALLOC: u32 = 8u32;
pub const PROCESS_HEAP_UNCOMMITTED_RANGE: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_ACTIVATION_CONTEXT_TRUST_POLICY {
    pub Anonymous: PROCESS_MITIGATION_ACTIVATION_CONTEXT_TRUST_POLICY_0,
}
impl Default for PROCESS_MITIGATION_ACTIVATION_CONTEXT_TRUST_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_ACTIVATION_CONTEXT_TRUST_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_ACTIVATION_CONTEXT_TRUST_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_ACTIVATION_CONTEXT_TRUST_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_ACTIVATION_CONTEXT_TRUST_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_ASLR_POLICY {
    pub Anonymous: PROCESS_MITIGATION_ASLR_POLICY_0,
}
impl Default for PROCESS_MITIGATION_ASLR_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_ASLR_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_ASLR_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_ASLR_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_ASLR_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY {
    pub Anonymous: PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY_0,
}
impl Default for PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_CHILD_PROCESS_POLICY {
    pub Anonymous: PROCESS_MITIGATION_CHILD_PROCESS_POLICY_0,
}
impl Default for PROCESS_MITIGATION_CHILD_PROCESS_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_CHILD_PROCESS_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_CHILD_PROCESS_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_CHILD_PROCESS_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_CHILD_PROCESS_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY {
    pub Anonymous: PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY_0,
}
impl Default for PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_DEP_POLICY {
    pub Anonymous: PROCESS_MITIGATION_DEP_POLICY_0,
    pub Permanent: bool,
}
impl Default for PROCESS_MITIGATION_DEP_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_DEP_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_DEP_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_DEP_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_DEP_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_DYNAMIC_CODE_POLICY {
    pub Anonymous: PROCESS_MITIGATION_DYNAMIC_CODE_POLICY_0,
}
impl Default for PROCESS_MITIGATION_DYNAMIC_CODE_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_DYNAMIC_CODE_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_DYNAMIC_CODE_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_DYNAMIC_CODE_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_DYNAMIC_CODE_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY {
    pub Anonymous: PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY_0,
}
impl Default for PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_FONT_DISABLE_POLICY {
    pub Anonymous: PROCESS_MITIGATION_FONT_DISABLE_POLICY_0,
}
impl Default for PROCESS_MITIGATION_FONT_DISABLE_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_FONT_DISABLE_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_FONT_DISABLE_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_FONT_DISABLE_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_FONT_DISABLE_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_IMAGE_LOAD_POLICY {
    pub Anonymous: PROCESS_MITIGATION_IMAGE_LOAD_POLICY_0,
}
impl Default for PROCESS_MITIGATION_IMAGE_LOAD_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_IMAGE_LOAD_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_IMAGE_LOAD_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_IMAGE_LOAD_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_IMAGE_LOAD_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY {
    pub Anonymous: PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY_0,
}
impl Default for PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY {
    pub Anonymous: PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY_0,
}
impl Default for PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_SEHOP_POLICY {
    pub Anonymous: PROCESS_MITIGATION_SEHOP_POLICY_0,
}
impl Default for PROCESS_MITIGATION_SEHOP_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_SEHOP_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_SEHOP_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_SEHOP_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_SEHOP_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY {
    pub Anonymous: PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY_0,
}
impl Default for PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY {
    pub Anonymous: PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY_0,
}
impl Default for PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY {
    pub Anonymous: PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY_0,
}
impl Default for PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY {
    pub Anonymous: PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY_0,
}
impl Default for PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY {
    pub Anonymous: PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY_0,
}
impl Default for PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY {
    pub Anonymous: PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY_0,
}
impl Default for PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY_0_0 {
    pub _bitfield: u32,
}
pub const PROCESS_TRUST_LABEL_SECURITY_INFORMATION: i32 = 128i32;
pub const PROC_IDLE_BUCKET_COUNT: u32 = 6u32;
pub const PROC_IDLE_BUCKET_COUNT_EX: u32 = 16u32;
pub const PRODUCT_ARM64_SERVER: u32 = 120u32;
pub const PRODUCT_AZURESTACKHCI_SERVER_CORE: u32 = 406u32;
pub const PRODUCT_AZURE_NANO_SERVER: u32 = 169u32;
pub const PRODUCT_AZURE_SERVER_CLOUDHOST: u32 = 199u32;
pub const PRODUCT_AZURE_SERVER_CLOUDMOS: u32 = 200u32;
pub const PRODUCT_AZURE_SERVER_CORE: u32 = 168u32;
pub const PRODUCT_CLOUD: u32 = 178u32;
pub const PRODUCT_CLOUDE: u32 = 183u32;
pub const PRODUCT_CLOUDEDITION: u32 = 203u32;
pub const PRODUCT_CLOUDEDITIONN: u32 = 202u32;
pub const PRODUCT_CLOUDEN: u32 = 186u32;
pub const PRODUCT_CLOUDN: u32 = 179u32;
pub const PRODUCT_CLOUD_HOST_INFRASTRUCTURE_SERVER: u32 = 124u32;
pub const PRODUCT_CLOUD_STORAGE_SERVER: u32 = 110u32;
pub const PRODUCT_CONNECTED_CAR: u32 = 117u32;
pub const PRODUCT_CORE_ARM: u32 = 97u32;
pub const PRODUCT_CORE_CONNECTED: u32 = 111u32;
pub const PRODUCT_CORE_CONNECTED_COUNTRYSPECIFIC: u32 = 116u32;
pub const PRODUCT_CORE_CONNECTED_N: u32 = 113u32;
pub const PRODUCT_CORE_CONNECTED_SINGLELANGUAGE: u32 = 115u32;
pub const PRODUCT_DATACENTER_EVALUATION_SERVER_CORE: u32 = 159u32;
pub const PRODUCT_DATACENTER_NANO_SERVER: u32 = 143u32;
pub const PRODUCT_DATACENTER_SERVER_AZURE_EDITION: u32 = 407u32;
pub const PRODUCT_DATACENTER_SERVER_CORE_AZURE_EDITION: u32 = 408u32;
pub const PRODUCT_DATACENTER_WS_SERVER_CORE: u32 = 147u32;
pub const PRODUCT_EMBEDDED: u32 = 65u32;
pub const PRODUCT_EMBEDDED_A: u32 = 88u32;
pub const PRODUCT_EMBEDDED_AUTOMOTIVE: u32 = 85u32;
pub const PRODUCT_EMBEDDED_E: u32 = 90u32;
pub const PRODUCT_EMBEDDED_EVAL: u32 = 107u32;
pub const PRODUCT_EMBEDDED_E_EVAL: u32 = 108u32;
pub const PRODUCT_EMBEDDED_INDUSTRY: u32 = 89u32;
pub const PRODUCT_EMBEDDED_INDUSTRY_A: u32 = 86u32;
pub const PRODUCT_EMBEDDED_INDUSTRY_A_E: u32 = 92u32;
pub const PRODUCT_EMBEDDED_INDUSTRY_E: u32 = 91u32;
pub const PRODUCT_EMBEDDED_INDUSTRY_EVAL: u32 = 105u32;
pub const PRODUCT_EMBEDDED_INDUSTRY_E_EVAL: u32 = 106u32;
pub const PRODUCT_ENTERPRISEG: u32 = 171u32;
pub const PRODUCT_ENTERPRISEGN: u32 = 172u32;
pub const PRODUCT_ENTERPRISE_SUBSCRIPTION: u32 = 140u32;
pub const PRODUCT_ENTERPRISE_SUBSCRIPTION_N: u32 = 141u32;
pub const PRODUCT_HOLOGRAPHIC: u32 = 135u32;
pub const PRODUCT_HOLOGRAPHIC_BUSINESS: u32 = 136u32;
pub const PRODUCT_HUBOS: u32 = 180u32;
pub const PRODUCT_INDUSTRY_HANDHELD: u32 = 118u32;
pub const PRODUCT_IOTEDGEOS: u32 = 187u32;
pub const PRODUCT_IOTENTERPRISE: u32 = 188u32;
pub const PRODUCT_IOTENTERPRISES: u32 = 191u32;
pub const PRODUCT_IOTOS: u32 = 185u32;
pub const PRODUCT_LITE: u32 = 189u32;
pub const PRODUCT_NANO_SERVER: u32 = 109u32;
pub const PRODUCT_ONECOREUPDATEOS: u32 = 182u32;
pub const PRODUCT_PPI_PRO: u32 = 119u32;
pub const PRODUCT_PROFESSIONAL_EMBEDDED: u32 = 58u32;
pub const PRODUCT_PROFESSIONAL_S: u32 = 127u32;
pub const PRODUCT_PROFESSIONAL_STUDENT: u32 = 112u32;
pub const PRODUCT_PROFESSIONAL_STUDENT_N: u32 = 114u32;
pub const PRODUCT_PROFESSIONAL_S_N: u32 = 128u32;
pub const PRODUCT_PRO_CHINA: u32 = 139u32;
pub const PRODUCT_PRO_FOR_EDUCATION: u32 = 164u32;
pub const PRODUCT_PRO_FOR_EDUCATION_N: u32 = 165u32;
pub const PRODUCT_PRO_SINGLE_LANGUAGE: u32 = 138u32;
pub const PRODUCT_SERVERRDSH: u32 = 175u32;
pub const PRODUCT_SOLUTION_EMBEDDEDSERVER_CORE: u32 = 57u32;
pub const PRODUCT_STANDARD_EVALUATION_SERVER_CORE: u32 = 160u32;
pub const PRODUCT_STANDARD_NANO_SERVER: u32 = 144u32;
pub const PRODUCT_STANDARD_SERVER_CORE: u32 = 13u32;
pub const PRODUCT_STANDARD_WS_SERVER_CORE: u32 = 148u32;
pub const PRODUCT_THINPC: u32 = 87u32;
pub const PRODUCT_UNLICENSED: u32 = 2882382797u32;
pub const PRODUCT_UTILITY_VM: u32 = 149u32;
pub const PRODUCT_XBOX_DURANGOHOSTOS: u32 = 196u32;
pub const PRODUCT_XBOX_ERAOS: u32 = 195u32;
pub const PRODUCT_XBOX_GAMEOS: u32 = 194u32;
pub const PRODUCT_XBOX_KEYSTONE: u32 = 198u32;
pub const PRODUCT_XBOX_SCARLETTHOSTOS: u32 = 197u32;
pub const PRODUCT_XBOX_SYSTEMOS: u32 = 192u32;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PTERMINATION_HANDLER = Option<unsafe extern "system" fn(_abnormal_termination: bool, establisherframe: *mut core::ffi::c_void)>;
#[cfg(target_arch = "aarch64")]
pub type PTERMINATION_HANDLER = Option<unsafe extern "system" fn(_abnormal_termination: bool, establisherframe: u64)>;
pub type PUMS_SCHEDULER_ENTRY_POINT = Option<unsafe extern "system" fn(reason: RTL_UMS_SCHEDULER_REASON, activationpayload: usize, schedulerparam: *const core::ffi::c_void)>;
pub const PcTeb: u32 = 24u32;
pub const PdataCrChained: ARM64_FNPDATA_CR = ARM64_FNPDATA_CR(3i32);
pub const PdataCrChainedWithPac: ARM64_FNPDATA_CR = ARM64_FNPDATA_CR(2i32);
pub const PdataCrUnchained: ARM64_FNPDATA_CR = ARM64_FNPDATA_CR(0i32);
pub const PdataCrUnchainedSavedLr: ARM64_FNPDATA_CR = ARM64_FNPDATA_CR(1i32);
pub const PdataPackedUnwindFragment: ARM64_FNPDATA_FLAGS = ARM64_FNPDATA_FLAGS(2i32);
pub const PdataPackedUnwindFunction: ARM64_FNPDATA_FLAGS = ARM64_FNPDATA_FLAGS(1i32);
pub const PdataRefToFullXdata: ARM64_FNPDATA_FLAGS = ARM64_FNPDATA_FLAGS(0i32);
pub const PowerMonitorDim: MONITOR_DISPLAY_STATE = MONITOR_DISPLAY_STATE(2i32);
pub const PowerMonitorOff: MONITOR_DISPLAY_STATE = MONITOR_DISPLAY_STATE(0i32);
pub const PowerMonitorOn: MONITOR_DISPLAY_STATE = MONITOR_DISPLAY_STATE(1i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct QUOTA_LIMITS_EX {
    pub PagedPoolLimit: usize,
    pub NonPagedPoolLimit: usize,
    pub MinimumWorkingSetSize: usize,
    pub MaximumWorkingSetSize: usize,
    pub PagefileLimit: usize,
    pub TimeLimit: i64,
    pub WorkingSetLimit: usize,
    pub Reserved2: usize,
    pub Reserved3: usize,
    pub Reserved4: usize,
    pub Flags: u32,
    pub CpuRateLimit: RATE_QUOTA_LIMIT,
}
impl Default for QUOTA_LIMITS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const QUOTA_LIMITS_USE_DEFAULT_LIMITS: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union RATE_QUOTA_LIMIT {
    pub RateData: u32,
    pub Anonymous: RATE_QUOTA_LIMIT_0,
}
impl Default for RATE_QUOTA_LIMIT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RATE_QUOTA_LIMIT_0 {
    pub _bitfield: u32,
}
pub const READ_THREAD_PROFILING_FLAG_DISPATCHING: u32 = 1u32;
pub const READ_THREAD_PROFILING_FLAG_HARDWARE_COUNTERS: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REARRANGE_FILE_DATA {
    pub SourceStartingOffset: u64,
    pub TargetOffset: u64,
    pub SourceFileHandle: super::super::Foundation::HANDLE,
    pub Length: u32,
    pub Flags: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REARRANGE_FILE_DATA32 {
    pub SourceStartingOffset: u64,
    pub TargetOffset: u64,
    pub SourceFileHandle: u32,
    pub Length: u32,
    pub Flags: u32,
}
pub const RECO_COPY: RECO_FLAGS = RECO_FLAGS(2u32);
pub const RECO_CUT: RECO_FLAGS = RECO_FLAGS(3u32);
pub const RECO_DRAG: RECO_FLAGS = RECO_FLAGS(4u32);
pub const RECO_DROP: RECO_FLAGS = RECO_FLAGS(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RECO_FLAGS(pub u32);
impl RECO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for RECO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for RECO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for RECO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for RECO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for RECO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const RECO_PASTE: RECO_FLAGS = RECO_FLAGS(0u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REDBOOK_DIGITAL_AUDIO_EXTRACTION_INFO {
    pub Version: u32,
    pub Accurate: u32,
    pub Supported: u32,
    pub AccurateMask0: u32,
}
pub const REDBOOK_DIGITAL_AUDIO_EXTRACTION_INFO_VERSION: u32 = 1u32;
pub const REG_APP_HIVE: i32 = 16i32;
pub const REG_APP_HIVE_OPEN_READ_ONLY: i32 = 8192i32;
pub const REG_BOOT_HIVE: i32 = 1024i32;
pub const REG_FLUSH_HIVE_FILE_GROWTH: i32 = 4096i32;
pub const REG_FORCE_UNLOAD: u32 = 1u32;
pub const REG_HIVE_EXACT_FILE_GROWTH: i32 = 128i32;
pub const REG_HIVE_NO_RM: i32 = 256i32;
pub const REG_HIVE_SINGLE_LOG: i32 = 512i32;
pub const REG_IMMUTABLE: i32 = 16384i32;
pub const REG_LOAD_HIVE_OPEN_HANDLE: i32 = 2048i32;
pub const REG_NO_IMPERSONATION_FALLBACK: i32 = 32768i32;
pub const REG_NO_LAZY_FLUSH: i32 = 4i32;
pub const REG_OPEN_READ_ONLY: i32 = 8192i32;
pub const REG_PROCESS_PRIVATE: i32 = 32i32;
pub const REG_REFRESH_HIVE: i32 = 2i32;
pub const REG_START_JOURNAL: i32 = 64i32;
pub const REG_UNLOAD_LEGAL_FLAGS: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RESOURCEMANAGER_BASIC_INFORMATION {
    pub ResourceManagerId: windows_core::GUID,
    pub DescriptionLength: u32,
    pub Description: [u16; 1],
}
impl Default for RESOURCEMANAGER_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RESOURCEMANAGER_COMPLETE_PROPAGATION: u32 = 64u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RESOURCEMANAGER_COMPLETION_INFORMATION {
    pub IoCompletionPortHandle: super::super::Foundation::HANDLE,
    pub CompletionKey: usize,
}
pub const RESOURCEMANAGER_ENLIST: u32 = 8u32;
pub const RESOURCEMANAGER_GET_NOTIFICATION: u32 = 16u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RESOURCEMANAGER_INFORMATION_CLASS(pub i32);
pub const RESOURCEMANAGER_QUERY_INFORMATION: u32 = 1u32;
pub const RESOURCEMANAGER_RECOVER: u32 = 4u32;
pub const RESOURCEMANAGER_REGISTER_PROTOCOL: u32 = 32u32;
pub const RESOURCEMANAGER_SET_INFORMATION: u32 = 2u32;
pub const ROT_COMPARE_MAX: u32 = 2048u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTL_UMS_SCHEDULER_REASON(pub i32);
pub const RTL_UMS_VERSION: u32 = 256u32;
pub const RTL_VIRTUAL_UNWIND2_VALIDATE_PAC: u32 = 1u32;
pub const RUNTIME_FUNCTION_INDIRECT: u32 = 1u32;
pub const RecognizerType: SERVICE_NODE_TYPE = SERVICE_NODE_TYPE(8i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RemHBITMAP {
    pub cbData: u32,
    pub data: [u8; 1],
}
impl Default for RemHBITMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RemHBRUSH {
    pub cbData: u32,
    pub data: [u8; 1],
}
impl Default for RemHBRUSH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RemHENHMETAFILE {
    pub cbData: u32,
    pub data: [u8; 1],
}
impl Default for RemHENHMETAFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RemHGLOBAL {
    pub fNullHGlobal: i32,
    pub cbData: u32,
    pub data: [u8; 1],
}
impl Default for RemHGLOBAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RemHMETAFILEPICT {
    pub mm: i32,
    pub xExt: i32,
    pub yExt: i32,
    pub cbData: u32,
    pub data: [u8; 1],
}
impl Default for RemHMETAFILEPICT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RemHPALETTE {
    pub cbData: u32,
    pub data: [u8; 1],
}
impl Default for RemHPALETTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RemotableHandle {
    pub fContext: i32,
    pub u: RemotableHandle_0,
}
impl Default for RemotableHandle {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RemotableHandle_0 {
    pub hInproc: i32,
    pub hRemote: i32,
}
impl Default for RemotableHandle_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ReplacesCorHdrNumericDefines(pub i32);
pub const ResourceManagerBasicInformation: RESOURCEMANAGER_INFORMATION_CLASS = RESOURCEMANAGER_INFORMATION_CLASS(0i32);
pub const ResourceManagerCompletionInformation: RESOURCEMANAGER_INFORMATION_CLASS = RESOURCEMANAGER_INFORMATION_CLASS(1i32);
pub const RunlevelInformationInActivationContext: ACTIVATION_CONTEXT_INFO_CLASS = ACTIVATION_CONTEXT_INFO_CLASS(5i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCOPE_TABLE_AMD64 {
    pub Count: u32,
    pub ScopeRecord: [SCOPE_TABLE_AMD64_0; 1],
}
impl Default for SCOPE_TABLE_AMD64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCOPE_TABLE_AMD64_0 {
    pub BeginAddress: u32,
    pub EndAddress: u32,
    pub HandlerAddress: u32,
    pub JumpTarget: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCOPE_TABLE_ARM {
    pub Count: u32,
    pub ScopeRecord: [SCOPE_TABLE_ARM_0; 1],
}
impl Default for SCOPE_TABLE_ARM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCOPE_TABLE_ARM_0 {
    pub BeginAddress: u32,
    pub EndAddress: u32,
    pub HandlerAddress: u32,
    pub JumpTarget: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCOPE_TABLE_ARM64 {
    pub Count: u32,
    pub ScopeRecord: [SCOPE_TABLE_ARM64_0; 1],
}
impl Default for SCOPE_TABLE_ARM64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCOPE_TABLE_ARM64_0 {
    pub BeginAddress: u32,
    pub EndAddress: u32,
    pub HandlerAddress: u32,
    pub JumpTarget: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCRUB_DATA_INPUT {
    pub Size: u32,
    pub Flags: u32,
    pub MaximumIos: u32,
    pub ObjectId: [u32; 4],
    pub Reserved: [u32; 41],
    pub ResumeContext: [u8; 1040],
}
impl Default for SCRUB_DATA_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCRUB_DATA_INPUT_FLAG_IGNORE_REDUNDANCY: u32 = 8u32;
pub const SCRUB_DATA_INPUT_FLAG_OPLOCK_NOT_ACQUIRED: u32 = 64u32;
pub const SCRUB_DATA_INPUT_FLAG_RESUME: u32 = 1u32;
pub const SCRUB_DATA_INPUT_FLAG_SCRUB_BY_OBJECT_ID: u32 = 32u32;
pub const SCRUB_DATA_INPUT_FLAG_SKIP_DATA: u32 = 16u32;
pub const SCRUB_DATA_INPUT_FLAG_SKIP_IN_SYNC: u32 = 2u32;
pub const SCRUB_DATA_INPUT_FLAG_SKIP_NON_INTEGRITY_DATA: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCRUB_DATA_OUTPUT {
    pub Size: u32,
    pub Flags: u32,
    pub Status: u32,
    pub ErrorFileOffset: u64,
    pub ErrorLength: u64,
    pub NumberOfBytesRepaired: u64,
    pub NumberOfBytesFailed: u64,
    pub InternalFileReference: u64,
    pub ResumeContextLength: u16,
    pub ParityExtentDataOffset: u16,
    pub Reserved: [u32; 9],
    pub NumberOfMetadataBytesProcessed: u64,
    pub NumberOfDataBytesProcessed: u64,
    pub TotalNumberOfMetadataBytesInUse: u64,
    pub TotalNumberOfDataBytesInUse: u64,
    pub DataBytesSkippedDueToNoAllocation: u64,
    pub DataBytesSkippedDueToInvalidRun: u64,
    pub DataBytesSkippedDueToIntegrityStream: u64,
    pub DataBytesSkippedDueToRegionBeingClean: u64,
    pub DataBytesSkippedDueToLockConflict: u64,
    pub DataBytesSkippedDueToNoScrubDataFlag: u64,
    pub DataBytesSkippedDueToNoScrubNonIntegrityStreamFlag: u64,
    pub DataBytesScrubbed: u64,
    pub ResumeContext: [u8; 1040],
}
impl Default for SCRUB_DATA_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCRUB_DATA_OUTPUT_FLAG_INCOMPLETE: u32 = 1u32;
pub const SCRUB_DATA_OUTPUT_FLAG_NON_USER_DATA_RANGE: u32 = 65536u32;
pub const SCRUB_DATA_OUTPUT_FLAG_PARITY_EXTENT_DATA_RETURNED: u32 = 131072u32;
pub const SCRUB_DATA_OUTPUT_FLAG_RESUME_CONTEXT_LENGTH_SPECIFIED: u32 = 262144u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCRUB_PARITY_EXTENT {
    pub Offset: i64,
    pub Length: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCRUB_PARITY_EXTENT_DATA {
    pub Size: u16,
    pub Flags: u16,
    pub NumberOfParityExtents: u16,
    pub MaximumNumberOfParityExtents: u16,
    pub ParityExtents: [SCRUB_PARITY_EXTENT; 1],
}
impl Default for SCRUB_PARITY_EXTENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SECURITY_ANONYMOUS_LOGON_RID: i32 = 7i32;
pub const SECURITY_APPPOOL_ID_BASE_RID: i32 = 82i32;
pub const SECURITY_APPPOOL_ID_RID_COUNT: i32 = 6i32;
pub const SECURITY_APP_PACKAGE_BASE_RID: i32 = 2i32;
pub const SECURITY_APP_PACKAGE_RID_COUNT: i32 = 8i32;
pub const SECURITY_AUTHENTICATED_USER_RID: i32 = 11i32;
pub const SECURITY_AUTHENTICATION_AUTHORITY_ASSERTED_RID: i32 = 1i32;
pub const SECURITY_AUTHENTICATION_AUTHORITY_RID_COUNT: i32 = 1i32;
pub const SECURITY_AUTHENTICATION_FRESH_KEY_AUTH_RID: i32 = 3i32;
pub const SECURITY_AUTHENTICATION_KEY_PROPERTY_ATTESTATION_RID: i32 = 6i32;
pub const SECURITY_AUTHENTICATION_KEY_PROPERTY_MFA_RID: i32 = 5i32;
pub const SECURITY_AUTHENTICATION_KEY_TRUST_RID: i32 = 4i32;
pub const SECURITY_AUTHENTICATION_SERVICE_ASSERTED_RID: i32 = 2i32;
pub const SECURITY_BATCH_RID: i32 = 3i32;
pub const SECURITY_BUILTIN_APP_PACKAGE_RID_COUNT: i32 = 2i32;
pub const SECURITY_BUILTIN_CAPABILITY_RID_COUNT: i32 = 2i32;
pub const SECURITY_BUILTIN_DOMAIN_RID: i32 = 32i32;
pub const SECURITY_BUILTIN_PACKAGE_ANY_PACKAGE: i32 = 1i32;
pub const SECURITY_BUILTIN_PACKAGE_ANY_RESTRICTED_PACKAGE: i32 = 2i32;
pub const SECURITY_CAPABILITY_APPOINTMENTS: i32 = 11i32;
pub const SECURITY_CAPABILITY_APP_RID: i32 = 1024i32;
pub const SECURITY_CAPABILITY_APP_SILO_RID: i32 = 65536i32;
pub const SECURITY_CAPABILITY_BASE_RID: i32 = 3i32;
pub const SECURITY_CAPABILITY_CONTACTS: i32 = 12i32;
pub const SECURITY_CAPABILITY_DOCUMENTS_LIBRARY: i32 = 7i32;
pub const SECURITY_CAPABILITY_ENTERPRISE_AUTHENTICATION: i32 = 8i32;
pub const SECURITY_CAPABILITY_INTERNET_CLIENT: i32 = 1i32;
pub const SECURITY_CAPABILITY_INTERNET_CLIENT_SERVER: i32 = 2i32;
pub const SECURITY_CAPABILITY_INTERNET_EXPLORER: i32 = 4096i32;
pub const SECURITY_CAPABILITY_MUSIC_LIBRARY: i32 = 6i32;
pub const SECURITY_CAPABILITY_PICTURES_LIBRARY: i32 = 4i32;
pub const SECURITY_CAPABILITY_PRIVATE_NETWORK_CLIENT_SERVER: i32 = 3i32;
pub const SECURITY_CAPABILITY_REMOVABLE_STORAGE: i32 = 10i32;
pub const SECURITY_CAPABILITY_RID_COUNT: i32 = 5i32;
pub const SECURITY_CAPABILITY_SHARED_USER_CERTIFICATES: i32 = 9i32;
pub const SECURITY_CAPABILITY_VIDEOS_LIBRARY: i32 = 5i32;
pub const SECURITY_CCG_ID_BASE_RID: i32 = 95i32;
pub const SECURITY_CHILD_PACKAGE_RID_COUNT: i32 = 12i32;
pub const SECURITY_CLOUD_INFRASTRUCTURE_SERVICES_ID_BASE_RID: i32 = 85i32;
pub const SECURITY_CLOUD_INFRASTRUCTURE_SERVICES_ID_RID_COUNT: i32 = 6i32;
pub const SECURITY_COM_ID_BASE_RID: i32 = 89i32;
pub const SECURITY_CREATOR_GROUP_RID: i32 = 1i32;
pub const SECURITY_CREATOR_GROUP_SERVER_RID: i32 = 3i32;
pub const SECURITY_CREATOR_OWNER_RID: i32 = 0i32;
pub const SECURITY_CREATOR_OWNER_RIGHTS_RID: i32 = 4i32;
pub const SECURITY_CREATOR_OWNER_SERVER_RID: i32 = 2i32;
pub const SECURITY_CRED_TYPE_BASE_RID: i32 = 65i32;
pub const SECURITY_CRED_TYPE_RID_COUNT: i32 = 2i32;
pub const SECURITY_CRED_TYPE_THIS_ORG_CERT_RID: i32 = 1i32;
pub const SECURITY_DASHOST_ID_BASE_RID: i32 = 92i32;
pub const SECURITY_DASHOST_ID_RID_COUNT: i32 = 6i32;
pub const SECURITY_DESCRIPTOR_REVISION: u32 = 1u32;
pub const SECURITY_DESCRIPTOR_REVISION1: u32 = 1u32;
pub const SECURITY_DIALUP_RID: i32 = 1i32;
pub const SECURITY_ENTERPRISE_CONTROLLERS_RID: i32 = 9i32;
pub const SECURITY_ENTERPRISE_READONLY_CONTROLLERS_RID: i32 = 22i32;
pub const SECURITY_INSTALLER_CAPABILITY_RID_COUNT: u32 = 10u32;
pub const SECURITY_INSTALLER_GROUP_CAPABILITY_BASE: u32 = 32u32;
pub const SECURITY_INSTALLER_GROUP_CAPABILITY_RID_COUNT: u32 = 9u32;
pub const SECURITY_INTERACTIVE_RID: i32 = 4i32;
pub const SECURITY_IUSER_RID: i32 = 17i32;
pub const SECURITY_LOCAL_ACCOUNT_AND_ADMIN_RID: i32 = 114i32;
pub const SECURITY_LOCAL_ACCOUNT_RID: i32 = 113i32;
pub const SECURITY_LOCAL_LOGON_RID: i32 = 1i32;
pub const SECURITY_LOCAL_RID: i32 = 0i32;
pub const SECURITY_LOCAL_SERVICE_RID: i32 = 19i32;
pub const SECURITY_LOCAL_SYSTEM_RID: i32 = 18i32;
pub const SECURITY_LOGON_IDS_RID: i32 = 5i32;
pub const SECURITY_LOGON_IDS_RID_COUNT: i32 = 3i32;
pub const SECURITY_MANDATORY_HIGH_RID: i32 = 12288i32;
pub const SECURITY_MANDATORY_LOW_RID: i32 = 4096i32;
pub const SECURITY_MANDATORY_MAXIMUM_USER_RID: i32 = 16384i32;
pub const SECURITY_MANDATORY_MEDIUM_PLUS_RID: u32 = 8448u32;
pub const SECURITY_MANDATORY_MEDIUM_RID: i32 = 8192i32;
pub const SECURITY_MANDATORY_PROTECTED_PROCESS_RID: i32 = 20480i32;
pub const SECURITY_MANDATORY_SYSTEM_RID: i32 = 16384i32;
pub const SECURITY_MANDATORY_UNTRUSTED_RID: i32 = 0i32;
pub const SECURITY_MAX_ALWAYS_FILTERED: i32 = 999i32;
pub const SECURITY_MAX_BASE_RID: i32 = 111i32;
pub const SECURITY_MIN_BASE_RID: i32 = 80i32;
pub const SECURITY_MIN_NEVER_FILTERED: i32 = 1000i32;
pub const SECURITY_NETWORK_RID: i32 = 2i32;
pub const SECURITY_NETWORK_SERVICE_RID: i32 = 20i32;
pub const SECURITY_NFS_ID_BASE_RID: i32 = 88i32;
pub const SECURITY_NT_NON_UNIQUE: i32 = 21i32;
pub const SECURITY_NT_NON_UNIQUE_SUB_AUTH_COUNT: i32 = 3i32;
pub const SECURITY_NULL_RID: i32 = 0i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECURITY_OBJECT_AI_PARAMS {
    pub Size: u32,
    pub ConstraintMask: u32,
}
pub const SECURITY_OTHER_ORGANIZATION_RID: i32 = 1000i32;
pub const SECURITY_PACKAGE_BASE_RID: i32 = 64i32;
pub const SECURITY_PACKAGE_DIGEST_RID: i32 = 21i32;
pub const SECURITY_PACKAGE_NTLM_RID: i32 = 10i32;
pub const SECURITY_PACKAGE_RID_COUNT: i32 = 2i32;
pub const SECURITY_PACKAGE_SCHANNEL_RID: i32 = 14i32;
pub const SECURITY_PARENT_PACKAGE_RID_COUNT: i32 = 8i32;
pub const SECURITY_PRINCIPAL_SELF_RID: i32 = 10i32;
pub const SECURITY_PROCESS_PROTECTION_LEVEL_ANTIMALWARE_RID: i32 = 1536i32;
pub const SECURITY_PROCESS_PROTECTION_LEVEL_APP_RID: i32 = 2048i32;
pub const SECURITY_PROCESS_PROTECTION_LEVEL_AUTHENTICODE_RID: i32 = 1024i32;
pub const SECURITY_PROCESS_PROTECTION_LEVEL_NONE_RID: i32 = 0i32;
pub const SECURITY_PROCESS_PROTECTION_LEVEL_WINDOWS_RID: i32 = 4096i32;
pub const SECURITY_PROCESS_PROTECTION_LEVEL_WINTCB_RID: i32 = 8192i32;
pub const SECURITY_PROCESS_PROTECTION_TYPE_FULL_RID: i32 = 1024i32;
pub const SECURITY_PROCESS_PROTECTION_TYPE_LITE_RID: i32 = 512i32;
pub const SECURITY_PROCESS_PROTECTION_TYPE_NONE_RID: i32 = 0i32;
pub const SECURITY_PROCESS_TRUST_AUTHORITY_RID_COUNT: i32 = 2i32;
pub const SECURITY_PROXY_RID: i32 = 8i32;
pub const SECURITY_RDV_GFX_BASE_RID: i32 = 91i32;
pub const SECURITY_REMOTE_LOGON_RID: i32 = 14i32;
pub const SECURITY_RESERVED_ID_BASE_RID: i32 = 81i32;
pub const SECURITY_RESTRICTED_CODE_RID: i32 = 12i32;
pub const SECURITY_SERVER_LOGON_RID: i32 = 9i32;
pub const SECURITY_SERVICE_ID_BASE_RID: i32 = 80i32;
pub const SECURITY_SERVICE_ID_RID_COUNT: i32 = 6i32;
pub const SECURITY_SERVICE_RID: i32 = 6i32;
pub const SECURITY_TASK_ID_BASE_RID: i32 = 87i32;
pub const SECURITY_TERMINAL_SERVER_RID: i32 = 13i32;
pub const SECURITY_THIS_ORGANIZATION_RID: i32 = 15i32;
pub const SECURITY_TRUSTED_INSTALLER_RID1: u32 = 956008885u32;
pub const SECURITY_TRUSTED_INSTALLER_RID2: u32 = 3418522649u32;
pub const SECURITY_TRUSTED_INSTALLER_RID3: u32 = 1831038044u32;
pub const SECURITY_TRUSTED_INSTALLER_RID4: u32 = 1853292631u32;
pub const SECURITY_TRUSTED_INSTALLER_RID5: u32 = 2271478464u32;
pub const SECURITY_UMFD_BASE_RID: i32 = 96i32;
pub const SECURITY_USERMANAGER_ID_BASE_RID: i32 = 93i32;
pub const SECURITY_USERMANAGER_ID_RID_COUNT: i32 = 6i32;
pub const SECURITY_USERMODEDRIVERHOST_ID_BASE_RID: i32 = 84i32;
pub const SECURITY_USERMODEDRIVERHOST_ID_RID_COUNT: i32 = 6i32;
pub const SECURITY_VIRTUALACCOUNT_ID_RID_COUNT: i32 = 6i32;
pub const SECURITY_VIRTUALSERVER_ID_BASE_RID: i32 = 83i32;
pub const SECURITY_VIRTUALSERVER_ID_RID_COUNT: i32 = 6i32;
pub const SECURITY_WINDOWSMOBILE_ID_BASE_RID: i32 = 112i32;
pub const SECURITY_WINDOW_MANAGER_BASE_RID: i32 = 90i32;
pub const SECURITY_WINRM_ID_BASE_RID: i32 = 94i32;
pub const SECURITY_WINRM_ID_RID_COUNT: i32 = 6i32;
pub const SECURITY_WMIHOST_ID_BASE_RID: i32 = 86i32;
pub const SECURITY_WMIHOST_ID_RID_COUNT: i32 = 6i32;
pub const SECURITY_WORLD_RID: i32 = 0i32;
pub const SECURITY_WRITE_RESTRICTED_CODE_RID: i32 = 33i32;
pub const SEC_HUGE_PAGES: u32 = 131072u32;
pub const SEF_AI_USE_EXTRA_PARAMS: u32 = 2048u32;
pub const SEF_FORCE_USER_MODE: u32 = 8192u32;
pub const SEF_NORMALIZE_OUTPUT_DESCRIPTOR: u32 = 16384u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SERVERSILO_BASIC_INFORMATION {
    pub ServiceSessionId: u32,
    pub State: SERVERSILO_STATE,
    pub ExitStatus: u32,
    pub IsDownlevelContainer: bool,
    pub ApiSetSchema: *mut core::ffi::c_void,
    pub HostApiSetSchema: *mut core::ffi::c_void,
}
impl Default for SERVERSILO_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SERVERSILO_INITING: SERVERSILO_STATE = SERVERSILO_STATE(0i32);
pub const SERVERSILO_SHUTTING_DOWN: SERVERSILO_STATE = SERVERSILO_STATE(2i32);
pub const SERVERSILO_STARTED: SERVERSILO_STATE = SERVERSILO_STATE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVERSILO_STATE(pub i32);
pub const SERVERSILO_TERMINATED: SERVERSILO_STATE = SERVERSILO_STATE(4i32);
pub const SERVERSILO_TERMINATING: SERVERSILO_STATE = SERVERSILO_STATE(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVICE_ERROR_TYPE(pub i32);
pub const SERVICE_INTERACTIVE_PROCESS: u32 = 256u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVICE_LOAD_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SERVICE_NODE_TYPE(pub i32);
pub const SERVICE_PKG_SERVICE: u32 = 512u32;
pub const SERVICE_USERSERVICE_INSTANCE: u32 = 128u32;
pub const SERVICE_USER_SERVICE: u32 = 64u32;
pub const SESSION_MODIFY_ACCESS: u32 = 2u32;
pub const SESSION_QUERY_ACCESS: u32 = 1u32;
pub const SE_ACCESS_CHECK_FLAG_NO_LEARNING_MODE_LOGGING: u32 = 8u32;
pub const SE_ACCESS_CHECK_VALID_FLAGS: u32 = 8u32;
pub const SE_ACTIVATE_AS_USER_CAPABILITY: windows_core::PCWSTR = windows_core::w!("activateAsUser");
pub const SE_APP_SILO_PRINT_CAPABILITY: windows_core::PCWSTR = windows_core::w!("isolatedWin32-print");
pub const SE_APP_SILO_PROFILES_ROOT_MINIMAL_CAPABILITY: windows_core::PCWSTR = windows_core::w!("isolatedWin32-profilesRootMinimal");
pub const SE_APP_SILO_USER_PROFILE_MINIMAL_CAPABILITY: windows_core::PCWSTR = windows_core::w!("isolatedWin32-userProfileMinimal");
pub const SE_APP_SILO_VOLUME_ROOT_MINIMAL_CAPABILITY: windows_core::PCWSTR = windows_core::w!("isolatedWin32-volumeRootMinimal");
pub const SE_CONSTRAINED_IMPERSONATION_CAPABILITY: windows_core::PCWSTR = windows_core::w!("constrainedImpersonation");
pub const SE_DEVELOPMENT_MODE_NETWORK_CAPABILITY: windows_core::PCWSTR = windows_core::w!("developmentModeNetwork");
pub const SE_GROUP_ENABLED: i32 = 4i32;
pub const SE_GROUP_ENABLED_BY_DEFAULT: i32 = 2i32;
pub const SE_GROUP_INTEGRITY: i32 = 32i32;
pub const SE_GROUP_INTEGRITY_ENABLED: i32 = 64i32;
pub const SE_GROUP_LOGON_ID: i32 = -1073741824i32;
pub const SE_GROUP_MANDATORY: i32 = 1i32;
pub const SE_GROUP_OWNER: i32 = 8i32;
pub const SE_GROUP_RESOURCE: i32 = 536870912i32;
pub const SE_GROUP_USE_FOR_DENY_ONLY: i32 = 16i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SE_IMAGE_SIGNATURE_TYPE(pub i32);
pub const SE_LEARNING_MODE_LOGGING_CAPABILITY: windows_core::PCWSTR = windows_core::w!("learningModeLogging");
pub const SE_MUMA_CAPABILITY: windows_core::PCWSTR = windows_core::w!("muma");
pub const SE_PERMISSIVE_LEARNING_MODE_CAPABILITY: windows_core::PCWSTR = windows_core::w!("permissiveLearningMode");
pub const SE_SECURITY_DESCRIPTOR_FLAG_NO_ACCESS_FILTER_ACE: u32 = 4u32;
pub const SE_SECURITY_DESCRIPTOR_FLAG_NO_LABEL_ACE: u32 = 2u32;
pub const SE_SECURITY_DESCRIPTOR_FLAG_NO_OWNER_ACE: u32 = 1u32;
pub const SE_SECURITY_DESCRIPTOR_VALID_FLAGS: u32 = 7u32;
pub const SE_SESSION_IMPERSONATION_CAPABILITY: windows_core::PCWSTR = windows_core::w!("sessionImpersonation");
pub const SE_SIGNING_LEVEL_ANTIMALWARE: u32 = 7u32;
pub const SE_SIGNING_LEVEL_AUTHENTICODE: u32 = 4u32;
pub const SE_SIGNING_LEVEL_CUSTOM_1: u32 = 3u32;
pub const SE_SIGNING_LEVEL_CUSTOM_2: u32 = 5u32;
pub const SE_SIGNING_LEVEL_CUSTOM_3: u32 = 7u32;
pub const SE_SIGNING_LEVEL_CUSTOM_4: u32 = 9u32;
pub const SE_SIGNING_LEVEL_CUSTOM_5: u32 = 10u32;
pub const SE_SIGNING_LEVEL_CUSTOM_6: u32 = 15u32;
pub const SE_SIGNING_LEVEL_CUSTOM_7: u32 = 13u32;
pub const SE_SIGNING_LEVEL_DEVELOPER: u32 = 3u32;
pub const SE_SIGNING_LEVEL_DYNAMIC_CODEGEN: u32 = 11u32;
pub const SE_SIGNING_LEVEL_ENTERPRISE: u32 = 2u32;
pub const SE_SIGNING_LEVEL_MICROSOFT: u32 = 8u32;
pub const SE_SIGNING_LEVEL_STORE: u32 = 6u32;
pub const SE_SIGNING_LEVEL_UNCHECKED: u32 = 0u32;
pub const SE_SIGNING_LEVEL_UNSIGNED: u32 = 1u32;
pub const SE_SIGNING_LEVEL_WINDOWS: u32 = 12u32;
pub const SE_SIGNING_LEVEL_WINDOWS_TCB: u32 = 14u32;
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub struct SE_TOKEN_USER {
    pub Anonymous1: SE_TOKEN_USER_0,
    pub Anonymous2: SE_TOKEN_USER_1,
}
#[cfg(feature = "Win32_Security")]
impl Default for SE_TOKEN_USER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub union SE_TOKEN_USER_0 {
    pub TokenUser: super::super::Security::TOKEN_USER,
    pub User: super::super::Security::SID_AND_ATTRIBUTES,
}
#[cfg(feature = "Win32_Security")]
impl Default for SE_TOKEN_USER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub union SE_TOKEN_USER_1 {
    pub Sid: super::super::Security::SID,
    pub Buffer: [u8; 68],
}
#[cfg(feature = "Win32_Security")]
impl Default for SE_TOKEN_USER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SFGAO_BROWSABLE: SFGAO_FLAGS = SFGAO_FLAGS(134217728u32);
pub const SFGAO_CANCOPY: SFGAO_FLAGS = SFGAO_FLAGS(1u32);
pub const SFGAO_CANDELETE: SFGAO_FLAGS = SFGAO_FLAGS(32u32);
pub const SFGAO_CANLINK: SFGAO_FLAGS = SFGAO_FLAGS(4u32);
pub const SFGAO_CANMONIKER: SFGAO_FLAGS = SFGAO_FLAGS(4194304u32);
pub const SFGAO_CANMOVE: SFGAO_FLAGS = SFGAO_FLAGS(2u32);
pub const SFGAO_CANRENAME: SFGAO_FLAGS = SFGAO_FLAGS(16u32);
pub const SFGAO_CAPABILITYMASK: SFGAO_FLAGS = SFGAO_FLAGS(375u32);
pub const SFGAO_COMPRESSED: SFGAO_FLAGS = SFGAO_FLAGS(67108864u32);
pub const SFGAO_CONTENTSMASK: SFGAO_FLAGS = SFGAO_FLAGS(2147483648u32);
pub const SFGAO_DISPLAYATTRMASK: SFGAO_FLAGS = SFGAO_FLAGS(1032192u32);
pub const SFGAO_DROPTARGET: SFGAO_FLAGS = SFGAO_FLAGS(256u32);
pub const SFGAO_ENCRYPTED: SFGAO_FLAGS = SFGAO_FLAGS(8192u32);
pub const SFGAO_FILESYSANCESTOR: SFGAO_FLAGS = SFGAO_FLAGS(268435456u32);
pub const SFGAO_FILESYSTEM: SFGAO_FLAGS = SFGAO_FLAGS(1073741824u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SFGAO_FLAGS(pub u32);
impl SFGAO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SFGAO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SFGAO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SFGAO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SFGAO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SFGAO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SFGAO_FOLDER: SFGAO_FLAGS = SFGAO_FLAGS(536870912u32);
pub const SFGAO_GHOSTED: SFGAO_FLAGS = SFGAO_FLAGS(32768u32);
pub const SFGAO_HASPROPSHEET: SFGAO_FLAGS = SFGAO_FLAGS(64u32);
pub const SFGAO_HASSTORAGE: SFGAO_FLAGS = SFGAO_FLAGS(4194304u32);
pub const SFGAO_HASSUBFOLDER: SFGAO_FLAGS = SFGAO_FLAGS(2147483648u32);
pub const SFGAO_HIDDEN: SFGAO_FLAGS = SFGAO_FLAGS(524288u32);
pub const SFGAO_ISSLOW: SFGAO_FLAGS = SFGAO_FLAGS(16384u32);
pub const SFGAO_LINK: SFGAO_FLAGS = SFGAO_FLAGS(65536u32);
pub const SFGAO_NEWCONTENT: SFGAO_FLAGS = SFGAO_FLAGS(2097152u32);
pub const SFGAO_NONENUMERATED: SFGAO_FLAGS = SFGAO_FLAGS(1048576u32);
pub const SFGAO_PKEYSFGAOMASK: SFGAO_FLAGS = SFGAO_FLAGS(2164539392u32);
pub const SFGAO_PLACEHOLDER: SFGAO_FLAGS = SFGAO_FLAGS(2048u32);
pub const SFGAO_READONLY: SFGAO_FLAGS = SFGAO_FLAGS(262144u32);
pub const SFGAO_REMOVABLE: SFGAO_FLAGS = SFGAO_FLAGS(33554432u32);
pub const SFGAO_SHARE: SFGAO_FLAGS = SFGAO_FLAGS(131072u32);
pub const SFGAO_STORAGE: SFGAO_FLAGS = SFGAO_FLAGS(8u32);
pub const SFGAO_STORAGEANCESTOR: SFGAO_FLAGS = SFGAO_FLAGS(8388608u32);
pub const SFGAO_STORAGECAPMASK: SFGAO_FLAGS = SFGAO_FLAGS(1891958792u32);
pub const SFGAO_STREAM: SFGAO_FLAGS = SFGAO_FLAGS(4194304u32);
pub const SFGAO_SYSTEM: SFGAO_FLAGS = SFGAO_FLAGS(4096u32);
pub const SFGAO_VALIDATE: SFGAO_FLAGS = SFGAO_FLAGS(16777216u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SHARED_VIRTUAL_DISK_SUPPORT {
    pub SharedVirtualDiskSupport: SharedVirtualDiskSupportType,
    pub HandleState: SharedVirtualDiskHandleState,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SHUFFLE_FILE_DATA {
    pub StartingOffset: i64,
    pub Length: i64,
    pub Flags: u32,
}
pub const SHUFFLE_FILE_FLAG_SKIP_INITIALIZING_NEW_CLUSTERS: u32 = 1u32;
pub const SID_HASH_SIZE: u32 = 32u32;
pub const SID_MAX_SUB_AUTHORITIES: u32 = 15u32;
pub const SID_RECOMMENDED_SUB_AUTHORITIES: u32 = 1u32;
pub const SID_REVISION: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SILOOBJECT_BASIC_INFORMATION {
    pub SiloId: u32,
    pub SiloParentId: u32,
    pub NumberOfProcesses: u32,
    pub IsInServerSilo: bool,
    pub Reserved: [u8; 3],
}
impl Default for SILOOBJECT_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SIZEOF_RFPO_DATA: u32 = 16u32;
pub const SIZE_OF_80387_REGISTERS: u32 = 80u32;
pub const SMB_CCF_APP_INSTANCE_EA_NAME: windows_core::PCSTR = windows_core::s!("ClusteredApplicationInstance");
pub const SMT_UNPARKING_POLICY_CORE: u32 = 0u32;
pub const SMT_UNPARKING_POLICY_CORE_PER_THREAD: u32 = 1u32;
pub const SMT_UNPARKING_POLICY_LP_ROUNDROBIN: u32 = 2u32;
pub const SMT_UNPARKING_POLICY_LP_SEQUENTIAL: u32 = 3u32;
pub const SORT_CHINESE_BIG5: u32 = 0u32;
pub const SORT_CHINESE_BOPOMOFO: u32 = 3u32;
pub const SORT_CHINESE_PRC: u32 = 2u32;
pub const SORT_CHINESE_PRCP: u32 = 0u32;
pub const SORT_CHINESE_RADICALSTROKE: u32 = 4u32;
pub const SORT_CHINESE_UNICODE: u32 = 1u32;
pub const SORT_DEFAULT: u32 = 0u32;
pub const SORT_GEORGIAN_MODERN: u32 = 1u32;
pub const SORT_GEORGIAN_TRADITIONAL: u32 = 0u32;
pub const SORT_GERMAN_PHONE_BOOK: u32 = 1u32;
pub const SORT_HUNGARIAN_DEFAULT: u32 = 0u32;
pub const SORT_HUNGARIAN_TECHNICAL: u32 = 1u32;
pub const SORT_INVARIANT_MATH: u32 = 1u32;
pub const SORT_JAPANESE_RADICALSTROKE: u32 = 4u32;
pub const SORT_JAPANESE_UNICODE: u32 = 1u32;
pub const SORT_JAPANESE_XJIS: u32 = 0u32;
pub const SORT_KOREAN_KSC: u32 = 0u32;
pub const SORT_KOREAN_UNICODE: u32 = 1u32;
pub const SS_BITMAP: STATIC_STYLES = STATIC_STYLES(14u32);
pub const SS_BLACKFRAME: STATIC_STYLES = STATIC_STYLES(7u32);
pub const SS_BLACKRECT: STATIC_STYLES = STATIC_STYLES(4u32);
pub const SS_CENTER: STATIC_STYLES = STATIC_STYLES(1u32);
pub const SS_CENTERIMAGE: STATIC_STYLES = STATIC_STYLES(512u32);
pub const SS_EDITCONTROL: STATIC_STYLES = STATIC_STYLES(8192u32);
pub const SS_ELLIPSISMASK: STATIC_STYLES = STATIC_STYLES(49152u32);
pub const SS_ENDELLIPSIS: STATIC_STYLES = STATIC_STYLES(16384u32);
pub const SS_ENHMETAFILE: STATIC_STYLES = STATIC_STYLES(15u32);
pub const SS_ETCHEDFRAME: STATIC_STYLES = STATIC_STYLES(18u32);
pub const SS_ETCHEDHORZ: STATIC_STYLES = STATIC_STYLES(16u32);
pub const SS_ETCHEDVERT: STATIC_STYLES = STATIC_STYLES(17u32);
pub const SS_GRAYFRAME: STATIC_STYLES = STATIC_STYLES(8u32);
pub const SS_GRAYRECT: STATIC_STYLES = STATIC_STYLES(5u32);
pub const SS_ICON: STATIC_STYLES = STATIC_STYLES(3u32);
pub const SS_LEFT: STATIC_STYLES = STATIC_STYLES(0u32);
pub const SS_LEFTNOWORDWRAP: STATIC_STYLES = STATIC_STYLES(12u32);
pub const SS_NOPREFIX: STATIC_STYLES = STATIC_STYLES(128u32);
pub const SS_NOTIFY: STATIC_STYLES = STATIC_STYLES(256u32);
pub const SS_OWNERDRAW: STATIC_STYLES = STATIC_STYLES(13u32);
pub const SS_PATHELLIPSIS: STATIC_STYLES = STATIC_STYLES(32768u32);
pub const SS_REALSIZECONTROL: STATIC_STYLES = STATIC_STYLES(64u32);
pub const SS_REALSIZEIMAGE: STATIC_STYLES = STATIC_STYLES(2048u32);
pub const SS_RIGHT: STATIC_STYLES = STATIC_STYLES(2u32);
pub const SS_RIGHTJUST: STATIC_STYLES = STATIC_STYLES(1024u32);
pub const SS_SIMPLE: STATIC_STYLES = STATIC_STYLES(11u32);
pub const SS_SUNKEN: STATIC_STYLES = STATIC_STYLES(4096u32);
pub const SS_TYPEMASK: STATIC_STYLES = STATIC_STYLES(31u32);
pub const SS_USERITEM: STATIC_STYLES = STATIC_STYLES(10u32);
pub const SS_WHITEFRAME: STATIC_STYLES = STATIC_STYLES(9u32);
pub const SS_WHITERECT: STATIC_STYLES = STATIC_STYLES(6u32);
pub const SS_WORDELLIPSIS: STATIC_STYLES = STATIC_STYLES(49152u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STATIC_STYLES(pub u32);
pub const SUBLANG_AFRIKAANS_SOUTH_AFRICA: u32 = 1u32;
pub const SUBLANG_ALBANIAN_ALBANIA: u32 = 1u32;
pub const SUBLANG_ALSATIAN_FRANCE: u32 = 1u32;
pub const SUBLANG_AMHARIC_ETHIOPIA: u32 = 1u32;
pub const SUBLANG_ARABIC_ALGERIA: u32 = 5u32;
pub const SUBLANG_ARABIC_BAHRAIN: u32 = 15u32;
pub const SUBLANG_ARABIC_EGYPT: u32 = 3u32;
pub const SUBLANG_ARABIC_IRAQ: u32 = 2u32;
pub const SUBLANG_ARABIC_JORDAN: u32 = 11u32;
pub const SUBLANG_ARABIC_KUWAIT: u32 = 13u32;
pub const SUBLANG_ARABIC_LEBANON: u32 = 12u32;
pub const SUBLANG_ARABIC_LIBYA: u32 = 4u32;
pub const SUBLANG_ARABIC_MOROCCO: u32 = 6u32;
pub const SUBLANG_ARABIC_OMAN: u32 = 8u32;
pub const SUBLANG_ARABIC_QATAR: u32 = 16u32;
pub const SUBLANG_ARABIC_SAUDI_ARABIA: u32 = 1u32;
pub const SUBLANG_ARABIC_SYRIA: u32 = 10u32;
pub const SUBLANG_ARABIC_TUNISIA: u32 = 7u32;
pub const SUBLANG_ARABIC_UAE: u32 = 14u32;
pub const SUBLANG_ARABIC_YEMEN: u32 = 9u32;
pub const SUBLANG_ARMENIAN_ARMENIA: u32 = 1u32;
pub const SUBLANG_ASSAMESE_INDIA: u32 = 1u32;
pub const SUBLANG_AZERBAIJANI_AZERBAIJAN_CYRILLIC: u32 = 2u32;
pub const SUBLANG_AZERBAIJANI_AZERBAIJAN_LATIN: u32 = 1u32;
pub const SUBLANG_AZERI_CYRILLIC: u32 = 2u32;
pub const SUBLANG_AZERI_LATIN: u32 = 1u32;
pub const SUBLANG_BANGLA_BANGLADESH: u32 = 2u32;
pub const SUBLANG_BANGLA_INDIA: u32 = 1u32;
pub const SUBLANG_BASHKIR_RUSSIA: u32 = 1u32;
pub const SUBLANG_BASQUE_BASQUE: u32 = 1u32;
pub const SUBLANG_BELARUSIAN_BELARUS: u32 = 1u32;
pub const SUBLANG_BENGALI_BANGLADESH: u32 = 2u32;
pub const SUBLANG_BENGALI_INDIA: u32 = 1u32;
pub const SUBLANG_BOSNIAN_BOSNIA_HERZEGOVINA_CYRILLIC: u32 = 8u32;
pub const SUBLANG_BOSNIAN_BOSNIA_HERZEGOVINA_LATIN: u32 = 5u32;
pub const SUBLANG_BRETON_FRANCE: u32 = 1u32;
pub const SUBLANG_BULGARIAN_BULGARIA: u32 = 1u32;
pub const SUBLANG_CATALAN_CATALAN: u32 = 1u32;
pub const SUBLANG_CENTRAL_KURDISH_IRAQ: u32 = 1u32;
pub const SUBLANG_CHEROKEE_CHEROKEE: u32 = 1u32;
pub const SUBLANG_CHINESE_HONGKONG: u32 = 3u32;
pub const SUBLANG_CHINESE_MACAU: u32 = 5u32;
pub const SUBLANG_CHINESE_SIMPLIFIED: u32 = 2u32;
pub const SUBLANG_CHINESE_SINGAPORE: u32 = 4u32;
pub const SUBLANG_CHINESE_TRADITIONAL: u32 = 1u32;
pub const SUBLANG_CORSICAN_FRANCE: u32 = 1u32;
pub const SUBLANG_CROATIAN_BOSNIA_HERZEGOVINA_LATIN: u32 = 4u32;
pub const SUBLANG_CROATIAN_CROATIA: u32 = 1u32;
pub const SUBLANG_CUSTOM_DEFAULT: u32 = 3u32;
pub const SUBLANG_CUSTOM_UNSPECIFIED: u32 = 4u32;
pub const SUBLANG_CZECH_CZECH_REPUBLIC: u32 = 1u32;
pub const SUBLANG_DANISH_DENMARK: u32 = 1u32;
pub const SUBLANG_DARI_AFGHANISTAN: u32 = 1u32;
pub const SUBLANG_DEFAULT: u32 = 1u32;
pub const SUBLANG_DIVEHI_MALDIVES: u32 = 1u32;
pub const SUBLANG_DUTCH: u32 = 1u32;
pub const SUBLANG_DUTCH_BELGIAN: u32 = 2u32;
pub const SUBLANG_ENGLISH_AUS: u32 = 3u32;
pub const SUBLANG_ENGLISH_BELIZE: u32 = 10u32;
pub const SUBLANG_ENGLISH_CAN: u32 = 4u32;
pub const SUBLANG_ENGLISH_CARIBBEAN: u32 = 9u32;
pub const SUBLANG_ENGLISH_EIRE: u32 = 6u32;
pub const SUBLANG_ENGLISH_INDIA: u32 = 16u32;
pub const SUBLANG_ENGLISH_JAMAICA: u32 = 8u32;
pub const SUBLANG_ENGLISH_MALAYSIA: u32 = 17u32;
pub const SUBLANG_ENGLISH_NZ: u32 = 5u32;
pub const SUBLANG_ENGLISH_PHILIPPINES: u32 = 13u32;
pub const SUBLANG_ENGLISH_SINGAPORE: u32 = 18u32;
pub const SUBLANG_ENGLISH_SOUTH_AFRICA: u32 = 7u32;
pub const SUBLANG_ENGLISH_TRINIDAD: u32 = 11u32;
pub const SUBLANG_ENGLISH_UK: u32 = 2u32;
pub const SUBLANG_ENGLISH_US: u32 = 1u32;
pub const SUBLANG_ENGLISH_ZIMBABWE: u32 = 12u32;
pub const SUBLANG_ESTONIAN_ESTONIA: u32 = 1u32;
pub const SUBLANG_FAEROESE_FAROE_ISLANDS: u32 = 1u32;
pub const SUBLANG_FILIPINO_PHILIPPINES: u32 = 1u32;
pub const SUBLANG_FINNISH_FINLAND: u32 = 1u32;
pub const SUBLANG_FRENCH: u32 = 1u32;
pub const SUBLANG_FRENCH_BELGIAN: u32 = 2u32;
pub const SUBLANG_FRENCH_CANADIAN: u32 = 3u32;
pub const SUBLANG_FRENCH_LUXEMBOURG: u32 = 5u32;
pub const SUBLANG_FRENCH_MONACO: u32 = 6u32;
pub const SUBLANG_FRENCH_SWISS: u32 = 4u32;
pub const SUBLANG_FRISIAN_NETHERLANDS: u32 = 1u32;
pub const SUBLANG_FULAH_SENEGAL: u32 = 2u32;
pub const SUBLANG_GALICIAN_GALICIAN: u32 = 1u32;
pub const SUBLANG_GEORGIAN_GEORGIA: u32 = 1u32;
pub const SUBLANG_GERMAN: u32 = 1u32;
pub const SUBLANG_GERMAN_AUSTRIAN: u32 = 3u32;
pub const SUBLANG_GERMAN_LIECHTENSTEIN: u32 = 5u32;
pub const SUBLANG_GERMAN_LUXEMBOURG: u32 = 4u32;
pub const SUBLANG_GERMAN_SWISS: u32 = 2u32;
pub const SUBLANG_GREEK_GREECE: u32 = 1u32;
pub const SUBLANG_GREENLANDIC_GREENLAND: u32 = 1u32;
pub const SUBLANG_GUJARATI_INDIA: u32 = 1u32;
pub const SUBLANG_HAUSA_NIGERIA_LATIN: u32 = 1u32;
pub const SUBLANG_HAWAIIAN_US: u32 = 1u32;
pub const SUBLANG_HEBREW_ISRAEL: u32 = 1u32;
pub const SUBLANG_HINDI_INDIA: u32 = 1u32;
pub const SUBLANG_HUNGARIAN_HUNGARY: u32 = 1u32;
pub const SUBLANG_ICELANDIC_ICELAND: u32 = 1u32;
pub const SUBLANG_IGBO_NIGERIA: u32 = 1u32;
pub const SUBLANG_INDONESIAN_INDONESIA: u32 = 1u32;
pub const SUBLANG_INUKTITUT_CANADA: u32 = 1u32;
pub const SUBLANG_INUKTITUT_CANADA_LATIN: u32 = 2u32;
pub const SUBLANG_IRISH_IRELAND: u32 = 2u32;
pub const SUBLANG_ITALIAN: u32 = 1u32;
pub const SUBLANG_ITALIAN_SWISS: u32 = 2u32;
pub const SUBLANG_JAPANESE_JAPAN: u32 = 1u32;
pub const SUBLANG_KANNADA_INDIA: u32 = 1u32;
pub const SUBLANG_KASHMIRI_INDIA: u32 = 2u32;
pub const SUBLANG_KASHMIRI_SASIA: u32 = 2u32;
pub const SUBLANG_KAZAK_KAZAKHSTAN: u32 = 1u32;
pub const SUBLANG_KHMER_CAMBODIA: u32 = 1u32;
pub const SUBLANG_KICHE_GUATEMALA: u32 = 1u32;
pub const SUBLANG_KINYARWANDA_RWANDA: u32 = 1u32;
pub const SUBLANG_KONKANI_INDIA: u32 = 1u32;
pub const SUBLANG_KOREAN: u32 = 1u32;
pub const SUBLANG_KYRGYZ_KYRGYZSTAN: u32 = 1u32;
pub const SUBLANG_LAO_LAO: u32 = 1u32;
pub const SUBLANG_LATVIAN_LATVIA: u32 = 1u32;
pub const SUBLANG_LITHUANIAN: u32 = 1u32;
pub const SUBLANG_LOWER_SORBIAN_GERMANY: u32 = 2u32;
pub const SUBLANG_LUXEMBOURGISH_LUXEMBOURG: u32 = 1u32;
pub const SUBLANG_MACEDONIAN_MACEDONIA: u32 = 1u32;
pub const SUBLANG_MALAYALAM_INDIA: u32 = 1u32;
pub const SUBLANG_MALAY_BRUNEI_DARUSSALAM: u32 = 2u32;
pub const SUBLANG_MALAY_MALAYSIA: u32 = 1u32;
pub const SUBLANG_MALTESE_MALTA: u32 = 1u32;
pub const SUBLANG_MAORI_NEW_ZEALAND: u32 = 1u32;
pub const SUBLANG_MAPUDUNGUN_CHILE: u32 = 1u32;
pub const SUBLANG_MARATHI_INDIA: u32 = 1u32;
pub const SUBLANG_MOHAWK_MOHAWK: u32 = 1u32;
pub const SUBLANG_MONGOLIAN_CYRILLIC_MONGOLIA: u32 = 1u32;
pub const SUBLANG_MONGOLIAN_PRC: u32 = 2u32;
pub const SUBLANG_NEPALI_INDIA: u32 = 2u32;
pub const SUBLANG_NEPALI_NEPAL: u32 = 1u32;
pub const SUBLANG_NEUTRAL: u32 = 0u32;
pub const SUBLANG_NORWEGIAN_BOKMAL: u32 = 1u32;
pub const SUBLANG_NORWEGIAN_NYNORSK: u32 = 2u32;
pub const SUBLANG_OCCITAN_FRANCE: u32 = 1u32;
pub const SUBLANG_ODIA_INDIA: u32 = 1u32;
pub const SUBLANG_ORIYA_INDIA: u32 = 1u32;
pub const SUBLANG_PASHTO_AFGHANISTAN: u32 = 1u32;
pub const SUBLANG_PERSIAN_IRAN: u32 = 1u32;
pub const SUBLANG_POLISH_POLAND: u32 = 1u32;
pub const SUBLANG_PORTUGUESE: u32 = 2u32;
pub const SUBLANG_PORTUGUESE_BRAZILIAN: u32 = 1u32;
pub const SUBLANG_PULAR_SENEGAL: u32 = 2u32;
pub const SUBLANG_PUNJABI_INDIA: u32 = 1u32;
pub const SUBLANG_PUNJABI_PAKISTAN: u32 = 2u32;
pub const SUBLANG_QUECHUA_BOLIVIA: u32 = 1u32;
pub const SUBLANG_QUECHUA_ECUADOR: u32 = 2u32;
pub const SUBLANG_QUECHUA_PERU: u32 = 3u32;
pub const SUBLANG_ROMANIAN_ROMANIA: u32 = 1u32;
pub const SUBLANG_ROMANSH_SWITZERLAND: u32 = 1u32;
pub const SUBLANG_RUSSIAN_RUSSIA: u32 = 1u32;
pub const SUBLANG_SAKHA_RUSSIA: u32 = 1u32;
pub const SUBLANG_SAMI_INARI_FINLAND: u32 = 9u32;
pub const SUBLANG_SAMI_LULE_NORWAY: u32 = 4u32;
pub const SUBLANG_SAMI_LULE_SWEDEN: u32 = 5u32;
pub const SUBLANG_SAMI_NORTHERN_FINLAND: u32 = 3u32;
pub const SUBLANG_SAMI_NORTHERN_NORWAY: u32 = 1u32;
pub const SUBLANG_SAMI_NORTHERN_SWEDEN: u32 = 2u32;
pub const SUBLANG_SAMI_SKOLT_FINLAND: u32 = 8u32;
pub const SUBLANG_SAMI_SOUTHERN_NORWAY: u32 = 6u32;
pub const SUBLANG_SAMI_SOUTHERN_SWEDEN: u32 = 7u32;
pub const SUBLANG_SANSKRIT_INDIA: u32 = 1u32;
pub const SUBLANG_SCOTTISH_GAELIC: u32 = 1u32;
pub const SUBLANG_SERBIAN_BOSNIA_HERZEGOVINA_CYRILLIC: u32 = 7u32;
pub const SUBLANG_SERBIAN_BOSNIA_HERZEGOVINA_LATIN: u32 = 6u32;
pub const SUBLANG_SERBIAN_CROATIA: u32 = 1u32;
pub const SUBLANG_SERBIAN_CYRILLIC: u32 = 3u32;
pub const SUBLANG_SERBIAN_LATIN: u32 = 2u32;
pub const SUBLANG_SERBIAN_MONTENEGRO_CYRILLIC: u32 = 12u32;
pub const SUBLANG_SERBIAN_MONTENEGRO_LATIN: u32 = 11u32;
pub const SUBLANG_SERBIAN_SERBIA_CYRILLIC: u32 = 10u32;
pub const SUBLANG_SERBIAN_SERBIA_LATIN: u32 = 9u32;
pub const SUBLANG_SINDHI_AFGHANISTAN: u32 = 2u32;
pub const SUBLANG_SINDHI_INDIA: u32 = 1u32;
pub const SUBLANG_SINDHI_PAKISTAN: u32 = 2u32;
pub const SUBLANG_SINHALESE_SRI_LANKA: u32 = 1u32;
pub const SUBLANG_SLOVAK_SLOVAKIA: u32 = 1u32;
pub const SUBLANG_SLOVENIAN_SLOVENIA: u32 = 1u32;
pub const SUBLANG_SOTHO_NORTHERN_SOUTH_AFRICA: u32 = 1u32;
pub const SUBLANG_SPANISH: u32 = 1u32;
pub const SUBLANG_SPANISH_ARGENTINA: u32 = 11u32;
pub const SUBLANG_SPANISH_BOLIVIA: u32 = 16u32;
pub const SUBLANG_SPANISH_CHILE: u32 = 13u32;
pub const SUBLANG_SPANISH_COLOMBIA: u32 = 9u32;
pub const SUBLANG_SPANISH_COSTA_RICA: u32 = 5u32;
pub const SUBLANG_SPANISH_DOMINICAN_REPUBLIC: u32 = 7u32;
pub const SUBLANG_SPANISH_ECUADOR: u32 = 12u32;
pub const SUBLANG_SPANISH_EL_SALVADOR: u32 = 17u32;
pub const SUBLANG_SPANISH_GUATEMALA: u32 = 4u32;
pub const SUBLANG_SPANISH_HONDURAS: u32 = 18u32;
pub const SUBLANG_SPANISH_MEXICAN: u32 = 2u32;
pub const SUBLANG_SPANISH_MODERN: u32 = 3u32;
pub const SUBLANG_SPANISH_NICARAGUA: u32 = 19u32;
pub const SUBLANG_SPANISH_PANAMA: u32 = 6u32;
pub const SUBLANG_SPANISH_PARAGUAY: u32 = 15u32;
pub const SUBLANG_SPANISH_PERU: u32 = 10u32;
pub const SUBLANG_SPANISH_PUERTO_RICO: u32 = 20u32;
pub const SUBLANG_SPANISH_URUGUAY: u32 = 14u32;
pub const SUBLANG_SPANISH_US: u32 = 21u32;
pub const SUBLANG_SPANISH_VENEZUELA: u32 = 8u32;
pub const SUBLANG_SWAHILI_KENYA: u32 = 1u32;
pub const SUBLANG_SWEDISH: u32 = 1u32;
pub const SUBLANG_SWEDISH_FINLAND: u32 = 2u32;
pub const SUBLANG_SYRIAC_SYRIA: u32 = 1u32;
pub const SUBLANG_SYS_DEFAULT: u32 = 2u32;
pub const SUBLANG_TAJIK_TAJIKISTAN: u32 = 1u32;
pub const SUBLANG_TAMAZIGHT_ALGERIA_LATIN: u32 = 2u32;
pub const SUBLANG_TAMAZIGHT_MOROCCO_TIFINAGH: u32 = 4u32;
pub const SUBLANG_TAMIL_INDIA: u32 = 1u32;
pub const SUBLANG_TAMIL_SRI_LANKA: u32 = 2u32;
pub const SUBLANG_TATAR_RUSSIA: u32 = 1u32;
pub const SUBLANG_TELUGU_INDIA: u32 = 1u32;
pub const SUBLANG_THAI_THAILAND: u32 = 1u32;
pub const SUBLANG_TIBETAN_PRC: u32 = 1u32;
pub const SUBLANG_TIGRIGNA_ERITREA: u32 = 2u32;
pub const SUBLANG_TIGRINYA_ERITREA: u32 = 2u32;
pub const SUBLANG_TIGRINYA_ETHIOPIA: u32 = 1u32;
pub const SUBLANG_TSWANA_BOTSWANA: u32 = 2u32;
pub const SUBLANG_TSWANA_SOUTH_AFRICA: u32 = 1u32;
pub const SUBLANG_TURKISH_TURKEY: u32 = 1u32;
pub const SUBLANG_TURKMEN_TURKMENISTAN: u32 = 1u32;
pub const SUBLANG_UIGHUR_PRC: u32 = 1u32;
pub const SUBLANG_UI_CUSTOM_DEFAULT: u32 = 5u32;
pub const SUBLANG_UKRAINIAN_UKRAINE: u32 = 1u32;
pub const SUBLANG_UPPER_SORBIAN_GERMANY: u32 = 1u32;
pub const SUBLANG_URDU_INDIA: u32 = 2u32;
pub const SUBLANG_URDU_PAKISTAN: u32 = 1u32;
pub const SUBLANG_UZBEK_CYRILLIC: u32 = 2u32;
pub const SUBLANG_UZBEK_LATIN: u32 = 1u32;
pub const SUBLANG_VALENCIAN_VALENCIA: u32 = 2u32;
pub const SUBLANG_VIETNAMESE_VIETNAM: u32 = 1u32;
pub const SUBLANG_WELSH_UNITED_KINGDOM: u32 = 1u32;
pub const SUBLANG_WOLOF_SENEGAL: u32 = 1u32;
pub const SUBLANG_XHOSA_SOUTH_AFRICA: u32 = 1u32;
pub const SUBLANG_YAKUT_RUSSIA: u32 = 1u32;
pub const SUBLANG_YI_PRC: u32 = 1u32;
pub const SUBLANG_YORUBA_NIGERIA: u32 = 1u32;
pub const SUBLANG_ZULU_SOUTH_AFRICA: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SUPPORTED_OS_INFO {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
pub const SYSTEM_ACCESS_FILTER_ACE_TYPE: u32 = 21u32;
pub const SYSTEM_ACCESS_FILTER_NOCONSTRAINT_MASK: u32 = 4294967295u32;
pub const SYSTEM_ACCESS_FILTER_VALID_MASK: u32 = 16777215u32;
pub const SYSTEM_ALARM_ACE_TYPE: u32 = 3u32;
pub const SYSTEM_ALARM_CALLBACK_ACE_TYPE: u32 = 14u32;
pub const SYSTEM_ALARM_CALLBACK_OBJECT_ACE_TYPE: u32 = 16u32;
pub const SYSTEM_ALARM_OBJECT_ACE_TYPE: u32 = 8u32;
pub const SYSTEM_AUDIT_ACE_TYPE: u32 = 2u32;
pub const SYSTEM_AUDIT_CALLBACK_ACE_TYPE: u32 = 13u32;
pub const SYSTEM_AUDIT_CALLBACK_OBJECT_ACE_TYPE: u32 = 15u32;
pub const SYSTEM_AUDIT_OBJECT_ACE_TYPE: u32 = 7u32;
pub const SYSTEM_CACHE_ALIGNMENT_SIZE: u32 = 64u32;
pub const SYSTEM_MANDATORY_LABEL_ACE_TYPE: u32 = 17u32;
pub const SYSTEM_MANDATORY_LABEL_NO_EXECUTE_UP: u32 = 4u32;
pub const SYSTEM_MANDATORY_LABEL_NO_READ_UP: u32 = 2u32;
pub const SYSTEM_MANDATORY_LABEL_NO_WRITE_UP: u32 = 1u32;
pub const SYSTEM_PROCESS_TRUST_LABEL_ACE_TYPE: u32 = 20u32;
pub const SYSTEM_PROCESS_TRUST_LABEL_VALID_MASK: u32 = 16777215u32;
pub const SYSTEM_PROCESS_TRUST_NOCONSTRAINT_MASK: u32 = 4294967295u32;
pub const SYSTEM_RESOURCE_ATTRIBUTE_ACE_TYPE: u32 = 18u32;
pub const SYSTEM_SCOPED_POLICY_ID_ACE_TYPE: u32 = 19u32;
pub const SeImageSignatureCache: SE_IMAGE_SIGNATURE_TYPE = SE_IMAGE_SIGNATURE_TYPE(2i32);
pub const SeImageSignatureCatalogCached: SE_IMAGE_SIGNATURE_TYPE = SE_IMAGE_SIGNATURE_TYPE(3i32);
pub const SeImageSignatureCatalogHint: SE_IMAGE_SIGNATURE_TYPE = SE_IMAGE_SIGNATURE_TYPE(5i32);
pub const SeImageSignatureCatalogNotCached: SE_IMAGE_SIGNATURE_TYPE = SE_IMAGE_SIGNATURE_TYPE(4i32);
pub const SeImageSignatureEmbedded: SE_IMAGE_SIGNATURE_TYPE = SE_IMAGE_SIGNATURE_TYPE(1i32);
pub const SeImageSignatureNone: SE_IMAGE_SIGNATURE_TYPE = SE_IMAGE_SIGNATURE_TYPE(0i32);
pub const SeImageSignaturePackageCatalog: SE_IMAGE_SIGNATURE_TYPE = SE_IMAGE_SIGNATURE_TYPE(6i32);
pub const SeImageSignaturePplMitigated: SE_IMAGE_SIGNATURE_TYPE = SE_IMAGE_SIGNATURE_TYPE(7i32);
pub const SevereError: SERVICE_ERROR_TYPE = SERVICE_ERROR_TYPE(2i32);
pub const SharedVirtualDiskCDPSnapshotsSupported: SharedVirtualDiskSupportType = SharedVirtualDiskSupportType(7i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SharedVirtualDiskHandleState(pub i32);
pub const SharedVirtualDiskHandleStateFileShared: SharedVirtualDiskHandleState = SharedVirtualDiskHandleState(1i32);
pub const SharedVirtualDiskHandleStateHandleShared: SharedVirtualDiskHandleState = SharedVirtualDiskHandleState(3i32);
pub const SharedVirtualDiskHandleStateNone: SharedVirtualDiskHandleState = SharedVirtualDiskHandleState(0i32);
pub const SharedVirtualDiskSnapshotsSupported: SharedVirtualDiskSupportType = SharedVirtualDiskSupportType(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SharedVirtualDiskSupportType(pub i32);
pub const SharedVirtualDisksSupported: SharedVirtualDiskSupportType = SharedVirtualDiskSupportType(1i32);
pub const SharedVirtualDisksUnsupported: SharedVirtualDiskSupportType = SharedVirtualDiskSupportType(0i32);
pub const SystemLoad: SERVICE_LOAD_TYPE = SERVICE_LOAD_TYPE(1i32);
pub const TAPE_CHECK_FOR_DRIVE_PROBLEM: i32 = 2i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TAPE_CREATE_PARTITION {
    pub Method: u32,
    pub Count: u32,
    pub Size: u32,
}
pub const TAPE_DRIVE_ABSOLUTE_BLK: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2147487744u32);
pub const TAPE_DRIVE_ABS_BLK_IMMED: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2147491840u32);
pub const TAPE_DRIVE_CLEAN_REQUESTS: u32 = 33554432u32;
pub const TAPE_DRIVE_COMPRESSION: u32 = 131072u32;
pub const TAPE_DRIVE_ECC: u32 = 65536u32;
pub const TAPE_DRIVE_EJECT_MEDIA: u32 = 16777216u32;
pub const TAPE_DRIVE_END_OF_DATA: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2147549184u32);
pub const TAPE_DRIVE_EOT_WZ_SIZE: u32 = 8192u32;
pub const TAPE_DRIVE_ERASE_BOP_ONLY: u32 = 64u32;
pub const TAPE_DRIVE_ERASE_IMMEDIATE: u32 = 128u32;
pub const TAPE_DRIVE_ERASE_LONG: u32 = 32u32;
pub const TAPE_DRIVE_ERASE_SHORT: u32 = 16u32;
pub const TAPE_DRIVE_FILEMARKS: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2147745792u32);
pub const TAPE_DRIVE_FIXED: u32 = 1u32;
pub const TAPE_DRIVE_FIXED_BLOCK: u32 = 1024u32;
pub const TAPE_DRIVE_FORMAT: u32 = 2684354560u32;
pub const TAPE_DRIVE_FORMAT_IMMEDIATE: u32 = 3221225472u32;
pub const TAPE_DRIVE_GET_ABSOLUTE_BLK: u32 = 1048576u32;
pub const TAPE_DRIVE_GET_LOGICAL_BLK: u32 = 2097152u32;
pub const TAPE_DRIVE_HIGH_FEATURES: u32 = 2147483648u32;
pub const TAPE_DRIVE_INITIATOR: u32 = 4u32;
pub const TAPE_DRIVE_LOAD_UNLD_IMMED: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2147483680u32);
pub const TAPE_DRIVE_LOAD_UNLOAD: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2147483649u32);
pub const TAPE_DRIVE_LOCK_UNLK_IMMED: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2147483776u32);
pub const TAPE_DRIVE_LOCK_UNLOCK: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2147483652u32);
pub const TAPE_DRIVE_LOGICAL_BLK: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2147500032u32);
pub const TAPE_DRIVE_LOG_BLK_IMMED: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2147516416u32);
pub const TAPE_DRIVE_PADDING: u32 = 262144u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TAPE_DRIVE_PROBLEM_TYPE(pub i32);
pub const TAPE_DRIVE_RELATIVE_BLKS: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2147614720u32);
pub const TAPE_DRIVE_REPORT_SMKS: u32 = 524288u32;
pub const TAPE_DRIVE_RESERVED_BIT: u32 = 2147483648u32;
pub const TAPE_DRIVE_REVERSE_POSITION: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2151677952u32);
pub const TAPE_DRIVE_REWIND_IMMEDIATE: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2147483656u32);
pub const TAPE_DRIVE_SELECT: u32 = 2u32;
pub const TAPE_DRIVE_SEQUENTIAL_FMKS: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2148007936u32);
pub const TAPE_DRIVE_SEQUENTIAL_SMKS: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2149580800u32);
pub const TAPE_DRIVE_SETMARKS: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2148532224u32);
pub const TAPE_DRIVE_SET_BLOCK_SIZE: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2147483664u32);
pub const TAPE_DRIVE_SET_CMP_BOP_ONLY: u32 = 67108864u32;
pub const TAPE_DRIVE_SET_COMPRESSION: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2147484160u32);
pub const TAPE_DRIVE_SET_ECC: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2147483904u32);
pub const TAPE_DRIVE_SET_EOT_WZ_SIZE: u32 = 4194304u32;
pub const TAPE_DRIVE_SET_PADDING: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2147484672u32);
pub const TAPE_DRIVE_SET_REPORT_SMKS: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2147485696u32);
pub const TAPE_DRIVE_SPACE_IMMEDIATE: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2155872256u32);
pub const TAPE_DRIVE_TAPE_CAPACITY: u32 = 256u32;
pub const TAPE_DRIVE_TAPE_REMAINING: u32 = 512u32;
pub const TAPE_DRIVE_TENSION: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2147483650u32);
pub const TAPE_DRIVE_TENSION_IMMED: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2147483712u32);
pub const TAPE_DRIVE_VARIABLE_BLOCK: u32 = 2048u32;
pub const TAPE_DRIVE_WRITE_FILEMARKS: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2181038080u32);
pub const TAPE_DRIVE_WRITE_LONG_FMKS: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2281701376u32);
pub const TAPE_DRIVE_WRITE_MARK_IMMED: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2415919104u32);
pub const TAPE_DRIVE_WRITE_PROTECT: u32 = 4096u32;
pub const TAPE_DRIVE_WRITE_SETMARKS: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2164260864u32);
pub const TAPE_DRIVE_WRITE_SHORT_FMKS: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(2214592512u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TAPE_GET_DRIVE_PARAMETERS {
    pub ECC: bool,
    pub Compression: bool,
    pub DataPadding: bool,
    pub ReportSetmarks: bool,
    pub DefaultBlockSize: u32,
    pub MaximumBlockSize: u32,
    pub MinimumBlockSize: u32,
    pub MaximumPartitionCount: u32,
    pub FeaturesLow: u32,
    pub FeaturesHigh: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH,
    pub EOTWarningZoneSize: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH(pub u32);
impl TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TAPE_GET_MEDIA_PARAMETERS {
    pub Capacity: i64,
    pub Remaining: i64,
    pub BlockSize: u32,
    pub PartitionCount: u32,
    pub WriteProtected: bool,
}
pub const TAPE_PSEUDO_LOGICAL_BLOCK: i32 = 3i32;
pub const TAPE_PSEUDO_LOGICAL_POSITION: i32 = 2i32;
pub const TAPE_QUERY_DEVICE_ERROR_DATA: i32 = 4i32;
pub const TAPE_QUERY_DRIVE_PARAMETERS: i32 = 0i32;
pub const TAPE_QUERY_IO_ERROR_DATA: i32 = 3i32;
pub const TAPE_QUERY_MEDIA_CAPACITY: i32 = 1i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TAPE_SET_DRIVE_PARAMETERS {
    pub ECC: bool,
    pub Compression: bool,
    pub DataPadding: bool,
    pub ReportSetmarks: bool,
    pub EOTWarningZoneSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TAPE_SET_MEDIA_PARAMETERS {
    pub BlockSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TAPE_WMI_OPERATIONS {
    pub Method: u32,
    pub DataBufferSize: u32,
    pub DataBuffer: *mut core::ffi::c_void,
}
impl Default for TAPE_WMI_OPERATIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const THREAD_BASE_PRIORITY_IDLE: i32 = -15i32;
pub const THREAD_BASE_PRIORITY_LOWRT: u32 = 15u32;
pub const THREAD_BASE_PRIORITY_MAX: u32 = 2u32;
pub const THREAD_BASE_PRIORITY_MIN: i32 = -2i32;
pub const THREAD_DYNAMIC_CODE_ALLOW: u32 = 1u32;
pub const THREAD_PROFILING_FLAG_DISPATCH: u32 = 1u32;
pub const TIME_ZONE_ID_DAYLIGHT: u32 = 2u32;
pub const TIME_ZONE_ID_STANDARD: u32 = 1u32;
pub const TIME_ZONE_ID_UNKNOWN: u32 = 0u32;
pub const TLS_MINIMUM_AVAILABLE: u32 = 64u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TOKEN_BNO_ISOLATION_INFORMATION {
    pub IsolationPrefix: windows_core::PWSTR,
    pub IsolationEnabled: bool,
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TOKEN_SID_INFORMATION {
    pub Sid: super::super::Security::PSID,
}
pub const TOKEN_SOURCE_LENGTH: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRANSACTIONMANAGER_BASIC_INFORMATION {
    pub TmIdentity: windows_core::GUID,
    pub VirtualClock: i64,
}
pub const TRANSACTIONMANAGER_BIND_TRANSACTION: u32 = 32u32;
pub const TRANSACTIONMANAGER_CREATE_RM: u32 = 16u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRANSACTIONMANAGER_INFORMATION_CLASS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TRANSACTIONMANAGER_LOGPATH_INFORMATION {
    pub LogPathLength: u32,
    pub LogPath: [u16; 1],
}
impl Default for TRANSACTIONMANAGER_LOGPATH_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRANSACTIONMANAGER_LOG_INFORMATION {
    pub LogIdentity: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRANSACTIONMANAGER_OLDEST_INFORMATION {
    pub OldestTransactionGuid: windows_core::GUID,
}
pub const TRANSACTIONMANAGER_QUERY_INFORMATION: u32 = 1u32;
pub const TRANSACTIONMANAGER_RECOVER: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRANSACTIONMANAGER_RECOVERY_INFORMATION {
    pub LastRecoveredLsn: u64,
}
pub const TRANSACTIONMANAGER_RENAME: u32 = 8u32;
pub const TRANSACTIONMANAGER_SET_INFORMATION: u32 = 2u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRANSACTION_BASIC_INFORMATION {
    pub TransactionId: windows_core::GUID,
    pub State: u32,
    pub Outcome: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRANSACTION_BIND_INFORMATION {
    pub TmHandle: super::super::Foundation::HANDLE,
}
pub const TRANSACTION_COMMIT: u32 = 8u32;
pub const TRANSACTION_ENLIST: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TRANSACTION_ENLISTMENTS_INFORMATION {
    pub NumberOfEnlistments: u32,
    pub EnlistmentPair: [TRANSACTION_ENLISTMENT_PAIR; 1],
}
impl Default for TRANSACTION_ENLISTMENTS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRANSACTION_ENLISTMENT_PAIR {
    pub EnlistmentId: windows_core::GUID,
    pub ResourceManagerId: windows_core::GUID,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRANSACTION_INFORMATION_CLASS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRANSACTION_LIST_ENTRY {
    pub UOW: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TRANSACTION_LIST_INFORMATION {
    pub NumberOfTransactions: u32,
    pub TransactionInformation: [TRANSACTION_LIST_ENTRY; 1],
}
impl Default for TRANSACTION_LIST_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TRANSACTION_PROPAGATE: u32 = 32u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TRANSACTION_PROPERTIES_INFORMATION {
    pub IsolationLevel: u32,
    pub IsolationFlags: u32,
    pub Timeout: i64,
    pub Outcome: u32,
    pub DescriptionLength: u32,
    pub Description: [u16; 1],
}
impl Default for TRANSACTION_PROPERTIES_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TRANSACTION_QUERY_INFORMATION: u32 = 1u32;
pub const TRANSACTION_RIGHT_RESERVED1: u32 = 64u32;
pub const TRANSACTION_ROLLBACK: u32 = 16u32;
pub const TRANSACTION_SET_INFORMATION: u32 = 2u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TRANSACTION_STATE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRANSACTION_SUPERIOR_ENLISTMENT_INFORMATION {
    pub SuperiorEnlistmentPair: TRANSACTION_ENLISTMENT_PAIR,
}
pub const TREE_CONNECT_ATTRIBUTE_GLOBAL: u32 = 4u32;
pub const TREE_CONNECT_ATTRIBUTE_INTEGRITY: u32 = 32768u32;
pub const TREE_CONNECT_ATTRIBUTE_PINNED: u32 = 2u32;
pub const TREE_CONNECT_ATTRIBUTE_PRIVACY: u32 = 16384u32;
pub const TRUST_PROTECTED_FILTER_ACE_FLAG: u32 = 64u32;
pub const TapeDriveCleanDriveNow: TAPE_DRIVE_PROBLEM_TYPE = TAPE_DRIVE_PROBLEM_TYPE(11i32);
pub const TapeDriveHardwareError: TAPE_DRIVE_PROBLEM_TYPE = TAPE_DRIVE_PROBLEM_TYPE(7i32);
pub const TapeDriveMediaLifeExpired: TAPE_DRIVE_PROBLEM_TYPE = TAPE_DRIVE_PROBLEM_TYPE(12i32);
pub const TapeDriveProblemNone: TAPE_DRIVE_PROBLEM_TYPE = TAPE_DRIVE_PROBLEM_TYPE(0i32);
pub const TapeDriveReadError: TAPE_DRIVE_PROBLEM_TYPE = TAPE_DRIVE_PROBLEM_TYPE(5i32);
pub const TapeDriveReadWarning: TAPE_DRIVE_PROBLEM_TYPE = TAPE_DRIVE_PROBLEM_TYPE(3i32);
pub const TapeDriveReadWriteError: TAPE_DRIVE_PROBLEM_TYPE = TAPE_DRIVE_PROBLEM_TYPE(2i32);
pub const TapeDriveReadWriteWarning: TAPE_DRIVE_PROBLEM_TYPE = TAPE_DRIVE_PROBLEM_TYPE(1i32);
pub const TapeDriveScsiConnectionError: TAPE_DRIVE_PROBLEM_TYPE = TAPE_DRIVE_PROBLEM_TYPE(9i32);
pub const TapeDriveSnappedTape: TAPE_DRIVE_PROBLEM_TYPE = TAPE_DRIVE_PROBLEM_TYPE(13i32);
pub const TapeDriveTimetoClean: TAPE_DRIVE_PROBLEM_TYPE = TAPE_DRIVE_PROBLEM_TYPE(10i32);
pub const TapeDriveUnsupportedMedia: TAPE_DRIVE_PROBLEM_TYPE = TAPE_DRIVE_PROBLEM_TYPE(8i32);
pub const TapeDriveWriteError: TAPE_DRIVE_PROBLEM_TYPE = TAPE_DRIVE_PROBLEM_TYPE(6i32);
pub const TapeDriveWriteWarning: TAPE_DRIVE_PROBLEM_TYPE = TAPE_DRIVE_PROBLEM_TYPE(4i32);
pub const TransactionBasicInformation: TRANSACTION_INFORMATION_CLASS = TRANSACTION_INFORMATION_CLASS(0i32);
pub const TransactionBindInformation: TRANSACTION_INFORMATION_CLASS = TRANSACTION_INFORMATION_CLASS(4i32);
pub const TransactionDTCPrivateInformation: TRANSACTION_INFORMATION_CLASS = TRANSACTION_INFORMATION_CLASS(5i32);
pub const TransactionEnlistmentInformation: TRANSACTION_INFORMATION_CLASS = TRANSACTION_INFORMATION_CLASS(2i32);
pub const TransactionManagerBasicInformation: TRANSACTIONMANAGER_INFORMATION_CLASS = TRANSACTIONMANAGER_INFORMATION_CLASS(0i32);
pub const TransactionManagerLogInformation: TRANSACTIONMANAGER_INFORMATION_CLASS = TRANSACTIONMANAGER_INFORMATION_CLASS(1i32);
pub const TransactionManagerLogPathInformation: TRANSACTIONMANAGER_INFORMATION_CLASS = TRANSACTIONMANAGER_INFORMATION_CLASS(2i32);
pub const TransactionManagerOldestTransactionInformation: TRANSACTIONMANAGER_INFORMATION_CLASS = TRANSACTIONMANAGER_INFORMATION_CLASS(5i32);
pub const TransactionManagerOnlineProbeInformation: TRANSACTIONMANAGER_INFORMATION_CLASS = TRANSACTIONMANAGER_INFORMATION_CLASS(3i32);
pub const TransactionManagerRecoveryInformation: TRANSACTIONMANAGER_INFORMATION_CLASS = TRANSACTIONMANAGER_INFORMATION_CLASS(4i32);
pub const TransactionPropertiesInformation: TRANSACTION_INFORMATION_CLASS = TRANSACTION_INFORMATION_CLASS(1i32);
pub const TransactionStateCommittedNotify: TRANSACTION_STATE = TRANSACTION_STATE(3i32);
pub const TransactionStateIndoubt: TRANSACTION_STATE = TRANSACTION_STATE(2i32);
pub const TransactionStateNormal: TRANSACTION_STATE = TRANSACTION_STATE(1i32);
pub const TransactionSuperiorEnlistmentInformation: TRANSACTION_INFORMATION_CLASS = TRANSACTION_INFORMATION_CLASS(3i32);
pub const UCSCHAR_INVALID_CHARACTER: u32 = 4294967295u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UMS_CREATE_THREAD_ATTRIBUTES {
    pub UmsVersion: u32,
    pub UmsContext: *mut core::ffi::c_void,
    pub UmsCompletionList: *mut core::ffi::c_void,
}
impl Default for UMS_CREATE_THREAD_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const UNICODE_STRING_MAX_CHARS: u32 = 32767u32;
pub const UNIFIEDBUILDREVISION_KEY: windows_core::PCWSTR = windows_core::w!("\\Registry\\Machine\\Software\\Microsoft\\Windows NT\\CurrentVersion");
pub const UNIFIEDBUILDREVISION_MIN: u32 = 0u32;
pub const UNIFIEDBUILDREVISION_VALUE: windows_core::PCWSTR = windows_core::w!("UBR");
pub const UNWIND_CHAIN_LIMIT: u32 = 32u32;
pub const UNWIND_HISTORY_TABLE_SIZE: u32 = 12u32;
pub const UNW_FLAG_NO_EPILOGUE: u32 = 2147483648u32;
pub const UmsSchedulerStartup: RTL_UMS_SCHEDULER_REASON = RTL_UMS_SCHEDULER_REASON(0i32);
pub const UmsSchedulerThreadBlocked: RTL_UMS_SCHEDULER_REASON = RTL_UMS_SCHEDULER_REASON(1i32);
pub const UmsSchedulerThreadYield: RTL_UMS_SCHEDULER_REASON = RTL_UMS_SCHEDULER_REASON(2i32);
pub const VALID_INHERIT_FLAGS: u32 = 31u32;
pub const VBS_BASIC_PAGE_MEASURED_DATA: u32 = 1u32;
pub const VBS_BASIC_PAGE_SYSTEM_CALL: u32 = 5u32;
pub const VBS_BASIC_PAGE_THREAD_DESCRIPTOR: u32 = 4u32;
pub const VBS_BASIC_PAGE_UNMEASURED_DATA: u32 = 2u32;
pub const VBS_BASIC_PAGE_ZERO_FILL: u32 = 3u32;
pub const VER_AND: u32 = 6u32;
pub const VER_CONDITION_MASK: u32 = 7u32;
pub const VER_EQUAL: u32 = 1u32;
pub const VER_GREATER: u32 = 2u32;
pub const VER_GREATER_EQUAL: u32 = 3u32;
pub const VER_LESS: u32 = 4u32;
pub const VER_LESS_EQUAL: u32 = 5u32;
pub const VER_NT_DOMAIN_CONTROLLER: u32 = 2u32;
pub const VER_NT_SERVER: u32 = 3u32;
pub const VER_NT_WORKSTATION: u32 = 1u32;
pub const VER_NUM_BITS_PER_CONDITION_MASK: u32 = 3u32;
pub const VER_OR: u32 = 7u32;
pub const VER_SERVER_NT: u32 = 2147483648u32;
pub const VER_SUITE_BACKOFFICE: u32 = 4u32;
pub const VER_SUITE_BLADE: u32 = 1024u32;
pub const VER_SUITE_COMMUNICATIONS: u32 = 8u32;
pub const VER_SUITE_COMPUTE_SERVER: u32 = 16384u32;
pub const VER_SUITE_DATACENTER: u32 = 128u32;
pub const VER_SUITE_EMBEDDEDNT: u32 = 64u32;
pub const VER_SUITE_EMBEDDED_RESTRICTED: u32 = 2048u32;
pub const VER_SUITE_ENTERPRISE: u32 = 2u32;
pub const VER_SUITE_MULTIUSERTS: u32 = 131072u32;
pub const VER_SUITE_PERSONAL: u32 = 512u32;
pub const VER_SUITE_SECURITY_APPLIANCE: u32 = 4096u32;
pub const VER_SUITE_SINGLEUSERTS: u32 = 256u32;
pub const VER_SUITE_SMALLBUSINESS: u32 = 1u32;
pub const VER_SUITE_SMALLBUSINESS_RESTRICTED: u32 = 32u32;
pub const VER_SUITE_STORAGE_SERVER: u32 = 8192u32;
pub const VER_SUITE_TERMINAL: u32 = 16u32;
pub const VER_SUITE_WH_SERVER: u32 = 32768u32;
pub const VER_WORKSTATION_NT: u32 = 1073741824u32;
pub const VRL_CUSTOM_CLASS_BEGIN: u32 = 256u32;
pub const VRL_ENABLE_KERNEL_BREAKS: u32 = 2147483648u32;
pub const VRL_PREDEFINED_CLASS_BEGIN: u32 = 1u32;
pub const WDT_INPROC64_CALL: u32 = 1349805143u32;
pub const WDT_INPROC_CALL: u32 = 1215587415u32;
pub const WDT_REMOTE_CALL: u32 = 1383359575u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WORD_WHEEL_OPEN_FLAGS(pub u32);
impl WORD_WHEEL_OPEN_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for WORD_WHEEL_OPEN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for WORD_WHEEL_OPEN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for WORD_WHEEL_OPEN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for WORD_WHEEL_OPEN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for WORD_WHEEL_OPEN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const WRITE_NV_MEMORY_FLAG_FLUSH: u32 = 1u32;
pub const WRITE_NV_MEMORY_FLAG_NON_TEMPORAL: u32 = 2u32;
pub const WRITE_NV_MEMORY_FLAG_NO_DRAIN: u32 = 256u32;
pub const WRITE_WATCH_FLAG_RESET: u32 = 1u32;
pub const WT_EXECUTEDELETEWAIT: u32 = 8u32;
pub const WT_EXECUTEINLONGTHREAD: u32 = 16u32;
pub const WT_EXECUTEINPERSISTENTIOTHREAD: u32 = 64u32;
pub const WT_EXECUTEINUITHREAD: u32 = 2u32;
pub const Win32ServiceOwnProcess: SERVICE_NODE_TYPE = SERVICE_NODE_TYPE(16i32);
pub const Win32ServiceShareProcess: SERVICE_NODE_TYPE = SERVICE_NODE_TYPE(32i32);
pub const X3_BTYPE_QP_INST_VAL_POS_X: u32 = 0u32;
pub const X3_BTYPE_QP_INST_WORD_POS_X: u32 = 23u32;
pub const X3_BTYPE_QP_INST_WORD_X: u32 = 2u32;
pub const X3_BTYPE_QP_SIZE_X: u32 = 9u32;
pub const X3_D_WH_INST_WORD_POS_X: u32 = 24u32;
pub const X3_D_WH_INST_WORD_X: u32 = 3u32;
pub const X3_D_WH_SIGN_VAL_POS_X: u32 = 0u32;
pub const X3_D_WH_SIZE_X: u32 = 3u32;
pub const X3_EMPTY_INST_VAL_POS_X: u32 = 0u32;
pub const X3_EMPTY_INST_WORD_POS_X: u32 = 14u32;
pub const X3_EMPTY_INST_WORD_X: u32 = 1u32;
pub const X3_EMPTY_SIZE_X: u32 = 2u32;
pub const X3_IMM20_INST_WORD_POS_X: u32 = 4u32;
pub const X3_IMM20_INST_WORD_X: u32 = 3u32;
pub const X3_IMM20_SIGN_VAL_POS_X: u32 = 0u32;
pub const X3_IMM20_SIZE_X: u32 = 20u32;
pub const X3_IMM39_1_INST_WORD_POS_X: u32 = 0u32;
pub const X3_IMM39_1_INST_WORD_X: u32 = 2u32;
pub const X3_IMM39_1_SIGN_VAL_POS_X: u32 = 36u32;
pub const X3_IMM39_1_SIZE_X: u32 = 23u32;
pub const X3_IMM39_2_INST_WORD_POS_X: u32 = 16u32;
pub const X3_IMM39_2_INST_WORD_X: u32 = 1u32;
pub const X3_IMM39_2_SIGN_VAL_POS_X: u32 = 20u32;
pub const X3_IMM39_2_SIZE_X: u32 = 16u32;
pub const X3_I_INST_WORD_POS_X: u32 = 27u32;
pub const X3_I_INST_WORD_X: u32 = 3u32;
pub const X3_I_SIGN_VAL_POS_X: u32 = 59u32;
pub const X3_I_SIZE_X: u32 = 1u32;
pub const X3_OPCODE_INST_WORD_POS_X: u32 = 28u32;
pub const X3_OPCODE_INST_WORD_X: u32 = 3u32;
pub const X3_OPCODE_SIGN_VAL_POS_X: u32 = 0u32;
pub const X3_OPCODE_SIZE_X: u32 = 4u32;
pub const X3_P_INST_WORD_POS_X: u32 = 0u32;
pub const X3_P_INST_WORD_X: u32 = 3u32;
pub const X3_P_SIGN_VAL_POS_X: u32 = 0u32;
pub const X3_P_SIZE_X: u32 = 4u32;
pub const X3_TMPLT_INST_WORD_POS_X: u32 = 0u32;
pub const X3_TMPLT_INST_WORD_X: u32 = 0u32;
pub const X3_TMPLT_SIGN_VAL_POS_X: u32 = 0u32;
pub const X3_TMPLT_SIZE_X: u32 = 4u32;
pub const X86_CACHE_ALIGNMENT_SIZE: u32 = 64u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XSAVE_CET_U_FORMAT {
    pub Ia32CetUMsr: u64,
    pub Ia32Pl3SspMsr: u64,
}
pub const XSTATE_ALIGN_BIT: u32 = 1u32;
pub const XSTATE_AMX_TILE_CONFIG: u32 = 17u32;
pub const XSTATE_AMX_TILE_DATA: u32 = 18u32;
pub const XSTATE_AVX: u32 = 2u32;
pub const XSTATE_AVX512_KMASK: u32 = 5u32;
pub const XSTATE_AVX512_ZMM: u32 = 7u32;
pub const XSTATE_AVX512_ZMM_H: u32 = 6u32;
pub const XSTATE_CET_S: u32 = 12u32;
pub const XSTATE_CET_U: u32 = 11u32;
pub const XSTATE_COMPACTION_ENABLE: u32 = 63u32;
pub const XSTATE_CONTROLFLAG_XFD_MASK: u32 = 4u32;
pub const XSTATE_CONTROLFLAG_XSAVEC_MASK: u32 = 2u32;
pub const XSTATE_CONTROLFLAG_XSAVEOPT_MASK: u32 = 1u32;
pub const XSTATE_GSSE: u32 = 2u32;
pub const XSTATE_IPT: u32 = 8u32;
pub const XSTATE_LEGACY_FLOATING_POINT: u32 = 0u32;
pub const XSTATE_LEGACY_SSE: u32 = 1u32;
pub const XSTATE_LWP: u32 = 62u32;
pub const XSTATE_MPX_BNDCSR: u32 = 4u32;
pub const XSTATE_MPX_BNDREGS: u32 = 3u32;
pub const XSTATE_PASID: u32 = 10u32;
pub const XSTATE_XFD_BIT: u32 = 2u32;
pub const _MM_HINT_NTA: u32 = 0u32;
pub const _MM_HINT_T0: u32 = 1u32;
pub const _MM_HINT_T1: u32 = 2u32;
pub const _MM_HINT_T2: u32 = 3u32;
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct remoteMETAFILEPICT {
    pub mm: i32,
    pub xExt: i32,
    pub yExt: i32,
    pub hMF: *mut userHMETAFILE,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for remoteMETAFILEPICT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct userBITMAP {
    pub bmType: i32,
    pub bmWidth: i32,
    pub bmHeight: i32,
    pub bmWidthBytes: i32,
    pub bmPlanes: u16,
    pub bmBitsPixel: u16,
    pub cbSize: u32,
    pub pBuffer: [u8; 1],
}
impl Default for userBITMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct userCLIPFORMAT {
    pub fContext: i32,
    pub u: userCLIPFORMAT_0,
}
impl Default for userCLIPFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union userCLIPFORMAT_0 {
    pub dwValue: u32,
    pub pwszName: windows_core::PWSTR,
}
impl Default for userCLIPFORMAT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct userHBITMAP {
    pub fContext: i32,
    pub u: userHBITMAP_0,
}
impl Default for userHBITMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union userHBITMAP_0 {
    pub hInproc: i32,
    pub hRemote: *mut userBITMAP,
    pub hInproc64: i64,
}
impl Default for userHBITMAP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct userHENHMETAFILE {
    pub fContext: i32,
    pub u: userHENHMETAFILE_0,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for userHENHMETAFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub union userHENHMETAFILE_0 {
    pub hInproc: i32,
    pub hRemote: *mut super::Com::BYTE_BLOB,
    pub hInproc64: i64,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for userHENHMETAFILE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct userHGLOBAL {
    pub fContext: i32,
    pub u: userHGLOBAL_0,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for userHGLOBAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub union userHGLOBAL_0 {
    pub hInproc: i32,
    pub hRemote: *mut super::Com::FLAGGED_BYTE_BLOB,
    pub hInproc64: i64,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for userHGLOBAL_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct userHMETAFILE {
    pub fContext: i32,
    pub u: userHMETAFILE_0,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for userHMETAFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub union userHMETAFILE_0 {
    pub hInproc: i32,
    pub hRemote: *mut super::Com::BYTE_BLOB,
    pub hInproc64: i64,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for userHMETAFILE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub struct userHMETAFILEPICT {
    pub fContext: i32,
    pub u: userHMETAFILEPICT_0,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for userHMETAFILEPICT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy)]
pub union userHMETAFILEPICT_0 {
    pub hInproc: i32,
    pub hRemote: *mut remoteMETAFILEPICT,
    pub hInproc64: i64,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for userHMETAFILEPICT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct userHPALETTE {
    pub fContext: i32,
    pub u: userHPALETTE_0,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for userHPALETTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub union userHPALETTE_0 {
    pub hInproc: i32,
    pub hRemote: *mut super::super::Graphics::Gdi::LOGPALETTE,
    pub hInproc64: i64,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for userHPALETTE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
