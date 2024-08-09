#[doc = "Register `TS_GENF2_COMP_LOW_REG` reader"]
pub type R = crate::R<TsGenf2CompLowRegSpec>;
#[doc = "Register `TS_GENF2_COMP_LOW_REG` writer"]
pub type W = crate::W<TsGenf2CompLowRegSpec>;
#[doc = "Field `TIME_STAMP_GENERATE` reader - 31:0\\]
Time Stamp Generate Function Comparison Low Value"]
pub type TimeStampGenerateR = crate::FieldReader<u32>;
#[doc = "Field `TIME_STAMP_GENERATE` writer - 31:0\\]
Time Stamp Generate Function Comparison Low Value"]
pub type TimeStampGenerateW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Time Stamp Generate Function Comparison Low Value"]
    #[inline(always)]
    pub fn time_stamp_generate(&self) -> TimeStampGenerateR {
        TimeStampGenerateR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Time Stamp Generate Function Comparison Low Value"]
    #[inline(always)]
    #[must_use]
    pub fn time_stamp_generate(&mut self) -> TimeStampGenerateW<TsGenf2CompLowRegSpec> {
        TimeStampGenerateW::new(self, 0)
    }
}
#[doc = "Time Stamp Generate Function Comparison Low Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf2_comp_low_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf2_comp_low_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsGenf2CompLowRegSpec;
impl crate::RegisterSpec for TsGenf2CompLowRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts_genf2_comp_low_reg::R`](R) reader structure"]
impl crate::Readable for TsGenf2CompLowRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ts_genf2_comp_low_reg::W`](W) writer structure"]
impl crate::Writable for TsGenf2CompLowRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TS_GENF2_COMP_LOW_REG to value 0"]
impl crate::Resettable for TsGenf2CompLowRegSpec {
    const RESET_VALUE: u32 = 0;
}
