#[doc = "Register `CH6CPREG1` reader"]
pub type R = crate::R<Ch6cpreg1Spec>;
#[doc = "Register `CH6CPREG1` writer"]
pub type W = crate::W<Ch6cpreg1Spec>;
#[doc = "Field `CH6CPREG1` reader - 31:0\\]
Multi Chirp 6 Parameters Register 1. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch6cpreg1R = crate::FieldReader<u32>;
#[doc = "Field `CH6CPREG1` writer - 31:0\\]
Multi Chirp 6 Parameters Register 1. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch6cpreg1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 6 Parameters Register 1. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn ch6cpreg1(&self) -> Ch6cpreg1R {
        Ch6cpreg1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 6 Parameters Register 1. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn ch6cpreg1(&mut self) -> Ch6cpreg1W<Ch6cpreg1Spec> {
        Ch6cpreg1W::new(self, 0)
    }
}
#[doc = "CH6CPREG1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cpreg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cpreg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch6cpreg1Spec;
impl crate::RegisterSpec for Ch6cpreg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch6cpreg1::R`](R) reader structure"]
impl crate::Readable for Ch6cpreg1Spec {}
#[doc = "`write(|w| ..)` method takes [`ch6cpreg1::W`](W) writer structure"]
impl crate::Writable for Ch6cpreg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH6CPREG1 to value 0"]
impl crate::Resettable for Ch6cpreg1Spec {
    const RESET_VALUE: u32 = 0;
}
