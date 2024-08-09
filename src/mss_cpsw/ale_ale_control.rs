#[doc = "Register `ALE_ALE_CONTROL` reader"]
pub type R = crate::R<AleAleControlSpec>;
#[doc = "Register `ALE_ALE_CONTROL` writer"]
pub type W = crate::W<AleAleControlSpec>;
#[doc = "Field `ENABLE_BROADCAST_AND` reader - 0:0\\]
Enable Broadcast and Multicast Rate Limit#br# 0 - Broadcast/Multicast rates not limited#br# 1 - Broadcast/Multicast packet reception limited to the port control register rate limit fields."]
pub type EnableBroadcastAndR = crate::BitReader;
#[doc = "Field `ENABLE_BROADCAST_AND` writer - 0:0\\]
Enable Broadcast and Multicast Rate Limit#br# 0 - Broadcast/Multicast rates not limited#br# 1 - Broadcast/Multicast packet reception limited to the port control register rate limit fields."]
pub type EnableBroadcastAndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_MAC_AUTHORIZATION` reader - 1:1\\]
Enable MAC Authorization Mode - Mac authorization mode requires that all table entries be made by the host software. There is no auto learning of addresses in authorization mode and the packet will be dropped if the source address is not found (and the destination address is not a multicast address with the super table entry bit set).#br# 0 - The ALE is not in MAC authorization mode#br# 1 - The ALE is in MAC authorization mode"]
pub type EnableMacAuthorizationR = crate::BitReader;
#[doc = "Field `ENABLE_MAC_AUTHORIZATION` writer - 1:1\\]
Enable MAC Authorization Mode - Mac authorization mode requires that all table entries be made by the host software. There is no auto learning of addresses in authorization mode and the packet will be dropped if the source address is not found (and the destination address is not a multicast address with the super table entry bit set).#br# 0 - The ALE is not in MAC authorization mode#br# 1 - The ALE is in MAC authorization mode"]
pub type EnableMacAuthorizationW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALE_VLAN_AWARE` reader - 2:2\\]
ALE VLAN Aware - Determines how traffic is forwarded using VLAN rules.#br# 0 - Simple switch rules, packets forwarded to all ports for unknown destinations.#br# 1 - VLAN Aware rules, packets forwarded based on VLAN members"]
pub type AleVlanAwareR = crate::BitReader;
#[doc = "Field `ALE_VLAN_AWARE` writer - 2:2\\]
ALE VLAN Aware - Determines how traffic is forwarded using VLAN rules.#br# 0 - Simple switch rules, packets forwarded to all ports for unknown destinations.#br# 1 - VLAN Aware rules, packets forwarded based on VLAN members"]
pub type AleVlanAwareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RATE_LIMIT_TRANSMIT` reader - 3:3\\]
Rate Limit Transmit mode#br# 0 - Broadcast and multicast rate limit counters are received port based#br# 1 - Broadcast and multicast rate limit counters are transmit port based"]
pub type RateLimitTransmitR = crate::BitReader;
#[doc = "Field `RATE_LIMIT_TRANSMIT` writer - 3:3\\]
Rate Limit Transmit mode#br# 0 - Broadcast and multicast rate limit counters are received port based#br# 1 - Broadcast and multicast rate limit counters are transmit port based"]
pub type RateLimitTransmitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALE_BYPASS_WHEN` reader - 4:4\\]
ALE Bypass - When set, packets received on non-host ports are sent to the host. It is expected that packets from the host are directed to the particular port.#br# 0 - no bypass#br# 1 - bypass the ALE"]
pub type AleBypassWhenR = crate::BitReader;
#[doc = "Field `ALE_BYPASS_WHEN` writer - 4:4\\]
ALE Bypass - When set, packets received on non-host ports are sent to the host. It is expected that packets from the host are directed to the particular port.#br# 0 - no bypass#br# 1 - bypass the ALE"]
pub type AleBypassWhenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_OUI_DENY` reader - 5:5\\]
Enable OUI Deny Mode - When set, any packet with a non-matching OUI source address will be dropped to the host unless the packet destination address matches a supervisory destination address table entry. When cleared, any packet source address matching an OUI address table entry will be dropped to the host unless the destination address matches with a supervisory destination address table entry."]
pub type EnableOuiDenyR = crate::BitReader;
#[doc = "Field `ENABLE_OUI_DENY` writer - 5:5\\]
Enable OUI Deny Mode - When set, any packet with a non-matching OUI source address will be dropped to the host unless the packet destination address matches a supervisory destination address table entry. When cleared, any packet source address matching an OUI address table entry will be dropped to the host unless the destination address matches with a supervisory destination address table entry."]
pub type EnableOuiDenyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_VLAN_ID` reader - 6:6\\]
Enable VLAN ID = 0 Mode#br# 0 - Process the priority tagged packet with VID = PORT_VLAN\\[11:0\\].#br# 1 - Process the priority tagged packet with VID = 0."]
pub type EnableVlanIdR = crate::BitReader;
#[doc = "Field `ENABLE_VLAN_ID` writer - 6:6\\]
Enable VLAN ID = 0 Mode#br# 0 - Process the priority tagged packet with VID = PORT_VLAN\\[11:0\\].#br# 1 - Process the priority tagged packet with VID = 0."]
pub type EnableVlanIdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEARN_NO_VID` reader - 7:7\\]
Learn No VID -#br# 0 - VID is learned with the source address#br# 1 - VID is not learned with the source address (source address is not tied to VID). Determines the entry type."]
pub type LearnNoVidR = crate::BitReader;
#[doc = "Field `LEARN_NO_VID` writer - 7:7\\]
Learn No VID -#br# 0 - VID is learned with the source address#br# 1 - VID is not learned with the source address (source address is not tied to VID). Determines the entry type."]
pub type LearnNoVidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNKNOWN_UNICAST_PACKETS` reader - 8:8\\]
Unknown unicast packets flood to host#br# 0 - unknown unicast packets are not sent to the host#br# 1 - unknown unicast packets flood to host port as well as other ports"]
pub type UnknownUnicastPacketsR = crate::BitReader;
#[doc = "Field `UNKNOWN_UNICAST_PACKETS` writer - 8:8\\]
Unknown unicast packets flood to host#br# 0 - unknown unicast packets are not sent to the host#br# 1 - unknown unicast packets flood to host port as well as other ports"]
pub type UnknownUnicastPacketsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIRROR_SOURCE_PORT` reader - 10:10\\]
Mirror Source Port Enable - This field enables the source port mirror option. When this bit is set any port with the ~ipX_mirror_sp set in the ALE Port Control registers set will have its received traffic also sent to the ~imirror_top port."]
pub type MirrorSourcePortR = crate::BitReader;
#[doc = "Field `MIRROR_SOURCE_PORT` writer - 10:10\\]
Mirror Source Port Enable - This field enables the source port mirror option. When this bit is set any port with the ~ipX_mirror_sp set in the ALE Port Control registers set will have its received traffic also sent to the ~imirror_top port."]
pub type MirrorSourcePortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIRROR_DESTINATION_PORT_1` reader - 11:11\\]
Mirror Destination Port Enable - This field enables the destination port mirror option. When this bit is set any traffic destined for the ~imirror_dp port will have its transmit traffic also sent to the ~imirror_top port."]
pub type MirrorDestinationPort1R = crate::BitReader;
#[doc = "Field `MIRROR_DESTINATION_PORT_1` writer - 11:11\\]
Mirror Destination Port Enable - This field enables the destination port mirror option. When this bit is set any traffic destined for the ~imirror_dp port will have its transmit traffic also sent to the ~imirror_top port."]
pub type MirrorDestinationPort1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIRROR_MATCH_ENTRY` reader - 12:12\\]
Mirror Match Entry Enable - This field enables the match mirror option. When this bit is set any traffic whose destination, source, VLAN or OUI matches the ~imirror_midx entry index will have that traffic also sent to the ~imirror_top port."]
pub type MirrorMatchEntryR = crate::BitReader;
#[doc = "Field `MIRROR_MATCH_ENTRY` writer - 12:12\\]
Mirror Match Entry Enable - This field enables the match mirror option. When this bit is set any traffic whose destination, source, VLAN or OUI matches the ~imirror_midx entry index will have that traffic also sent to the ~imirror_top port."]
pub type MirrorMatchEntryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNKNOWN_VLAN_NO` reader - 13:13\\]
Unknown VLAN No Learn - This field when set will prevent source addresses of unknown VLAN IDs from being automatically added into the look up table if learning is enabled."]
pub type UnknownVlanNoR = crate::BitReader;
#[doc = "Field `UNKNOWN_VLAN_NO` writer - 13:13\\]
Unknown VLAN No Learn - This field when set will prevent source addresses of unknown VLAN IDs from being automatically added into the look up table if learning is enabled."]
pub type UnknownVlanNoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDATE_STATIC_ENTRIES` reader - 15:15\\]
Update Static Entries - A static Entry is an entry that is not agable. When clear this bit will prevent any static entry (agable bit clear) from being updated due to port change. When set it allows static entries (agable bit clear) to update the source port if required. This bit should normally be '0' for most switch configurations."]
pub type UpdateStaticEntriesR = crate::BitReader;
#[doc = "Field `UPDATE_STATIC_ENTRIES` writer - 15:15\\]
Update Static Entries - A static Entry is an entry that is not agable. When clear this bit will prevent any static entry (agable bit clear) from being updated due to port change. When set it allows static entries (agable bit clear) to update the source port if required. This bit should normally be '0' for most switch configurations."]
pub type UpdateStaticEntriesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIRROR_TO_PORT` reader - 16:16\\]
Mirror To Port - This field defines the destination port for the mirror traffic. If the traffic is received or transmitted on the mirror destination port it will not be duplicated. Traffic defined as mirror traffic only may be dropped by the switch due to congestion."]
pub type MirrorToPortR = crate::BitReader;
#[doc = "Field `MIRROR_TO_PORT` writer - 16:16\\]
Mirror To Port - This field defines the destination port for the mirror traffic. If the traffic is received or transmitted on the mirror destination port it will not be duplicated. Traffic defined as mirror traffic only may be dropped by the switch due to congestion."]
pub type MirrorToPortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THE_ IUPD_BW_CTRL_FIELD` reader - 23:21\\]
The ~iupd_bw_ctrl field allows for up to 8 times the rate in which adds, updates, touches, writes, and aging updates can occur. At frequencies of 350Mhz, the table update rate should be at it lowest or 5 Million updates per second. When operating the switch core at frequencies or above, the ~iupd_bw_ctrl can be programmed more aggressive. If the ~iupd_bw_ctrl is set but the frequency of the switch subsystem is below the associated value, ALE will drop packets due to insufficient time to complete lookup under high traffic loads.#br# 0 - 350Mhz, 5M#br# 1 - 359Mhz, 11M#br# 2 - 367Mhz, 16M#br# 3 - 375Mhz, 22M#br# 4 - 384Mhz, 28M#br# 5 - 392Mhz, 34M#br# 6 - 400Mhz, 39M#br# 7 - 409Mhz, 45M"]
pub type TheIupdBwCtrlFieldR = crate::FieldReader;
#[doc = "Field `THE_ IUPD_BW_CTRL_FIELD` writer - 23:21\\]
The ~iupd_bw_ctrl field allows for up to 8 times the rate in which adds, updates, touches, writes, and aging updates can occur. At frequencies of 350Mhz, the table update rate should be at it lowest or 5 Million updates per second. When operating the switch core at frequencies or above, the ~iupd_bw_ctrl can be programmed more aggressive. If the ~iupd_bw_ctrl is set but the frequency of the switch subsystem is below the associated value, ALE will drop packets due to insufficient time to complete lookup under high traffic loads.#br# 0 - 350Mhz, 5M#br# 1 - 359Mhz, 11M#br# 2 - 367Mhz, 16M#br# 3 - 375Mhz, 22M#br# 4 - 384Mhz, 28M#br# 5 - 392Mhz, 34M#br# 6 - 400Mhz, 39M#br# 7 - 409Mhz, 45M"]
pub type TheIupdBwCtrlFieldW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MIRROR_DESTINATION_PORT` reader - 24:24\\]
Mirror Destination Port - This field defines the port to which destination traffic destined will be duplicated. That is all traffic that is forwarded to this port will also be mirrored to the ~imirror_top port."]
pub type MirrorDestinationPortR = crate::BitReader;
#[doc = "Field `MIRROR_DESTINATION_PORT` writer - 24:24\\]
Mirror Destination Port - This field defines the port to which destination traffic destined will be duplicated. That is all traffic that is forwarded to this port will also be mirrored to the ~imirror_top port."]
pub type MirrorDestinationPortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AGE_OUT_ADDRESS` reader - 29:29\\]
Age Out Address Table Now - Setting this bit causes the ALE hardware to remove (free up) any ageable table entry that does not have a set touch bit. This bit is cleared when the age out process has completed. This bit may be read. The age out process takes four times the number of table entries clock cycles (4096 cycles for 1K addresses) best case (no ale packet processing during ageout) and sixty five times the number of table entries clock cycles (66560 cycles for 1K addresses) absolute worst case."]
pub type AgeOutAddressR = crate::BitReader;
#[doc = "Field `AGE_OUT_ADDRESS` writer - 29:29\\]
Age Out Address Table Now - Setting this bit causes the ALE hardware to remove (free up) any ageable table entry that does not have a set touch bit. This bit is cleared when the age out process has completed. This bit may be read. The age out process takes four times the number of table entries clock cycles (4096 cycles for 1K addresses) best case (no ale packet processing during ageout) and sixty five times the number of table entries clock cycles (66560 cycles for 1K addresses) absolute worst case."]
pub type AgeOutAddressW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_ALE_ADDRESS` reader - 30:30\\]
Clear ALE address table - Setting this bit causes the ALE hardware to write all table bit values to zero. Software must perform a clear table operation as part of the ALE setup/configuration process. Setting this bit causes all ALE accesses to be held up for 64 clocks while the clear is performed. Access to all ALE registers will be blocked (wait states) until the 64 clocks have completed. This bit cannot be read as one because the read is blocked until the clear table is completed at which time this bit is cleared to zero."]
pub type ClearAleAddressR = crate::BitReader;
#[doc = "Field `CLEAR_ALE_ADDRESS` writer - 30:30\\]
Clear ALE address table - Setting this bit causes the ALE hardware to write all table bit values to zero. Software must perform a clear table operation as part of the ALE setup/configuration process. Setting this bit causes all ALE accesses to be held up for 64 clocks while the clear is performed. Access to all ALE registers will be blocked (wait states) until the 64 clocks have completed. This bit cannot be read as one because the read is blocked until the clear table is completed at which time this bit is cleared to zero."]
pub type ClearAleAddressW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_ALE` reader - 31:31\\]
Enable ALE#br# 0 - Drop all packets#br# 1 - Enable ALE packet processing"]
pub type EnableAleR = crate::BitReader;
#[doc = "Field `ENABLE_ALE` writer - 31:31\\]
Enable ALE#br# 0 - Drop all packets#br# 1 - Enable ALE packet processing"]
pub type EnableAleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable Broadcast and Multicast Rate Limit#br# 0 - Broadcast/Multicast rates not limited#br# 1 - Broadcast/Multicast packet reception limited to the port control register rate limit fields."]
    #[inline(always)]
    pub fn enable_broadcast_and(&self) -> EnableBroadcastAndR {
        EnableBroadcastAndR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable MAC Authorization Mode - Mac authorization mode requires that all table entries be made by the host software. There is no auto learning of addresses in authorization mode and the packet will be dropped if the source address is not found (and the destination address is not a multicast address with the super table entry bit set).#br# 0 - The ALE is not in MAC authorization mode#br# 1 - The ALE is in MAC authorization mode"]
    #[inline(always)]
    pub fn enable_mac_authorization(&self) -> EnableMacAuthorizationR {
        EnableMacAuthorizationR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
ALE VLAN Aware - Determines how traffic is forwarded using VLAN rules.#br# 0 - Simple switch rules, packets forwarded to all ports for unknown destinations.#br# 1 - VLAN Aware rules, packets forwarded based on VLAN members"]
    #[inline(always)]
    pub fn ale_vlan_aware(&self) -> AleVlanAwareR {
        AleVlanAwareR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Rate Limit Transmit mode#br# 0 - Broadcast and multicast rate limit counters are received port based#br# 1 - Broadcast and multicast rate limit counters are transmit port based"]
    #[inline(always)]
    pub fn rate_limit_transmit(&self) -> RateLimitTransmitR {
        RateLimitTransmitR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
ALE Bypass - When set, packets received on non-host ports are sent to the host. It is expected that packets from the host are directed to the particular port.#br# 0 - no bypass#br# 1 - bypass the ALE"]
    #[inline(always)]
    pub fn ale_bypass_when(&self) -> AleBypassWhenR {
        AleBypassWhenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Enable OUI Deny Mode - When set, any packet with a non-matching OUI source address will be dropped to the host unless the packet destination address matches a supervisory destination address table entry. When cleared, any packet source address matching an OUI address table entry will be dropped to the host unless the destination address matches with a supervisory destination address table entry."]
    #[inline(always)]
    pub fn enable_oui_deny(&self) -> EnableOuiDenyR {
        EnableOuiDenyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable VLAN ID = 0 Mode#br# 0 - Process the priority tagged packet with VID = PORT_VLAN\\[11:0\\].#br# 1 - Process the priority tagged packet with VID = 0."]
    #[inline(always)]
    pub fn enable_vlan_id(&self) -> EnableVlanIdR {
        EnableVlanIdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Learn No VID -#br# 0 - VID is learned with the source address#br# 1 - VID is not learned with the source address (source address is not tied to VID). Determines the entry type."]
    #[inline(always)]
    pub fn learn_no_vid(&self) -> LearnNoVidR {
        LearnNoVidR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Unknown unicast packets flood to host#br# 0 - unknown unicast packets are not sent to the host#br# 1 - unknown unicast packets flood to host port as well as other ports"]
    #[inline(always)]
    pub fn unknown_unicast_packets(&self) -> UnknownUnicastPacketsR {
        UnknownUnicastPacketsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Mirror Source Port Enable - This field enables the source port mirror option. When this bit is set any port with the ~ipX_mirror_sp set in the ALE Port Control registers set will have its received traffic also sent to the ~imirror_top port."]
    #[inline(always)]
    pub fn mirror_source_port(&self) -> MirrorSourcePortR {
        MirrorSourcePortR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Mirror Destination Port Enable - This field enables the destination port mirror option. When this bit is set any traffic destined for the ~imirror_dp port will have its transmit traffic also sent to the ~imirror_top port."]
    #[inline(always)]
    pub fn mirror_destination_port_1(&self) -> MirrorDestinationPort1R {
        MirrorDestinationPort1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Mirror Match Entry Enable - This field enables the match mirror option. When this bit is set any traffic whose destination, source, VLAN or OUI matches the ~imirror_midx entry index will have that traffic also sent to the ~imirror_top port."]
    #[inline(always)]
    pub fn mirror_match_entry(&self) -> MirrorMatchEntryR {
        MirrorMatchEntryR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Unknown VLAN No Learn - This field when set will prevent source addresses of unknown VLAN IDs from being automatically added into the look up table if learning is enabled."]
    #[inline(always)]
    pub fn unknown_vlan_no(&self) -> UnknownVlanNoR {
        UnknownVlanNoR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Update Static Entries - A static Entry is an entry that is not agable. When clear this bit will prevent any static entry (agable bit clear) from being updated due to port change. When set it allows static entries (agable bit clear) to update the source port if required. This bit should normally be '0' for most switch configurations."]
    #[inline(always)]
    pub fn update_static_entries(&self) -> UpdateStaticEntriesR {
        UpdateStaticEntriesR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Mirror To Port - This field defines the destination port for the mirror traffic. If the traffic is received or transmitted on the mirror destination port it will not be duplicated. Traffic defined as mirror traffic only may be dropped by the switch due to congestion."]
    #[inline(always)]
    pub fn mirror_to_port(&self) -> MirrorToPortR {
        MirrorToPortR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 21:23 - 23:21\\]
The ~iupd_bw_ctrl field allows for up to 8 times the rate in which adds, updates, touches, writes, and aging updates can occur. At frequencies of 350Mhz, the table update rate should be at it lowest or 5 Million updates per second. When operating the switch core at frequencies or above, the ~iupd_bw_ctrl can be programmed more aggressive. If the ~iupd_bw_ctrl is set but the frequency of the switch subsystem is below the associated value, ALE will drop packets due to insufficient time to complete lookup under high traffic loads.#br# 0 - 350Mhz, 5M#br# 1 - 359Mhz, 11M#br# 2 - 367Mhz, 16M#br# 3 - 375Mhz, 22M#br# 4 - 384Mhz, 28M#br# 5 - 392Mhz, 34M#br# 6 - 400Mhz, 39M#br# 7 - 409Mhz, 45M"]
    #[inline(always)]
    pub fn the_iupd_bw_ctrl_field(&self) -> TheIupdBwCtrlFieldR {
        TheIupdBwCtrlFieldR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Mirror Destination Port - This field defines the port to which destination traffic destined will be duplicated. That is all traffic that is forwarded to this port will also be mirrored to the ~imirror_top port."]
    #[inline(always)]
    pub fn mirror_destination_port(&self) -> MirrorDestinationPortR {
        MirrorDestinationPortR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Age Out Address Table Now - Setting this bit causes the ALE hardware to remove (free up) any ageable table entry that does not have a set touch bit. This bit is cleared when the age out process has completed. This bit may be read. The age out process takes four times the number of table entries clock cycles (4096 cycles for 1K addresses) best case (no ale packet processing during ageout) and sixty five times the number of table entries clock cycles (66560 cycles for 1K addresses) absolute worst case."]
    #[inline(always)]
    pub fn age_out_address(&self) -> AgeOutAddressR {
        AgeOutAddressR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Clear ALE address table - Setting this bit causes the ALE hardware to write all table bit values to zero. Software must perform a clear table operation as part of the ALE setup/configuration process. Setting this bit causes all ALE accesses to be held up for 64 clocks while the clear is performed. Access to all ALE registers will be blocked (wait states) until the 64 clocks have completed. This bit cannot be read as one because the read is blocked until the clear table is completed at which time this bit is cleared to zero."]
    #[inline(always)]
    pub fn clear_ale_address(&self) -> ClearAleAddressR {
        ClearAleAddressR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Enable ALE#br# 0 - Drop all packets#br# 1 - Enable ALE packet processing"]
    #[inline(always)]
    pub fn enable_ale(&self) -> EnableAleR {
        EnableAleR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable Broadcast and Multicast Rate Limit#br# 0 - Broadcast/Multicast rates not limited#br# 1 - Broadcast/Multicast packet reception limited to the port control register rate limit fields."]
    #[inline(always)]
    #[must_use]
    pub fn enable_broadcast_and(&mut self) -> EnableBroadcastAndW<AleAleControlSpec> {
        EnableBroadcastAndW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable MAC Authorization Mode - Mac authorization mode requires that all table entries be made by the host software. There is no auto learning of addresses in authorization mode and the packet will be dropped if the source address is not found (and the destination address is not a multicast address with the super table entry bit set).#br# 0 - The ALE is not in MAC authorization mode#br# 1 - The ALE is in MAC authorization mode"]
    #[inline(always)]
    #[must_use]
    pub fn enable_mac_authorization(&mut self) -> EnableMacAuthorizationW<AleAleControlSpec> {
        EnableMacAuthorizationW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
ALE VLAN Aware - Determines how traffic is forwarded using VLAN rules.#br# 0 - Simple switch rules, packets forwarded to all ports for unknown destinations.#br# 1 - VLAN Aware rules, packets forwarded based on VLAN members"]
    #[inline(always)]
    #[must_use]
    pub fn ale_vlan_aware(&mut self) -> AleVlanAwareW<AleAleControlSpec> {
        AleVlanAwareW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Rate Limit Transmit mode#br# 0 - Broadcast and multicast rate limit counters are received port based#br# 1 - Broadcast and multicast rate limit counters are transmit port based"]
    #[inline(always)]
    #[must_use]
    pub fn rate_limit_transmit(&mut self) -> RateLimitTransmitW<AleAleControlSpec> {
        RateLimitTransmitW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
ALE Bypass - When set, packets received on non-host ports are sent to the host. It is expected that packets from the host are directed to the particular port.#br# 0 - no bypass#br# 1 - bypass the ALE"]
    #[inline(always)]
    #[must_use]
    pub fn ale_bypass_when(&mut self) -> AleBypassWhenW<AleAleControlSpec> {
        AleBypassWhenW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Enable OUI Deny Mode - When set, any packet with a non-matching OUI source address will be dropped to the host unless the packet destination address matches a supervisory destination address table entry. When cleared, any packet source address matching an OUI address table entry will be dropped to the host unless the destination address matches with a supervisory destination address table entry."]
    #[inline(always)]
    #[must_use]
    pub fn enable_oui_deny(&mut self) -> EnableOuiDenyW<AleAleControlSpec> {
        EnableOuiDenyW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Enable VLAN ID = 0 Mode#br# 0 - Process the priority tagged packet with VID = PORT_VLAN\\[11:0\\].#br# 1 - Process the priority tagged packet with VID = 0."]
    #[inline(always)]
    #[must_use]
    pub fn enable_vlan_id(&mut self) -> EnableVlanIdW<AleAleControlSpec> {
        EnableVlanIdW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Learn No VID -#br# 0 - VID is learned with the source address#br# 1 - VID is not learned with the source address (source address is not tied to VID). Determines the entry type."]
    #[inline(always)]
    #[must_use]
    pub fn learn_no_vid(&mut self) -> LearnNoVidW<AleAleControlSpec> {
        LearnNoVidW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Unknown unicast packets flood to host#br# 0 - unknown unicast packets are not sent to the host#br# 1 - unknown unicast packets flood to host port as well as other ports"]
    #[inline(always)]
    #[must_use]
    pub fn unknown_unicast_packets(&mut self) -> UnknownUnicastPacketsW<AleAleControlSpec> {
        UnknownUnicastPacketsW::new(self, 8)
    }
    #[doc = "Bit 10 - 10:10\\]
Mirror Source Port Enable - This field enables the source port mirror option. When this bit is set any port with the ~ipX_mirror_sp set in the ALE Port Control registers set will have its received traffic also sent to the ~imirror_top port."]
    #[inline(always)]
    #[must_use]
    pub fn mirror_source_port(&mut self) -> MirrorSourcePortW<AleAleControlSpec> {
        MirrorSourcePortW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Mirror Destination Port Enable - This field enables the destination port mirror option. When this bit is set any traffic destined for the ~imirror_dp port will have its transmit traffic also sent to the ~imirror_top port."]
    #[inline(always)]
    #[must_use]
    pub fn mirror_destination_port_1(&mut self) -> MirrorDestinationPort1W<AleAleControlSpec> {
        MirrorDestinationPort1W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Mirror Match Entry Enable - This field enables the match mirror option. When this bit is set any traffic whose destination, source, VLAN or OUI matches the ~imirror_midx entry index will have that traffic also sent to the ~imirror_top port."]
    #[inline(always)]
    #[must_use]
    pub fn mirror_match_entry(&mut self) -> MirrorMatchEntryW<AleAleControlSpec> {
        MirrorMatchEntryW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Unknown VLAN No Learn - This field when set will prevent source addresses of unknown VLAN IDs from being automatically added into the look up table if learning is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn unknown_vlan_no(&mut self) -> UnknownVlanNoW<AleAleControlSpec> {
        UnknownVlanNoW::new(self, 13)
    }
    #[doc = "Bit 15 - 15:15\\]
Update Static Entries - A static Entry is an entry that is not agable. When clear this bit will prevent any static entry (agable bit clear) from being updated due to port change. When set it allows static entries (agable bit clear) to update the source port if required. This bit should normally be '0' for most switch configurations."]
    #[inline(always)]
    #[must_use]
    pub fn update_static_entries(&mut self) -> UpdateStaticEntriesW<AleAleControlSpec> {
        UpdateStaticEntriesW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Mirror To Port - This field defines the destination port for the mirror traffic. If the traffic is received or transmitted on the mirror destination port it will not be duplicated. Traffic defined as mirror traffic only may be dropped by the switch due to congestion."]
    #[inline(always)]
    #[must_use]
    pub fn mirror_to_port(&mut self) -> MirrorToPortW<AleAleControlSpec> {
        MirrorToPortW::new(self, 16)
    }
    #[doc = "Bits 21:23 - 23:21\\]
The ~iupd_bw_ctrl field allows for up to 8 times the rate in which adds, updates, touches, writes, and aging updates can occur. At frequencies of 350Mhz, the table update rate should be at it lowest or 5 Million updates per second. When operating the switch core at frequencies or above, the ~iupd_bw_ctrl can be programmed more aggressive. If the ~iupd_bw_ctrl is set but the frequency of the switch subsystem is below the associated value, ALE will drop packets due to insufficient time to complete lookup under high traffic loads.#br# 0 - 350Mhz, 5M#br# 1 - 359Mhz, 11M#br# 2 - 367Mhz, 16M#br# 3 - 375Mhz, 22M#br# 4 - 384Mhz, 28M#br# 5 - 392Mhz, 34M#br# 6 - 400Mhz, 39M#br# 7 - 409Mhz, 45M"]
    #[inline(always)]
    #[must_use]
    pub fn the_iupd_bw_ctrl_field(&mut self) -> TheIupdBwCtrlFieldW<AleAleControlSpec> {
        TheIupdBwCtrlFieldW::new(self, 21)
    }
    #[doc = "Bit 24 - 24:24\\]
Mirror Destination Port - This field defines the port to which destination traffic destined will be duplicated. That is all traffic that is forwarded to this port will also be mirrored to the ~imirror_top port."]
    #[inline(always)]
    #[must_use]
    pub fn mirror_destination_port(&mut self) -> MirrorDestinationPortW<AleAleControlSpec> {
        MirrorDestinationPortW::new(self, 24)
    }
    #[doc = "Bit 29 - 29:29\\]
Age Out Address Table Now - Setting this bit causes the ALE hardware to remove (free up) any ageable table entry that does not have a set touch bit. This bit is cleared when the age out process has completed. This bit may be read. The age out process takes four times the number of table entries clock cycles (4096 cycles for 1K addresses) best case (no ale packet processing during ageout) and sixty five times the number of table entries clock cycles (66560 cycles for 1K addresses) absolute worst case."]
    #[inline(always)]
    #[must_use]
    pub fn age_out_address(&mut self) -> AgeOutAddressW<AleAleControlSpec> {
        AgeOutAddressW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Clear ALE address table - Setting this bit causes the ALE hardware to write all table bit values to zero. Software must perform a clear table operation as part of the ALE setup/configuration process. Setting this bit causes all ALE accesses to be held up for 64 clocks while the clear is performed. Access to all ALE registers will be blocked (wait states) until the 64 clocks have completed. This bit cannot be read as one because the read is blocked until the clear table is completed at which time this bit is cleared to zero."]
    #[inline(always)]
    #[must_use]
    pub fn clear_ale_address(&mut self) -> ClearAleAddressW<AleAleControlSpec> {
        ClearAleAddressW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Enable ALE#br# 0 - Drop all packets#br# 1 - Enable ALE packet processing"]
    #[inline(always)]
    #[must_use]
    pub fn enable_ale(&mut self) -> EnableAleW<AleAleControlSpec> {
        EnableAleW::new(self, 31)
    }
}
#[doc = "The ALE Control Register is used to set the ALE modes used for all ports.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AleAleControlSpec;
impl crate::RegisterSpec for AleAleControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_ale_control::R`](R) reader structure"]
impl crate::Readable for AleAleControlSpec {}
#[doc = "`write(|w| ..)` method takes [`ale_ale_control::W`](W) writer structure"]
impl crate::Writable for AleAleControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_ALE_CONTROL to value 0"]
impl crate::Resettable for AleAleControlSpec {
    const RESET_VALUE: u32 = 0;
}
