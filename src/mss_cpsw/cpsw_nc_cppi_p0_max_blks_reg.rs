#[doc = "Register `CPSW_NC_CPPI_P0_MAX_BLKS_REG` reader"]
pub type R = crate::R<CpswNcCppiP0MaxBlksRegSpec>;
#[doc = "Register `CPSW_NC_CPPI_P0_MAX_BLKS_REG` writer"]
pub type W = crate::W<CpswNcCppiP0MaxBlksRegSpec>;
#[doc = "Field `RECEIVE_FIFO_MAXIMUM` reader - 7:0\\]
Receive FIFO maximum blocks"]
pub type ReceiveFifoMaximumR = crate::FieldReader;
#[doc = "Field `RECEIVE_FIFO_MAXIMUM` writer - 7:0\\]
Receive FIFO maximum blocks"]
pub type ReceiveFifoMaximumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRANSMIT_FIFO_MAXIMUM` reader - 15:8\\]
Transmit FIFO maximum blocks"]
pub type TransmitFifoMaximumR = crate::FieldReader;
#[doc = "Field `TRANSMIT_FIFO_MAXIMUM` writer - 15:8\\]
Transmit FIFO maximum blocks"]
pub type TransmitFifoMaximumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Receive FIFO maximum blocks"]
    #[inline(always)]
    pub fn receive_fifo_maximum(&self) -> ReceiveFifoMaximumR {
        ReceiveFifoMaximumR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Transmit FIFO maximum blocks"]
    #[inline(always)]
    pub fn transmit_fifo_maximum(&self) -> TransmitFifoMaximumR {
        TransmitFifoMaximumR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Receive FIFO maximum blocks"]
    #[inline(always)]
    #[must_use]
    pub fn receive_fifo_maximum(&mut self) -> ReceiveFifoMaximumW<CpswNcCppiP0MaxBlksRegSpec> {
        ReceiveFifoMaximumW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Transmit FIFO maximum blocks"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_fifo_maximum(&mut self) -> TransmitFifoMaximumW<CpswNcCppiP0MaxBlksRegSpec> {
        TransmitFifoMaximumW::new(self, 8)
    }
}
#[doc = "Port 0 FIFO Max Blocks\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_cppi_p0_max_blks_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_cppi_p0_max_blks_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcCppiP0MaxBlksRegSpec;
impl crate::RegisterSpec for CpswNcCppiP0MaxBlksRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_cppi_p0_max_blks_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcCppiP0MaxBlksRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_cppi_p0_max_blks_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcCppiP0MaxBlksRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_CPPI_P0_MAX_BLKS_REG to value 0"]
impl crate::Resettable for CpswNcCppiP0MaxBlksRegSpec {
    const RESET_VALUE: u32 = 0;
}
