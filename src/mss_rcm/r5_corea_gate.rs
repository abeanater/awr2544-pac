#[doc = "Register `R5_COREA_GATE` reader"]
pub type R = crate::R<R5CoreaGateSpec>;
#[doc = "Register `R5_COREA_GATE` writer"]
pub type W = crate::W<R5CoreaGateSpec>;
#[doc = "Field `clkgate` reader - 2:0\\]
writing '111' will gate clock to CR5A related peripherals inside Cortexr5ss"]
pub type ClkgateR = crate::FieldReader;
#[doc = "Field `clkgate` writer - 2:0\\]
writing '111' will gate clock to CR5A related peripherals inside Cortexr5ss"]
pub type ClkgateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
writing '111' will gate clock to CR5A related peripherals inside Cortexr5ss"]
    #[inline(always)]
    pub fn clkgate(&self) -> ClkgateR {
        ClkgateR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
writing '111' will gate clock to CR5A related peripherals inside Cortexr5ss"]
    #[inline(always)]
    #[must_use]
    pub fn clkgate(&mut self) -> ClkgateW<R5CoreaGateSpec> {
        ClkgateW::new(self, 0)
    }
}
#[doc = "R5_COREA_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`r5_corea_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5_corea_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R5CoreaGateSpec;
impl crate::RegisterSpec for R5CoreaGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r5_corea_gate::R`](R) reader structure"]
impl crate::Readable for R5CoreaGateSpec {}
#[doc = "`write(|w| ..)` method takes [`r5_corea_gate::W`](W) writer structure"]
impl crate::Writable for R5CoreaGateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R5_COREA_GATE to value 0"]
impl crate::Resettable for R5CoreaGateSpec {
    const RESET_VALUE: u32 = 0;
}
