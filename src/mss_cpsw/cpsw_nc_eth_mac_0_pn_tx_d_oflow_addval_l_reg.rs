#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_TX_D_OFLOW_ADDVAL_L_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnTxDOflowAddvalLRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_TX_D_OFLOW_ADDVAL_L_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnTxDOflowAddvalLRegSpec>;
#[doc = "Field `PORT_PFC_DESTINATION` reader - 4:0\\]
Port PFC Destination Based Out Flow Add Value for Priority 0"]
pub type PortPfcDestinationR = crate::FieldReader;
#[doc = "Field `PORT_PFC_DESTINATION` writer - 4:0\\]
Port PFC Destination Based Out Flow Add Value for Priority 0"]
pub type PortPfcDestinationW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PORT_PFC_DESTINATION` reader - 12:8\\]
Port PFC Destination Based Out Flow Add Value for Priority 1"]
pub type PortPfcDestinationR = crate::FieldReader;
#[doc = "Field `PORT_PFC_DESTINATION` writer - 12:8\\]
Port PFC Destination Based Out Flow Add Value for Priority 1"]
pub type PortPfcDestinationW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PORT_PFC_DESTINATION` reader - 20:16\\]
Port PFC Destination Based Out Flow Add Value for Priority 2"]
pub type PortPfcDestinationR = crate::FieldReader;
#[doc = "Field `PORT_PFC_DESTINATION` writer - 20:16\\]
Port PFC Destination Based Out Flow Add Value for Priority 2"]
pub type PortPfcDestinationW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PORT_PFC_DESTINATION` reader - 28:24\\]
Port PFC Destination Based Out Flow Add Value for Priority 3"]
pub type PortPfcDestinationR = crate::FieldReader;
#[doc = "Field `PORT_PFC_DESTINATION` writer - 28:24\\]
Port PFC Destination Based Out Flow Add Value for Priority 3"]
pub type PortPfcDestinationW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Port PFC Destination Based Out Flow Add Value for Priority 0"]
    #[inline(always)]
    pub fn port_pfc_destination(&self) -> PortPfcDestinationR {
        PortPfcDestinationR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Port PFC Destination Based Out Flow Add Value for Priority 1"]
    #[inline(always)]
    pub fn port_pfc_destination(&self) -> PortPfcDestinationR {
        PortPfcDestinationR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Port PFC Destination Based Out Flow Add Value for Priority 2"]
    #[inline(always)]
    pub fn port_pfc_destination(&self) -> PortPfcDestinationR {
        PortPfcDestinationR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Port PFC Destination Based Out Flow Add Value for Priority 3"]
    #[inline(always)]
    pub fn port_pfc_destination(&self) -> PortPfcDestinationR {
        PortPfcDestinationR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Port PFC Destination Based Out Flow Add Value for Priority 0"]
    #[inline(always)]
    #[must_use]
    pub fn port_pfc_destination(
        &mut self,
    ) -> PortPfcDestinationW<CpswNcEthMac0PnTxDOflowAddvalLRegSpec> {
        PortPfcDestinationW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Port PFC Destination Based Out Flow Add Value for Priority 1"]
    #[inline(always)]
    #[must_use]
    pub fn port_pfc_destination(
        &mut self,
    ) -> PortPfcDestinationW<CpswNcEthMac0PnTxDOflowAddvalLRegSpec> {
        PortPfcDestinationW::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Port PFC Destination Based Out Flow Add Value for Priority 2"]
    #[inline(always)]
    #[must_use]
    pub fn port_pfc_destination(
        &mut self,
    ) -> PortPfcDestinationW<CpswNcEthMac0PnTxDOflowAddvalLRegSpec> {
        PortPfcDestinationW::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Port PFC Destination Based Out Flow Add Value for Priority 3"]
    #[inline(always)]
    #[must_use]
    pub fn port_pfc_destination(
        &mut self,
    ) -> PortPfcDestinationW<CpswNcEthMac0PnTxDOflowAddvalLRegSpec> {
        PortPfcDestinationW::new(self, 24)
    }
}
#[doc = "Enet Port N Tx Destination Out Flow Add Values Low\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_tx_d_oflow_addval_l_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_tx_d_oflow_addval_l_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnTxDOflowAddvalLRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnTxDOflowAddvalLRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_tx_d_oflow_addval_l_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnTxDOflowAddvalLRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_tx_d_oflow_addval_l_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnTxDOflowAddvalLRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_TX_D_OFLOW_ADDVAL_L_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnTxDOflowAddvalLRegSpec {
    const RESET_VALUE: u32 = 0;
}
