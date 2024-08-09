#[doc = "Register `CPSW_NC_THRU_RATE_REG` reader"]
pub type R = crate::R<CpswNcThruRateRegSpec>;
#[doc = "Register `CPSW_NC_THRU_RATE_REG` writer"]
pub type W = crate::W<CpswNcThruRateRegSpec>;
#[doc = "Field `CPPI_FIFO_RECEIVE` reader - 3:0\\]
CPPI FIFO receive through rate"]
pub type CppiFifoReceiveR = crate::FieldReader;
#[doc = "Field `CPPI_FIFO_RECEIVE` writer - 3:0\\]
CPPI FIFO receive through rate"]
pub type CppiFifoReceiveW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SWITCH_FIFO_RECEIVE` reader - 15:12\\]
Switch FIFO receive through rate"]
pub type SwitchFifoReceiveR = crate::FieldReader;
#[doc = "Field `SWITCH_FIFO_RECEIVE` writer - 15:12\\]
Switch FIFO receive through rate"]
pub type SwitchFifoReceiveW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
CPPI FIFO receive through rate"]
    #[inline(always)]
    pub fn cppi_fifo_receive(&self) -> CppiFifoReceiveR {
        CppiFifoReceiveR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Switch FIFO receive through rate"]
    #[inline(always)]
    pub fn switch_fifo_receive(&self) -> SwitchFifoReceiveR {
        SwitchFifoReceiveR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
CPPI FIFO receive through rate"]
    #[inline(always)]
    #[must_use]
    pub fn cppi_fifo_receive(&mut self) -> CppiFifoReceiveW<CpswNcThruRateRegSpec> {
        CppiFifoReceiveW::new(self, 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Switch FIFO receive through rate"]
    #[inline(always)]
    #[must_use]
    pub fn switch_fifo_receive(&mut self) -> SwitchFifoReceiveW<CpswNcThruRateRegSpec> {
        SwitchFifoReceiveW::new(self, 12)
    }
}
#[doc = "CPSW Thru Rate\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_thru_rate_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_thru_rate_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcThruRateRegSpec;
impl crate::RegisterSpec for CpswNcThruRateRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_thru_rate_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcThruRateRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_thru_rate_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcThruRateRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_THRU_RATE_REG to value 0"]
impl crate::Resettable for CpswNcThruRateRegSpec {
    const RESET_VALUE: u32 = 0;
}
