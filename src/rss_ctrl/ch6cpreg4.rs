#[doc = "Register `CH6CPREG4` reader"]
pub type R = crate::R<Ch6cpreg4Spec>;
#[doc = "Register `CH6CPREG4` writer"]
pub type W = crate::W<Ch6cpreg4Spec>;
#[doc = "Field `CH6CPREG4` reader - 31:0\\]
Multi Chirp 6 Parameters Register 4. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch6cpreg4R = crate::FieldReader<u32>;
#[doc = "Field `CH6CPREG4` writer - 31:0\\]
Multi Chirp 6 Parameters Register 4. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch6cpreg4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 6 Parameters Register 4. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn ch6cpreg4(&self) -> Ch6cpreg4R {
        Ch6cpreg4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 6 Parameters Register 4. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn ch6cpreg4(&mut self) -> Ch6cpreg4W<Ch6cpreg4Spec> {
        Ch6cpreg4W::new(self, 0)
    }
}
#[doc = "CH6CPREG4\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6cpreg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6cpreg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch6cpreg4Spec;
impl crate::RegisterSpec for Ch6cpreg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch6cpreg4::R`](R) reader structure"]
impl crate::Readable for Ch6cpreg4Spec {}
#[doc = "`write(|w| ..)` method takes [`ch6cpreg4::W`](W) writer structure"]
impl crate::Writable for Ch6cpreg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH6CPREG4 to value 0"]
impl crate::Resettable for Ch6cpreg4Spec {
    const RESET_VALUE: u32 = 0;
}
