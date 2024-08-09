#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_EEE_STATUS_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnEeeStatusRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_EEE_STATUS_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnEeeStatusRegSpec>;
#[doc = "Field `CPPI_PORT_0` reader - 0:0\\]
CPPI port 0 wait idle to LPI - asserted when port 0 is counting the IDLE2LPI time"]
pub type CppiPort0R = crate::BitReader;
#[doc = "Field `CPPI_PORT_0` writer - 0:0\\]
CPPI port 0 wait idle to LPI - asserted when port 0 is counting the IDLE2LPI time"]
pub type CppiPort0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECEIVE_LPI_STATE` reader - 1:1\\]
Receive LPI state - asserted when the port 0 receive is in the LPI state"]
pub type ReceiveLpiStateR = crate::BitReader;
#[doc = "Field `RECEIVE_LPI_STATE` writer - 1:1\\]
Receive LPI state - asserted when the port 0 receive is in the LPI state"]
pub type ReceiveLpiStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSMIT_LPI_STATE` reader - 2:2\\]
Transmit LPI state - asserted when the port 0 transmit is in the LPI state"]
pub type TransmitLpiStateR = crate::BitReader;
#[doc = "Field `TRANSMIT_LPI_STATE` writer - 2:2\\]
Transmit LPI state - asserted when the port 0 transmit is in the LPI state"]
pub type TransmitLpiStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSMIT_WAKEUP_ASSERTED` reader - 3:3\\]
Transmit wakeup - asserted in the transmit LPI2WAKE count time"]
pub type TransmitWakeupAssertedR = crate::BitReader;
#[doc = "Field `TRANSMIT_WAKEUP_ASSERTED` writer - 3:3\\]
Transmit wakeup - asserted in the transmit LPI2WAKE count time"]
pub type TransmitWakeupAssertedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSMIT_FIFO_HOLD` reader - 4:4\\]
Transmit FIFO hold - asserted in the LPI state and during the LPI2WAKE count time"]
pub type TransmitFifoHoldR = crate::BitReader;
#[doc = "Field `TRANSMIT_FIFO_HOLD` writer - 4:4\\]
Transmit FIFO hold - asserted in the LPI state and during the LPI2WAKE count time"]
pub type TransmitFifoHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECEIVE_FIFO_(SWITCH` reader - 5:5\\]
Receive FIFO (switch ingress) is empty - contains no packets"]
pub type ReceiveFifoSwitchR = crate::BitReader;
#[doc = "Field `RECEIVE_FIFO_(SWITCH` writer - 5:5\\]
Receive FIFO (switch ingress) is empty - contains no packets"]
pub type ReceiveFifoSwitchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSMIT_FIFO_(SWITCH` reader - 6:6\\]
Transmit FIFO (switch egress) is empty - contains no packets"]
pub type TransmitFifoSwitchR = crate::BitReader;
#[doc = "Field `TRANSMIT_FIFO_(SWITCH` writer - 6:6\\]
Transmit FIFO (switch egress) is empty - contains no packets"]
pub type TransmitFifoSwitchW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
CPPI port 0 wait idle to LPI - asserted when port 0 is counting the IDLE2LPI time"]
    #[inline(always)]
    pub fn cppi_port_0(&self) -> CppiPort0R {
        CppiPort0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Receive LPI state - asserted when the port 0 receive is in the LPI state"]
    #[inline(always)]
    pub fn receive_lpi_state(&self) -> ReceiveLpiStateR {
        ReceiveLpiStateR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Transmit LPI state - asserted when the port 0 transmit is in the LPI state"]
    #[inline(always)]
    pub fn transmit_lpi_state(&self) -> TransmitLpiStateR {
        TransmitLpiStateR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Transmit wakeup - asserted in the transmit LPI2WAKE count time"]
    #[inline(always)]
    pub fn transmit_wakeup_asserted(&self) -> TransmitWakeupAssertedR {
        TransmitWakeupAssertedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Transmit FIFO hold - asserted in the LPI state and during the LPI2WAKE count time"]
    #[inline(always)]
    pub fn transmit_fifo_hold(&self) -> TransmitFifoHoldR {
        TransmitFifoHoldR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Receive FIFO (switch ingress) is empty - contains no packets"]
    #[inline(always)]
    pub fn receive_fifo_switch(&self) -> ReceiveFifoSwitchR {
        ReceiveFifoSwitchR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Transmit FIFO (switch egress) is empty - contains no packets"]
    #[inline(always)]
    pub fn transmit_fifo_switch(&self) -> TransmitFifoSwitchR {
        TransmitFifoSwitchR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
CPPI port 0 wait idle to LPI - asserted when port 0 is counting the IDLE2LPI time"]
    #[inline(always)]
    #[must_use]
    pub fn cppi_port_0(&mut self) -> CppiPort0W<CpswNcEthMac0PnEeeStatusRegSpec> {
        CppiPort0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Receive LPI state - asserted when the port 0 receive is in the LPI state"]
    #[inline(always)]
    #[must_use]
    pub fn receive_lpi_state(&mut self) -> ReceiveLpiStateW<CpswNcEthMac0PnEeeStatusRegSpec> {
        ReceiveLpiStateW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Transmit LPI state - asserted when the port 0 transmit is in the LPI state"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_lpi_state(&mut self) -> TransmitLpiStateW<CpswNcEthMac0PnEeeStatusRegSpec> {
        TransmitLpiStateW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Transmit wakeup - asserted in the transmit LPI2WAKE count time"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_wakeup_asserted(
        &mut self,
    ) -> TransmitWakeupAssertedW<CpswNcEthMac0PnEeeStatusRegSpec> {
        TransmitWakeupAssertedW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Transmit FIFO hold - asserted in the LPI state and during the LPI2WAKE count time"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_fifo_hold(&mut self) -> TransmitFifoHoldW<CpswNcEthMac0PnEeeStatusRegSpec> {
        TransmitFifoHoldW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Receive FIFO (switch ingress) is empty - contains no packets"]
    #[inline(always)]
    #[must_use]
    pub fn receive_fifo_switch(&mut self) -> ReceiveFifoSwitchW<CpswNcEthMac0PnEeeStatusRegSpec> {
        ReceiveFifoSwitchW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Transmit FIFO (switch egress) is empty - contains no packets"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_fifo_switch(&mut self) -> TransmitFifoSwitchW<CpswNcEthMac0PnEeeStatusRegSpec> {
        TransmitFifoSwitchW::new(self, 6)
    }
}
#[doc = "Enet Port N EEE status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_eee_status_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_eee_status_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnEeeStatusRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnEeeStatusRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_eee_status_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnEeeStatusRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_eee_status_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnEeeStatusRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_EEE_STATUS_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnEeeStatusRegSpec {
    const RESET_VALUE: u32 = 0;
}
