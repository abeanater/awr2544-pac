#[doc = "Register `CH5CPREG3` reader"]
pub type R = crate::R<Ch5cpreg3Spec>;
#[doc = "Register `CH5CPREG3` writer"]
pub type W = crate::W<Ch5cpreg3Spec>;
#[doc = "Field `CH5CPREG3` reader - 31:0\\]
Multi Chirp 5 Parameters Register 3. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch5cpreg3R = crate::FieldReader<u32>;
#[doc = "Field `CH5CPREG3` writer - 31:0\\]
Multi Chirp 5 Parameters Register 3. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch5cpreg3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 5 Parameters Register 3. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn ch5cpreg3(&self) -> Ch5cpreg3R {
        Ch5cpreg3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 5 Parameters Register 3. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn ch5cpreg3(&mut self) -> Ch5cpreg3W<Ch5cpreg3Spec> {
        Ch5cpreg3W::new(self, 0)
    }
}
#[doc = "CH5CPREG3\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5cpreg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cpreg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch5cpreg3Spec;
impl crate::RegisterSpec for Ch5cpreg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch5cpreg3::R`](R) reader structure"]
impl crate::Readable for Ch5cpreg3Spec {}
#[doc = "`write(|w| ..)` method takes [`ch5cpreg3::W`](W) writer structure"]
impl crate::Writable for Ch5cpreg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH5CPREG3 to value 0"]
impl crate::Resettable for Ch5cpreg3Spec {
    const RESET_VALUE: u32 = 0;
}
