#[doc = "Register `RSS_TPTC_XID_REORDER_CFG` reader"]
pub type R = crate::R<RssTptcXidReorderCfgSpec>;
#[doc = "Register `RSS_TPTC_XID_REORDER_CFG` writer"]
pub type W = crate::W<RssTptcXidReorderCfgSpec>;
#[doc = "Field `tptc_a0_disable` reader - 0:0\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
pub type TptcA0DisableR = crate::BitReader;
#[doc = "Field `tptc_a0_disable` writer - 0:0\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
pub type TptcA0DisableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
    #[inline(always)]
    pub fn tptc_a0_disable(&self) -> TptcA0DisableR {
        TptcA0DisableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
6 bit signal used for deciding the boundary crossing size for CID-RID-SID reordering of TPTC Example: writing 6'd19 decidies boundary to be 2^19 i.e. 512 KB"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a0_disable(&mut self) -> TptcA0DisableW<RssTptcXidReorderCfgSpec> {
        TptcA0DisableW::new(self, 0)
    }
}
#[doc = "RSS_TPTC_XID_REORDER_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`rss_tptc_xid_reorder_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rss_tptc_xid_reorder_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RssTptcXidReorderCfgSpec;
impl crate::RegisterSpec for RssTptcXidReorderCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rss_tptc_xid_reorder_cfg::R`](R) reader structure"]
impl crate::Readable for RssTptcXidReorderCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`rss_tptc_xid_reorder_cfg::W`](W) writer structure"]
impl crate::Writable for RssTptcXidReorderCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSS_TPTC_XID_REORDER_CFG to value 0"]
impl crate::Resettable for RssTptcXidReorderCfgSpec {
    const RESET_VALUE: u32 = 0;
}
