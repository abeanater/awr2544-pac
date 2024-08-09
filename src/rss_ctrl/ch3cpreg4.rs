#[doc = "Register `CH3CPREG4` reader"]
pub type R = crate::R<Ch3cpreg4Spec>;
#[doc = "Register `CH3CPREG4` writer"]
pub type W = crate::W<Ch3cpreg4Spec>;
#[doc = "Field `CH3CPREG4` reader - 31:0\\]
Multi Chirp 3 Parameters Register 4. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch3cpreg4R = crate::FieldReader<u32>;
#[doc = "Field `CH3CPREG4` writer - 31:0\\]
Multi Chirp 3 Parameters Register 4. Refer to Chirp Parameter section for more details (DSS_CP)"]
pub type Ch3cpreg4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 3 Parameters Register 4. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    pub fn ch3cpreg4(&self) -> Ch3cpreg4R {
        Ch3cpreg4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Multi Chirp 3 Parameters Register 4. Refer to Chirp Parameter section for more details (DSS_CP)"]
    #[inline(always)]
    #[must_use]
    pub fn ch3cpreg4(&mut self) -> Ch3cpreg4W<Ch3cpreg4Spec> {
        Ch3cpreg4W::new(self, 0)
    }
}
#[doc = "CH3CPREG4\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3cpreg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3cpreg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3cpreg4Spec;
impl crate::RegisterSpec for Ch3cpreg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3cpreg4::R`](R) reader structure"]
impl crate::Readable for Ch3cpreg4Spec {}
#[doc = "`write(|w| ..)` method takes [`ch3cpreg4::W`](W) writer structure"]
impl crate::Writable for Ch3cpreg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH3CPREG4 to value 0"]
impl crate::Resettable for Ch3cpreg4Spec {
    const RESET_VALUE: u32 = 0;
}
