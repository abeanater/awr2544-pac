#[doc = "Register `TS_ESTF_COMP_LOW_REG` reader"]
pub type R = crate::R<TsEstfCompLowRegSpec>;
#[doc = "Register `TS_ESTF_COMP_LOW_REG` writer"]
pub type W = crate::W<TsEstfCompLowRegSpec>;
#[doc = "Field `TIME_STAMP_ESTF` reader - 31:0\\]
Time Stamp ESTF Generate Function Comparison Low Value"]
pub type TimeStampEstfR = crate::FieldReader<u32>;
#[doc = "Field `TIME_STAMP_ESTF` writer - 31:0\\]
Time Stamp ESTF Generate Function Comparison Low Value"]
pub type TimeStampEstfW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Time Stamp ESTF Generate Function Comparison Low Value"]
    #[inline(always)]
    pub fn time_stamp_estf(&self) -> TimeStampEstfR {
        TimeStampEstfR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Time Stamp ESTF Generate Function Comparison Low Value"]
    #[inline(always)]
    #[must_use]
    pub fn time_stamp_estf(&mut self) -> TimeStampEstfW<TsEstfCompLowRegSpec> {
        TimeStampEstfW::new(self, 0)
    }
}
#[doc = "Time Stamp ESTF Generate Function Comparison Low Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_estf_comp_low_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_estf_comp_low_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsEstfCompLowRegSpec;
impl crate::RegisterSpec for TsEstfCompLowRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts_estf_comp_low_reg::R`](R) reader structure"]
impl crate::Readable for TsEstfCompLowRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ts_estf_comp_low_reg::W`](W) writer structure"]
impl crate::Writable for TsEstfCompLowRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TS_ESTF_COMP_LOW_REG to value 0"]
impl crate::Resettable for TsEstfCompLowRegSpec {
    const RESET_VALUE: u32 = 0;
}
