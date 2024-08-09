#[doc = "Register `PLL_CORE_MN2DIV` reader"]
pub type R = crate::R<PllCoreMn2divSpec>;
#[doc = "Register `PLL_CORE_MN2DIV` writer"]
pub type W = crate::W<PllCoreMn2divSpec>;
#[doc = "Field `M` reader - 11:0\\]
Feedback Multiplier is REGM"]
pub type MR = crate::FieldReader<u16>;
#[doc = "Field `M` writer - 11:0\\]
Feedback Multiplier is REGM"]
pub type MW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `N2` reader - 19:16\\]
Bypass divider is REGN2+1"]
pub type N2R = crate::FieldReader;
#[doc = "Field `N2` writer - 19:16\\]
Bypass divider is REGN2+1"]
pub type N2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Feedback Multiplier is REGM"]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Bypass divider is REGN2+1"]
    #[inline(always)]
    pub fn n2(&self) -> N2R {
        N2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Feedback Multiplier is REGM"]
    #[inline(always)]
    #[must_use]
    pub fn m(&mut self) -> MW<PllCoreMn2divSpec> {
        MW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Bypass divider is REGN2+1"]
    #[inline(always)]
    #[must_use]
    pub fn n2(&mut self) -> N2W<PllCoreMn2divSpec> {
        N2W::new(self, 16)
    }
}
#[doc = "PLL_CORE_MN2DIV\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_core_mn2div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_core_mn2div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllCoreMn2divSpec;
impl crate::RegisterSpec for PllCoreMn2divSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_core_mn2div::R`](R) reader structure"]
impl crate::Readable for PllCoreMn2divSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_core_mn2div::W`](W) writer structure"]
impl crate::Writable for PllCoreMn2divSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL_CORE_MN2DIV to value 0"]
impl crate::Resettable for PllCoreMn2divSpec {
    const RESET_VALUE: u32 = 0;
}
