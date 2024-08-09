#[doc = "Register `ALE_ALE_CTRL2` reader"]
pub type R = crate::R<AleAleCtrl2Spec>;
#[doc = "Register `ALE_ALE_CTRL2` writer"]
pub type W = crate::W<AleAleCtrl2Spec>;
#[doc = "Field `MIRROR_INDEX_THIS` reader - 4:0\\]
Mirror Index - This field is the ALE lookup table entry index that when a match occurs will cause this traffic to be mirrored to the ~imirror_top port. That is any VLAN, ONU or address with or withou VLAN can be selected for traffic mirroring."]
pub type MirrorIndexThisR = crate::FieldReader;
#[doc = "Field `MIRROR_INDEX_THIS` writer - 4:0\\]
Mirror Index - This field is the ALE lookup table entry index that when a match occurs will cause this traffic to be mirrored to the ~imirror_top port. That is any VLAN, ONU or address with or withou VLAN can be selected for traffic mirroring."]
pub type MirrorIndexThisW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `THE_ MULTIHOST_ALLOWS` reader - 15:15\\]
The ~multihost allows host traffic to be sent bact to the host if the DA is market for the host port."]
pub type TheMultihostAllowsR = crate::BitReader;
#[doc = "Field `THE_ MULTIHOST_ALLOWS` writer - 15:15\\]
The ~multihost allows host traffic to be sent bact to the host if the DA is market for the host port."]
pub type TheMultihostAllowsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRUNK_BASE_THIS` reader - 18:16\\]
Trunk Base - This field is the hash formula starting value. Changing this value will cause the packet distribution on trunk ports to be changed. If all the ~itrk_en_dst, ~itrk_en_src, ~itrk_en_pri and ~itrk_en_vlan are '0', this value is used as the distribution index. That is a '0' will select the 1st bit of an 'N' link trunk, a '1' will select the second, etc. #br#Below is the distribution across the trunk links. The first number in the ~iitalic sequence indicates the traffic is sent to the lowest numbered port of a trunk group. For example if you have a 3 port trunk, the hash result 0 will go to the base port (0), hash result 1 will go to the highest port of the trunk group (2), hash result 2 will go to the middle port (1), etc.#br# 1 - ~i00000000#br# 2 - ~i01010101#br# 3 - ~i02102102#br# 4 - ~i03210321"]
pub type TrunkBaseThisR = crate::FieldReader;
#[doc = "Field `TRUNK_BASE_THIS` writer - 18:16\\]
Trunk Base - This field is the hash formula starting value. Changing this value will cause the packet distribution on trunk ports to be changed. If all the ~itrk_en_dst, ~itrk_en_src, ~itrk_en_pri and ~itrk_en_vlan are '0', this value is used as the distribution index. That is a '0' will select the 1st bit of an 'N' link trunk, a '1' will select the second, etc. #br#Below is the distribution across the trunk links. The first number in the ~iitalic sequence indicates the traffic is sent to the lowest numbered port of a trunk group. For example if you have a 3 port trunk, the hash result 0 will go to the base port (0), hash result 1 will go to the highest port of the trunk group (2), hash result 2 will go to the middle port (1), etc.#br# 1 - ~i00000000#br# 2 - ~i01010101#br# 3 - ~i02102102#br# 4 - ~i03210321"]
pub type TrunkBaseThisW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DEFAULT_LIMIT_NEXT` reader - 20:20\\]
Default limit next header field will cause an IPv4 protocol or IPv6 next header packet to be dropped if a VLAN entry is not found and the protocol or next header does not match the ~iALE_NXT_HDR register values."]
pub type DefaultLimitNextR = crate::BitReader;
#[doc = "Field `DEFAULT_LIMIT_NEXT` writer - 20:20\\]
Default limit next header field will cause an IPv4 protocol or IPv6 next header packet to be dropped if a VLAN entry is not found and the protocol or next header does not match the ~iALE_NXT_HDR register values."]
pub type DefaultLimitNextW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEFAULT_NO_FRAG` reader - 21:21\\]
Default No Frag field will cause an IPv4 fragmented packet to be dropped if a VLAN entry is not found."]
pub type DefaultNoFragR = crate::BitReader;
#[doc = "Field `DEFAULT_NO_FRAG` writer - 21:21\\]
Default No Frag field will cause an IPv4 fragmented packet to be dropped if a VLAN entry is not found."]
pub type DefaultNoFragW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_DROP_SOURCE` reader - 22:22\\]
No Drop Source Multicast will disable the dropping of any source address with the multicast bit set."]
pub type NoDropSourceR = crate::BitReader;
#[doc = "Field `NO_DROP_SOURCE` writer - 22:22\\]
No Drop Source Multicast will disable the dropping of any source address with the multicast bit set."]
pub type NoDropSourceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DROP_BAD_LENGTH` reader - 23:23\\]
Drop Bad Length will drop any packet that the 802.3 length field is larger than the packet. Ethertypes 0-1500 are 802.3 lengths, all others are Ether types."]
pub type DropBadLengthR = crate::BitReader;
#[doc = "Field `DROP_BAD_LENGTH` writer - 23:23\\]
Drop Bad Length will drop any packet that the 802.3 length field is larger than the packet. Ethertypes 0-1500 are 802.3 lengths, all others are Ether types."]
pub type DropBadLengthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRUNK_ENABLE_DESTINATION` reader - 24:24\\]
Trunk Enable Destination IP Address - This field enables the destination IP address to be used with the hash function G(X) = 1 + X + X^3 and affect the trunk port transmit link determination.#br#This feature supports No tag, Priority tagged, VLAN tagged, Q-in-Q double tagging for both IPV6 and IPV4."]
pub type TrunkEnableDestinationR = crate::BitReader;
#[doc = "Field `TRUNK_ENABLE_DESTINATION` writer - 24:24\\]
Trunk Enable Destination IP Address - This field enables the destination IP address to be used with the hash function G(X) = 1 + X + X^3 and affect the trunk port transmit link determination.#br#This feature supports No tag, Priority tagged, VLAN tagged, Q-in-Q double tagging for both IPV6 and IPV4."]
pub type TrunkEnableDestinationW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRUNK_ENABLE_SOURCE` reader - 25:25\\]
Trunk Enable Source IP Address - This field enables the source IP address to be used with the hash function G(X) = 1 + X + X^3 and affect the trunk port transmit link determination.#br#This feature supports No tag, Priority tagged, VLAN tagged, Q-in-Q double tagging for both IPV6 and IPV4."]
pub type TrunkEnableSourceR = crate::BitReader;
#[doc = "Field `TRUNK_ENABLE_SOURCE` writer - 25:25\\]
Trunk Enable Source IP Address - This field enables the source IP address to be used with the hash function G(X) = 1 + X + X^3 and affect the trunk port transmit link determination.#br#This feature supports No tag, Priority tagged, VLAN tagged, Q-in-Q double tagging for both IPV6 and IPV4."]
pub type TrunkEnableSourceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRUNK_ENABLE_INNER` reader - 27:27\\]
Trunk Enable Inner VLAN - This field enables the inner VLAN ID value (C-VLANID) to be used with the hash function G(X) = 1 + X + X^3 and affect the trunk port transmit link determination."]
pub type TrunkEnableInnerR = crate::BitReader;
#[doc = "Field `TRUNK_ENABLE_INNER` writer - 27:27\\]
Trunk Enable Inner VLAN - This field enables the inner VLAN ID value (C-VLANID) to be used with the hash function G(X) = 1 + X + X^3 and affect the trunk port transmit link determination."]
pub type TrunkEnableInnerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRUNK_ENABLE_PRIORITY` reader - 29:29\\]
Trunk Enable Priority - This field enables the VLAN Priority bits to be used with the hash function G(X) = 1 + X + X^3 and affect the trunk port transmit link determination. In the event that DSCP mapping is enabled and there is no VLAN the DSCP priority will be used. For all other non IP frames without VLAN the port default priority is used."]
pub type TrunkEnablePriorityR = crate::BitReader;
#[doc = "Field `TRUNK_ENABLE_PRIORITY` writer - 29:29\\]
Trunk Enable Priority - This field enables the VLAN Priority bits to be used with the hash function G(X) = 1 + X + X^3 and affect the trunk port transmit link determination. In the event that DSCP mapping is enabled and there is no VLAN the DSCP priority will be used. For all other non IP frames without VLAN the port default priority is used."]
pub type TrunkEnablePriorityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRUNK_ENABLE_SOURCE` reader - 30:30\\]
Trunk Enable Source Address - This field enables the source MAC address to be used with the hash function G(X) = 1 + X + X^3 and affect the trunk port transmit link determination."]
pub type TrunkEnableSourceR = crate::BitReader;
#[doc = "Field `TRUNK_ENABLE_SOURCE` writer - 30:30\\]
Trunk Enable Source Address - This field enables the source MAC address to be used with the hash function G(X) = 1 + X + X^3 and affect the trunk port transmit link determination."]
pub type TrunkEnableSourceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRUNK_ENABLE_DESTINATION` reader - 31:31\\]
Trunk Enable Destination Address - This field enables the destination MAC address to be used with the hash function G(X) = 1 + X + X^3 and affect the trunk port transmit link determination."]
pub type TrunkEnableDestinationR = crate::BitReader;
#[doc = "Field `TRUNK_ENABLE_DESTINATION` writer - 31:31\\]
Trunk Enable Destination Address - This field enables the destination MAC address to be used with the hash function G(X) = 1 + X + X^3 and affect the trunk port transmit link determination."]
pub type TrunkEnableDestinationW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Mirror Index - This field is the ALE lookup table entry index that when a match occurs will cause this traffic to be mirrored to the ~imirror_top port. That is any VLAN, ONU or address with or withou VLAN can be selected for traffic mirroring."]
    #[inline(always)]
    pub fn mirror_index_this(&self) -> MirrorIndexThisR {
        MirrorIndexThisR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
The ~multihost allows host traffic to be sent bact to the host if the DA is market for the host port."]
    #[inline(always)]
    pub fn the_multihost_allows(&self) -> TheMultihostAllowsR {
        TheMultihostAllowsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Trunk Base - This field is the hash formula starting value. Changing this value will cause the packet distribution on trunk ports to be changed. If all the ~itrk_en_dst, ~itrk_en_src, ~itrk_en_pri and ~itrk_en_vlan are '0', this value is used as the distribution index. That is a '0' will select the 1st bit of an 'N' link trunk, a '1' will select the second, etc. #br#Below is the distribution across the trunk links. The first number in the ~iitalic sequence indicates the traffic is sent to the lowest numbered port of a trunk group. For example if you have a 3 port trunk, the hash result 0 will go to the base port (0), hash result 1 will go to the highest port of the trunk group (2), hash result 2 will go to the middle port (1), etc.#br# 1 - ~i00000000#br# 2 - ~i01010101#br# 3 - ~i02102102#br# 4 - ~i03210321"]
    #[inline(always)]
    pub fn trunk_base_this(&self) -> TrunkBaseThisR {
        TrunkBaseThisR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
Default limit next header field will cause an IPv4 protocol or IPv6 next header packet to be dropped if a VLAN entry is not found and the protocol or next header does not match the ~iALE_NXT_HDR register values."]
    #[inline(always)]
    pub fn default_limit_next(&self) -> DefaultLimitNextR {
        DefaultLimitNextR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Default No Frag field will cause an IPv4 fragmented packet to be dropped if a VLAN entry is not found."]
    #[inline(always)]
    pub fn default_no_frag(&self) -> DefaultNoFragR {
        DefaultNoFragR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
No Drop Source Multicast will disable the dropping of any source address with the multicast bit set."]
    #[inline(always)]
    pub fn no_drop_source(&self) -> NoDropSourceR {
        NoDropSourceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Drop Bad Length will drop any packet that the 802.3 length field is larger than the packet. Ethertypes 0-1500 are 802.3 lengths, all others are Ether types."]
    #[inline(always)]
    pub fn drop_bad_length(&self) -> DropBadLengthR {
        DropBadLengthR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Trunk Enable Destination IP Address - This field enables the destination IP address to be used with the hash function G(X) = 1 + X + X^3 and affect the trunk port transmit link determination.#br#This feature supports No tag, Priority tagged, VLAN tagged, Q-in-Q double tagging for both IPV6 and IPV4."]
    #[inline(always)]
    pub fn trunk_enable_destination(&self) -> TrunkEnableDestinationR {
        TrunkEnableDestinationR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Trunk Enable Source IP Address - This field enables the source IP address to be used with the hash function G(X) = 1 + X + X^3 and affect the trunk port transmit link determination.#br#This feature supports No tag, Priority tagged, VLAN tagged, Q-in-Q double tagging for both IPV6 and IPV4."]
    #[inline(always)]
    pub fn trunk_enable_source(&self) -> TrunkEnableSourceR {
        TrunkEnableSourceR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Trunk Enable Inner VLAN - This field enables the inner VLAN ID value (C-VLANID) to be used with the hash function G(X) = 1 + X + X^3 and affect the trunk port transmit link determination."]
    #[inline(always)]
    pub fn trunk_enable_inner(&self) -> TrunkEnableInnerR {
        TrunkEnableInnerR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Trunk Enable Priority - This field enables the VLAN Priority bits to be used with the hash function G(X) = 1 + X + X^3 and affect the trunk port transmit link determination. In the event that DSCP mapping is enabled and there is no VLAN the DSCP priority will be used. For all other non IP frames without VLAN the port default priority is used."]
    #[inline(always)]
    pub fn trunk_enable_priority(&self) -> TrunkEnablePriorityR {
        TrunkEnablePriorityR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Trunk Enable Source Address - This field enables the source MAC address to be used with the hash function G(X) = 1 + X + X^3 and affect the trunk port transmit link determination."]
    #[inline(always)]
    pub fn trunk_enable_source(&self) -> TrunkEnableSourceR {
        TrunkEnableSourceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Trunk Enable Destination Address - This field enables the destination MAC address to be used with the hash function G(X) = 1 + X + X^3 and affect the trunk port transmit link determination."]
    #[inline(always)]
    pub fn trunk_enable_destination(&self) -> TrunkEnableDestinationR {
        TrunkEnableDestinationR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Mirror Index - This field is the ALE lookup table entry index that when a match occurs will cause this traffic to be mirrored to the ~imirror_top port. That is any VLAN, ONU or address with or withou VLAN can be selected for traffic mirroring."]
    #[inline(always)]
    #[must_use]
    pub fn mirror_index_this(&mut self) -> MirrorIndexThisW<AleAleCtrl2Spec> {
        MirrorIndexThisW::new(self, 0)
    }
    #[doc = "Bit 15 - 15:15\\]
The ~multihost allows host traffic to be sent bact to the host if the DA is market for the host port."]
    #[inline(always)]
    #[must_use]
    pub fn the_multihost_allows(&mut self) -> TheMultihostAllowsW<AleAleCtrl2Spec> {
        TheMultihostAllowsW::new(self, 15)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Trunk Base - This field is the hash formula starting value. Changing this value will cause the packet distribution on trunk ports to be changed. If all the ~itrk_en_dst, ~itrk_en_src, ~itrk_en_pri and ~itrk_en_vlan are '0', this value is used as the distribution index. That is a '0' will select the 1st bit of an 'N' link trunk, a '1' will select the second, etc. #br#Below is the distribution across the trunk links. The first number in the ~iitalic sequence indicates the traffic is sent to the lowest numbered port of a trunk group. For example if you have a 3 port trunk, the hash result 0 will go to the base port (0), hash result 1 will go to the highest port of the trunk group (2), hash result 2 will go to the middle port (1), etc.#br# 1 - ~i00000000#br# 2 - ~i01010101#br# 3 - ~i02102102#br# 4 - ~i03210321"]
    #[inline(always)]
    #[must_use]
    pub fn trunk_base_this(&mut self) -> TrunkBaseThisW<AleAleCtrl2Spec> {
        TrunkBaseThisW::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Default limit next header field will cause an IPv4 protocol or IPv6 next header packet to be dropped if a VLAN entry is not found and the protocol or next header does not match the ~iALE_NXT_HDR register values."]
    #[inline(always)]
    #[must_use]
    pub fn default_limit_next(&mut self) -> DefaultLimitNextW<AleAleCtrl2Spec> {
        DefaultLimitNextW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Default No Frag field will cause an IPv4 fragmented packet to be dropped if a VLAN entry is not found."]
    #[inline(always)]
    #[must_use]
    pub fn default_no_frag(&mut self) -> DefaultNoFragW<AleAleCtrl2Spec> {
        DefaultNoFragW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
No Drop Source Multicast will disable the dropping of any source address with the multicast bit set."]
    #[inline(always)]
    #[must_use]
    pub fn no_drop_source(&mut self) -> NoDropSourceW<AleAleCtrl2Spec> {
        NoDropSourceW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Drop Bad Length will drop any packet that the 802.3 length field is larger than the packet. Ethertypes 0-1500 are 802.3 lengths, all others are Ether types."]
    #[inline(always)]
    #[must_use]
    pub fn drop_bad_length(&mut self) -> DropBadLengthW<AleAleCtrl2Spec> {
        DropBadLengthW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Trunk Enable Destination IP Address - This field enables the destination IP address to be used with the hash function G(X) = 1 + X + X^3 and affect the trunk port transmit link determination.#br#This feature supports No tag, Priority tagged, VLAN tagged, Q-in-Q double tagging for both IPV6 and IPV4."]
    #[inline(always)]
    #[must_use]
    pub fn trunk_enable_destination(&mut self) -> TrunkEnableDestinationW<AleAleCtrl2Spec> {
        TrunkEnableDestinationW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Trunk Enable Source IP Address - This field enables the source IP address to be used with the hash function G(X) = 1 + X + X^3 and affect the trunk port transmit link determination.#br#This feature supports No tag, Priority tagged, VLAN tagged, Q-in-Q double tagging for both IPV6 and IPV4."]
    #[inline(always)]
    #[must_use]
    pub fn trunk_enable_source(&mut self) -> TrunkEnableSourceW<AleAleCtrl2Spec> {
        TrunkEnableSourceW::new(self, 25)
    }
    #[doc = "Bit 27 - 27:27\\]
Trunk Enable Inner VLAN - This field enables the inner VLAN ID value (C-VLANID) to be used with the hash function G(X) = 1 + X + X^3 and affect the trunk port transmit link determination."]
    #[inline(always)]
    #[must_use]
    pub fn trunk_enable_inner(&mut self) -> TrunkEnableInnerW<AleAleCtrl2Spec> {
        TrunkEnableInnerW::new(self, 27)
    }
    #[doc = "Bit 29 - 29:29\\]
Trunk Enable Priority - This field enables the VLAN Priority bits to be used with the hash function G(X) = 1 + X + X^3 and affect the trunk port transmit link determination. In the event that DSCP mapping is enabled and there is no VLAN the DSCP priority will be used. For all other non IP frames without VLAN the port default priority is used."]
    #[inline(always)]
    #[must_use]
    pub fn trunk_enable_priority(&mut self) -> TrunkEnablePriorityW<AleAleCtrl2Spec> {
        TrunkEnablePriorityW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Trunk Enable Source Address - This field enables the source MAC address to be used with the hash function G(X) = 1 + X + X^3 and affect the trunk port transmit link determination."]
    #[inline(always)]
    #[must_use]
    pub fn trunk_enable_source(&mut self) -> TrunkEnableSourceW<AleAleCtrl2Spec> {
        TrunkEnableSourceW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Trunk Enable Destination Address - This field enables the destination MAC address to be used with the hash function G(X) = 1 + X + X^3 and affect the trunk port transmit link determination."]
    #[inline(always)]
    #[must_use]
    pub fn trunk_enable_destination(&mut self) -> TrunkEnableDestinationW<AleAleCtrl2Spec> {
        TrunkEnableDestinationW::new(self, 31)
    }
}
#[doc = "The ALE Control 2 Register is used to set the extended features used for all ports.\n\nYou can [`read`](crate::Reg::read) this register and get [`ale_ale_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ale_ale_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AleAleCtrl2Spec;
impl crate::RegisterSpec for AleAleCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ale_ale_ctrl2::R`](R) reader structure"]
impl crate::Readable for AleAleCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ale_ale_ctrl2::W`](W) writer structure"]
impl crate::Writable for AleAleCtrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALE_ALE_CTRL2 to value 0"]
impl crate::Resettable for AleAleCtrl2Spec {
    const RESET_VALUE: u32 = 0;
}
