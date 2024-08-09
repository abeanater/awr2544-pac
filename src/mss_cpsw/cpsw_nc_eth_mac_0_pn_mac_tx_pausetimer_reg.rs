#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_MAC_TX_PAUSETIMER_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnMacTxPausetimerRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_MAC_TX_PAUSETIMER_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnMacTxPausetimerRegSpec>;
#[doc = "Field `TX_PAUSE_TIMER` reader - 15:0\\]
TX Pause Timer Value"]
pub type TxPauseTimerR = crate::FieldReader<u16>;
#[doc = "Field `TX_PAUSE_TIMER` writer - 15:0\\]
TX Pause Timer Value"]
pub type TxPauseTimerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
TX Pause Timer Value"]
    #[inline(always)]
    pub fn tx_pause_timer(&self) -> TxPauseTimerR {
        TxPauseTimerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
TX Pause Timer Value"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pause_timer(&mut self) -> TxPauseTimerW<CpswNcEthMac0PnMacTxPausetimerRegSpec> {
        TxPauseTimerW::new(self, 0)
    }
}
#[doc = "Enet Port N 802.3 Tx Pause Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_tx_pausetimer_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_tx_pausetimer_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnMacTxPausetimerRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnMacTxPausetimerRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_mac_tx_pausetimer_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnMacTxPausetimerRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_mac_tx_pausetimer_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnMacTxPausetimerRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_MAC_TX_PAUSETIMER_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnMacTxPausetimerRegSpec {
    const RESET_VALUE: u32 = 0;
}
