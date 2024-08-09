#[doc = "Register `MSS_TPTC_XID_REORDER_CFG` reader"]
pub type R = crate::R<MssTptcXidReorderCfgSpec>;
#[doc = "Register `MSS_TPTC_XID_REORDER_CFG` writer"]
pub type W = crate::W<MssTptcXidReorderCfgSpec>;
#[doc = "Field `tptc_a0_disable` reader - 0:0\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for MSS_TPTC_A0"]
pub type TptcA0DisableR = crate::BitReader;
#[doc = "Field `tptc_a0_disable` writer - 0:0\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for MSS_TPTC_A0"]
pub type TptcA0DisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_a1_disable` reader - 8:8\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for MSS_TPTC_A1"]
pub type TptcA1DisableR = crate::BitReader;
#[doc = "Field `tptc_a1_disable` writer - 8:8\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for MSS_TPTC_A1"]
pub type TptcA1DisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_b0_disable` reader - 16:16\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for MSS_TPTC_B0"]
pub type TptcB0DisableR = crate::BitReader;
#[doc = "Field `tptc_b0_disable` writer - 16:16\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for MSS_TPTC_B0"]
pub type TptcB0DisableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for MSS_TPTC_A0"]
    #[inline(always)]
    pub fn tptc_a0_disable(&self) -> TptcA0DisableR {
        TptcA0DisableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for MSS_TPTC_A1"]
    #[inline(always)]
    pub fn tptc_a1_disable(&self) -> TptcA1DisableR {
        TptcA1DisableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for MSS_TPTC_B0"]
    #[inline(always)]
    pub fn tptc_b0_disable(&self) -> TptcB0DisableR {
        TptcB0DisableR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for MSS_TPTC_A0"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a0_disable(&mut self) -> TptcA0DisableW<MssTptcXidReorderCfgSpec> {
        TptcA0DisableW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for MSS_TPTC_A1"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a1_disable(&mut self) -> TptcA1DisableW<MssTptcXidReorderCfgSpec> {
        TptcA1DisableW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
writing 1'b1 will disable the CID-RID-SID reodering feature for MSS_TPTC_B0"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b0_disable(&mut self) -> TptcB0DisableW<MssTptcXidReorderCfgSpec> {
        TptcB0DisableW::new(self, 16)
    }
}
#[doc = "MSS_TPTC_XID_REORDER_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_tptc_xid_reorder_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_tptc_xid_reorder_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssTptcXidReorderCfgSpec;
impl crate::RegisterSpec for MssTptcXidReorderCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_tptc_xid_reorder_cfg::R`](R) reader structure"]
impl crate::Readable for MssTptcXidReorderCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`mss_tptc_xid_reorder_cfg::W`](W) writer structure"]
impl crate::Writable for MssTptcXidReorderCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_TPTC_XID_REORDER_CFG to value 0"]
impl crate::Resettable for MssTptcXidReorderCfgSpec {
    const RESET_VALUE: u32 = 0;
}
