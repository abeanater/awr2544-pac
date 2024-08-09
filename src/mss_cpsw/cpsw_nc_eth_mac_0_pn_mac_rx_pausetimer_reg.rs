#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_MAC_RX_PAUSETIMER_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnMacRxPausetimerRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_MAC_RX_PAUSETIMER_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnMacRxPausetimerRegSpec>;
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
    pub fn rx_pause_timer(&mut self) -> RxPauseTimerW<CpswNcEthMac0PnMacRxPausetimerRegSpec> {
        RxPauseTimerW::new(self, 0)
    }
}
#[doc = "Enet Port N 802.3 Receive Pause Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_rx_pausetimer_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_rx_pausetimer_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnMacRxPausetimerRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnMacRxPausetimerRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_mac_rx_pausetimer_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnMacRxPausetimerRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_mac_rx_pausetimer_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnMacRxPausetimerRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_MAC_RX_PAUSETIMER_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnMacRxPausetimerRegSpec {
    const RESET_VALUE: u32 = 0;
}
