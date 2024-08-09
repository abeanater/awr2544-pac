#[doc = "Register `STC_CLKDIV` reader"]
pub type R = crate::R<StcClkdivSpec>;
#[doc = "Register `STC_CLKDIV` writer"]
pub type W = crate::W<StcClkdivSpec>;
#[doc = "Field `CLKDIV3` reader - 2:0\\]
Clock division for Seg3 (RWP - Read, Priviledge Mode Write only) *NOT SUPPORTED X = Division ratio is X+1 for Segment 3"]
pub type Clkdiv3R = crate::FieldReader;
#[doc = "Field `CLKDIV3` writer - 2:0\\]
Clock division for Seg3 (RWP - Read, Priviledge Mode Write only) *NOT SUPPORTED X = Division ratio is X+1 for Segment 3"]
pub type Clkdiv3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NU11` reader - 7:3\\]
Reserved bits"]
pub type Nu11R = crate::FieldReader;
#[doc = "Field `NU11` writer - 7:3\\]
Reserved bits"]
pub type Nu11W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CLKDIV2` reader - 10:8\\]
Clock division for Seg2 (RWP - Read, Priviledge Mode Write only) *NOT SUPPORTED X = Division ratio is X+1 for Segment 2"]
pub type Clkdiv2R = crate::FieldReader;
#[doc = "Field `CLKDIV2` writer - 10:8\\]
Clock division for Seg2 (RWP - Read, Priviledge Mode Write only) *NOT SUPPORTED X = Division ratio is X+1 for Segment 2"]
pub type Clkdiv2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NU10` reader - 15:11\\]
Reserved bits"]
pub type Nu10R = crate::FieldReader;
#[doc = "Field `NU10` writer - 15:11\\]
Reserved bits"]
pub type Nu10W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CLKDIV1` reader - 18:16\\]
Clock division for Seg1 (RWP - Read, Priviledge Mode Write only) *NOT SUPPORTED X = Division ratio is X+1 for Segment 1"]
pub type Clkdiv1R = crate::FieldReader;
#[doc = "Field `CLKDIV1` writer - 18:16\\]
Clock division for Seg1 (RWP - Read, Priviledge Mode Write only) *NOT SUPPORTED X = Division ratio is X+1 for Segment 1"]
pub type Clkdiv1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NU9` reader - 23:19\\]
Reserved bits"]
pub type Nu9R = crate::FieldReader;
#[doc = "Field `NU9` writer - 23:19\\]
Reserved bits"]
pub type Nu9W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CLKDIV0` reader - 26:24\\]
Clock division for Seg0 (RWP - Read, Priviledge Mode Write only) *NOT SUPPORTED X = Division ratio is X+1 for Segment 0"]
pub type Clkdiv0R = crate::FieldReader;
#[doc = "Field `CLKDIV0` writer - 26:24\\]
Clock division for Seg0 (RWP - Read, Priviledge Mode Write only) *NOT SUPPORTED X = Division ratio is X+1 for Segment 0"]
pub type Clkdiv0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NU8` reader - 31:27\\]
Reserved bits"]
pub type Nu8R = crate::FieldReader;
#[doc = "Field `NU8` writer - 31:27\\]
Reserved bits"]
pub type Nu8W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Clock division for Seg3 (RWP - Read, Priviledge Mode Write only) *NOT SUPPORTED X = Division ratio is X+1 for Segment 3"]
    #[inline(always)]
    pub fn clkdiv3(&self) -> Clkdiv3R {
        Clkdiv3R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Reserved bits"]
    #[inline(always)]
    pub fn nu11(&self) -> Nu11R {
        Nu11R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Clock division for Seg2 (RWP - Read, Priviledge Mode Write only) *NOT SUPPORTED X = Division ratio is X+1 for Segment 2"]
    #[inline(always)]
    pub fn clkdiv2(&self) -> Clkdiv2R {
        Clkdiv2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Reserved bits"]
    #[inline(always)]
    pub fn nu10(&self) -> Nu10R {
        Nu10R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Clock division for Seg1 (RWP - Read, Priviledge Mode Write only) *NOT SUPPORTED X = Division ratio is X+1 for Segment 1"]
    #[inline(always)]
    pub fn clkdiv1(&self) -> Clkdiv1R {
        Clkdiv1R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Reserved bits"]
    #[inline(always)]
    pub fn nu9(&self) -> Nu9R {
        Nu9R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Clock division for Seg0 (RWP - Read, Priviledge Mode Write only) *NOT SUPPORTED X = Division ratio is X+1 for Segment 0"]
    #[inline(always)]
    pub fn clkdiv0(&self) -> Clkdiv0R {
        Clkdiv0R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Reserved bits"]
    #[inline(always)]
    pub fn nu8(&self) -> Nu8R {
        Nu8R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Clock division for Seg3 (RWP - Read, Priviledge Mode Write only) *NOT SUPPORTED X = Division ratio is X+1 for Segment 3"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv3(&mut self) -> Clkdiv3W<StcClkdivSpec> {
        Clkdiv3W::new(self, 0)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Reserved bits"]
    #[inline(always)]
    #[must_use]
    pub fn nu11(&mut self) -> Nu11W<StcClkdivSpec> {
        Nu11W::new(self, 3)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Clock division for Seg2 (RWP - Read, Priviledge Mode Write only) *NOT SUPPORTED X = Division ratio is X+1 for Segment 2"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv2(&mut self) -> Clkdiv2W<StcClkdivSpec> {
        Clkdiv2W::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Reserved bits"]
    #[inline(always)]
    #[must_use]
    pub fn nu10(&mut self) -> Nu10W<StcClkdivSpec> {
        Nu10W::new(self, 11)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Clock division for Seg1 (RWP - Read, Priviledge Mode Write only) *NOT SUPPORTED X = Division ratio is X+1 for Segment 1"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv1(&mut self) -> Clkdiv1W<StcClkdivSpec> {
        Clkdiv1W::new(self, 16)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Reserved bits"]
    #[inline(always)]
    #[must_use]
    pub fn nu9(&mut self) -> Nu9W<StcClkdivSpec> {
        Nu9W::new(self, 19)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Clock division for Seg0 (RWP - Read, Priviledge Mode Write only) *NOT SUPPORTED X = Division ratio is X+1 for Segment 0"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv0(&mut self) -> Clkdiv0W<StcClkdivSpec> {
        Clkdiv0W::new(self, 24)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Reserved bits"]
    #[inline(always)]
    #[must_use]
    pub fn nu8(&mut self) -> Nu8W<StcClkdivSpec> {
        Nu8W::new(self, 27)
    }
}
#[doc = "Clock Divider Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stc_clkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stc_clkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StcClkdivSpec;
impl crate::RegisterSpec for StcClkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stc_clkdiv::R`](R) reader structure"]
impl crate::Readable for StcClkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`stc_clkdiv::W`](W) writer structure"]
impl crate::Writable for StcClkdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STC_CLKDIV to value 0"]
impl crate::Resettable for StcClkdivSpec {
    const RESET_VALUE: u32 = 0;
}
