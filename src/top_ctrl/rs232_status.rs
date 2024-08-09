#[doc = "Register `RS232_STATUS` reader"]
pub type R = crate::R<Rs232StatusSpec>;
#[doc = "Register `RS232_STATUS` writer"]
pub type W = crate::W<Rs232StatusSpec>;
#[doc = "Field `clk_duration` reader - 11:0\\]
Number of vbusp clocks measured in one sleep clock frequency"]
pub type ClkDurationR = crate::FieldReader<u16>;
#[doc = "Field `clk_duration` writer - 11:0\\]
Number of vbusp clocks measured in one sleep clock frequency"]
pub type ClkDurationW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `curr_bit_interval` reader - 27:16\\]
Current RS232 bit interval being used"]
pub type CurrBitIntervalR = crate::FieldReader<u16>;
#[doc = "Field `curr_bit_interval` writer - 27:16\\]
Current RS232 bit interval being used"]
pub type CurrBitIntervalW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Number of vbusp clocks measured in one sleep clock frequency"]
    #[inline(always)]
    pub fn clk_duration(&self) -> ClkDurationR {
        ClkDurationR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Current RS232 bit interval being used"]
    #[inline(always)]
    pub fn curr_bit_interval(&self) -> CurrBitIntervalR {
        CurrBitIntervalR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Number of vbusp clocks measured in one sleep clock frequency"]
    #[inline(always)]
    #[must_use]
    pub fn clk_duration(&mut self) -> ClkDurationW<Rs232StatusSpec> {
        ClkDurationW::new(self, 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Current RS232 bit interval being used"]
    #[inline(always)]
    #[must_use]
    pub fn curr_bit_interval(&mut self) -> CurrBitIntervalW<Rs232StatusSpec> {
        CurrBitIntervalW::new(self, 16)
    }
}
#[doc = "RS232_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`rs232_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rs232_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rs232StatusSpec;
impl crate::RegisterSpec for Rs232StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rs232_status::R`](R) reader structure"]
impl crate::Readable for Rs232StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`rs232_status::W`](W) writer structure"]
impl crate::Writable for Rs232StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RS232_STATUS to value 0"]
impl crate::Resettable for Rs232StatusSpec {
    const RESET_VALUE: u32 = 0;
}
