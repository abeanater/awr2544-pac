#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_MAC_TX_GAP_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnMacTxGapRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_MAC_TX_GAP_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnMacTxGapRegSpec>;
#[doc = "Field `TRANSMIT_INTERPACKET_GAP` reader - 15:0\\]
Transmit Inter-Packet Gap"]
pub type TransmitInterpacketGapR = crate::FieldReader<u16>;
#[doc = "Field `TRANSMIT_INTERPACKET_GAP` writer - 15:0\\]
Transmit Inter-Packet Gap"]
pub type TransmitInterpacketGapW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Transmit Inter-Packet Gap"]
    #[inline(always)]
    pub fn transmit_interpacket_gap(&self) -> TransmitInterpacketGapR {
        TransmitInterpacketGapR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Transmit Inter-Packet Gap"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_interpacket_gap(
        &mut self,
    ) -> TransmitInterpacketGapW<CpswNcEthMac0PnMacTxGapRegSpec> {
        TransmitInterpacketGapW::new(self, 0)
    }
}
#[doc = "Enet Port N Tx Inter Packet Gap\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_mac_tx_gap_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_mac_tx_gap_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnMacTxGapRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnMacTxGapRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_mac_tx_gap_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnMacTxGapRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_mac_tx_gap_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnMacTxGapRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_MAC_TX_GAP_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnMacTxGapRegSpec {
    const RESET_VALUE: u32 = 0;
}
