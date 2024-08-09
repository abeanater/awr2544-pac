#[doc = "Register `CH1CPREG4` reader"]
pub type R = crate::R<Ch1cpreg4Spec>;
#[doc = "Register `CH1CPREG4` writer"]
pub type W = crate::W<Ch1cpreg4Spec>;
#[doc = "Field `CH1CPREG4` reader - 31:0\\]
Multi Chirp 1 Parameters Register 4. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch1cpreg4R = crate::FieldReader<u32>;
#[doc = "Field `CH1CPREG4` writer - 31:0\\]
Multi Chirp 1 Parameters Register 4. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch1cpreg4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 1 Parameters Register 4. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn ch1cpreg4(&self) -> Ch1cpreg4R {
        Ch1cpreg4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 1 Parameters Register 4. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn ch1cpreg4(&mut self) -> Ch1cpreg4W<Ch1cpreg4Spec> {
        Ch1cpreg4W::new(self, 0)
    }
}
#[doc = "CH1CPREG4\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cpreg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cpreg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1cpreg4Spec;
impl crate::RegisterSpec for Ch1cpreg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1cpreg4::R`](R) reader structure"]
impl crate::Readable for Ch1cpreg4Spec {}
#[doc = "`write(|w| ..)` method takes [`ch1cpreg4::W`](W) writer structure"]
impl crate::Writable for Ch1cpreg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH1CPREG4 to value 0"]
impl crate::Resettable for Ch1cpreg4Spec {
    const RESET_VALUE: u32 = 0;
}
