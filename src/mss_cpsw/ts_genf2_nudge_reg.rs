#[doc = "Register `TS_GENF2_NUDGE_REG` reader"]
pub type R = crate::R<TsGenf2NudgeRegSpec>;
#[doc = "Register `TS_GENF2_NUDGE_REG` writer"]
pub type W = crate::W<TsGenf2NudgeRegSpec>;
#[doc = "Field `TIME_STAMP_GENERATE` reader - 7:0\\]
Time Stamp Generate Function Nudge Value"]
pub type TimeStampGenerateR = crate::FieldReader;
#[doc = "Field `TIME_STAMP_GENERATE` writer - 7:0\\]
Time Stamp Generate Function Nudge Value"]
pub type TimeStampGenerateW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Time Stamp Generate Function Nudge Value"]
    #[inline(always)]
    pub fn time_stamp_generate(&self) -> TimeStampGenerateR {
        TimeStampGenerateR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Time Stamp Generate Function Nudge Value"]
    #[inline(always)]
    #[must_use]
    pub fn time_stamp_generate(&mut self) -> TimeStampGenerateW<TsGenf2NudgeRegSpec> {
        TimeStampGenerateW::new(self, 0)
    }
}
#[doc = "Time Stamp Generate Function Nudge Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_genf2_nudge_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_genf2_nudge_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsGenf2NudgeRegSpec;
impl crate::RegisterSpec for TsGenf2NudgeRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts_genf2_nudge_reg::R`](R) reader structure"]
impl crate::Readable for TsGenf2NudgeRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ts_genf2_nudge_reg::W`](W) writer structure"]
impl crate::Writable for TsGenf2NudgeRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TS_GENF2_NUDGE_REG to value 0"]
impl crate::Resettable for TsGenf2NudgeRegSpec {
    const RESET_VALUE: u32 = 0;
}
