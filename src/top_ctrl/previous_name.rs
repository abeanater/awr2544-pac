#[doc = "Register `PREVIOUS_NAME` reader"]
pub type R = crate::R<PreviousNameSpec>;
#[doc = "Register `PREVIOUS_NAME` writer"]
pub type W = crate::W<PreviousNameSpec>;
#[doc = "Field `val` reader - 15:0\\]
EFUSE Device Type"]
pub type ValR = crate::FieldReader<u16>;
#[doc = "Field `val` writer - 15:0\\]
EFUSE Device Type"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
EFUSE Device Type"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
EFUSE Device Type"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<PreviousNameSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "PREVIOUS_NAME\n\nYou can [`read`](crate::Reg::read) this register and get [`previous_name::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`previous_name::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PreviousNameSpec;
impl crate::RegisterSpec for PreviousNameSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`previous_name::R`](R) reader structure"]
impl crate::Readable for PreviousNameSpec {}
#[doc = "`write(|w| ..)` method takes [`previous_name::W`](W) writer structure"]
impl crate::Writable for PreviousNameSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PREVIOUS_NAME to value 0"]
impl crate::Resettable for PreviousNameSpec {
    const RESET_VALUE: u32 = 0;
}
