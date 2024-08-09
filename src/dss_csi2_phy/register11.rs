#[doc = "Register `REGISTER11` reader"]
pub type R = crate::R<Register11Spec>;
#[doc = "Register `REGISTER11` writer"]
pub type W = crate::W<Register11Spec>;
#[doc = "Field `LOOPBACKDATABYTE0` reader - 7:0\\]
First byte transmitted in loop-back mode"]
pub type Loopbackdatabyte0R = crate::FieldReader;
#[doc = "Field `LOOPBACKDATABYTE0` writer - 7:0\\]
First byte transmitted in loop-back mode"]
pub type Loopbackdatabyte0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LOOPBACKDATABYTE1` reader - 15:8\\]
Second byte transmitted in loop-back mode"]
pub type Loopbackdatabyte1R = crate::FieldReader;
#[doc = "Field `LOOPBACKDATABYTE1` writer - 15:8\\]
Second byte transmitted in loop-back mode"]
pub type Loopbackdatabyte1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LOOPBACKDATABYTE2` reader - 23:16\\]
Third byte transmitted in loop-back mode"]
pub type Loopbackdatabyte2R = crate::FieldReader;
#[doc = "Field `LOOPBACKDATABYTE2` writer - 23:16\\]
Third byte transmitted in loop-back mode"]
pub type Loopbackdatabyte2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LOOPBACKDATABYTE3` reader - 31:24\\]
Fourth byte transmitted in loop-back mode"]
pub type Loopbackdatabyte3R = crate::FieldReader;
#[doc = "Field `LOOPBACKDATABYTE3` writer - 31:24\\]
Fourth byte transmitted in loop-back mode"]
pub type Loopbackdatabyte3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
First byte transmitted in loop-back mode"]
    #[inline(always)]
    pub fn loopbackdatabyte0(&self) -> Loopbackdatabyte0R {
        Loopbackdatabyte0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Second byte transmitted in loop-back mode"]
    #[inline(always)]
    pub fn loopbackdatabyte1(&self) -> Loopbackdatabyte1R {
        Loopbackdatabyte1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Third byte transmitted in loop-back mode"]
    #[inline(always)]
    pub fn loopbackdatabyte2(&self) -> Loopbackdatabyte2R {
        Loopbackdatabyte2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Fourth byte transmitted in loop-back mode"]
    #[inline(always)]
    pub fn loopbackdatabyte3(&self) -> Loopbackdatabyte3R {
        Loopbackdatabyte3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
First byte transmitted in loop-back mode"]
    #[inline(always)]
    #[must_use]
    pub fn loopbackdatabyte0(&mut self) -> Loopbackdatabyte0W<Register11Spec> {
        Loopbackdatabyte0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Second byte transmitted in loop-back mode"]
    #[inline(always)]
    #[must_use]
    pub fn loopbackdatabyte1(&mut self) -> Loopbackdatabyte1W<Register11Spec> {
        Loopbackdatabyte1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Third byte transmitted in loop-back mode"]
    #[inline(always)]
    #[must_use]
    pub fn loopbackdatabyte2(&mut self) -> Loopbackdatabyte2W<Register11Spec> {
        Loopbackdatabyte2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Fourth byte transmitted in loop-back mode"]
    #[inline(always)]
    #[must_use]
    pub fn loopbackdatabyte3(&mut self) -> Loopbackdatabyte3W<Register11Spec> {
        Loopbackdatabyte3W::new(self, 24)
    }
}
#[doc = "REGISTER11\n\nYou can [`read`](crate::Reg::read) this register and get [`register11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Register11Spec;
impl crate::RegisterSpec for Register11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register11::R`](R) reader structure"]
impl crate::Readable for Register11Spec {}
#[doc = "`write(|w| ..)` method takes [`register11::W`](W) writer structure"]
impl crate::Writable for Register11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGISTER11 to value 0"]
impl crate::Resettable for Register11Spec {
    const RESET_VALUE: u32 = 0;
}
