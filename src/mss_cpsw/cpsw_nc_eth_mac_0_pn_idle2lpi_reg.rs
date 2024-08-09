#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_IDLE2LPI_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnIdle2lpiRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_IDLE2LPI_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnIdle2lpiRegSpec>;
#[doc = "Field `EEE_IDLE_TO` reader - 23:0\\]
EEE Idle to LPI counter load value"]
pub type EeeIdleToR = crate::FieldReader<u32>;
#[doc = "Field `EEE_IDLE_TO` writer - 23:0\\]
EEE Idle to LPI counter load value"]
pub type EeeIdleToW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
EEE Idle to LPI counter load value"]
    #[inline(always)]
    pub fn eee_idle_to(&self) -> EeeIdleToR {
        EeeIdleToR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
EEE Idle to LPI counter load value"]
    #[inline(always)]
    #[must_use]
    pub fn eee_idle_to(&mut self) -> EeeIdleToW<CpswNcEthMac0PnIdle2lpiRegSpec> {
        EeeIdleToW::new(self, 0)
    }
}
#[doc = "Enet Port N EEE Idle to LPI counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_idle2lpi_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_idle2lpi_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnIdle2lpiRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnIdle2lpiRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_idle2lpi_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnIdle2lpiRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_idle2lpi_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnIdle2lpiRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_IDLE2LPI_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnIdle2lpiRegSpec {
    const RESET_VALUE: u32 = 0;
}
