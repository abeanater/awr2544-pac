#[doc = "Register `DSS_TPTC_BOUNDARY_CFG0` reader"]
pub type R = crate::R<DssTptcBoundaryCfg0Spec>;
#[doc = "Register `DSS_TPTC_BOUNDARY_CFG0` writer"]
pub type W = crate::W<DssTptcBoundaryCfg0Spec>;
#[doc = "Field `tptc_a0_size` reader - 5:0\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC Example: writing 6d19 decidies boundary to be 2^19 i.e. 512 KB"]
pub type TptcA0SizeR = crate::FieldReader;
#[doc = "Field `tptc_a0_size` writer - 5:0\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC Example: writing 6d19 decidies boundary to be 2^19 i.e. 512 KB"]
pub type TptcA0SizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tptc_a1_size` reader - 13:8\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC Example: writing 6d19 decidies boundary to be 2^19 i.e. 512 KB"]
pub type TptcA1SizeR = crate::FieldReader;
#[doc = "Field `tptc_a1_size` writer - 13:8\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC Example: writing 6d19 decidies boundary to be 2^19 i.e. 512 KB"]
pub type TptcA1SizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tptc_b0_size` reader - 21:16\\]
RESERVED: Dont Use"]
pub type TptcB0SizeR = crate::FieldReader;
#[doc = "Field `tptc_b0_size` writer - 21:16\\]
RESERVED: Dont Use"]
pub type TptcB0SizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tptc_b1_size` reader - 29:24\\]
RESERVED: Dont Use"]
pub type TptcB1SizeR = crate::FieldReader;
#[doc = "Field `tptc_b1_size` writer - 29:24\\]
RESERVED: Dont Use"]
pub type TptcB1SizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC Example: writing 6d19 decidies boundary to be 2^19 i.e. 512 KB"]
    #[inline(always)]
    pub fn tptc_a0_size(&self) -> TptcA0SizeR {
        TptcA0SizeR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC Example: writing 6d19 decidies boundary to be 2^19 i.e. 512 KB"]
    #[inline(always)]
    pub fn tptc_a1_size(&self) -> TptcA1SizeR {
        TptcA1SizeR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tptc_b0_size(&self) -> TptcB0SizeR {
        TptcB0SizeR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tptc_b1_size(&self) -> TptcB1SizeR {
        TptcB1SizeR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC Example: writing 6d19 decidies boundary to be 2^19 i.e. 512 KB"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a0_size(&mut self) -> TptcA0SizeW<DssTptcBoundaryCfg0Spec> {
        TptcA0SizeW::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC Example: writing 6d19 decidies boundary to be 2^19 i.e. 512 KB"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a1_size(&mut self) -> TptcA1SizeW<DssTptcBoundaryCfg0Spec> {
        TptcA1SizeW::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b0_size(&mut self) -> TptcB0SizeW<DssTptcBoundaryCfg0Spec> {
        TptcB0SizeW::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b1_size(&mut self) -> TptcB1SizeW<DssTptcBoundaryCfg0Spec> {
        TptcB1SizeW::new(self, 24)
    }
}
#[doc = "DSS_TPTC_BOUNDARY_CFG0\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tptc_boundary_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tptc_boundary_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssTptcBoundaryCfg0Spec;
impl crate::RegisterSpec for DssTptcBoundaryCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_tptc_boundary_cfg0::R`](R) reader structure"]
impl crate::Readable for DssTptcBoundaryCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`dss_tptc_boundary_cfg0::W`](W) writer structure"]
impl crate::Writable for DssTptcBoundaryCfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_TPTC_BOUNDARY_CFG0 to value 0"]
impl crate::Resettable for DssTptcBoundaryCfg0Spec {
    const RESET_VALUE: u32 = 0;
}
