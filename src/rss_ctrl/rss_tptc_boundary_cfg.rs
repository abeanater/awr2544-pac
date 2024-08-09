#[doc = "Register `RSS_TPTC_BOUNDARY_CFG` reader"]
pub type R = crate::R<RssTptcBoundaryCfgSpec>;
#[doc = "Register `RSS_TPTC_BOUNDARY_CFG` writer"]
pub type W = crate::W<RssTptcBoundaryCfgSpec>;
#[doc = "Field `tptc_a0_size` reader - 5:0\\]
Writing 1'b1 will disable the CID-RID-SID reodering feature for the TPTC instance"]
pub type TptcA0SizeR = crate::FieldReader;
#[doc = "Field `tptc_a0_size` writer - 5:0\\]
Writing 1'b1 will disable the CID-RID-SID reodering feature for the TPTC instance"]
pub type TptcA0SizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Writing 1'b1 will disable the CID-RID-SID reodering feature for the TPTC instance"]
    #[inline(always)]
    pub fn tptc_a0_size(&self) -> TptcA0SizeR {
        TptcA0SizeR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Writing 1'b1 will disable the CID-RID-SID reodering feature for the TPTC instance"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a0_size(&mut self) -> TptcA0SizeW<RssTptcBoundaryCfgSpec> {
        TptcA0SizeW::new(self, 0)
    }
}
#[doc = "RSS_TPTC_BOUNDARY_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_tptc_boundary_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_tptc_boundary_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssTptcBoundaryCfgSpec;
impl crate::RegisterSpec for RssTptcBoundaryCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rss_tptc_boundary_cfg::R`](R) reader structure"]
impl crate::Readable for RssTptcBoundaryCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`rss_tptc_boundary_cfg::W`](W) writer structure"]
impl crate::Writable for RssTptcBoundaryCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSS_TPTC_BOUNDARY_CFG to value 0"]
impl crate::Resettable for RssTptcBoundaryCfgSpec {
    const RESET_VALUE: u32 = 0;
}
