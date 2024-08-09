#[doc = "Register `CPTS_TS_NUDGE_VAL_REG` reader"]
pub type R = crate::R<CptsTsNudgeValRegSpec>;
#[doc = "Register `CPTS_TS_NUDGE_VAL_REG` writer"]
pub type W = crate::W<CptsTsNudgeValRegSpec>;
#[doc = "Field `TIME_STAMP_NUDGE` reader - 7:0\\]
Time stamp Nudge value"]
pub type TimeStampNudgeR = crate::FieldReader;
#[doc = "Field `TIME_STAMP_NUDGE` writer - 7:0\\]
Time stamp Nudge value"]
pub type TimeStampNudgeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Time stamp Nudge value"]
    #[inline(always)]
    pub fn time_stamp_nudge(&self) -> TimeStampNudgeR {
        TimeStampNudgeR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Time stamp Nudge value"]
    #[inline(always)]
    #[must_use]
    pub fn time_stamp_nudge(&mut self) -> TimeStampNudgeW<CptsTsNudgeValRegSpec> {
        TimeStampNudgeW::new(self, 0)
    }
}
#[doc = "Time Stamp Nudge Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_ts_nudge_val_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_ts_nudge_val_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsTsNudgeValRegSpec;
impl crate::RegisterSpec for CptsTsNudgeValRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_ts_nudge_val_reg::R`](R) reader structure"]
impl crate::Readable for CptsTsNudgeValRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_ts_nudge_val_reg::W`](W) writer structure"]
impl crate::Writable for CptsTsNudgeValRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_TS_NUDGE_VAL_REG to value 0"]
impl crate::Resettable for CptsTsNudgeValRegSpec {
    const RESET_VALUE: u32 = 0;
}
