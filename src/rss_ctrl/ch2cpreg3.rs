#[doc = "Register `CH2CPREG3` reader"]
pub type R = crate::R<Ch2cpreg3Spec>;
#[doc = "Register `CH2CPREG3` writer"]
pub type W = crate::W<Ch2cpreg3Spec>;
#[doc = "Field `CH2CPREG3` reader - 31:0\\]
Multi Chirp 2 Parameters Register 3. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch2cpreg3R = crate::FieldReader<u32>;
#[doc = "Field `CH2CPREG3` writer - 31:0\\]
Multi Chirp 2 Parameters Register 3. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch2cpreg3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 2 Parameters Register 3. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn ch2cpreg3(&self) -> Ch2cpreg3R {
        Ch2cpreg3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 2 Parameters Register 3. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn ch2cpreg3(&mut self) -> Ch2cpreg3W<Ch2cpreg3Spec> {
        Ch2cpreg3W::new(self, 0)
    }
}
#[doc = "CH2CPREG3\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2cpreg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2cpreg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2cpreg3Spec;
impl crate::RegisterSpec for Ch2cpreg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2cpreg3::R`](R) reader structure"]
impl crate::Readable for Ch2cpreg3Spec {}
#[doc = "`write(|w| ..)` method takes [`ch2cpreg3::W`](W) writer structure"]
impl crate::Writable for Ch2cpreg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH2CPREG3 to value 0"]
impl crate::Resettable for Ch2cpreg3Spec {
    const RESET_VALUE: u32 = 0;
}
