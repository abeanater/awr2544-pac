#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_RESERVED_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnReservedRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_RESERVED_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnReservedRegSpec>;
#[doc = "Field `RESERVED_REGISTER_FOR` reader - 31:0\\]
Reserved register for memory map alignment"]
pub type ReservedRegisterForR = crate::FieldReader<u32>;
#[doc = "Field `RESERVED_REGISTER_FOR` writer - 31:0\\]
Reserved register for memory map alignment"]
pub type ReservedRegisterForW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved register for memory map alignment"]
    #[inline(always)]
    pub fn reserved_register_for(&self) -> ReservedRegisterForR {
        ReservedRegisterForR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved register for memory map alignment"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_register_for(
        &mut self,
    ) -> ReservedRegisterForW<CpswNcEthMac0PnReservedRegSpec> {
        ReservedRegisterForW::new(self, 0)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_reserved_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_reserved_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnReservedRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnReservedRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_reserved_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnReservedRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_reserved_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnReservedRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_RESERVED_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnReservedRegSpec {
    const RESET_VALUE: u32 = 0;
}
