#[doc = "Register `CORE2_CURMISR_8` reader"]
pub type R = crate::R<Core2Curmisr8Spec>;
#[doc = "Register `CORE2_CURMISR_8` writer"]
pub type W = crate::W<Core2Curmisr8Spec>;
#[doc = "Field `C2MISR8` reader - 31:0\\]
MISR Signature for CORE2 This register contains the MISR data from the CORE2 for the current interval. This is applicable to Segment 0 alone. This value will be compared with the GOLDEN MISR value copied from ROM. This register gets reset to its default value with Power on or system reset assertion. The MISR values should be read only after the Self Test is completed."]
pub type C2misr8R = crate::FieldReader<u32>;
#[doc = "Field `C2MISR8` writer - 31:0\\]
MISR Signature for CORE2 This register contains the MISR data from the CORE2 for the current interval. This is applicable to Segment 0 alone. This value will be compared with the GOLDEN MISR value copied from ROM. This register gets reset to its default value with Power on or system reset assertion. The MISR values should be read only after the Self Test is completed."]
pub type C2misr8W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
MISR Signature for CORE2 This register contains the MISR data from the CORE2 for the current interval. This is applicable to Segment 0 alone. This value will be compared with the GOLDEN MISR value copied from ROM. This register gets reset to its default value with Power on or system reset assertion. The MISR values should be read only after the Self Test is completed."]
    #[inline(always)]
    pub fn c2misr8(&self) -> C2misr8R {
        C2misr8R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
MISR Signature for CORE2 This register contains the MISR data from the CORE2 for the current interval. This is applicable to Segment 0 alone. This value will be compared with the GOLDEN MISR value copied from ROM. This register gets reset to its default value with Power on or system reset assertion. The MISR values should be read only after the Self Test is completed."]
    #[inline(always)]
    #[must_use]
    pub fn c2misr8(&mut self) -> C2misr8W<Core2Curmisr8Spec> {
        C2misr8W::new(self, 0)
    }
}
#[doc = "Holds the MISR signature for CORE2\n\nYou can [`read`](crate::Reg::read) this register and get [`core2_curmisr_8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core2_curmisr_8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core2Curmisr8Spec;
impl crate::RegisterSpec for Core2Curmisr8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core2_curmisr_8::R`](R) reader structure"]
impl crate::Readable for Core2Curmisr8Spec {}
#[doc = "`write(|w| ..)` method takes [`core2_curmisr_8::W`](W) writer structure"]
impl crate::Writable for Core2Curmisr8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE2_CURMISR_8 to value 0"]
impl crate::Resettable for Core2Curmisr8Spec {
    const RESET_VALUE: u32 = 0;
}
