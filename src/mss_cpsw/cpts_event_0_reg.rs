#[doc = "Register `CPTS_EVENT_0_REG` reader"]
pub type R = crate::R<CptsEvent0RegSpec>;
#[doc = "Register `CPTS_EVENT_0_REG` writer"]
pub type W = crate::W<CptsEvent0RegSpec>;
#[doc = "Field `TIME_STAMP` reader - 31:0\\]
Time Stamp"]
pub type TimeStampR = crate::FieldReader<u32>;
#[doc = "Field `TIME_STAMP` writer - 31:0\\]
Time Stamp"]
pub type TimeStampW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Time Stamp"]
    #[inline(always)]
    pub fn time_stamp(&self) -> TimeStampR {
        TimeStampR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Time Stamp"]
    #[inline(always)]
    #[must_use]
    pub fn time_stamp(&mut self) -> TimeStampW<CptsEvent0RegSpec> {
        TimeStampW::new(self, 0)
    }
}
#[doc = "Event 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpts_event_0_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpts_event_0_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsEvent0RegSpec;
impl crate::RegisterSpec for CptsEvent0RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_event_0_reg::R`](R) reader structure"]
impl crate::Readable for CptsEvent0RegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_event_0_reg::W`](W) writer structure"]
impl crate::Writable for CptsEvent0RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_EVENT_0_REG to value 0"]
impl crate::Resettable for CptsEvent0RegSpec {
    const RESET_VALUE: u32 = 0;
}
