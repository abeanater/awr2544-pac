#[doc = "Register `INTMAP_4` reader"]
pub type R = crate::R<Intmap4Spec>;
#[doc = "Register `INTMAP_4` writer"]
pub type W = crate::W<Intmap4Spec>;
#[doc = "Field `MASK` reader - 31:0\\]
This field is used to indicate which interrupt the corresponding event influences (if enabled) for event group M. Each bit corresponds to event Q where Q = Mx32+Bit 0 IRQ Interrupt (default) 1 FIQ Interrupt"]
pub type MaskR = crate::FieldReader<u32>;
#[doc = "Field `MASK` writer - 31:0\\]
This field is used to indicate which interrupt the corresponding event influences (if enabled) for event group M. Each bit corresponds to event Q where Q = Mx32+Bit 0 IRQ Interrupt (default) 1 FIQ Interrupt"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This field is used to indicate which interrupt the corresponding event influences (if enabled) for event group M. Each bit corresponds to event Q where Q = Mx32+Bit 0 IRQ Interrupt (default) 1 FIQ Interrupt"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This field is used to indicate which interrupt the corresponding event influences (if enabled) for event group M. Each bit corresponds to event Q where Q = Mx32+Bit 0 IRQ Interrupt (default) 1 FIQ Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MaskW<Intmap4Spec> {
        MaskW::new(self, 0)
    }
}
#[doc = "Group M Interrupt Map Register (M is 0 to 7) h400 + M x h20 + h18\n\nYou can [`read`](crate::Reg::read) this register and get [`intmap_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmap_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intmap4Spec;
impl crate::RegisterSpec for Intmap4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intmap_4::R`](R) reader structure"]
impl crate::Readable for Intmap4Spec {}
#[doc = "`write(|w| ..)` method takes [`intmap_4::W`](W) writer structure"]
impl crate::Writable for Intmap4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTMAP_4 to value 0"]
impl crate::Resettable for Intmap4Spec {
    const RESET_VALUE: u32 = 0;
}
