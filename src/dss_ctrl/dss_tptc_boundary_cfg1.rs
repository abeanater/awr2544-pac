#[doc = "Register `DSS_TPTC_BOUNDARY_CFG1` reader"]
pub type R = crate::R<DssTptcBoundaryCfg1Spec>;
#[doc = "Register `DSS_TPTC_BOUNDARY_CFG1` writer"]
pub type W = crate::W<DssTptcBoundaryCfg1Spec>;
#[doc = "Field `tptc_c0_size` reader - 5:0\\]
RESERVED: Dont Use"]
pub type TptcC0SizeR = crate::FieldReader;
#[doc = "Field `tptc_c0_size` writer - 5:0\\]
RESERVED: Dont Use"]
pub type TptcC0SizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tptc_c1_size` reader - 13:8\\]
RESERVED: Dont Use"]
pub type TptcC1SizeR = crate::FieldReader;
#[doc = "Field `tptc_c1_size` writer - 13:8\\]
RESERVED: Dont Use"]
pub type TptcC1SizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tptc_c2_size` reader - 21:16\\]
RESERVED: Dont Use"]
pub type TptcC2SizeR = crate::FieldReader;
#[doc = "Field `tptc_c2_size` writer - 21:16\\]
RESERVED: Dont Use"]
pub type TptcC2SizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tptc_c3_size` reader - 29:24\\]
RESERVED: Dont Use"]
pub type TptcC3SizeR = crate::FieldReader;
#[doc = "Field `tptc_c3_size` writer - 29:24\\]
RESERVED: Dont Use"]
pub type TptcC3SizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tptc_c0_size(&self) -> TptcC0SizeR {
        TptcC0SizeR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tptc_c1_size(&self) -> TptcC1SizeR {
        TptcC1SizeR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tptc_c2_size(&self) -> TptcC2SizeR {
        TptcC2SizeR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tptc_c3_size(&self) -> TptcC3SizeR {
        TptcC3SizeR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_c0_size(&mut self) -> TptcC0SizeW<DssTptcBoundaryCfg1Spec> {
        TptcC0SizeW::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_c1_size(&mut self) -> TptcC1SizeW<DssTptcBoundaryCfg1Spec> {
        TptcC1SizeW::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_c2_size(&mut self) -> TptcC2SizeW<DssTptcBoundaryCfg1Spec> {
        TptcC2SizeW::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_c3_size(&mut self) -> TptcC3SizeW<DssTptcBoundaryCfg1Spec> {
        TptcC3SizeW::new(self, 24)
    }
}
#[doc = "DSS_TPTC_BOUNDARY_CFG1\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tptc_boundary_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tptc_boundary_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssTptcBoundaryCfg1Spec;
impl crate::RegisterSpec for DssTptcBoundaryCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_tptc_boundary_cfg1::R`](R) reader structure"]
impl crate::Readable for DssTptcBoundaryCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`dss_tptc_boundary_cfg1::W`](W) writer structure"]
impl crate::Writable for DssTptcBoundaryCfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_TPTC_BOUNDARY_CFG1 to value 0"]
impl crate::Resettable for DssTptcBoundaryCfg1Spec {
    const RESET_VALUE: u32 = 0;
}
