#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_EST_CONTROL_REG` reader"]
pub type R = crate::R<CpswNcEthMac0PnEstControlRegSpec>;
#[doc = "Register `CPSW_NC_ETH_MAC_0_PN_EST_CONTROL_REG` writer"]
pub type W = crate::W<CpswNcEthMac0PnEstControlRegSpec>;
#[doc = "Field `TRANSMIT_FIFO_EST_8` reader - 0:0\\]
Transmit FIFO EST One Buffer"]
pub type TransmitFifoEst8R = crate::BitReader;
#[doc = "Field `TRANSMIT_FIFO_EST_8` writer - 0:0\\]
Transmit FIFO EST One Buffer"]
pub type TransmitFifoEst8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSMIT_FIFO_EST_7` reader - 1:1\\]
Transmit FIFO EST Buffer Select"]
pub type TransmitFifoEst7R = crate::BitReader;
#[doc = "Field `TRANSMIT_FIFO_EST_7` writer - 1:1\\]
Transmit FIFO EST Buffer Select"]
pub type TransmitFifoEst7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSMIT_FIFO_EST_6` reader - 2:2\\]
Transmit FIFO EST TimeStamp Enable"]
pub type TransmitFifoEst6R = crate::BitReader;
#[doc = "Field `TRANSMIT_FIFO_EST_6` writer - 2:2\\]
Transmit FIFO EST TimeStamp Enable"]
pub type TransmitFifoEst6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSMIT_FIFO_EST_5` reader - 3:3\\]
Transmit FIFO EST TimeStamp First Express Packet"]
pub type TransmitFifoEst5R = crate::BitReader;
#[doc = "Field `TRANSMIT_FIFO_EST_5` writer - 3:3\\]
Transmit FIFO EST TimeStamp First Express Packet"]
pub type TransmitFifoEst5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSMIT_FIFO_EST_4` reader - 4:4\\]
Transmit FIFO EST TimeStamp One Priority"]
pub type TransmitFifoEst4R = crate::BitReader;
#[doc = "Field `TRANSMIT_FIFO_EST_4` writer - 4:4\\]
Transmit FIFO EST TimeStamp One Priority"]
pub type TransmitFifoEst4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSMIT_FIFO_EST_3` reader - 7:5\\]
Transmit FIFO EST TimeStamp Priority"]
pub type TransmitFifoEst3R = crate::FieldReader;
#[doc = "Field `TRANSMIT_FIFO_EST_3` writer - 7:5\\]
Transmit FIFO EST TimeStamp Priority"]
pub type TransmitFifoEst3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRANSMIT_FIFO_EST_2` reader - 8:8\\]
Transmit FIFO EST Fill Enable"]
pub type TransmitFifoEst2R = crate::BitReader;
#[doc = "Field `TRANSMIT_FIFO_EST_2` writer - 8:8\\]
Transmit FIFO EST Fill Enable"]
pub type TransmitFifoEst2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSMIT_FIFO_EST_1` reader - 15:9\\]
Transmit FIFO EST Prempt Comparison Value to Clear wire"]
pub type TransmitFifoEst1R = crate::FieldReader;
#[doc = "Field `TRANSMIT_FIFO_EST_1` writer - 15:9\\]
Transmit FIFO EST Prempt Comparison Value to Clear wire"]
pub type TransmitFifoEst1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TRANSMIT_FIFO_EST` reader - 25:16\\]
Transmit FIFO EST Fill Margin"]
pub type TransmitFifoEstR = crate::FieldReader<u16>;
#[doc = "Field `TRANSMIT_FIFO_EST` writer - 25:16\\]
Transmit FIFO EST Fill Margin"]
pub type TransmitFifoEstW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Transmit FIFO EST One Buffer"]
    #[inline(always)]
    pub fn transmit_fifo_est_8(&self) -> TransmitFifoEst8R {
        TransmitFifoEst8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit FIFO EST Buffer Select"]
    #[inline(always)]
    pub fn transmit_fifo_est_7(&self) -> TransmitFifoEst7R {
        TransmitFifoEst7R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Transmit FIFO EST TimeStamp Enable"]
    #[inline(always)]
    pub fn transmit_fifo_est_6(&self) -> TransmitFifoEst6R {
        TransmitFifoEst6R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Transmit FIFO EST TimeStamp First Express Packet"]
    #[inline(always)]
    pub fn transmit_fifo_est_5(&self) -> TransmitFifoEst5R {
        TransmitFifoEst5R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Transmit FIFO EST TimeStamp One Priority"]
    #[inline(always)]
    pub fn transmit_fifo_est_4(&self) -> TransmitFifoEst4R {
        TransmitFifoEst4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Transmit FIFO EST TimeStamp Priority"]
    #[inline(always)]
    pub fn transmit_fifo_est_3(&self) -> TransmitFifoEst3R {
        TransmitFifoEst3R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Transmit FIFO EST Fill Enable"]
    #[inline(always)]
    pub fn transmit_fifo_est_2(&self) -> TransmitFifoEst2R {
        TransmitFifoEst2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Transmit FIFO EST Prempt Comparison Value to Clear wire"]
    #[inline(always)]
    pub fn transmit_fifo_est_1(&self) -> TransmitFifoEst1R {
        TransmitFifoEst1R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Transmit FIFO EST Fill Margin"]
    #[inline(always)]
    pub fn transmit_fifo_est(&self) -> TransmitFifoEstR {
        TransmitFifoEstR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Transmit FIFO EST One Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_fifo_est_8(&mut self) -> TransmitFifoEst8W<CpswNcEthMac0PnEstControlRegSpec> {
        TransmitFifoEst8W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit FIFO EST Buffer Select"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_fifo_est_7(&mut self) -> TransmitFifoEst7W<CpswNcEthMac0PnEstControlRegSpec> {
        TransmitFifoEst7W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Transmit FIFO EST TimeStamp Enable"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_fifo_est_6(&mut self) -> TransmitFifoEst6W<CpswNcEthMac0PnEstControlRegSpec> {
        TransmitFifoEst6W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Transmit FIFO EST TimeStamp First Express Packet"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_fifo_est_5(&mut self) -> TransmitFifoEst5W<CpswNcEthMac0PnEstControlRegSpec> {
        TransmitFifoEst5W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Transmit FIFO EST TimeStamp One Priority"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_fifo_est_4(&mut self) -> TransmitFifoEst4W<CpswNcEthMac0PnEstControlRegSpec> {
        TransmitFifoEst4W::new(self, 4)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Transmit FIFO EST TimeStamp Priority"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_fifo_est_3(&mut self) -> TransmitFifoEst3W<CpswNcEthMac0PnEstControlRegSpec> {
        TransmitFifoEst3W::new(self, 5)
    }
    #[doc = "Bit 8 - 8:8\\]
Transmit FIFO EST Fill Enable"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_fifo_est_2(&mut self) -> TransmitFifoEst2W<CpswNcEthMac0PnEstControlRegSpec> {
        TransmitFifoEst2W::new(self, 8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Transmit FIFO EST Prempt Comparison Value to Clear wire"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_fifo_est_1(&mut self) -> TransmitFifoEst1W<CpswNcEthMac0PnEstControlRegSpec> {
        TransmitFifoEst1W::new(self, 9)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Transmit FIFO EST Fill Margin"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_fifo_est(&mut self) -> TransmitFifoEstW<CpswNcEthMac0PnEstControlRegSpec> {
        TransmitFifoEstW::new(self, 16)
    }
}
#[doc = "Enet Port N EST CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_eth_mac_0_pn_est_control_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_eth_mac_0_pn_est_control_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcEthMac0PnEstControlRegSpec;
impl crate::RegisterSpec for CpswNcEthMac0PnEstControlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_eth_mac_0_pn_est_control_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcEthMac0PnEstControlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_eth_mac_0_pn_est_control_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcEthMac0PnEstControlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_ETH_MAC_0_PN_EST_CONTROL_REG to value 0"]
impl crate::Resettable for CpswNcEthMac0PnEstControlRegSpec {
    const RESET_VALUE: u32 = 0;
}
