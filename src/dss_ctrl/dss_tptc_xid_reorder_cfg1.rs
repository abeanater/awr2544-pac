#[doc = "Register `DSS_TPTC_XID_REORDER_CFG1` reader"]
pub type R = crate::R<DssTptcXidReorderCfg1Spec>;
#[doc = "Register `DSS_TPTC_XID_REORDER_CFG1` writer"]
pub type W = crate::W<DssTptcXidReorderCfg1Spec>;
#[doc = "Field `tptc_c0_disable` reader - 0:0\\]
RESERVED: Dont Use"]
pub type TptcC0DisableR = crate::BitReader;
#[doc = "Field `tptc_c0_disable` writer - 0:0\\]
RESERVED: Dont Use"]
pub type TptcC0DisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_c1_disable` reader - 8:8\\]
RESERVED: Dont Use"]
pub type TptcC1DisableR = crate::BitReader;
#[doc = "Field `tptc_c1_disable` writer - 8:8\\]
RESERVED: Dont Use"]
pub type TptcC1DisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_c2_disable` reader - 16:16\\]
RESERVED: Dont Use"]
pub type TptcC2DisableR = crate::BitReader;
#[doc = "Field `tptc_c2_disable` writer - 16:16\\]
RESERVED: Dont Use"]
pub type TptcC2DisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_c3_disable` reader - 24:24\\]
RESERVED: Dont Use"]
pub type TptcC3DisableR = crate::BitReader;
#[doc = "Field `tptc_c3_disable` writer - 24:24\\]
RESERVED: Dont Use"]
pub type TptcC3DisableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tptc_c0_disable(&self) -> TptcC0DisableR {
        TptcC0DisableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tptc_c1_disable(&self) -> TptcC1DisableR {
        TptcC1DisableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tptc_c2_disable(&self) -> TptcC2DisableR {
        TptcC2DisableR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tptc_c3_disable(&self) -> TptcC3DisableR {
        TptcC3DisableR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_c0_disable(&mut self) -> TptcC0DisableW<DssTptcXidReorderCfg1Spec> {
        TptcC0DisableW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_c1_disable(&mut self) -> TptcC1DisableW<DssTptcXidReorderCfg1Spec> {
        TptcC1DisableW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_c2_disable(&mut self) -> TptcC2DisableW<DssTptcXidReorderCfg1Spec> {
        TptcC2DisableW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_c3_disable(&mut self) -> TptcC3DisableW<DssTptcXidReorderCfg1Spec> {
        TptcC3DisableW::new(self, 24)
    }
}
#[doc = "DSS_TPTC_XID_REORDER_CFG1\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tptc_xid_reorder_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tptc_xid_reorder_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssTptcXidReorderCfg1Spec;
impl crate::RegisterSpec for DssTptcXidReorderCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_tptc_xid_reorder_cfg1::R`](R) reader structure"]
impl crate::Readable for DssTptcXidReorderCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`dss_tptc_xid_reorder_cfg1::W`](W) writer structure"]
impl crate::Writable for DssTptcXidReorderCfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_TPTC_XID_REORDER_CFG1 to value 0"]
impl crate::Resettable for DssTptcXidReorderCfg1Spec {
    const RESET_VALUE: u32 = 0;
}
