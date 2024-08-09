#[doc = "Register `CCMKEYR2` reader"]
pub type R = crate::R<Ccmkeyr2Spec>;
#[doc = "Register `CCMKEYR2` writer"]
pub type W = crate::W<Ccmkeyr2Spec>;
#[doc = "Field `MKEY2` reader - 3:0\\]
Mode Key 0000 = lock step mode 0110 = self test mode 1001 = error forcing mode 1111 = self test error forcing mode"]
pub type Mkey2R = crate::FieldReader;
#[doc = "Field `MKEY2` writer - 3:0\\]
Mode Key 0000 = lock step mode 0110 = self test mode 1001 = error forcing mode 1111 = self test error forcing mode"]
pub type Mkey2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU7` reader - 31:4\\]
Reserved"]
pub type Nu7R = crate::FieldReader<u32>;
#[doc = "Field `NU7` writer - 31:4\\]
Reserved"]
pub type Nu7W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Mode Key 0000 = lock step mode 0110 = self test mode 1001 = error forcing mode 1111 = self test error forcing mode"]
    #[inline(always)]
    pub fn mkey2(&self) -> Mkey2R {
        Mkey2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reserved"]
    #[inline(always)]
    pub fn nu7(&self) -> Nu7R {
        Nu7R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Mode Key 0000 = lock step mode 0110 = self test mode 1001 = error forcing mode 1111 = self test error forcing mode"]
    #[inline(always)]
    #[must_use]
    pub fn mkey2(&mut self) -> Mkey2W<Ccmkeyr2Spec> {
        Mkey2W::new(self, 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu7(&mut self) -> Nu7W<Ccmkeyr2Spec> {
        Nu7W::new(self, 4)
    }
}
#[doc = "VIM Compare Key Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmkeyr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmkeyr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmkeyr2Spec;
impl crate::RegisterSpec for Ccmkeyr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmkeyr2::R`](R) reader structure"]
impl crate::Readable for Ccmkeyr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ccmkeyr2::W`](W) writer structure"]
impl crate::Writable for Ccmkeyr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCMKEYR2 to value 0"]
impl crate::Resettable for Ccmkeyr2Spec {
    const RESET_VALUE: u32 = 0;
}
