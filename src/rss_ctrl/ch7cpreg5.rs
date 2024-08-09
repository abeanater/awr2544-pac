#[doc = "Register `CH7CPREG5` reader"]
pub type R = crate::R<Ch7cpreg5Spec>;
#[doc = "Register `CH7CPREG5` writer"]
pub type W = crate::W<Ch7cpreg5Spec>;
#[doc = "Field `CH7CPREG5` reader - 31:0\\]
Multi Chirp 7 Parameters Register 5. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch7cpreg5R = crate::FieldReader<u32>;
#[doc = "Field `CH7CPREG5` writer - 31:0\\]
Multi Chirp 7 Parameters Register 5. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch7cpreg5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 7 Parameters Register 5. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn ch7cpreg5(&self) -> Ch7cpreg5R {
        Ch7cpreg5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 7 Parameters Register 5. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn ch7cpreg5(&mut self) -> Ch7cpreg5W<Ch7cpreg5Spec> {
        Ch7cpreg5W::new(self, 0)
    }
}
#[doc = "CH7CPREG5\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7cpreg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7cpreg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch7cpreg5Spec;
impl crate::RegisterSpec for Ch7cpreg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch7cpreg5::R`](R) reader structure"]
impl crate::Readable for Ch7cpreg5Spec {}
#[doc = "`write(|w| ..)` method takes [`ch7cpreg5::W`](W) writer structure"]
impl crate::Writable for Ch7cpreg5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH7CPREG5 to value 0"]
impl crate::Resettable for Ch7cpreg5Spec {
    const RESET_VALUE: u32 = 0;
}
