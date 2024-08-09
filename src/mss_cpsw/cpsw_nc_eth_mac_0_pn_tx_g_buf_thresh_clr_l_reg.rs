#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_TX_G_BUF_THRESH_CLR_L_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnTxGBufThreshClrLRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_TX_G_BUF_THRESH_CLR_L_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnTxGBufThreshClrLRegSpec>;
#[doc = "Field `PORT_PRIORITY_BASED_3` reader - 4:0\\]
Port Priority Based Flow Control Threshold Clear Value for Priority 0"]
pub type PortPriorityBased3R = crate::FieldReader;
#[doc = "Field `PORT_PRIORITY_BASED_3` writer - 4:0\\]
Port Priority Based Flow Control Threshold Clear Value for Priority 0"]
pub type PortPriorityBased3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PORT_PRIORITY_BASED_2` reader - 12:8\\]
Port Priority Based Flow Control Threshold Clear Value for Priority 1"]
pub type PortPriorityBased2R = crate::FieldReader;
#[doc = "Field `PORT_PRIORITY_BASED_2` writer - 12:8\\]
Port Priority Based Flow Control Threshold Clear Value for Priority 1"]
pub type PortPriorityBased2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PORT_PRIORITY_BASED_1` reader - 20:16\\]
Port Priority Based Flow Control Threshold Clear Value for Priority 2"]
pub type PortPriorityBased1R = crate::FieldReader;
#[doc = "Field `PORT_PRIORITY_BASED_1` writer - 20:16\\]
Port Priority Based Flow Control Threshold Clear Value for Priority 2"]
pub type PortPriorityBased1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PORT_PRIORITY_BASED` reader - 28:24\\]
Port Priority Based Flow Control Threshold Clear Value for Priority 3"]
pub type PortPriorityBasedR = crate::FieldReader;
#[doc = "Field `PORT_PRIORITY_BASED` writer - 28:24\\]
Port Priority Based Flow Control Threshold Clear Value for Priority 3"]
pub type PortPriorityBasedW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Port Priority Based Flow Control Threshold Clear Value for Priority 0"]
    #[inline(always)]
    pub fn port_priority_based_3(&self) -> PortPriorityBased3R {
        PortPriorityBased3R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Port Priority Based Flow Control Threshold Clear Value for Priority 1"]
    #[inline(always)]
    pub fn port_priority_based_2(&self) -> PortPriorityBased2R {
        PortPriorityBased2R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Port Priority Based Flow Control Threshold Clear Value for Priority 2"]
    #[inline(always)]
    pub fn port_priority_based_1(&self) -> PortPriorityBased1R {
        PortPriorityBased1R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Port Priority Based Flow Control Threshold Clear Value for Priority 3"]
    #[inline(always)]
    pub fn port_priority_based(&self) -> PortPriorityBasedR {
        PortPriorityBasedR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Port Priority Based Flow Control Threshold Clear Value for Priority 0"]
    #[inline(always)]
    #[must_use]
    pub fn port_priority_based_3(
        &mut self,
    ) -> PortPriorityBased3W<CpswNcEthMac0PnTxGBufThreshClrLRegSpec> {
        PortPriorityBased3W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Port Priority Based Flow Control Threshold Clear Value for Priority 1"]
    #[inline(always)]
    #[must_use]
    pub fn port_priority_based_2(
        &mut self,
    ) -> PortPriorityBased2W<CpswNcEthMac0PnTxGBufThreshClrLRegSpec> {
        PortPriorityBased2W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Port Priority Based Flow Control Threshold Clear Value for Priority 2"]
    #[inline(always)]
    #[must_use]
    pub fn port_priority_based_1(
        &mut self,
    ) -> PortPriorityBased1W<CpswNcEthMac0PnTxGBufThreshClrLRegSpec> {
        PortPriorityBased1W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Port Priority Based Flow Control Threshold Clear Value for Priority 3"]
    #[inline(always)]
    #[must_use]
    pub fn port_priority_based(
        &mut self,
    ) -> PortPriorityBasedW<CpswNcEthMac0PnTxGBufThreshClrLRegSpec> {
        PortPriorityBasedW::new(self, 24)
    }
}
#[doc = "Enet Port N Tx PFC Global Buffer Threshold Clr Low\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_clr_l_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_clr_l_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnTxGBufThreshClrLRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnTxGBufThreshClrLRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_clr_l_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnTxGBufThreshClrLRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_tx_g_buf_thresh_clr_l_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnTxGBufThreshClrLRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_TX_G_BUF_THRESH_CLR_L_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnTxGBufThreshClrLRegSpec {
    const RESET_VALUE: u32 = 0;
}
