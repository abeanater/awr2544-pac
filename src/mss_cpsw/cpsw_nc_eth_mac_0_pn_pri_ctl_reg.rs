#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_PRI_CTL_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnPriCtlRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_PRI_CTL_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnPriCtlRegSpec>;
#[doc = "Field `TRANSMIT_FIFO_BLOCKS` reader - 15:12\\]
Transmit FIFO Blocks that must be free before a non rate-limited CPPI Port 0 receive thread can begin sending a packet"]
pub type TransmitFifoBlocksR = crate::FieldReader;
#[doc = "Field `TRANSMIT_FIFO_BLOCKS` writer - 15:12\\]
Transmit FIFO Blocks that must be free before a non rate-limited CPPI Port 0 receive thread can begin sending a packet"]
pub type TransmitFifoBlocksW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RECEIVE_PRIORITY_BASED` reader - 23:16\\]
Receive Priority Based Flow Control Enable (per priority)"]
pub type ReceivePriorityBasedR = crate::FieldReader;
#[doc = "Field `RECEIVE_PRIORITY_BASED` writer - 23:16\\]
Receive Priority Based Flow Control Enable (per priority)"]
pub type ReceivePriorityBasedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRANSMIT_PRIORITY_BASED` reader - 31:24\\]
Transmit Priority Based Flow Control Enable (per priority)"]
pub type TransmitPriorityBasedR = crate::FieldReader;
#[doc = "Field `TRANSMIT_PRIORITY_BASED` writer - 31:24\\]
Transmit Priority Based Flow Control Enable (per priority)"]
pub type TransmitPriorityBasedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 12:15 - 15:12\\]
Transmit FIFO Blocks that must be free before a non rate-limited CPPI Port 0 receive thread can begin sending a packet"]
    #[inline(always)]
    pub fn transmit_fifo_blocks(&self) -> TransmitFifoBlocksR {
        TransmitFifoBlocksR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Receive Priority Based Flow Control Enable (per priority)"]
    #[inline(always)]
    pub fn receive_priority_based(&self) -> ReceivePriorityBasedR {
        ReceivePriorityBasedR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Transmit Priority Based Flow Control Enable (per priority)"]
    #[inline(always)]
    pub fn transmit_priority_based(&self) -> TransmitPriorityBasedR {
        TransmitPriorityBasedR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - 15:12\\]
Transmit FIFO Blocks that must be free before a non rate-limited CPPI Port 0 receive thread can begin sending a packet"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_fifo_blocks(&mut self) -> TransmitFifoBlocksW<CpswNcEthMac0PnPriCtlRegSpec> {
        TransmitFifoBlocksW::new(self, 12)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Receive Priority Based Flow Control Enable (per priority)"]
    #[inline(always)]
    #[must_use]
    pub fn receive_priority_based(
        &mut self,
    ) -> ReceivePriorityBasedW<CpswNcEthMac0PnPriCtlRegSpec> {
        ReceivePriorityBasedW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Transmit Priority Based Flow Control Enable (per priority)"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_priority_based(
        &mut self,
    ) -> TransmitPriorityBasedW<CpswNcEthMac0PnPriCtlRegSpec> {
        TransmitPriorityBasedW::new(self, 24)
    }
}
#[doc = "Enet Port N Priority Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_pri_ctl_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_pri_ctl_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnPriCtlRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnPriCtlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_pri_ctl_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnPriCtlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_pri_ctl_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnPriCtlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_PRI_CTL_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnPriCtlRegSpec {
    const RESET_VALUE: u32 = 0;
}
