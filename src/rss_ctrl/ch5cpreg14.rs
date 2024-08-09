#[doc = "Register `CH5CPREG14` reader"]
pub type R = crate::R<Ch5cpreg14Spec>;
#[doc = "Register `CH5CPREG14` writer"]
pub type W = crate::W<Ch5cpreg14Spec>;
#[doc = "Field `CH5CPREG14` reader - 31:0\\]
Multi Chirp 5 Parameters Register 14. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch5cpreg14R = crate::FieldReader<u32>;
#[doc = "Field `CH5CPREG14` writer - 31:0\\]
Multi Chirp 5 Parameters Register 14. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch5cpreg14W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 5 Parameters Register 14. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn ch5cpreg14(&self) -> Ch5cpreg14R {
        Ch5cpreg14R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 5 Parameters Register 14. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn ch5cpreg14(&mut self) -> Ch5cpreg14W<Ch5cpreg14Spec> {
        Ch5cpreg14W::new(self, 0)
    }
}
#[doc = "CH5CPREG14\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5cpreg14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cpreg14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch5cpreg14Spec;
impl crate::RegisterSpec for Ch5cpreg14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch5cpreg14::R`](R) reader structure"]
impl crate::Readable for Ch5cpreg14Spec {}
#[doc = "`write(|w| ..)` method takes [`ch5cpreg14::W`](W) writer structure"]
impl crate::Writable for Ch5cpreg14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH5CPREG14 to value 0"]
impl crate::Resettable for Ch5cpreg14Spec {
    const RESET_VALUE: u32 = 0;
}
