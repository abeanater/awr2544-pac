#[doc = "Register `REGISTER5` reader"]
pub type R = crate::R<Register5Spec>;
#[doc = "Register `REGISTER5` writer"]
pub type W = crate::W<Register5Spec>;
#[doc = "Field `EMPTY` reader - 23:0\\]
Reserved"]
pub type EmptyR = crate::FieldReader<u32>;
#[doc = "Field `EMPTY` writer - 23:0\\]
Reserved"]
pub type EmptyW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `RESETDONETXCLKESC0` reader - 24:24\\]
RESETDONETXCLKESC0"]
pub type Resetdonetxclkesc0R = crate::BitReader;
#[doc = "Field `RESETDONETXCLKESC0` writer - 24:24\\]
RESETDONETXCLKESC0"]
pub type Resetdonetxclkesc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETDONETXCLKESC1` reader - 25:25\\]
RESETDONETXCLKESC1"]
pub type Resetdonetxclkesc1R = crate::BitReader;
#[doc = "Field `RESETDONETXCLKESC1` writer - 25:25\\]
RESETDONETXCLKESC1"]
pub type Resetdonetxclkesc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETDONETXCLKESC2` reader - 26:26\\]
RESETDONETXCLKESC2"]
pub type Resetdonetxclkesc2R = crate::BitReader;
#[doc = "Field `RESETDONETXCLKESC2` writer - 26:26\\]
RESETDONETXCLKESC2"]
pub type Resetdonetxclkesc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETDONETXCLKESC3` reader - 27:27\\]
RESETDONETXCLKESC3"]
pub type Resetdonetxclkesc3R = crate::BitReader;
#[doc = "Field `RESETDONETXCLKESC3` writer - 27:27\\]
RESETDONETXCLKESC3"]
pub type Resetdonetxclkesc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETDONETXCLKESC4` reader - 28:28\\]
RESETDONETXCLKESC4"]
pub type Resetdonetxclkesc4R = crate::BitReader;
#[doc = "Field `RESETDONETXCLKESC4` writer - 28:28\\]
RESETDONETXCLKESC4"]
pub type Resetdonetxclkesc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETDONEPWRCLK` reader - 29:29\\]
RESETDONEPWRCLK"]
pub type ResetdonepwrclkR = crate::BitReader;
#[doc = "Field `RESETDONEPWRCLK` writer - 29:29\\]
RESETDONEPWRCLK"]
pub type ResetdonepwrclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETDONESCPCLK` reader - 30:30\\]
RESETDONESCPCLK"]
pub type ResetdonescpclkR = crate::BitReader;
#[doc = "Field `RESETDONESCPCLK` writer - 30:30\\]
RESETDONESCPCLK"]
pub type ResetdonescpclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETDONETXBYTECLK` reader - 31:31\\]
RESETDONETXBYTECLK"]
pub type ResetdonetxbyteclkR = crate::BitReader;
#[doc = "Field `RESETDONETXBYTECLK` writer - 31:31\\]
RESETDONETXBYTECLK"]
pub type ResetdonetxbyteclkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Reserved"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - 24:24\\]
RESETDONETXCLKESC0"]
    #[inline(always)]
    pub fn resetdonetxclkesc0(&self) -> Resetdonetxclkesc0R {
        Resetdonetxclkesc0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
RESETDONETXCLKESC1"]
    #[inline(always)]
    pub fn resetdonetxclkesc1(&self) -> Resetdonetxclkesc1R {
        Resetdonetxclkesc1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
RESETDONETXCLKESC2"]
    #[inline(always)]
    pub fn resetdonetxclkesc2(&self) -> Resetdonetxclkesc2R {
        Resetdonetxclkesc2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
RESETDONETXCLKESC3"]
    #[inline(always)]
    pub fn resetdonetxclkesc3(&self) -> Resetdonetxclkesc3R {
        Resetdonetxclkesc3R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
RESETDONETXCLKESC4"]
    #[inline(always)]
    pub fn resetdonetxclkesc4(&self) -> Resetdonetxclkesc4R {
        Resetdonetxclkesc4R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
RESETDONEPWRCLK"]
    #[inline(always)]
    pub fn resetdonepwrclk(&self) -> ResetdonepwrclkR {
        ResetdonepwrclkR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
RESETDONESCPCLK"]
    #[inline(always)]
    pub fn resetdonescpclk(&self) -> ResetdonescpclkR {
        ResetdonescpclkR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
RESETDONETXBYTECLK"]
    #[inline(always)]
    pub fn resetdonetxbyteclk(&self) -> ResetdonetxbyteclkR {
        ResetdonetxbyteclkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EmptyW<Register5Spec> {
        EmptyW::new(self, 0)
    }
    #[doc = "Bit 24 - 24:24\\]
RESETDONETXCLKESC0"]
    #[inline(always)]
    #[must_use]
    pub fn resetdonetxclkesc0(&mut self) -> Resetdonetxclkesc0W<Register5Spec> {
        Resetdonetxclkesc0W::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
RESETDONETXCLKESC1"]
    #[inline(always)]
    #[must_use]
    pub fn resetdonetxclkesc1(&mut self) -> Resetdonetxclkesc1W<Register5Spec> {
        Resetdonetxclkesc1W::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
RESETDONETXCLKESC2"]
    #[inline(always)]
    #[must_use]
    pub fn resetdonetxclkesc2(&mut self) -> Resetdonetxclkesc2W<Register5Spec> {
        Resetdonetxclkesc2W::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
RESETDONETXCLKESC3"]
    #[inline(always)]
    #[must_use]
    pub fn resetdonetxclkesc3(&mut self) -> Resetdonetxclkesc3W<Register5Spec> {
        Resetdonetxclkesc3W::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
RESETDONETXCLKESC4"]
    #[inline(always)]
    #[must_use]
    pub fn resetdonetxclkesc4(&mut self) -> Resetdonetxclkesc4W<Register5Spec> {
        Resetdonetxclkesc4W::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
RESETDONEPWRCLK"]
    #[inline(always)]
    #[must_use]
    pub fn resetdonepwrclk(&mut self) -> ResetdonepwrclkW<Register5Spec> {
        ResetdonepwrclkW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
RESETDONESCPCLK"]
    #[inline(always)]
    #[must_use]
    pub fn resetdonescpclk(&mut self) -> ResetdonescpclkW<Register5Spec> {
        ResetdonescpclkW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
RESETDONETXBYTECLK"]
    #[inline(always)]
    #[must_use]
    pub fn resetdonetxbyteclk(&mut self) -> ResetdonetxbyteclkW<Register5Spec> {
        ResetdonetxbyteclkW::new(self, 31)
    }
}
#[doc = "REGISTER5\n\nYou can [`read`](crate::Reg::read) this register and get [`register5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Register5Spec;
impl crate::RegisterSpec for Register5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register5::R`](R) reader structure"]
impl crate::Readable for Register5Spec {}
#[doc = "`write(|w| ..)` method takes [`register5::W`](W) writer structure"]
impl crate::Writable for Register5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGISTER5 to value 0"]
impl crate::Resettable for Register5Spec {
    const RESET_VALUE: u32 = 0;
}
