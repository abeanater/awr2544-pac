#[doc = "Register `INTERF_RESERVED_5` reader"]
pub type R = crate::R<InterfReserved5Spec>;
#[doc = "Register `INTERF_RESERVED_5` writer"]
pub type W = crate::W<InterfReserved5Spec>;
#[doc = "Field `interf_reserved_5` reader - 31:0\\]
Reserved for future addition"]
pub type InterfReserved5R = crate::FieldReader<u32>;
#[doc = "Field `interf_reserved_5` writer - 31:0\\]
Reserved for future addition"]
pub type InterfReserved5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for future addition"]
    #[inline(always)]
    pub fn interf_reserved_5(&self) -> InterfReserved5R {
        InterfReserved5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for future addition"]
    #[inline(always)]
    #[must_use]
    pub fn interf_reserved_5(&mut self) -> InterfReserved5W<InterfReserved5Spec> {
        InterfReserved5W::new(self, 0)
    }
}
#[doc = "INTERF_RESERVED_5\n\nYou can [`read`](crate::Reg::read) this register and get [`interf_reserved_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interf_reserved_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterfReserved5Spec;
impl crate::RegisterSpec for InterfReserved5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interf_reserved_5::R`](R) reader structure"]
impl crate::Readable for InterfReserved5Spec {}
#[doc = "`write(|w| ..)` method takes [`interf_reserved_5::W`](W) writer structure"]
impl crate::Writable for InterfReserved5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERF_RESERVED_5 to value 0"]
impl crate::Resettable for InterfReserved5Spec {
    const RESET_VALUE: u32 = 0;
}
