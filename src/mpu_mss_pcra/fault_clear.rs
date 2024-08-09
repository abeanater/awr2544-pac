#[doc = "Register `Fault Clear` reader"]
pub type R = crate::R<FaultClearSpec>;
#[doc = "Register `Fault Clear` writer"]
pub type W = crate::W<FaultClearSpec>;
#[doc = "Field `fault_clr` reader - 0:0\\]
Fault clear. Writing a 1 clears the current fault. Writing a 0 has no effect."]
pub type FaultClrR = crate::BitReader;
#[doc = "Field `fault_clr` writer - 0:0\\]
Fault clear. Writing a 1 clears the current fault. Writing a 0 has no effect."]
pub type FaultClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Fault clear. Writing a 1 clears the current fault. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn fault_clr(&self) -> FaultClrR {
        FaultClrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Fault clear. Writing a 1 clears the current fault. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn fault_clr(&mut self) -> FaultClrW<FaultClearSpec> {
        FaultClrW::new(self, 0)
    }
}
#[doc = "Fault Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`fault_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fault_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FaultClearSpec;
impl crate::RegisterSpec for FaultClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fault_clear::R`](R) reader structure"]
impl crate::Readable for FaultClearSpec {}
#[doc = "`write(|w| ..)` method takes [`fault_clear::W`](W) writer structure"]
impl crate::Writable for FaultClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Fault Clear to value 0"]
impl crate::Resettable for FaultClearSpec {
    const RESET_VALUE: u32 = 0;
}
