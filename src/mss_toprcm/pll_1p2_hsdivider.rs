#[doc = "Register `PLL_1P2_HSDIVIDER` reader"]
pub type R = crate::R<Pll1p2HsdividerSpec>;
#[doc = "Register `PLL_1P2_HSDIVIDER` writer"]
pub type W = crate::W<Pll1p2HsdividerSpec>;
#[doc = "Field `BYPASS` reader - 0:0\\]
Do not use. TI Reserved."]
pub type BypassR = crate::BitReader;
#[doc = "Field `BYPASS` writer - 0:0\\]
Do not use. TI Reserved."]
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDOPWDN` reader - 1:1\\]
Do not use. TI Reserved."]
pub type LdopwdnR = crate::BitReader;
#[doc = "Field `LDOPWDN` writer - 1:1\\]
Do not use. TI Reserved."]
pub type LdopwdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TENABLEDIV` reader - 2:2\\]
Do not use. TI Reserved."]
pub type TenabledivR = crate::BitReader;
#[doc = "Field `TENABLEDIV` writer - 2:2\\]
Do not use. TI Reserved."]
pub type TenabledivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPASSACKZ` reader - 16:16\\]
Do not use. TI Reserved."]
pub type BypassackzR = crate::BitReader;
#[doc = "Field `BYPASSACKZ` writer - 16:16\\]
Do not use. TI Reserved."]
pub type BypassackzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDOPWDNACK` reader - 17:17\\]
Do not use. TI Reserved."]
pub type LdopwdnackR = crate::BitReader;
#[doc = "Field `LDOPWDNACK` writer - 17:17\\]
Do not use. TI Reserved."]
pub type LdopwdnackW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn ldopwdn(&self) -> LdopwdnR {
        LdopwdnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn tenablediv(&self) -> TenabledivR {
        TenabledivR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn bypassackz(&self) -> BypassackzR {
        BypassackzR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Do not use. TI Reserved."]
    #[inline(always)]
    pub fn ldopwdnack(&self) -> LdopwdnackR {
        LdopwdnackR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BypassW<Pll1p2HsdividerSpec> {
        BypassW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn ldopwdn(&mut self) -> LdopwdnW<Pll1p2HsdividerSpec> {
        LdopwdnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn tenablediv(&mut self) -> TenabledivW<Pll1p2HsdividerSpec> {
        TenabledivW::new(self, 2)
    }
    #[doc = "Bit 16 - 16:16\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bypassackz(&mut self) -> BypassackzW<Pll1p2HsdividerSpec> {
        BypassackzW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Do not use. TI Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn ldopwdnack(&mut self) -> LdopwdnackW<Pll1p2HsdividerSpec> {
        LdopwdnackW::new(self, 17)
    }
}
#[doc = "PLL_1P2_HSDIVIDER\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_1p2_hsdivider::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_1p2_hsdivider::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll1p2HsdividerSpec;
impl crate::RegisterSpec for Pll1p2HsdividerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_1p2_hsdivider::R`](R) reader structure"]
impl crate::Readable for Pll1p2HsdividerSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_1p2_hsdivider::W`](W) writer structure"]
impl crate::Writable for Pll1p2HsdividerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL_1P2_HSDIVIDER to value 0"]
impl crate::Resettable for Pll1p2HsdividerSpec {
    const RESET_VALUE: u32 = 0;
}
