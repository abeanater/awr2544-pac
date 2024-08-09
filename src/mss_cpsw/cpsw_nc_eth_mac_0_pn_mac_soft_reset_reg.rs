#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_MAC_SOFT_RESET_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnMacSoftResetRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_MAC_SOFT_RESET_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnMacSoftResetRegSpec>;
#[doc = "Field `SOFTWARE_RESET` reader - 0:0\\]
Software reset"]
pub type SoftwareResetR = crate::BitReader;
#[doc = "Field `SOFTWARE_RESET` writer - 0:0\\]
Software reset"]
pub type SoftwareResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software reset"]
    #[inline(always)]
    pub fn software_reset(&self) -> SoftwareResetR {
        SoftwareResetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn software_reset(&mut self) -> SoftwareResetW<CpswNcEthMac0PnMacSoftResetRegSpec> {
        SoftwareResetW::new(self, 0)
    }
}
#[doc = "Enet Port N Mac Soft Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_soft_reset_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_soft_reset_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnMacSoftResetRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnMacSoftResetRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_mac_soft_reset_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnMacSoftResetRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_mac_soft_reset_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnMacSoftResetRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_MAC_SOFT_RESET_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnMacSoftResetRegSpec {
    const RESET_VALUE: u32 = 0;
}
