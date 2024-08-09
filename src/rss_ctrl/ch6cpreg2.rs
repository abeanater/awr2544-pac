#[doc = "Register `CH6CPREG2` reader"]
pub type R = crate::R<Ch6cpreg2Spec>;
#[doc = "Register `CH6CPREG2` writer"]
pub type W = crate::W<Ch6cpreg2Spec>;
#[doc = "Field `CH6CPREG2` reader - 31:0\\]
Multi Chirp 6 Parameters Register 2. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch6cpreg2R = crate::FieldReader<u32>;
#[doc = "Field `CH6CPREG2` writer - 31:0\\]
Multi Chirp 6 Parameters Register 2. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch6cpreg2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 6 Parameters Register 2. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn ch6cpreg2(&self) -> Ch6cpreg2R {
        Ch6cpreg2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 6 Parameters Register 2. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn ch6cpreg2(&mut self) -> Ch6cpreg2W<Ch6cpreg2Spec> {
        Ch6cpreg2W::new(self, 0)
    }
}
#[doc = "CH6CPREG2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cpreg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cpreg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch6cpreg2Spec;
impl crate::RegisterSpec for Ch6cpreg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch6cpreg2::R`](R) reader structure"]
impl crate::Readable for Ch6cpreg2Spec {}
#[doc = "`write(|w| ..)` method takes [`ch6cpreg2::W`](W) writer structure"]
impl crate::Writable for Ch6cpreg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH6CPREG2 to value 0"]
impl crate::Resettable for Ch6cpreg2Spec {
    const RESET_VALUE: u32 = 0;
}
