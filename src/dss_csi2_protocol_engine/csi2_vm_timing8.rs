#[doc = "Register `CSI2_VM_TIMING8` reader"]
pub type R = crate::R<Csi2VmTiming8Spec>;
#[doc = "Register `CSI2_VM_TIMING8` writer"]
pub type W = crate::W<Csi2VmTiming8Spec>;
#[doc = "Field `HFPX` reader - 1:0\\]
Extension to the HFP register. Additional bits added to MSB."]
pub type HfpxR = crate::FieldReader;
#[doc = "Field `HFPX` writer - 1:0\\]
Extension to the HFP register. Additional bits added to MSB."]
pub type HfpxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Extension to the HFP register. Additional bits added to MSB."]
    #[inline(always)]
    pub fn hfpx(&self) -> HfpxR {
        HfpxR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Extension to the HFP register. Additional bits added to MSB."]
    #[inline(always)]
    #[must_use]
    pub fn hfpx(&mut self) -> HfpxW<Csi2VmTiming8Spec> {
        HfpxW::new(self, 0)
    }
}
#[doc = "VIDEO MODE TIMING REGISTER This register defines the video mode timing.\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_vm_timing8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_vm_timing8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2VmTiming8Spec;
impl crate::RegisterSpec for Csi2VmTiming8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_vm_timing8::R`](R) reader structure"]
impl crate::Readable for Csi2VmTiming8Spec {}
#[doc = "`write(|w| ..)` method takes [`csi2_vm_timing8::W`](W) writer structure"]
impl crate::Writable for Csi2VmTiming8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_VM_TIMING8 to value 0"]
impl crate::Resettable for Csi2VmTiming8Spec {
    const RESET_VALUE: u32 = 0;
}
