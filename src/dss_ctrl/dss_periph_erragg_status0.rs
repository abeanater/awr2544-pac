#[doc = "Register `DSS_PERIPH_ERRAGG_STATUS0` reader"]
pub type R = crate::R<DssPeriphErraggStatus0Spec>;
#[doc = "Register `DSS_PERIPH_ERRAGG_STATUS0` writer"]
pub type W = crate::W<DssPeriphErraggStatus0Spec>;
#[doc = "Field `dss_rcm_rd` reader - 0:0\\]
Status of the Read error from DSS_RCM space. Read 1 : Read error occurred on access to the DSS_RCM space"]
pub type DssRcmRdR = crate::BitReader;
#[doc = "Field `dss_rcm_rd` writer - 0:0\\]
Status of the Read error from DSS_RCM space. Read 1 : Read error occurred on access to the DSS_RCM space"]
pub type DssRcmRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dss_rcm_wr` reader - 1:1\\]
Status of the Write error from DSS_RCM space. Read 1 : Read error occurred on access to the DSS_RCM space"]
pub type DssRcmWrR = crate::BitReader;
#[doc = "Field `dss_rcm_wr` writer - 1:1\\]
Status of the Write error from DSS_RCM space. Read 1 : Read error occurred on access to the DSS_RCM space"]
pub type DssRcmWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dss_ctrl_rd` reader - 2:2\\]
Status of the Read error from DSS_CTRL space. Read 1 : Read error occurred on access to the DSS_CTRL space"]
pub type DssCtrlRdR = crate::BitReader;
#[doc = "Field `dss_ctrl_rd` writer - 2:2\\]
Status of the Read error from DSS_CTRL space. Read 1 : Read error occurred on access to the DSS_CTRL space"]
pub type DssCtrlRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dss_ctrl_wr` reader - 3:3\\]
Status of the Write error from DSS_CTRL space. Read 1 : Read error occurred on access to the DSS_CTRL space"]
pub type DssCtrlWrR = crate::BitReader;
#[doc = "Field `dss_ctrl_wr` writer - 3:3\\]
Status of the Write error from DSS_CTRL space. Read 1 : Read error occurred on access to the DSS_CTRL space"]
pub type DssCtrlWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dss_cm4_ctrl_rd` reader - 4:4\\]
Status of the Read error from DSS_CM4_CTRL space. Read 1 : Read error occurred on access to the DSS_CM4_CTRL space"]
pub type DssCm4CtrlRdR = crate::BitReader;
#[doc = "Field `dss_cm4_ctrl_rd` writer - 4:4\\]
Status of the Read error from DSS_CM4_CTRL space. Read 1 : Read error occurred on access to the DSS_CM4_CTRL space"]
pub type DssCm4CtrlRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dss_cm4_ctrl_wr` reader - 5:5\\]
Status of the Write error from DSS_CM4_CTRL space. Read 1 : Read error occurred on access to the DSS_CM4_CTRL space"]
pub type DssCm4CtrlWrR = crate::BitReader;
#[doc = "Field `dss_cm4_ctrl_wr` writer - 5:5\\]
Status of the Write error from DSS_CM4_CTRL space. Read 1 : Read error occurred on access to the DSS_CM4_CTRL space"]
pub type DssCm4CtrlWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dss_hwa_cfg_rd` reader - 6:6\\]
Status of the Read error from DSS_HWA_CFG space. Read 1 : Read error occurred on access to the DSS_HWA_CFG space"]
pub type DssHwaCfgRdR = crate::BitReader;
#[doc = "Field `dss_hwa_cfg_rd` writer - 6:6\\]
Status of the Read error from DSS_HWA_CFG space. Read 1 : Read error occurred on access to the DSS_HWA_CFG space"]
pub type DssHwaCfgRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dss_hwa_cfg_wr` reader - 7:7\\]
Status of the Write error from DSS_HWA_CFG space. Read 1 : Read error occurred on access to the DSS_HWA_CFG space"]
pub type DssHwaCfgWrR = crate::BitReader;
#[doc = "Field `dss_hwa_cfg_wr` writer - 7:7\\]
Status of the Write error from DSS_HWA_CFG space. Read 1 : Read error occurred on access to the DSS_HWA_CFG space"]
pub type DssHwaCfgWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rcss_rcm_rd` reader - 8:8\\]
Status of the Read error from RCSS_RCM space. Read 1 : Read error occurred on access to the RCSS_RCM space"]
pub type RcssRcmRdR = crate::BitReader;
#[doc = "Field `rcss_rcm_rd` writer - 8:8\\]
Status of the Read error from RCSS_RCM space. Read 1 : Read error occurred on access to the RCSS_RCM space"]
pub type RcssRcmRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rcss_rcm_wr` reader - 9:9\\]
Status of the Write error from RCSS_RCM space. Read 1 : Read error occurred on access to the RCSS_RCM space"]
pub type RcssRcmWrR = crate::BitReader;
#[doc = "Field `rcss_rcm_wr` writer - 9:9\\]
Status of the Write error from RCSS_RCM space. Read 1 : Read error occurred on access to the RCSS_RCM space"]
pub type RcssRcmWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rcss_ctrl_rd` reader - 10:10\\]
Status of the Read error from RCSS_CTRL space. Read 1 : Read error occurred on access to the RCSS_CTRL space"]
pub type RcssCtrlRdR = crate::BitReader;
#[doc = "Field `rcss_ctrl_rd` writer - 10:10\\]
Status of the Read error from RCSS_CTRL space. Read 1 : Read error occurred on access to the RCSS_CTRL space"]
pub type RcssCtrlRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rcss_ctrl_wr` reader - 11:11\\]
Status of the Write error from RCSS_CTRL space. Read 1 : Read error occurred on access to the RCSS_CTRL space"]
pub type RcssCtrlWrR = crate::BitReader;
#[doc = "Field `rcss_ctrl_wr` writer - 11:11\\]
Status of the Write error from RCSS_CTRL space. Read 1 : Read error occurred on access to the RCSS_CTRL space"]
pub type RcssCtrlWrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status of the Read error from DSS_RCM space. Read 1 : Read error occurred on access to the DSS_RCM space"]
    #[inline(always)]
    pub fn dss_rcm_rd(&self) -> DssRcmRdR {
        DssRcmRdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status of the Write error from DSS_RCM space. Read 1 : Read error occurred on access to the DSS_RCM space"]
    #[inline(always)]
    pub fn dss_rcm_wr(&self) -> DssRcmWrR {
        DssRcmWrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Status of the Read error from DSS_CTRL space. Read 1 : Read error occurred on access to the DSS_CTRL space"]
    #[inline(always)]
    pub fn dss_ctrl_rd(&self) -> DssCtrlRdR {
        DssCtrlRdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Status of the Write error from DSS_CTRL space. Read 1 : Read error occurred on access to the DSS_CTRL space"]
    #[inline(always)]
    pub fn dss_ctrl_wr(&self) -> DssCtrlWrR {
        DssCtrlWrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Status of the Read error from DSS_CM4_CTRL space. Read 1 : Read error occurred on access to the DSS_CM4_CTRL space"]
    #[inline(always)]
    pub fn dss_cm4_ctrl_rd(&self) -> DssCm4CtrlRdR {
        DssCm4CtrlRdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Status of the Write error from DSS_CM4_CTRL space. Read 1 : Read error occurred on access to the DSS_CM4_CTRL space"]
    #[inline(always)]
    pub fn dss_cm4_ctrl_wr(&self) -> DssCm4CtrlWrR {
        DssCm4CtrlWrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Status of the Read error from DSS_HWA_CFG space. Read 1 : Read error occurred on access to the DSS_HWA_CFG space"]
    #[inline(always)]
    pub fn dss_hwa_cfg_rd(&self) -> DssHwaCfgRdR {
        DssHwaCfgRdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Status of the Write error from DSS_HWA_CFG space. Read 1 : Read error occurred on access to the DSS_HWA_CFG space"]
    #[inline(always)]
    pub fn dss_hwa_cfg_wr(&self) -> DssHwaCfgWrR {
        DssHwaCfgWrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Status of the Read error from RCSS_RCM space. Read 1 : Read error occurred on access to the RCSS_RCM space"]
    #[inline(always)]
    pub fn rcss_rcm_rd(&self) -> RcssRcmRdR {
        RcssRcmRdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Status of the Write error from RCSS_RCM space. Read 1 : Read error occurred on access to the RCSS_RCM space"]
    #[inline(always)]
    pub fn rcss_rcm_wr(&self) -> RcssRcmWrR {
        RcssRcmWrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Status of the Read error from RCSS_CTRL space. Read 1 : Read error occurred on access to the RCSS_CTRL space"]
    #[inline(always)]
    pub fn rcss_ctrl_rd(&self) -> RcssCtrlRdR {
        RcssCtrlRdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Status of the Write error from RCSS_CTRL space. Read 1 : Read error occurred on access to the RCSS_CTRL space"]
    #[inline(always)]
    pub fn rcss_ctrl_wr(&self) -> RcssCtrlWrR {
        RcssCtrlWrR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status of the Read error from DSS_RCM space. Read 1 : Read error occurred on access to the DSS_RCM space"]
    #[inline(always)]
    #[must_use]
    pub fn dss_rcm_rd(&mut self) -> DssRcmRdW<DssPeriphErraggStatus0Spec> {
        DssRcmRdW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status of the Write error from DSS_RCM space. Read 1 : Read error occurred on access to the DSS_RCM space"]
    #[inline(always)]
    #[must_use]
    pub fn dss_rcm_wr(&mut self) -> DssRcmWrW<DssPeriphErraggStatus0Spec> {
        DssRcmWrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Status of the Read error from DSS_CTRL space. Read 1 : Read error occurred on access to the DSS_CTRL space"]
    #[inline(always)]
    #[must_use]
    pub fn dss_ctrl_rd(&mut self) -> DssCtrlRdW<DssPeriphErraggStatus0Spec> {
        DssCtrlRdW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Status of the Write error from DSS_CTRL space. Read 1 : Read error occurred on access to the DSS_CTRL space"]
    #[inline(always)]
    #[must_use]
    pub fn dss_ctrl_wr(&mut self) -> DssCtrlWrW<DssPeriphErraggStatus0Spec> {
        DssCtrlWrW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Status of the Read error from DSS_CM4_CTRL space. Read 1 : Read error occurred on access to the DSS_CM4_CTRL space"]
    #[inline(always)]
    #[must_use]
    pub fn dss_cm4_ctrl_rd(&mut self) -> DssCm4CtrlRdW<DssPeriphErraggStatus0Spec> {
        DssCm4CtrlRdW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Status of the Write error from DSS_CM4_CTRL space. Read 1 : Read error occurred on access to the DSS_CM4_CTRL space"]
    #[inline(always)]
    #[must_use]
    pub fn dss_cm4_ctrl_wr(&mut self) -> DssCm4CtrlWrW<DssPeriphErraggStatus0Spec> {
        DssCm4CtrlWrW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Status of the Read error from DSS_HWA_CFG space. Read 1 : Read error occurred on access to the DSS_HWA_CFG space"]
    #[inline(always)]
    #[must_use]
    pub fn dss_hwa_cfg_rd(&mut self) -> DssHwaCfgRdW<DssPeriphErraggStatus0Spec> {
        DssHwaCfgRdW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Status of the Write error from DSS_HWA_CFG space. Read 1 : Read error occurred on access to the DSS_HWA_CFG space"]
    #[inline(always)]
    #[must_use]
    pub fn dss_hwa_cfg_wr(&mut self) -> DssHwaCfgWrW<DssPeriphErraggStatus0Spec> {
        DssHwaCfgWrW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Status of the Read error from RCSS_RCM space. Read 1 : Read error occurred on access to the RCSS_RCM space"]
    #[inline(always)]
    #[must_use]
    pub fn rcss_rcm_rd(&mut self) -> RcssRcmRdW<DssPeriphErraggStatus0Spec> {
        RcssRcmRdW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Status of the Write error from RCSS_RCM space. Read 1 : Read error occurred on access to the RCSS_RCM space"]
    #[inline(always)]
    #[must_use]
    pub fn rcss_rcm_wr(&mut self) -> RcssRcmWrW<DssPeriphErraggStatus0Spec> {
        RcssRcmWrW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Status of the Read error from RCSS_CTRL space. Read 1 : Read error occurred on access to the RCSS_CTRL space"]
    #[inline(always)]
    #[must_use]
    pub fn rcss_ctrl_rd(&mut self) -> RcssCtrlRdW<DssPeriphErraggStatus0Spec> {
        RcssCtrlRdW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Status of the Write error from RCSS_CTRL space. Read 1 : Read error occurred on access to the RCSS_CTRL space"]
    #[inline(always)]
    #[must_use]
    pub fn rcss_ctrl_wr(&mut self) -> RcssCtrlWrW<DssPeriphErraggStatus0Spec> {
        RcssCtrlWrW::new(self, 11)
    }
}
#[doc = "DSS_PERIPH_ERRAGG_STATUS0\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_periph_erragg_status0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_periph_erragg_status0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssPeriphErraggStatus0Spec;
impl crate::RegisterSpec for DssPeriphErraggStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_periph_erragg_status0::R`](R) reader structure"]
impl crate::Readable for DssPeriphErraggStatus0Spec {}
#[doc = "`write(|w| ..)` method takes [`dss_periph_erragg_status0::W`](W) writer structure"]
impl crate::Writable for DssPeriphErraggStatus0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_PERIPH_ERRAGG_STATUS0 to value 0"]
impl crate::Resettable for DssPeriphErraggStatus0Spec {
    const RESET_VALUE: u32 = 0;
}
