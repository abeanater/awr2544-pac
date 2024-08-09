#[doc = "Register `MSS_PERIPH_ERRAGG_STATUS_RAW0` reader"]
pub type R = crate::R<MssPeriphErraggStatusRaw0Spec>;
#[doc = "Register `MSS_PERIPH_ERRAGG_STATUS_RAW0` writer"]
pub type W = crate::W<MssPeriphErraggStatusRaw0Spec>;
#[doc = "Field `mss_ctrl_rd` reader - 0:0\\]
Raw Status of Interrupt from MSS_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type MssCtrlRdR = crate::BitReader;
#[doc = "Field `mss_ctrl_rd` writer - 0:0\\]
Raw Status of Interrupt from MSS_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type MssCtrlRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mss_ctrl_wr` reader - 1:1\\]
Raw Status of Interrupt from MSS_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type MssCtrlWrR = crate::BitReader;
#[doc = "Field `mss_ctrl_wr` writer - 1:1\\]
Raw Status of Interrupt from MSS_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type MssCtrlWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mss_rcm_rd` reader - 2:2\\]
Raw Status of Interrupt from MSS_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type MssRcmRdR = crate::BitReader;
#[doc = "Field `mss_rcm_rd` writer - 2:2\\]
Raw Status of Interrupt from MSS_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type MssRcmRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mss_rcm_wr` reader - 3:3\\]
Raw Status of Interrupt from MSS_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type MssRcmWrR = crate::BitReader;
#[doc = "Field `mss_rcm_wr` writer - 3:3\\]
Raw Status of Interrupt from MSS_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type MssRcmWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `top_ctrl_rd` reader - 4:4\\]
Raw Status of Interrupt from TOP_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type TopCtrlRdR = crate::BitReader;
#[doc = "Field `top_ctrl_rd` writer - 4:4\\]
Raw Status of Interrupt from TOP_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type TopCtrlRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `top_ctrl_wr` reader - 5:5\\]
Raw Status of Interrupt from TOP_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type TopCtrlWrR = crate::BitReader;
#[doc = "Field `top_ctrl_wr` writer - 5:5\\]
Raw Status of Interrupt from TOP_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type TopCtrlWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `top_rcm_rd` reader - 6:6\\]
Raw Status of Interrupt from TOP_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type TopRcmRdR = crate::BitReader;
#[doc = "Field `top_rcm_rd` writer - 6:6\\]
Raw Status of Interrupt from TOP_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type TopRcmRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `top_rcm_wr` reader - 7:7\\]
Raw Status of Interrupt from TOP_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type TopRcmWrR = crate::BitReader;
#[doc = "Field `top_rcm_wr` writer - 7:7\\]
Raw Status of Interrupt from TOP_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type TopRcmWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `top_aurora_rd` reader - 8:8\\]
RESERVED:Dont Use"]
pub type TopAuroraRdR = crate::BitReader;
#[doc = "Field `top_aurora_rd` writer - 8:8\\]
RESERVED:Dont Use"]
pub type TopAuroraRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `top_aurora_wr` reader - 9:9\\]
RESERVED:Dont Use"]
pub type TopAuroraWrR = crate::BitReader;
#[doc = "Field `top_aurora_wr` writer - 9:9\\]
RESERVED:Dont Use"]
pub type TopAuroraWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hsm_soc_ctrl_rd` reader - 10:10\\]
Raw Status of Interrupt from HSM_SOC_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type HsmSocCtrlRdR = crate::BitReader;
#[doc = "Field `hsm_soc_ctrl_rd` writer - 10:10\\]
Raw Status of Interrupt from HSM_SOC_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type HsmSocCtrlRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hsm_soc_ctrl_wr` reader - 11:11\\]
Raw Status of Interrupt from HSM_SOC_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type HsmSocCtrlWrR = crate::BitReader;
#[doc = "Field `hsm_soc_ctrl_wr` writer - 11:11\\]
Raw Status of Interrupt from HSM_SOC_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type HsmSocCtrlWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hsm_ctrl_rd` reader - 12:12\\]
Raw Status of Interrupt from HSM_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type HsmCtrlRdR = crate::BitReader;
#[doc = "Field `hsm_ctrl_rd` writer - 12:12\\]
Raw Status of Interrupt from HSM_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type HsmCtrlRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hsm_ctrl_wr` reader - 13:13\\]
Raw Status of Interrupt from HSM_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type HsmCtrlWrR = crate::BitReader;
#[doc = "Field `hsm_ctrl_wr` writer - 13:13\\]
Raw Status of Interrupt from HSM_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type HsmCtrlWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dss_ctrl_rd` reader - 14:14\\]
Raw Status of Interrupt from DSS_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type DssCtrlRdR = crate::BitReader;
#[doc = "Field `dss_ctrl_rd` writer - 14:14\\]
Raw Status of Interrupt from DSS_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type DssCtrlRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dss_ctrl_wr` reader - 15:15\\]
Raw Status of Interrupt from DSS_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type DssCtrlWrR = crate::BitReader;
#[doc = "Field `dss_ctrl_wr` writer - 15:15\\]
Raw Status of Interrupt from DSS_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type DssCtrlWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dss_rcm_rd` reader - 16:16\\]
Raw Status of Interrupt from DSS_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type DssRcmRdR = crate::BitReader;
#[doc = "Field `dss_rcm_rd` writer - 16:16\\]
Raw Status of Interrupt from DSS_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type DssRcmRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dss_rcm_wr` reader - 17:17\\]
Raw Status of Interrupt from DSS_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type DssRcmWrR = crate::BitReader;
#[doc = "Field `dss_rcm_wr` writer - 17:17\\]
Raw Status of Interrupt from DSS_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type DssRcmWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dss_cm4_ctrl_rd` reader - 18:18\\]
RESERVED:Dont Use"]
pub type DssCm4CtrlRdR = crate::BitReader;
#[doc = "Field `dss_cm4_ctrl_rd` writer - 18:18\\]
RESERVED:Dont Use"]
pub type DssCm4CtrlRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dss_cm4_ctrl_wr` reader - 19:19\\]
RESERVED:Dont Use"]
pub type DssCm4CtrlWrR = crate::BitReader;
#[doc = "Field `dss_cm4_ctrl_wr` writer - 19:19\\]
RESERVED:Dont Use"]
pub type DssCm4CtrlWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hwa_cfg_rd` reader - 20:20\\]
Raw Status of Interrupt from HWA_CFG. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type HwaCfgRdR = crate::BitReader;
#[doc = "Field `hwa_cfg_rd` writer - 20:20\\]
Raw Status of Interrupt from HWA_CFG. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type HwaCfgRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hwa_cfg_wr` reader - 21:21\\]
Raw Status of Interrupt from HWA_CFG. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type HwaCfgWrR = crate::BitReader;
#[doc = "Field `hwa_cfg_wr` writer - 21:21\\]
Raw Status of Interrupt from HWA_CFG. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type HwaCfgWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rcss_ctrl_rd` reader - 22:22\\]
Raw Status of Interrupt from RSS_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type RcssCtrlRdR = crate::BitReader;
#[doc = "Field `rcss_ctrl_rd` writer - 22:22\\]
Raw Status of Interrupt from RSS_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type RcssCtrlRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rcss_ctrl_wr` reader - 23:23\\]
Raw Status of Interrupt from RSS_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type RcssCtrlWrR = crate::BitReader;
#[doc = "Field `rcss_ctrl_wr` writer - 23:23\\]
Raw Status of Interrupt from RSS_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type RcssCtrlWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rcss_rcm_rd` reader - 24:24\\]
Raw Status of Interrupt from RSS_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type RcssRcmRdR = crate::BitReader;
#[doc = "Field `rcss_rcm_rd` writer - 24:24\\]
Raw Status of Interrupt from RSS_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type RcssRcmRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rcss_rcm_wr` reader - 25:25\\]
Raw Status of Interrupt from RSS_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type RcssRcmWrR = crate::BitReader;
#[doc = "Field `rcss_rcm_wr` writer - 25:25\\]
Raw Status of Interrupt from RSS_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
pub type RcssRcmWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `top_mdo_rd` reader - 26:26\\]
RESERVED:Dont Use"]
pub type TopMdoRdR = crate::BitReader;
#[doc = "Field `top_mdo_rd` writer - 26:26\\]
RESERVED:Dont Use"]
pub type TopMdoRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `top_mdo_wr` reader - 27:27\\]
RESERVED:Dont Use"]
pub type TopMdoWrR = crate::BitReader;
#[doc = "Field `top_mdo_wr` writer - 27:27\\]
RESERVED:Dont Use"]
pub type TopMdoWrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Raw Status of Interrupt from MSS_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    pub fn mss_ctrl_rd(&self) -> MssCtrlRdR {
        MssCtrlRdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Raw Status of Interrupt from MSS_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    pub fn mss_ctrl_wr(&self) -> MssCtrlWrR {
        MssCtrlWrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Raw Status of Interrupt from MSS_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    pub fn mss_rcm_rd(&self) -> MssRcmRdR {
        MssRcmRdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Raw Status of Interrupt from MSS_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    pub fn mss_rcm_wr(&self) -> MssRcmWrR {
        MssRcmWrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Raw Status of Interrupt from TOP_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    pub fn top_ctrl_rd(&self) -> TopCtrlRdR {
        TopCtrlRdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Raw Status of Interrupt from TOP_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    pub fn top_ctrl_wr(&self) -> TopCtrlWrR {
        TopCtrlWrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Raw Status of Interrupt from TOP_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    pub fn top_rcm_rd(&self) -> TopRcmRdR {
        TopRcmRdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Raw Status of Interrupt from TOP_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    pub fn top_rcm_wr(&self) -> TopRcmWrR {
        TopRcmWrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
RESERVED:Dont Use"]
    #[inline(always)]
    pub fn top_aurora_rd(&self) -> TopAuroraRdR {
        TopAuroraRdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
RESERVED:Dont Use"]
    #[inline(always)]
    pub fn top_aurora_wr(&self) -> TopAuroraWrR {
        TopAuroraWrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Raw Status of Interrupt from HSM_SOC_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    pub fn hsm_soc_ctrl_rd(&self) -> HsmSocCtrlRdR {
        HsmSocCtrlRdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Raw Status of Interrupt from HSM_SOC_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    pub fn hsm_soc_ctrl_wr(&self) -> HsmSocCtrlWrR {
        HsmSocCtrlWrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Raw Status of Interrupt from HSM_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    pub fn hsm_ctrl_rd(&self) -> HsmCtrlRdR {
        HsmCtrlRdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Raw Status of Interrupt from HSM_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    pub fn hsm_ctrl_wr(&self) -> HsmCtrlWrR {
        HsmCtrlWrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Raw Status of Interrupt from DSS_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    pub fn dss_ctrl_rd(&self) -> DssCtrlRdR {
        DssCtrlRdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Raw Status of Interrupt from DSS_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    pub fn dss_ctrl_wr(&self) -> DssCtrlWrR {
        DssCtrlWrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Raw Status of Interrupt from DSS_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    pub fn dss_rcm_rd(&self) -> DssRcmRdR {
        DssRcmRdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Raw Status of Interrupt from DSS_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    pub fn dss_rcm_wr(&self) -> DssRcmWrR {
        DssRcmWrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
RESERVED:Dont Use"]
    #[inline(always)]
    pub fn dss_cm4_ctrl_rd(&self) -> DssCm4CtrlRdR {
        DssCm4CtrlRdR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
RESERVED:Dont Use"]
    #[inline(always)]
    pub fn dss_cm4_ctrl_wr(&self) -> DssCm4CtrlWrR {
        DssCm4CtrlWrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Raw Status of Interrupt from HWA_CFG. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    pub fn hwa_cfg_rd(&self) -> HwaCfgRdR {
        HwaCfgRdR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Raw Status of Interrupt from HWA_CFG. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    pub fn hwa_cfg_wr(&self) -> HwaCfgWrR {
        HwaCfgWrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Raw Status of Interrupt from RSS_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    pub fn rcss_ctrl_rd(&self) -> RcssCtrlRdR {
        RcssCtrlRdR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Raw Status of Interrupt from RSS_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    pub fn rcss_ctrl_wr(&self) -> RcssCtrlWrR {
        RcssCtrlWrR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Raw Status of Interrupt from RSS_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    pub fn rcss_rcm_rd(&self) -> RcssRcmRdR {
        RcssRcmRdR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Raw Status of Interrupt from RSS_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    pub fn rcss_rcm_wr(&self) -> RcssRcmWrR {
        RcssRcmWrR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
RESERVED:Dont Use"]
    #[inline(always)]
    pub fn top_mdo_rd(&self) -> TopMdoRdR {
        TopMdoRdR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
RESERVED:Dont Use"]
    #[inline(always)]
    pub fn top_mdo_wr(&self) -> TopMdoWrR {
        TopMdoWrR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Raw Status of Interrupt from MSS_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    #[must_use]
    pub fn mss_ctrl_rd(&mut self) -> MssCtrlRdW<MssPeriphErraggStatusRaw0Spec> {
        MssCtrlRdW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Raw Status of Interrupt from MSS_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    #[must_use]
    pub fn mss_ctrl_wr(&mut self) -> MssCtrlWrW<MssPeriphErraggStatusRaw0Spec> {
        MssCtrlWrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Raw Status of Interrupt from MSS_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    #[must_use]
    pub fn mss_rcm_rd(&mut self) -> MssRcmRdW<MssPeriphErraggStatusRaw0Spec> {
        MssRcmRdW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Raw Status of Interrupt from MSS_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    #[must_use]
    pub fn mss_rcm_wr(&mut self) -> MssRcmWrW<MssPeriphErraggStatusRaw0Spec> {
        MssRcmWrW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Raw Status of Interrupt from TOP_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    #[must_use]
    pub fn top_ctrl_rd(&mut self) -> TopCtrlRdW<MssPeriphErraggStatusRaw0Spec> {
        TopCtrlRdW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Raw Status of Interrupt from TOP_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    #[must_use]
    pub fn top_ctrl_wr(&mut self) -> TopCtrlWrW<MssPeriphErraggStatusRaw0Spec> {
        TopCtrlWrW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Raw Status of Interrupt from TOP_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    #[must_use]
    pub fn top_rcm_rd(&mut self) -> TopRcmRdW<MssPeriphErraggStatusRaw0Spec> {
        TopRcmRdW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Raw Status of Interrupt from TOP_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    #[must_use]
    pub fn top_rcm_wr(&mut self) -> TopRcmWrW<MssPeriphErraggStatusRaw0Spec> {
        TopRcmWrW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
RESERVED:Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn top_aurora_rd(&mut self) -> TopAuroraRdW<MssPeriphErraggStatusRaw0Spec> {
        TopAuroraRdW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
RESERVED:Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn top_aurora_wr(&mut self) -> TopAuroraWrW<MssPeriphErraggStatusRaw0Spec> {
        TopAuroraWrW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Raw Status of Interrupt from HSM_SOC_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    #[must_use]
    pub fn hsm_soc_ctrl_rd(&mut self) -> HsmSocCtrlRdW<MssPeriphErraggStatusRaw0Spec> {
        HsmSocCtrlRdW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Raw Status of Interrupt from HSM_SOC_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    #[must_use]
    pub fn hsm_soc_ctrl_wr(&mut self) -> HsmSocCtrlWrW<MssPeriphErraggStatusRaw0Spec> {
        HsmSocCtrlWrW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Raw Status of Interrupt from HSM_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    #[must_use]
    pub fn hsm_ctrl_rd(&mut self) -> HsmCtrlRdW<MssPeriphErraggStatusRaw0Spec> {
        HsmCtrlRdW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Raw Status of Interrupt from HSM_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    #[must_use]
    pub fn hsm_ctrl_wr(&mut self) -> HsmCtrlWrW<MssPeriphErraggStatusRaw0Spec> {
        HsmCtrlWrW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Raw Status of Interrupt from DSS_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    #[must_use]
    pub fn dss_ctrl_rd(&mut self) -> DssCtrlRdW<MssPeriphErraggStatusRaw0Spec> {
        DssCtrlRdW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Raw Status of Interrupt from DSS_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    #[must_use]
    pub fn dss_ctrl_wr(&mut self) -> DssCtrlWrW<MssPeriphErraggStatusRaw0Spec> {
        DssCtrlWrW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Raw Status of Interrupt from DSS_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    #[must_use]
    pub fn dss_rcm_rd(&mut self) -> DssRcmRdW<MssPeriphErraggStatusRaw0Spec> {
        DssRcmRdW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Raw Status of Interrupt from DSS_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    #[must_use]
    pub fn dss_rcm_wr(&mut self) -> DssRcmWrW<MssPeriphErraggStatusRaw0Spec> {
        DssRcmWrW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
RESERVED:Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn dss_cm4_ctrl_rd(&mut self) -> DssCm4CtrlRdW<MssPeriphErraggStatusRaw0Spec> {
        DssCm4CtrlRdW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
RESERVED:Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn dss_cm4_ctrl_wr(&mut self) -> DssCm4CtrlWrW<MssPeriphErraggStatusRaw0Spec> {
        DssCm4CtrlWrW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Raw Status of Interrupt from HWA_CFG. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_cfg_rd(&mut self) -> HwaCfgRdW<MssPeriphErraggStatusRaw0Spec> {
        HwaCfgRdW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Raw Status of Interrupt from HWA_CFG. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_cfg_wr(&mut self) -> HwaCfgWrW<MssPeriphErraggStatusRaw0Spec> {
        HwaCfgWrW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Raw Status of Interrupt from RSS_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    #[must_use]
    pub fn rcss_ctrl_rd(&mut self) -> RcssCtrlRdW<MssPeriphErraggStatusRaw0Spec> {
        RcssCtrlRdW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Raw Status of Interrupt from RSS_CTRL. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    #[must_use]
    pub fn rcss_ctrl_wr(&mut self) -> RcssCtrlWrW<MssPeriphErraggStatusRaw0Spec> {
        RcssCtrlWrW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Raw Status of Interrupt from RSS_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    #[must_use]
    pub fn rcss_rcm_rd(&mut self) -> RcssRcmRdW<MssPeriphErraggStatusRaw0Spec> {
        RcssRcmRdW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Raw Status of Interrupt from RSS_RCM. Set irrespective if the Interupt is masked or unmasked in MSS_PERIPH_ERRAGG_MASK0"]
    #[inline(always)]
    #[must_use]
    pub fn rcss_rcm_wr(&mut self) -> RcssRcmWrW<MssPeriphErraggStatusRaw0Spec> {
        RcssRcmWrW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
RESERVED:Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn top_mdo_rd(&mut self) -> TopMdoRdW<MssPeriphErraggStatusRaw0Spec> {
        TopMdoRdW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
RESERVED:Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn top_mdo_wr(&mut self) -> TopMdoWrW<MssPeriphErraggStatusRaw0Spec> {
        TopMdoWrW::new(self, 27)
    }
}
#[doc = "MSS_PERIPH_ERRAGG_STATUS_RAW0\n\nYou can [`read`](crate::Reg::read) this register and get [`mss_periph_erragg_status_raw0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mss_periph_erragg_status_raw0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MssPeriphErraggStatusRaw0Spec;
impl crate::RegisterSpec for MssPeriphErraggStatusRaw0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss_periph_erragg_status_raw0::R`](R) reader structure"]
impl crate::Readable for MssPeriphErraggStatusRaw0Spec {}
#[doc = "`write(|w| ..)` method takes [`mss_periph_erragg_status_raw0::W`](W) writer structure"]
impl crate::Writable for MssPeriphErraggStatusRaw0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSS_PERIPH_ERRAGG_STATUS_RAW0 to value 0"]
impl crate::Resettable for MssPeriphErraggStatusRaw0Spec {
    const RESET_VALUE: u32 = 0;
}
