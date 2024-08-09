#[doc = "Register `INTTYPE_3` reader"]
pub type R = crate::R<Inttype3Spec>;
#[doc = "Register `INTTYPE_3` writer"]
pub type W = crate::W<Inttype3Spec>;
#[doc = "Field `VAL` reader - 31:0\\]
This field is used to indicate whether the source of an interrupt is a level (default) or a pulse for event group M. This is informational so that an ISR may query this register and know whether it has to clear a pulse event or a level event (see 3.4 Interrupt Handling). The value has no effect on how the VIM hardware functions. The input interrupts are agnostic as to whether they are pulse or level. Each bit corresponds to event Q where Q = Mx32+Bit 0 Level (default) 1 Pulse"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 31:0\\]
This field is used to indicate whether the source of an interrupt is a level (default) or a pulse for event group M. This is informational so that an ISR may query this register and know whether it has to clear a pulse event or a level event (see 3.4 Interrupt Handling). The value has no effect on how the VIM hardware functions. The input interrupts are agnostic as to whether they are pulse or level. Each bit corresponds to event Q where Q = Mx32+Bit 0 Level (default) 1 Pulse"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This field is used to indicate whether the source of an interrupt is a level (default) or a pulse for event group M. This is informational so that an ISR may query this register and know whether it has to clear a pulse event or a level event (see 3.4 Interrupt Handling). The value has no effect on how the VIM hardware functions. The input interrupts are agnostic as to whether they are pulse or level. Each bit corresponds to event Q where Q = Mx32+Bit 0 Level (default) 1 Pulse"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This field is used to indicate whether the source of an interrupt is a level (default) or a pulse for event group M. This is informational so that an ISR may query this register and know whether it has to clear a pulse event or a level event (see 3.4 Interrupt Handling). The value has no effect on how the VIM hardware functions. The input interrupts are agnostic as to whether they are pulse or level. Each bit corresponds to event Q where Q = Mx32+Bit 0 Level (default) 1 Pulse"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Inttype3Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Group M Type Map Register (M is 0 to 7) h400 + M x h20 + 0x1C\n\nYou can [`read`](crate::Reg::read) this register and get [`inttype_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inttype_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Inttype3Spec;
impl crate::RegisterSpec for Inttype3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inttype_3::R`](R) reader structure"]
impl crate::Readable for Inttype3Spec {}
#[doc = "`write(|w| ..)` method takes [`inttype_3::W`](W) writer structure"]
impl crate::Writable for Inttype3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTTYPE_3 to value 0"]
impl crate::Resettable for Inttype3Spec {
    const RESET_VALUE: u32 = 0;
}
