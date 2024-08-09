#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_BLK_CNT_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnBlkCntRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_BLK_CNT_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnBlkCntRegSpec>;
#[doc = "Field `RECEIVE_BLOCK_COUNT` reader - 5:0\\]
Receive Block Count Usage"]
pub type ReceiveBlockCountR = crate::FieldReader;
#[doc = "Field `RECEIVE_BLOCK_COUNT` writer - 5:0\\]
Receive Block Count Usage"]
pub type ReceiveBlockCountW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TRANSMIT_BLOCK_COUNT` reader - 12:8\\]
Transmit Block Count Usage"]
pub type TransmitBlockCountR = crate::FieldReader;
#[doc = "Field `TRANSMIT_BLOCK_COUNT` writer - 12:8\\]
Transmit Block Count Usage"]
pub type TransmitBlockCountW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RECEIVE_PREMPT_QUEUE` reader - 21:16\\]
Receive Prempt Queue Block Count Usage"]
pub type ReceivePremptQueueR = crate::FieldReader;
#[doc = "Field `RECEIVE_PREMPT_QUEUE` writer - 21:16\\]
Receive Prempt Queue Block Count Usage"]
pub type ReceivePremptQueueW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Receive Block Count Usage"]
    #[inline(always)]
    pub fn receive_block_count(&self) -> ReceiveBlockCountR {
        ReceiveBlockCountR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Transmit Block Count Usage"]
    #[inline(always)]
    pub fn transmit_block_count(&self) -> TransmitBlockCountR {
        TransmitBlockCountR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Receive Prempt Queue Block Count Usage"]
    #[inline(always)]
    pub fn receive_prempt_queue(&self) -> ReceivePremptQueueR {
        ReceivePremptQueueR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Receive Block Count Usage"]
    #[inline(always)]
    #[must_use]
    pub fn receive_block_count(&mut self) -> ReceiveBlockCountW<CpswNcEthMac0PnBlkCntRegSpec> {
        ReceiveBlockCountW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Transmit Block Count Usage"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_block_count(&mut self) -> TransmitBlockCountW<CpswNcEthMac0PnBlkCntRegSpec> {
        TransmitBlockCountW::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Receive Prempt Queue Block Count Usage"]
    #[inline(always)]
    #[must_use]
    pub fn receive_prempt_queue(&mut self) -> ReceivePremptQueueW<CpswNcEthMac0PnBlkCntRegSpec> {
        ReceivePremptQueueW::new(self, 16)
    }
}
#[doc = "Enet Port N FIFO Block Usage Count\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_blk_cnt_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_blk_cnt_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnBlkCntRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnBlkCntRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_blk_cnt_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnBlkCntRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_blk_cnt_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnBlkCntRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_BLK_CNT_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnBlkCntRegSpec {
    const RESET_VALUE: u32 = 0;
}
