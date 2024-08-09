#[doc = "Register `MSS_TPTC_BOUNDARY_CFG` reader"]
pub type R = crate::R<MssTptcBoundaryCfgSpec>;
#[doc = "Register `MSS_TPTC_BOUNDARY_CFG` writer"]
pub type W = crate::W<MssTptcBoundaryCfgSpec>;
#[doc = "Field `tptc_a0_size` reader - 5:0\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of MSS_TPTC_A0 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
pub type TptcA0SizeR = crate::FieldReader;
#[doc = "Field `tptc_a0_size` writer - 5:0\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of MSS_TPTC_A0 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
pub type TptcA0SizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tptc_a1_size` reader - 13:8\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of MSS_TPTC_A1 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
pub type TptcA1SizeR = crate::FieldReader;
#[doc = "Field `tptc_a1_size` writer - 13:8\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of MSS_TPTC_A1 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
pub type TptcA1SizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `tptc_b0_size` reader - 21:16\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of MSS_TPTC_B0 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
pub type TptcB0SizeR = crate::FieldReader;
#[doc = "Field `tptc_b0_size` writer - 21:16\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of MSS_TPTC_B0 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
pub type TptcB0SizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of MSS_TPTC_A0 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
    #[inline(always)]
    pub fn tptc_a0_size(&self) -> TptcA0SizeR {
        TptcA0SizeR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of MSS_TPTC_A1 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
    #[inline(always)]
    pub fn tptc_a1_size(&self) -> TptcA1SizeR {
        TptcA1SizeR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of MSS_TPTC_B0 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
    #[inline(always)]
    pub fn tptc_b0_size(&self) -> TptcB0SizeR {
        TptcB0SizeR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of MSS_TPTC_A0 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a0_size(&mut self) -> TptcA0SizeW<MssTptcBoundaryCfgSpec> {
        TptcA0SizeW::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of MSS_TPTC_A1 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a1_size(&mut self) -> TptcA1SizeW<MssTptcBoundaryCfgSpec> {
        TptcA1SizeW::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of MSS_TPTC_B0 Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b0_size(&mut self) -> TptcB0SizeW<MssTptcBoundaryCfgSpec> {
        TptcB0SizeW::new(self, 16)
    }
}
#[doc = "MSS_TPTC_BOUNDARY_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_tptc_boundary_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_tptc_boundary_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssTptcBoundaryCfgSpec;
impl crate::RegisterSpec for MssTptcBoundaryCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_tptc_boundary_cfg::R`](R) reader structure"]
impl crate::Readable for MssTptcBoundaryCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_tptc_boundary_cfg::W`](W) writer structure"]
impl crate::Writable for MssTptcBoundaryCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_TPTC_BOUNDARY_CFG to value 0"]
impl crate::Resettable for MssTptcBoundaryCfgSpec {
    const RESET_VALUE: u32 = 0;
}
