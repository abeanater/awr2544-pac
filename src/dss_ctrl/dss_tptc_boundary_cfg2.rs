#[doc = "Register `DSS_TPTC_BOUNDARY_CFG2` reader"]
pub type R = crate::R<DssTptcBoundaryCfg2Spec>;
#[doc = "Register `DSS_TPTC_BOUNDARY_CFG2` writer"]
pub type W = crate::W<DssTptcBoundaryCfg2Spec>;
#[doc = "Field `tptc_c4_size` reader - 5:0\\]
RESERVED: Dont Use"]
pub type TptcC4SizeR = crate::FieldReader;
#[doc = "Field `tptc_c4_size` writer - 5:0\\]
RESERVED: Dont Use"]
pub type TptcC4SizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tptc_c5_size` reader - 13:8\\]
RESERVED: Dont Use"]
pub type TptcC5SizeR = crate::FieldReader;
#[doc = "Field `tptc_c5_size` writer - 13:8\\]
RESERVED: Dont Use"]
pub type TptcC5SizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tptc_c4_size(&self) -> TptcC4SizeR {
        TptcC4SizeR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tptc_c5_size(&self) -> TptcC5SizeR {
        TptcC5SizeR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_c4_size(&mut self) -> TptcC4SizeW<DssTptcBoundaryCfg2Spec> {
        TptcC4SizeW::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_c5_size(&mut self) -> TptcC5SizeW<DssTptcBoundaryCfg2Spec> {
        TptcC5SizeW::new(self, 8)
    }
}
#[doc = "DSS_TPTC_BOUNDARY_CFG2\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tptc_boundary_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tptc_boundary_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssTptcBoundaryCfg2Spec;
impl crate::RegisterSpec for DssTptcBoundaryCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_tptc_boundary_cfg2::R`](R) reader structure"]
impl crate::Readable for DssTptcBoundaryCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`dss_tptc_boundary_cfg2::W`](W) writer structure"]
impl crate::Writable for DssTptcBoundaryCfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_TPTC_BOUNDARY_CFG2 to value 0"]
impl crate::Resettable for DssTptcBoundaryCfg2Spec {
    const RESET_VALUE: u32 = 0;
}
