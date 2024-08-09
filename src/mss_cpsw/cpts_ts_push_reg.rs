#[doc = "Register `CPTS_TS_PUSH_REG` reader"]
pub type R = crate::R<CptsTsPushRegSpec>;
#[doc = "Register `CPTS_TS_PUSH_REG` writer"]
pub type W = crate::W<CptsTsPushRegSpec>;
#[doc = "Field `TIME_STAMP_EVENT` reader - 0:0\\]
Time stamp event push"]
pub type TimeStampEventR = crate::BitReader;
#[doc = "Field `TIME_STAMP_EVENT` writer - 0:0\\]
Time stamp event push"]
pub type TimeStampEventW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Time stamp event push"]
    #[inline(always)]
    pub fn time_stamp_event(&self) -> TimeStampEventR {
        TimeStampEventR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Time stamp event push"]
    #[inline(always)]
    #[must_use]
    pub fn time_stamp_event(&mut self) -> TimeStampEventW<CptsTsPushRegSpec> {
        TimeStampEventW::new(self, 0)
    }
}
#[doc = "Time Stamp Event Push Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_ts_push_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_ts_push_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsTsPushRegSpec;
impl crate::RegisterSpec for CptsTsPushRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_ts_push_reg::R`](R) reader structure"]
impl crate::Readable for CptsTsPushRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_ts_push_reg::W`](W) writer structure"]
impl crate::Writable for CptsTsPushRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_TS_PUSH_REG to value 0"]
impl crate::Resettable for CptsTsPushRegSpec {
    const RESET_VALUE: u32 = 0;
}
