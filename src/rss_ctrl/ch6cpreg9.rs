#[doc = "Register `CH6CPREG9` reader"]
pub type R = crate::R<Ch6cpreg9Spec>;
#[doc = "Register `CH6CPREG9` writer"]
pub type W = crate::W<Ch6cpreg9Spec>;
#[doc = "Field `CH6CPREG9` reader - 31:0\\]
Multi Chirp 6 Parameters Register 9. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch6cpreg9R = crate::FieldReader<u32>;
#[doc = "Field `CH6CPREG9` writer - 31:0\\]
Multi Chirp 6 Parameters Register 9. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch6cpreg9W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 6 Parameters Register 9. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn ch6cpreg9(&self) -> Ch6cpreg9R {
        Ch6cpreg9R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 6 Parameters Register 9. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn ch6cpreg9(&mut self) -> Ch6cpreg9W<Ch6cpreg9Spec> {
        Ch6cpreg9W::new(self, 0)
    }
}
#[doc = "CH6CPREG9\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cpreg9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cpreg9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch6cpreg9Spec;
impl crate::RegisterSpec for Ch6cpreg9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch6cpreg9::R`](R) reader structure"]
impl crate::Readable for Ch6cpreg9Spec {}
#[doc = "`write(|w| ..)` method takes [`ch6cpreg9::W`](W) writer structure"]
impl crate::Writable for Ch6cpreg9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH6CPREG9 to value 0"]
impl crate::Resettable for Ch6cpreg9Spec {
    const RESET_VALUE: u32 = 0;
}
