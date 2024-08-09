#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_MAC_TXN_PAUSETIMER_REG_4` reader"]
pub type R = crate::R<CpswNcEthMac0PnMacTxnPausetimerReg4Spec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_MAC_TXN_PAUSETIMER_REG_4` writer"]
pub type W = crate::W<CpswNcEthMac0PnMacTxnPausetimerReg4Spec>;
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
    pub fn tx_pause_timer(&mut self) -> TxPauseTimerW<CpswNcEthMac0PnMacTxnPausetimerReg4Spec> {
        TxPauseTimerW::new(self, 0)
    }
}
#[doc = "Enet Port N PFC Priority P Tx Pause Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnMacTxnPausetimerReg4Spec;
impl crate::RegisterSpec for CpswNcEthMac0PnMacTxnPausetimerReg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_4::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnMacTxnPausetimerReg4Spec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_mac_txn_pausetimer_reg_4::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnMacTxnPausetimerReg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_MAC_TXN_PAUSETIMER_REG_4 to value 0"]
impl crate::Resettable for CpswNcEthMac0PnMacTxnPausetimerReg4Spec {
    const RESET_VALUE: u32 = 0;
}
