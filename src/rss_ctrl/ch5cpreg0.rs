#[doc = "Register `CH5CPREG0` reader"]
pub type R = crate::R<Ch5cpreg0Spec>;
#[doc = "Register `CH5CPREG0` writer"]
pub type W = crate::W<Ch5cpreg0Spec>;
#[doc = "Field `CH5CPREG0` reader - 31:0\\]
Multi Chirp 5 Parameters Register 0. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch5cpreg0R = crate::FieldReader<u32>;
#[doc = "Field `CH5CPREG0` writer - 31:0\\]
Multi Chirp 5 Parameters Register 0. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch5cpreg0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 5 Parameters Register 0. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn ch5cpreg0(&self) -> Ch5cpreg0R {
        Ch5cpreg0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 5 Parameters Register 0. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn ch5cpreg0(&mut self) -> Ch5cpreg0W<Ch5cpreg0Spec> {
        Ch5cpreg0W::new(self, 0)
    }
}
#[doc = "CH5CPREG0\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5cpreg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cpreg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch5cpreg0Spec;
impl crate::RegisterSpec for Ch5cpreg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch5cpreg0::R`](R) reader structure"]
impl crate::Readable for Ch5cpreg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ch5cpreg0::W`](W) writer structure"]
impl crate::Writable for Ch5cpreg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH5CPREG0 to value 0"]
impl crate::Resettable for Ch5cpreg0Spec {
    const RESET_VALUE: u32 = 0;
}
