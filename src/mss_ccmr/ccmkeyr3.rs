#[doc = "Register `CCMKEYR3` reader"]
pub type R = crate::R<Ccmkeyr3Spec>;
#[doc = "Register `CCMKEYR3` writer"]
pub type W = crate::W<Ccmkeyr3Spec>;
#[doc = "Field `MKEY3` reader - 3:0\\]
Mode Key 0000 = lock step mode 0110 = self test mode 1001 = error forcing mode 1111 = self test error forcing mode"]
pub type Mkey3R = crate::FieldReader;
#[doc = "Field `MKEY3` writer - 3:0\\]
Mode Key 0000 = lock step mode 0110 = self test mode 1001 = error forcing mode 1111 = self test error forcing mode"]
pub type Mkey3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU11` reader - 31:4\\]
Reserved"]
pub type Nu11R = crate::FieldReader<u32>;
#[doc = "Field `NU11` writer - 31:4\\]
Reserved"]
pub type Nu11W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Mode Key 0000 = lock step mode 0110 = self test mode 1001 = error forcing mode 1111 = self test error forcing mode"]
    #[inline(always)]
    pub fn mkey3(&self) -> Mkey3R {
        Mkey3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reserved"]
    #[inline(always)]
    pub fn nu11(&self) -> Nu11R {
        Nu11R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Mode Key 0000 = lock step mode 0110 = self test mode 1001 = error forcing mode 1111 = self test error forcing mode"]
    #[inline(always)]
    #[must_use]
    pub fn mkey3(&mut self) -> Mkey3W<Ccmkeyr3Spec> {
        Mkey3W::new(self, 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu11(&mut self) -> Nu11W<Ccmkeyr3Spec> {
        Nu11W::new(self, 4)
    }
}
#[doc = "Inactivity Monitor Key Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmkeyr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmkeyr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmkeyr3Spec;
impl crate::RegisterSpec for Ccmkeyr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmkeyr3::R`](R) reader structure"]
impl crate::Readable for Ccmkeyr3Spec {}
#[doc = "`write(|w| ..)` method takes [`ccmkeyr3::W`](W) writer structure"]
impl crate::Writable for Ccmkeyr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCMKEYR3 to value 0"]
impl crate::Resettable for Ccmkeyr3Spec {
    const RESET_VALUE: u32 = 0;
}
