#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_FIFO_STATUS_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnFifoStatusRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_FIFO_STATUS_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnFifoStatusRegSpec>;
#[doc = "Field `TRANSMIT_FIFO_PRIORITY` reader - 7:0\\]
Transmit FIFO Priority Active"]
pub type TransmitFifoPriorityR = crate::FieldReader;
#[doc = "Field `TRANSMIT_FIFO_PRIORITY` writer - 7:0\\]
Transmit FIFO Priority Active"]
pub type TransmitFifoPriorityW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRANSMIT_FIFO_EXPRESS` reader - 15:8\\]
Transmit FIFO Express Queue Priority Allow"]
pub type TransmitFifoExpressR = crate::FieldReader;
#[doc = "Field `TRANSMIT_FIFO_EXPRESS` writer - 15:8\\]
Transmit FIFO Express Queue Priority Allow"]
pub type TransmitFifoExpressW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRANSMIT_FIFO_EST_2` reader - 16:16\\]
Transmit FIFO EST Count Error"]
pub type TransmitFifoEst2R = crate::BitReader;
#[doc = "Field `TRANSMIT_FIFO_EST_2` writer - 16:16\\]
Transmit FIFO EST Count Error"]
pub type TransmitFifoEst2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSMIT_FIFO_EST_1` reader - 17:17\\]
Transmit FIFO EST Address Error"]
pub type TransmitFifoEst1R = crate::BitReader;
#[doc = "Field `TRANSMIT_FIFO_EST_1` writer - 17:17\\]
Transmit FIFO EST Address Error"]
pub type TransmitFifoEst1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSMIT_FIFO_EST` reader - 18:18\\]
Transmit FIFO EST Buffer Active"]
pub type TransmitFifoEstR = crate::BitReader;
#[doc = "Field `TRANSMIT_FIFO_EST` writer - 18:18\\]
Transmit FIFO EST Buffer Active"]
pub type TransmitFifoEstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Transmit FIFO Priority Active"]
    #[inline(always)]
    pub fn transmit_fifo_priority(&self) -> TransmitFifoPriorityR {
        TransmitFifoPriorityR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Transmit FIFO Express Queue Priority Allow"]
    #[inline(always)]
    pub fn transmit_fifo_express(&self) -> TransmitFifoExpressR {
        TransmitFifoExpressR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Transmit FIFO EST Count Error"]
    #[inline(always)]
    pub fn transmit_fifo_est_2(&self) -> TransmitFifoEst2R {
        TransmitFifoEst2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Transmit FIFO EST Address Error"]
    #[inline(always)]
    pub fn transmit_fifo_est_1(&self) -> TransmitFifoEst1R {
        TransmitFifoEst1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Transmit FIFO EST Buffer Active"]
    #[inline(always)]
    pub fn transmit_fifo_est(&self) -> TransmitFifoEstR {
        TransmitFifoEstR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Transmit FIFO Priority Active"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_fifo_priority(
        &mut self,
    ) -> TransmitFifoPriorityW<CpswNcEthMac0PnFifoStatusRegSpec> {
        TransmitFifoPriorityW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Transmit FIFO Express Queue Priority Allow"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_fifo_express(
        &mut self,
    ) -> TransmitFifoExpressW<CpswNcEthMac0PnFifoStatusRegSpec> {
        TransmitFifoExpressW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Transmit FIFO EST Count Error"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_fifo_est_2(&mut self) -> TransmitFifoEst2W<CpswNcEthMac0PnFifoStatusRegSpec> {
        TransmitFifoEst2W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Transmit FIFO EST Address Error"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_fifo_est_1(&mut self) -> TransmitFifoEst1W<CpswNcEthMac0PnFifoStatusRegSpec> {
        TransmitFifoEst1W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Transmit FIFO EST Buffer Active"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_fifo_est(&mut self) -> TransmitFifoEstW<CpswNcEthMac0PnFifoStatusRegSpec> {
        TransmitFifoEstW::new(self, 18)
    }
}
#[doc = "Enet Port N FIFO STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_fifo_status_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_fifo_status_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnFifoStatusRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnFifoStatusRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_fifo_status_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnFifoStatusRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_fifo_status_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnFifoStatusRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_FIFO_STATUS_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnFifoStatusRegSpec {
    const RESET_VALUE: u32 = 0;
}
