#[doc = "Register `MPFAR` reader"]
pub type R = crate::R<MpfarSpec>;
#[doc = "Register `MPFAR` writer"]
pub type W = crate::W<MpfarSpec>;
#[doc = "Field `FADDR` reader - 31:0\\]
Fault Address 32-bit read-only status register containing the faulting address when a memory protection violation is detected. This register can only be cleared via the MPFCR."]
pub type FaddrR = crate::FieldReader<u32>;
#[doc = "Field `FADDR` writer - 31:0\\]
Fault Address 32-bit read-only status register containing the faulting address when a memory protection violation is detected. This register can only be cleared via the MPFCR."]
pub type FaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Fault Address 32-bit read-only status register containing the faulting address when a memory protection violation is detected. This register can only be cleared via the MPFCR."]
    #[inline(always)]
    pub fn faddr(&self) -> FaddrR {
        FaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Fault Address 32-bit read-only status register containing the faulting address when a memory protection violation is detected. This register can only be cleared via the MPFCR."]
    #[inline(always)]
    #[must_use]
    pub fn faddr(&mut self) -> FaddrW<MpfarSpec> {
        FaddrW::new(self, 0)
    }
}
#[doc = "Memory Protect Fault Address\n\nYou can [`read`](crate::Reg::read) this register and get [`mpfar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpfar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MpfarSpec;
impl crate::RegisterSpec for MpfarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpfar::R`](R) reader structure"]
impl crate::Readable for MpfarSpec {}
#[doc = "`write(|w| ..)` method takes [`mpfar::W`](W) writer structure"]
impl crate::Writable for MpfarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPFAR to value 0"]
impl crate::Resettable for MpfarSpec {
    const RESET_VALUE: u32 = 0;
}
