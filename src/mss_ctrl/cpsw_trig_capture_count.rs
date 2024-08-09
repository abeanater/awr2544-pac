#[doc = "Register `CPSW_TRIG_CAPTURE_COUNT` reader"]
pub type R = crate::R<CpswTrigCaptureCountSpec>;
#[doc = "Register `CPSW_TRIG_CAPTURE_COUNT` writer"]
pub type W = crate::W<CpswTrigCaptureCountSpec>;
#[doc = "Field `trig_count` reader - 3:0\\]
Configuration of number of cycles for pulse extender of TRIG signal to CPSW"]
pub type TrigCountR = crate::FieldReader;
#[doc = "Field `trig_count` writer - 3:0\\]
Configuration of number of cycles for pulse extender of TRIG signal to CPSW"]
pub type TrigCountW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Configuration of number of cycles for pulse extender of TRIG signal to CPSW"]
    #[inline(always)]
    pub fn trig_count(&self) -> TrigCountR {
        TrigCountR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Configuration of number of cycles for pulse extender of TRIG signal to CPSW"]
    #[inline(always)]
    #[must_use]
    pub fn trig_count(&mut self) -> TrigCountW<CpswTrigCaptureCountSpec> {
        TrigCountW::new(self, 0)
    }
}
#[doc = "CPSW_TRIG_CAPTURE_COUNT\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_trig_capture_count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_trig_capture_count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswTrigCaptureCountSpec;
impl crate::RegisterSpec for CpswTrigCaptureCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_trig_capture_count::R`](R) reader structure"]
impl crate::Readable for CpswTrigCaptureCountSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_trig_capture_count::W`](W) writer structure"]
impl crate::Writable for CpswTrigCaptureCountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_TRIG_CAPTURE_COUNT to value 0"]
impl crate::Resettable for CpswTrigCaptureCountSpec {
    const RESET_VALUE: u32 = 0;
}
