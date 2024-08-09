#[doc = "Register `R5_COREB_HALT` reader"]
pub type R = crate::R<R5CorebHaltSpec>;
#[doc = "Register `R5_COREB_HALT` writer"]
pub type W = crate::W<R5CorebHaltSpec>;
#[doc = "Field `halt` reader - 2:0\\]
RESERVED: Dont Use"]
pub type HaltR = crate::FieldReader;
#[doc = "Field `halt` writer - 2:0\\]
RESERVED: Dont Use"]
pub type HaltW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn halt(&self) -> HaltR {
        HaltR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HaltW<R5CorebHaltSpec> {
        HaltW::new(self, 0)
    }
}
#[doc = "R5_COREB_HALT\n\nYou can [`read`](crate::Reg::read) this register and get [`r5_coreb_halt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r5_coreb_halt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R5CorebHaltSpec;
impl crate::RegisterSpec for R5CorebHaltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r5_coreb_halt::R`](R) reader structure"]
impl crate::Readable for R5CorebHaltSpec {}
#[doc = "`write(|w| ..)` method takes [`r5_coreb_halt::W`](W) writer structure"]
impl crate::Writable for R5CorebHaltSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R5_COREB_HALT to value 0"]
impl crate::Resettable for R5CorebHaltSpec {
    const RESET_VALUE: u32 = 0;
}
