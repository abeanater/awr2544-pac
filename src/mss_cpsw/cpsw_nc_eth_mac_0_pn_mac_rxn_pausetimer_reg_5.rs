#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_MAC_RXN_PAUSETIMER_REG_5` reader"]
pub type R = crate::R<CpswNcEthMac0PnMacRxnPausetimerReg5Spec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_MAC_RXN_PAUSETIMER_REG_5` writer"]
pub type W = crate::W<CpswNcEthMac0PnMacRxnPausetimerReg5Spec>;
#[doc = "Field `RX_PAUSE_TIMER` reader - 15:0\\]
RX Pause Timer Value"]
pub type RxPauseTimerR = crate::FieldReader<u16>;
#[doc = "Field `RX_PAUSE_TIMER` writer - 15:0\\]
RX Pause Timer Value"]
pub type RxPauseTimerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
RX Pause Timer Value"]
    #[inline(always)]
    pub fn rx_pause_timer(&self) -> RxPauseTimerR {
        RxPauseTimerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
RX Pause Timer Value"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pause_timer(&mut self) -> RxPauseTimerW<CpswNcEthMac0PnMacRxnPausetimerReg5Spec> {
        RxPauseTimerW::new(self, 0)
    }
}
#[doc = "Enet Port N PFC Priority P Rx Pause Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnMacRxnPausetimerReg5Spec;
impl crate::RegisterSpec for CpswNcEthMac0PnMacRxnPausetimerReg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_5::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnMacRxnPausetimerReg5Spec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_mac_rxn_pausetimer_reg_5::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnMacRxnPausetimerReg5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_MAC_RXN_PAUSETIMER_REG_5 to value 0"]
impl crate::Resettable for CpswNcEthMac0PnMacRxnPausetimerReg5Spec {
    const RESET_VALUE: u32 = 0;
}
