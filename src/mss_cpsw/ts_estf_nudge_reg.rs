#[doc = "Register `TS_ESTF_NUDGE_REG` reader"]
pub type R = crate::R<TsEstfNudgeRegSpec>;
#[doc = "Register `TS_ESTF_NUDGE_REG` writer"]
pub type W = crate::W<TsEstfNudgeRegSpec>;
#[doc = "Field `TIME_STAMP_ESTF` reader - 7:0\\]
Time Stamp ESTF Generate Function Nudge Value"]
pub type TimeStampEstfR = crate::FieldReader;
#[doc = "Field `TIME_STAMP_ESTF` writer - 7:0\\]
Time Stamp ESTF Generate Function Nudge Value"]
pub type TimeStampEstfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Time Stamp ESTF Generate Function Nudge Value"]
    #[inline(always)]
    pub fn time_stamp_estf(&self) -> TimeStampEstfR {
        TimeStampEstfR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Time Stamp ESTF Generate Function Nudge Value"]
    #[inline(always)]
    #[must_use]
    pub fn time_stamp_estf(&mut self) -> TimeStampEstfW<TsEstfNudgeRegSpec> {
        TimeStampEstfW::new(self, 0)
    }
}
#[doc = "Time Stamp ESTF Generate Function Nudge Value\n\nYou can [`read`](crate::Reg::read) this register and get [`ts_estf_nudge_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts_estf_nudge_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsEstfNudgeRegSpec;
impl crate::RegisterSpec for TsEstfNudgeRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts_estf_nudge_reg::R`](R) reader structure"]
impl crate::Readable for TsEstfNudgeRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ts_estf_nudge_reg::W`](W) writer structure"]
impl crate::Writable for TsEstfNudgeRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TS_ESTF_NUDGE_REG to value 0"]
impl crate::Resettable for TsEstfNudgeRegSpec {
    const RESET_VALUE: u32 = 0;
}
