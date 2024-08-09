#[doc = "Register `DSS_TPTC_XID_REORDER_CFG2` reader"]
pub type R = crate::R<DssTptcXidReorderCfg2Spec>;
#[doc = "Register `DSS_TPTC_XID_REORDER_CFG2` writer"]
pub type W = crate::W<DssTptcXidReorderCfg2Spec>;
#[doc = "Field `tptc_c4_disable` reader - 0:0\\]
RESERVED: Dont Use"]
pub type TptcC4DisableR = crate::BitReader;
#[doc = "Field `tptc_c4_disable` writer - 0:0\\]
RESERVED: Dont Use"]
pub type TptcC4DisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_c5_disable` reader - 8:8\\]
RESERVED: Dont Use"]
pub type TptcC5DisableR = crate::BitReader;
#[doc = "Field `tptc_c5_disable` writer - 8:8\\]
RESERVED: Dont Use"]
pub type TptcC5DisableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tptc_c4_disable(&self) -> TptcC4DisableR {
        TptcC4DisableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tptc_c5_disable(&self) -> TptcC5DisableR {
        TptcC5DisableR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_c4_disable(&mut self) -> TptcC4DisableW<DssTptcXidReorderCfg2Spec> {
        TptcC4DisableW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_c5_disable(&mut self) -> TptcC5DisableW<DssTptcXidReorderCfg2Spec> {
        TptcC5DisableW::new(self, 8)
    }
}
#[doc = "DSS_TPTC_XID_REORDER_CFG2\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tptc_xid_reorder_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tptc_xid_reorder_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssTptcXidReorderCfg2Spec;
impl crate::RegisterSpec for DssTptcXidReorderCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_tptc_xid_reorder_cfg2::R`](R) reader structure"]
impl crate::Readable for DssTptcXidReorderCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`dss_tptc_xid_reorder_cfg2::W`](W) writer structure"]
impl crate::Writable for DssTptcXidReorderCfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_TPTC_XID_REORDER_CFG2 to value 0"]
impl crate::Resettable for DssTptcXidReorderCfg2Spec {
    const RESET_VALUE: u32 = 0;
}
