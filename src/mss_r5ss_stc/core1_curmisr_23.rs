#[doc = "Register `CORE1_CURMISR_23` reader"]
pub type R = crate::R<Core1Curmisr23Spec>;
#[doc = "Register `CORE1_CURMISR_23` writer"]
pub type W = crate::W<Core1Curmisr23Spec>;
#[doc = "Field `C1MISR23` reader - 31:0\\]
MISR Signature for CORE1 This register contains the MISR data of the current interval for CORE1 in the case of segment0 and the remaining Segments 1 to 3. This value will be compared with the GOLDEN MISR value copied from ROM. This register gets reset to its default value with Power on or system reset assertion. The MISR values should be read only after the Self Test is completed."]
pub type C1misr23R = crate::FieldReader<u32>;
#[doc = "Field `C1MISR23` writer - 31:0\\]
MISR Signature for CORE1 This register contains the MISR data of the current interval for CORE1 in the case of segment0 and the remaining Segments 1 to 3. This value will be compared with the GOLDEN MISR value copied from ROM. This register gets reset to its default value with Power on or system reset assertion. The MISR values should be read only after the Self Test is completed."]
pub type C1misr23W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
MISR Signature for CORE1 This register contains the MISR data of the current interval for CORE1 in the case of segment0 and the remaining Segments 1 to 3. This value will be compared with the GOLDEN MISR value copied from ROM. This register gets reset to its default value with Power on or system reset assertion. The MISR values should be read only after the Self Test is completed."]
    #[inline(always)]
    pub fn c1misr23(&self) -> C1misr23R {
        C1misr23R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
MISR Signature for CORE1 This register contains the MISR data of the current interval for CORE1 in the case of segment0 and the remaining Segments 1 to 3. This value will be compared with the GOLDEN MISR value copied from ROM. This register gets reset to its default value with Power on or system reset assertion. The MISR values should be read only after the Self Test is completed."]
    #[inline(always)]
    #[must_use]
    pub fn c1misr23(&mut self) -> C1misr23W<Core1Curmisr23Spec> {
        C1misr23W::new(self, 0)
    }
}
#[doc = "Holds the MISR signature for CORE1\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_curmisr_23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_curmisr_23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core1Curmisr23Spec;
impl crate::RegisterSpec for Core1Curmisr23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core1_curmisr_23::R`](R) reader structure"]
impl crate::Readable for Core1Curmisr23Spec {}
#[doc = "`write(|w| ..)` method takes [`core1_curmisr_23::W`](W) writer structure"]
impl crate::Writable for Core1Curmisr23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE1_CURMISR_23 to value 0"]
impl crate::Resettable for Core1Curmisr23Spec {
    const RESET_VALUE: u32 = 0;
}
