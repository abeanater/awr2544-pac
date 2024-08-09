#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_INTERVLAN_OPX_D_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnIntervlanOpxDRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_INTERVLAN_OPX_D_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnIntervlanOpxDRegSpec>;
#[doc = "Field `VLAN_ID` reader - 11:0\\]
VLAN ID"]
pub type VlanIdR = crate::FieldReader<u16>;
#[doc = "Field `VLAN_ID` writer - 11:0\\]
VLAN ID"]
pub type VlanIdW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `REPLACE_VLAN_ID` reader - 12:12\\]
Replace VLAN ID: When set this bit indicates that the VLAN ID should be replaced for the routed packet."]
pub type ReplaceVlanIdR = crate::BitReader;
#[doc = "Field `REPLACE_VLAN_ID` writer - 12:12\\]
Replace VLAN ID: When set this bit indicates that the VLAN ID should be replaced for the routed packet."]
pub type ReplaceVlanIdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REPLACE_DESTINATION_ADDRESS` reader - 13:13\\]
Replace Destination Address and Source Address: When set this bit indicates that the routed packet destination address should be replaced by da\\[47:0\\]
and the source address should be replaced by sa\\[47:0\\]."]
pub type ReplaceDestinationAddressR = crate::BitReader;
#[doc = "Field `REPLACE_DESTINATION_ADDRESS` writer - 13:13\\]
Replace Destination Address and Source Address: When set this bit indicates that the routed packet destination address should be replaced by da\\[47:0\\]
and the source address should be replaced by sa\\[47:0\\]."]
pub type ReplaceDestinationAddressW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESTINATION_VLAN_FORCE` reader - 14:14\\]
Destination VLAN Force Untagged Egress: When set, this bit indicates that the VLAN should be removed on egress for the routed packet."]
pub type DestinationVlanForceR = crate::BitReader;
#[doc = "Field `DESTINATION_VLAN_FORCE` writer - 14:14\\]
Destination VLAN Force Untagged Egress: When set, this bit indicates that the VLAN should be removed on egress for the routed packet."]
pub type DestinationVlanForceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DECREMENT_TIME_TO` reader - 15:15\\]
Decrement Time To Live: When set, the Time To Live (TTL) field in the header is decremented."]
pub type DecrementTimeToR = crate::BitReader;
#[doc = "Field `DECREMENT_TIME_TO` writer - 15:15\\]
Decrement Time To Live: When set, the Time To Live (TTL) field in the header is decremented."]
pub type DecrementTimeToW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
VLAN ID"]
    #[inline(always)]
    pub fn vlan_id(&self) -> VlanIdR {
        VlanIdR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - 12:12\\]
Replace VLAN ID: When set this bit indicates that the VLAN ID should be replaced for the routed packet."]
    #[inline(always)]
    pub fn replace_vlan_id(&self) -> ReplaceVlanIdR {
        ReplaceVlanIdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Replace Destination Address and Source Address: When set this bit indicates that the routed packet destination address should be replaced by da\\[47:0\\]
and the source address should be replaced by sa\\[47:0\\]."]
    #[inline(always)]
    pub fn replace_destination_address(&self) -> ReplaceDestinationAddressR {
        ReplaceDestinationAddressR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Destination VLAN Force Untagged Egress: When set, this bit indicates that the VLAN should be removed on egress for the routed packet."]
    #[inline(always)]
    pub fn destination_vlan_force(&self) -> DestinationVlanForceR {
        DestinationVlanForceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Decrement Time To Live: When set, the Time To Live (TTL) field in the header is decremented."]
    #[inline(always)]
    pub fn decrement_time_to(&self) -> DecrementTimeToR {
        DecrementTimeToR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
VLAN ID"]
    #[inline(always)]
    #[must_use]
    pub fn vlan_id(&mut self) -> VlanIdW<CpswNcEthMac0PnIntervlanOpxDRegSpec> {
        VlanIdW::new(self, 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Replace VLAN ID: When set this bit indicates that the VLAN ID should be replaced for the routed packet."]
    #[inline(always)]
    #[must_use]
    pub fn replace_vlan_id(&mut self) -> ReplaceVlanIdW<CpswNcEthMac0PnIntervlanOpxDRegSpec> {
        ReplaceVlanIdW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Replace Destination Address and Source Address: When set this bit indicates that the routed packet destination address should be replaced by da\\[47:0\\]
and the source address should be replaced by sa\\[47:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn replace_destination_address(
        &mut self,
    ) -> ReplaceDestinationAddressW<CpswNcEthMac0PnIntervlanOpxDRegSpec> {
        ReplaceDestinationAddressW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Destination VLAN Force Untagged Egress: When set, this bit indicates that the VLAN should be removed on egress for the routed packet."]
    #[inline(always)]
    #[must_use]
    pub fn destination_vlan_force(
        &mut self,
    ) -> DestinationVlanForceW<CpswNcEthMac0PnIntervlanOpxDRegSpec> {
        DestinationVlanForceW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Decrement Time To Live: When set, the Time To Live (TTL) field in the header is decremented."]
    #[inline(always)]
    #[must_use]
    pub fn decrement_time_to(&mut self) -> DecrementTimeToW<CpswNcEthMac0PnIntervlanOpxDRegSpec> {
        DecrementTimeToW::new(self, 15)
    }
}
#[doc = "Enet Port N Tx Egress InterVLAN D\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_intervlan_opx_d_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_intervlan_opx_d_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnIntervlanOpxDRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnIntervlanOpxDRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_intervlan_opx_d_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnIntervlanOpxDRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_intervlan_opx_d_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnIntervlanOpxDRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_INTERVLAN_OPX_D_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnIntervlanOpxDRegSpec {
    const RESET_VALUE: u32 = 0;
}
