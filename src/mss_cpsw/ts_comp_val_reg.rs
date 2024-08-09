#[doc = "Register `TS_COMP_VAL_REG` reader"]
pub type R = crate::R<TsCompValRegSpec>;
#[doc = "Register `TS_COMP_VAL_REG` writer"]
pub type W = crate::W<TsCompValRegSpec>;
#[doc = "Field `TIME_STAMP_COMPARISON` reader - 31:0\\]
Time stamp comparison low value"]
pub type TimeStampComparisonR = crate::FieldReader<u32>;
#[doc = "Field `TIME_STAMP_COMPARISON` writer - 31:0\\]
Time stamp comparison low value"]
pub type TimeStampComparisonW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Time stamp comparison low value"]
    #[inline(always)]
    pub fn time_stamp_comparison(&self) -> TimeStampComparisonR {
        TimeStampComparisonR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Time stamp comparison low value"]
    #[inline(always)]
    #[must_use]
    pub fn time_stamp_comparison(&mut self) -> TimeStampComparisonW<TsCompValRegSpec> {
        TimeStampComparisonW::new(self, 0)
    }
}
#[doc = "Time Stamp Comparison Low Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_comp_val_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_comp_val_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsCompValRegSpec;
impl crate::RegisterSpec for TsCompValRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts_comp_val_reg::R`](R) reader structure"]
impl crate::Readable for TsCompValRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ts_comp_val_reg::W`](W) writer structure"]
impl crate::Writable for TsCompValRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TS_COMP_VAL_REG to value 0"]
impl crate::Resettable for TsCompValRegSpec {
    const RESET_VALUE: u32 = 0;
}
