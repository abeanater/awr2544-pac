#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_INTERVLAN_OPX_POINTER_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnIntervlanOpxPointerRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_INTERVLAN_OPX_POINTER_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnIntervlanOpxPointerRegSpec>;
#[doc = "Field `INTERVLAN_LOCATION_POINTER` reader - 2:0\\]
InterVLAN location pointer: This field points to the InterVLAN location that will be read/written by accesses to Enet_Pn_InterVLANx_A/B."]
pub type IntervlanLocationPointerR = crate::FieldReader;
#[doc = "Field `INTERVLAN_LOCATION_POINTER` writer - 2:0\\]
InterVLAN location pointer: This field points to the InterVLAN location that will be read/written by accesses to Enet_Pn_InterVLANx_A/B."]
pub type IntervlanLocationPointerW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
InterVLAN location pointer: This field points to the InterVLAN location that will be read/written by accesses to Enet_Pn_InterVLANx_A/B."]
    #[inline(always)]
    pub fn intervlan_location_pointer(&self) -> IntervlanLocationPointerR {
        IntervlanLocationPointerR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
InterVLAN location pointer: This field points to the InterVLAN location that will be read/written by accesses to Enet_Pn_InterVLANx_A/B."]
    #[inline(always)]
    #[must_use]
    pub fn intervlan_location_pointer(
        &mut self,
    ) -> IntervlanLocationPointerW<CpswNcEthMac0PnIntervlanOpxPointerRegSpec> {
        IntervlanLocationPointerW::new(self, 0)
    }
}
#[doc = "Enet Port N Tx Egress InterVLAN Operation Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_intervlan_opx_pointer_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_intervlan_opx_pointer_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnIntervlanOpxPointerRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnIntervlanOpxPointerRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_intervlan_opx_pointer_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnIntervlanOpxPointerRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_intervlan_opx_pointer_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnIntervlanOpxPointerRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_INTERVLAN_OPX_POINTER_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnIntervlanOpxPointerRegSpec {
    const RESET_VALUE: u32 = 0;
}
