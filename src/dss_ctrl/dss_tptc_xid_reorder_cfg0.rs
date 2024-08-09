#[doc = "Register `DSS_TPTC_XID_REORDER_CFG0` reader"]
pub type R = crate::R<DssTptcXidReorderCfg0Spec>;
#[doc = "Register `DSS_TPTC_XID_REORDER_CFG0` writer"]
pub type W = crate::W<DssTptcXidReorderCfg0Spec>;
#[doc = "Field `tptc_a0_disable` reader - 0:0\\]
Writing 1b1 will disable the CID-RID-SID reodering feature for the TPTC instance"]
pub type TptcA0DisableR = crate::BitReader;
#[doc = "Field `tptc_a0_disable` writer - 0:0\\]
Writing 1b1 will disable the CID-RID-SID reodering feature for the TPTC instance"]
pub type TptcA0DisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_a1_disable` reader - 8:8\\]
Writing 1b1 will disable the CID-RID-SID reodering feature for the TPTC instance"]
pub type TptcA1DisableR = crate::BitReader;
#[doc = "Field `tptc_a1_disable` writer - 8:8\\]
Writing 1b1 will disable the CID-RID-SID reodering feature for the TPTC instance"]
pub type TptcA1DisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_b0_disable` reader - 16:16\\]
RESERVED: Dont Use"]
pub type TptcB0DisableR = crate::BitReader;
#[doc = "Field `tptc_b0_disable` writer - 16:16\\]
RESERVED: Dont Use"]
pub type TptcB0DisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_b1_disable` reader - 24:24\\]
RESERVED: Dont Use"]
pub type TptcB1DisableR = crate::BitReader;
#[doc = "Field `tptc_b1_disable` writer - 24:24\\]
RESERVED: Dont Use"]
pub type TptcB1DisableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Writing 1b1 will disable the CID-RID-SID reodering feature for the TPTC instance"]
    #[inline(always)]
    pub fn tptc_a0_disable(&self) -> TptcA0DisableR {
        TptcA0DisableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 1b1 will disable the CID-RID-SID reodering feature for the TPTC instance"]
    #[inline(always)]
    pub fn tptc_a1_disable(&self) -> TptcA1DisableR {
        TptcA1DisableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tptc_b0_disable(&self) -> TptcB0DisableR {
        TptcB0DisableR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tptc_b1_disable(&self) -> TptcB1DisableR {
        TptcB1DisableR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Writing 1b1 will disable the CID-RID-SID reodering feature for the TPTC instance"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a0_disable(&mut self) -> TptcA0DisableW<DssTptcXidReorderCfg0Spec> {
        TptcA0DisableW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 1b1 will disable the CID-RID-SID reodering feature for the TPTC instance"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a1_disable(&mut self) -> TptcA1DisableW<DssTptcXidReorderCfg0Spec> {
        TptcA1DisableW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b0_disable(&mut self) -> TptcB0DisableW<DssTptcXidReorderCfg0Spec> {
        TptcB0DisableW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_b1_disable(&mut self) -> TptcB1DisableW<DssTptcXidReorderCfg0Spec> {
        TptcB1DisableW::new(self, 24)
    }
}
#[doc = "DSS_TPTC_XID_REORDER_CFG0\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tptc_xid_reorder_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tptc_xid_reorder_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssTptcXidReorderCfg0Spec;
impl crate::RegisterSpec for DssTptcXidReorderCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_tptc_xid_reorder_cfg0::R`](R) reader structure"]
impl crate::Readable for DssTptcXidReorderCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`dss_tptc_xid_reorder_cfg0::W`](W) writer structure"]
impl crate::Writable for DssTptcXidReorderCfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_TPTC_XID_REORDER_CFG0 to value 0"]
impl crate::Resettable for DssTptcXidReorderCfg0Spec {
    const RESET_VALUE: u32 = 0;
}
