#[doc = "Register `ANA_REG_WU_TMUX_CTRL_LOWV` reader"]
pub type R = crate::R<AnaRegWuTmuxCtrlLowvSpec>;
#[doc = "Register `ANA_REG_WU_TMUX_CTRL_LOWV` writer"]
pub type W = crate::W<AnaRegWuTmuxCtrlLowvSpec>;
#[doc = "Field `SCALED_VDDA_OSC_DIV3` reader - 0:0\\]
Test Mux Control. One Hot Control Scaling Factor: VDDA_OSC/3 0x0 = Functional Reset"]
pub type ScaledVddaOscDiv3R = crate::BitReader;
#[doc = "Field `SCALED_VDDA_OSC_DIV3` writer - 0:0\\]
Test Mux Control. One Hot Control Scaling Factor: VDDA_OSC/3 0x0 = Functional Reset"]
pub type ScaledVddaOscDiv3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCALED_VIO3318` reader - 1:1\\]
Test Mux Control. One Hot Control Scaling Factor: VIOIN/5.5 (3.3V mode), VIOIN/3 (1.8V mode) 0x0 = Functional Reset"]
pub type ScaledVio3318R = crate::BitReader;
#[doc = "Field `SCALED_VIO3318` writer - 1:1\\]
Test Mux Control. One Hot Control Scaling Factor: VIOIN/5.5 (3.3V mode), VIOIN/3 (1.8V mode) 0x0 = Functional Reset"]
pub type ScaledVio3318W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCALED_VDDA18BB` reader - 2:2\\]
Test Mux Control. One Hot Control Scaling Factor: VDDA18BB/3 0x0 = Functional Reset"]
pub type ScaledVdda18bbR = crate::BitReader;
#[doc = "Field `SCALED_VDDA18BB` writer - 2:2\\]
Test Mux Control. One Hot Control Scaling Factor: VDDA18BB/3 0x0 = Functional Reset"]
pub type ScaledVdda18bbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VFB_0P6V` reader - 3:3\\]
Test Mux Control. One Hot Control Scaling Factor: 0.5 * VDD 0x0 = Functional Reset"]
pub type Vfb0p6vR = crate::BitReader;
#[doc = "Field `VFB_0P6V` writer - 3:3\\]
Test Mux Control. One Hot Control Scaling Factor: 0.5 * VDD 0x0 = Functional Reset"]
pub type Vfb0p6vW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCALED_VDDA18VCO` reader - 4:4\\]
Test Mux Control. One Hot Control Scaling Factor: VDDA18VCO/3 0x0 = Functional Reset"]
pub type ScaledVdda18vcoR = crate::BitReader;
#[doc = "Field `SCALED_VDDA18VCO` writer - 4:4\\]
Test Mux Control. One Hot Control Scaling Factor: VDDA18VCO/3 0x0 = Functional Reset"]
pub type ScaledVdda18vcoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCALED_VDDA10RF1` reader - 5:5\\]
Test Mux Control. One Hot Control Scaling Factor: VDDA10RF1/2 0x0 = Functional Reset"]
pub type ScaledVdda10rf1R = crate::BitReader;
#[doc = "Field `SCALED_VDDA10RF1` writer - 5:5\\]
Test Mux Control. One Hot Control Scaling Factor: VDDA10RF1/2 0x0 = Functional Reset"]
pub type ScaledVdda10rf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCALED_VDDA10RF2` reader - 6:6\\]
Test Mux Control. One Hot Control Scaling Factor: VDDA10RF2/2 0x0 = Functional Reset"]
pub type ScaledVdda10rf2R = crate::BitReader;
#[doc = "Field `SCALED_VDDA10RF2` writer - 6:6\\]
Test Mux Control. One Hot Control Scaling Factor: VDDA10RF2/2 0x0 = Functional Reset"]
pub type ScaledVdda10rf2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREF_0P9V` reader - 7:7\\]
Test Mux Control. One Hot Control VREF_0P9V 0x0 = Functional Reset"]
pub type Vref0p9vR = crate::BitReader;
#[doc = "Field `VREF_0P9V` writer - 7:7\\]
Test Mux Control. One Hot Control VREF_0P9V 0x0 = Functional Reset"]
pub type Vref0p9vW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDA_OSC_UV_VREF` reader - 8:8\\]
Test Mux Control. One Hot Control Vref for VDDA_OSC_UV VMON 0x0 = Functional Reset"]
pub type VddaOscUvVrefR = crate::BitReader;
#[doc = "Field `VDDA_OSC_UV_VREF` writer - 8:8\\]
Test Mux Control. One Hot Control Vref for VDDA_OSC_UV VMON 0x0 = Functional Reset"]
pub type VddaOscUvVrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VIOIN33_UV_VREF` reader - 9:9\\]
Test Mux Control. One Hot Control Vref for VIOIN33_UV VMON 0x0 = Functional Reset"]
pub type Vioin33UvVrefR = crate::BitReader;
#[doc = "Field `VIOIN33_UV_VREF` writer - 9:9\\]
Test Mux Control. One Hot Control Vref for VIOIN33_UV VMON 0x0 = Functional Reset"]
pub type Vioin33UvVrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDA18BB_UV_VREF` reader - 10:10\\]
Test Mux Control. One Hot Control Vref for VDDA18BB_UV VMON 0x0 = Functional Reset"]
pub type Vdda18bbUvVrefR = crate::BitReader;
#[doc = "Field `VDDA18BB_UV_VREF` writer - 10:10\\]
Test Mux Control. One Hot Control Vref for VDDA18BB_UV VMON 0x0 = Functional Reset"]
pub type Vdda18bbUvVrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDD_OV_VREF` reader - 11:11\\]
Test Mux Control. One Hot Control Vref for VDD_OV VMON 0x0 = Functional Reset"]
pub type VddOvVrefR = crate::BitReader;
#[doc = "Field `VDD_OV_VREF` writer - 11:11\\]
Test Mux Control. One Hot Control Vref for VDD_OV VMON 0x0 = Functional Reset"]
pub type VddOvVrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDD_UV_VREF` reader - 12:12\\]
Test Mux Control. One Hot Control Vref for VDD_UV VMON 0x0 = Functional Reset"]
pub type VddUvVrefR = crate::BitReader;
#[doc = "Field `VDD_UV_VREF` writer - 12:12\\]
Test Mux Control. One Hot Control Vref for VDD_UV VMON 0x0 = Functional Reset"]
pub type VddUvVrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDA18VCO_UV_VREF` reader - 13:13\\]
Test Mux Control. One Hot Control Vref for VDDA18VCO_UV VMON 0x0 = Functional Reset"]
pub type Vdda18vcoUvVrefR = crate::BitReader;
#[doc = "Field `VDDA18VCO_UV_VREF` writer - 13:13\\]
Test Mux Control. One Hot Control Vref for VDDA18VCO_UV VMON 0x0 = Functional Reset"]
pub type Vdda18vcoUvVrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDA10RF1_UV_VREF` reader - 14:14\\]
Test Mux Control. One Hot Control Vref for VDDA10RF1_UV VMON 0x0 = Functional Reset"]
pub type Vdda10rf1UvVrefR = crate::BitReader;
#[doc = "Field `VDDA10RF1_UV_VREF` writer - 14:14\\]
Test Mux Control. One Hot Control Vref for VDDA10RF1_UV VMON 0x0 = Functional Reset"]
pub type Vdda10rf1UvVrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDA10RF2_UV_VREF` reader - 15:15\\]
Test Mux Control. One Hot Control Vref for VDDA10RF2_UV VMON 0x0 = Functional Reset"]
pub type Vdda10rf2UvVrefR = crate::BitReader;
#[doc = "Field `VDDA10RF2_UV_VREF` writer - 15:15\\]
Test Mux Control. One Hot Control Vref for VDDA10RF2_UV VMON 0x0 = Functional Reset"]
pub type Vdda10rf2UvVrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VT_ANA_SIG` reader - 16:16\\]
Test Mux Control. One Hot Control VT Detect Signal Level on VDDA_LVDS_1P8V 0x0 = Functional Reset"]
pub type VtAnaSigR = crate::BitReader;
#[doc = "Field `VT_ANA_SIG` writer - 16:16\\]
Test Mux Control. One Hot Control VT Detect Signal Level on VDDA_LVDS_1P8V 0x0 = Functional Reset"]
pub type VtAnaSigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VT_DIG_SIG_UV` reader - 17:17\\]
Test Mux Control. One Hot Control VT Detect Signal Level on VDD12 Crude UV VMON 0x0 = Functional Reset"]
pub type VtDigSigUvR = crate::BitReader;
#[doc = "Field `VT_DIG_SIG_UV` writer - 17:17\\]
Test Mux Control. One Hot Control VT Detect Signal Level on VDD12 Crude UV VMON 0x0 = Functional Reset"]
pub type VtDigSigUvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VT_DIG_SIG_OV` reader - 18:18\\]
Test Mux Control. One Hot Control VT Detect Signal Level on VDD12 Crude OV VMON 0x0 = Functional Reset"]
pub type VtDigSigOvR = crate::BitReader;
#[doc = "Field `VT_DIG_SIG_OV` writer - 18:18\\]
Test Mux Control. One Hot Control VT Detect Signal Level on VDD12 Crude OV VMON 0x0 = Functional Reset"]
pub type VtDigSigOvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCALED_VDDA_OSC_DIV22_39` reader - 19:19\\]
Scaled VDDA_OSC supply for crude supply detect Scaling Factor: VDDA_OSC*(22/39) 0x0 = Functional Reset"]
pub type ScaledVddaOscDiv22_39R = crate::BitReader;
#[doc = "Field `SCALED_VDDA_OSC_DIV22_39` writer - 19:19\\]
Scaled VDDA_OSC supply for crude supply detect Scaling Factor: VDDA_OSC*(22/39) 0x0 = Functional Reset"]
pub type ScaledVddaOscDiv22_39W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDSINT18` reader - 20:20\\]
VIOIN scaled supply for VIOIN Detect Scaling Factor: VIOIN*(52/90) 0x0 = Functional Reset"]
pub type Vddsint18R = crate::BitReader;
#[doc = "Field `VDDSINT18` writer - 20:20\\]
VIOIN scaled supply for VIOIN Detect Scaling Factor: VIOIN*(52/90) 0x0 = Functional Reset"]
pub type Vddsint18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VFB_0P85V` reader - 21:21\\]
Scaled VDD 1.2V used as reference for VDDA_OSC crude supply detect 0x0 = Functional Reset"]
pub type Vfb0p85vR = crate::BitReader;
#[doc = "Field `VFB_0P85V` writer - 21:21\\]
Scaled VDD 1.2V used as reference for VDDA_OSC crude supply detect 0x0 = Functional Reset"]
pub type Vfb0p85vW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCALED_VDDA_LVDS_1P8V_1P2` reader - 22:22\\]
Test Mux Control. One Hot Control Change in TPR VDDS_3P3V IO DET reference (1.8V mode) Scaling Factor: 0.67 * VDDA_LVDS_1P8V 0x0 = Functional Reset"]
pub type ScaledVddaLvds1p8v1p2R = crate::BitReader;
#[doc = "Field `SCALED_VDDA_LVDS_1P8V_1P2` writer - 22:22\\]
Test Mux Control. One Hot Control Change in TPR VDDS_3P3V IO DET reference (1.8V mode) Scaling Factor: 0.67 * VDDA_LVDS_1P8V 0x0 = Functional Reset"]
pub type ScaledVddaLvds1p8v1p2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCALED_VDDA_LVDS_1P8V` reader - 23:23\\]
Test Mux Control. One Hot Control Change in TPR VDDS_3P3V IO DET reference (3.3V mode) Scaling Factor: 0.4 * VDDA_LVDS_1P8V 0x0 = Functional Reset"]
pub type ScaledVddaLvds1p8vR = crate::BitReader;
#[doc = "Field `SCALED_VDDA_LVDS_1P8V` writer - 23:23\\]
Test Mux Control. One Hot Control Change in TPR VDDS_3P3V IO DET reference (3.3V mode) Scaling Factor: 0.4 * VDDA_LVDS_1P8V 0x0 = Functional Reset"]
pub type ScaledVddaLvds1p8vW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCALED_VDDS33` reader - 24:24\\]
Test Mux Control. One Hot Control Scaling Factor: 0.289 * VDDS33 0x0 = Functional Reset"]
pub type ScaledVdds33R = crate::BitReader;
#[doc = "Field `SCALED_VDDS33` writer - 24:24\\]
Test Mux Control. One Hot Control Scaling Factor: 0.289 * VDDS33 0x0 = Functional Reset"]
pub type ScaledVdds33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED0` reader - 30:25\\]
Reserved Reads return 0x0 and writes have no effect. 0x00 = Functional Reset"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - 30:25\\]
Reserved Reads return 0x0 and writes have no effect. 0x00 = Functional Reset"]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WU_TMUX_EN` reader - 31:31\\]
WU TMUX Enable 0 = TMUX Disabled 1 = TMUX Enabled 0x0 = Functional Reset"]
pub type WuTmuxEnR = crate::BitReader;
#[doc = "Field `WU_TMUX_EN` writer - 31:31\\]
WU TMUX Enable 0 = TMUX Disabled 1 = TMUX Enabled 0x0 = Functional Reset"]
pub type WuTmuxEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Test Mux Control. One Hot Control Scaling Factor: VDDA_OSC/3 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn scaled_vdda_osc_div3(&self) -> ScaledVddaOscDiv3R {
        ScaledVddaOscDiv3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Test Mux Control. One Hot Control Scaling Factor: VIOIN/5.5 (3.3V mode), VIOIN/3 (1.8V mode) 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn scaled_vio3318(&self) -> ScaledVio3318R {
        ScaledVio3318R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Test Mux Control. One Hot Control Scaling Factor: VDDA18BB/3 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn scaled_vdda18bb(&self) -> ScaledVdda18bbR {
        ScaledVdda18bbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Test Mux Control. One Hot Control Scaling Factor: 0.5 * VDD 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vfb_0p6v(&self) -> Vfb0p6vR {
        Vfb0p6vR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Test Mux Control. One Hot Control Scaling Factor: VDDA18VCO/3 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn scaled_vdda18vco(&self) -> ScaledVdda18vcoR {
        ScaledVdda18vcoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Test Mux Control. One Hot Control Scaling Factor: VDDA10RF1/2 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn scaled_vdda10rf1(&self) -> ScaledVdda10rf1R {
        ScaledVdda10rf1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Test Mux Control. One Hot Control Scaling Factor: VDDA10RF2/2 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn scaled_vdda10rf2(&self) -> ScaledVdda10rf2R {
        ScaledVdda10rf2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Test Mux Control. One Hot Control VREF_0P9V 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vref_0p9v(&self) -> Vref0p9vR {
        Vref0p9vR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Test Mux Control. One Hot Control Vref for VDDA_OSC_UV VMON 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vdda_osc_uv_vref(&self) -> VddaOscUvVrefR {
        VddaOscUvVrefR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Test Mux Control. One Hot Control Vref for VIOIN33_UV VMON 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vioin33_uv_vref(&self) -> Vioin33UvVrefR {
        Vioin33UvVrefR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Test Mux Control. One Hot Control Vref for VDDA18BB_UV VMON 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vdda18bb_uv_vref(&self) -> Vdda18bbUvVrefR {
        Vdda18bbUvVrefR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Test Mux Control. One Hot Control Vref for VDD_OV VMON 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vdd_ov_vref(&self) -> VddOvVrefR {
        VddOvVrefR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Test Mux Control. One Hot Control Vref for VDD_UV VMON 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vdd_uv_vref(&self) -> VddUvVrefR {
        VddUvVrefR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Test Mux Control. One Hot Control Vref for VDDA18VCO_UV VMON 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vdda18vco_uv_vref(&self) -> Vdda18vcoUvVrefR {
        Vdda18vcoUvVrefR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Test Mux Control. One Hot Control Vref for VDDA10RF1_UV VMON 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vdda10rf1_uv_vref(&self) -> Vdda10rf1UvVrefR {
        Vdda10rf1UvVrefR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Test Mux Control. One Hot Control Vref for VDDA10RF2_UV VMON 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vdda10rf2_uv_vref(&self) -> Vdda10rf2UvVrefR {
        Vdda10rf2UvVrefR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Test Mux Control. One Hot Control VT Detect Signal Level on VDDA_LVDS_1P8V 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vt_ana_sig(&self) -> VtAnaSigR {
        VtAnaSigR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Test Mux Control. One Hot Control VT Detect Signal Level on VDD12 Crude UV VMON 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vt_dig_sig_uv(&self) -> VtDigSigUvR {
        VtDigSigUvR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Test Mux Control. One Hot Control VT Detect Signal Level on VDD12 Crude OV VMON 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vt_dig_sig_ov(&self) -> VtDigSigOvR {
        VtDigSigOvR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Scaled VDDA_OSC supply for crude supply detect Scaling Factor: VDDA_OSC*(22/39) 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn scaled_vdda_osc_div22_39(&self) -> ScaledVddaOscDiv22_39R {
        ScaledVddaOscDiv22_39R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
VIOIN scaled supply for VIOIN Detect Scaling Factor: VIOIN*(52/90) 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vddsint18(&self) -> Vddsint18R {
        Vddsint18R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Scaled VDD 1.2V used as reference for VDDA_OSC crude supply detect 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vfb_0p85v(&self) -> Vfb0p85vR {
        Vfb0p85vR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Test Mux Control. One Hot Control Change in TPR VDDS_3P3V IO DET reference (1.8V mode) Scaling Factor: 0.67 * VDDA_LVDS_1P8V 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn scaled_vdda_lvds_1p8v_1p2(&self) -> ScaledVddaLvds1p8v1p2R {
        ScaledVddaLvds1p8v1p2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Test Mux Control. One Hot Control Change in TPR VDDS_3P3V IO DET reference (3.3V mode) Scaling Factor: 0.4 * VDDA_LVDS_1P8V 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn scaled_vdda_lvds_1p8v(&self) -> ScaledVddaLvds1p8vR {
        ScaledVddaLvds1p8vR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Test Mux Control. One Hot Control Scaling Factor: 0.289 * VDDS33 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn scaled_vdds33(&self) -> ScaledVdds33R {
        ScaledVdds33R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:30 - 30:25\\]
Reserved Reads return 0x0 and writes have no effect. 0x00 = Functional Reset"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 25) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
WU TMUX Enable 0 = TMUX Disabled 1 = TMUX Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn wu_tmux_en(&self) -> WuTmuxEnR {
        WuTmuxEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Test Mux Control. One Hot Control Scaling Factor: VDDA_OSC/3 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn scaled_vdda_osc_div3(&mut self) -> ScaledVddaOscDiv3W<AnaRegWuTmuxCtrlLowvSpec> {
        ScaledVddaOscDiv3W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Test Mux Control. One Hot Control Scaling Factor: VIOIN/5.5 (3.3V mode), VIOIN/3 (1.8V mode) 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn scaled_vio3318(&mut self) -> ScaledVio3318W<AnaRegWuTmuxCtrlLowvSpec> {
        ScaledVio3318W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Test Mux Control. One Hot Control Scaling Factor: VDDA18BB/3 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn scaled_vdda18bb(&mut self) -> ScaledVdda18bbW<AnaRegWuTmuxCtrlLowvSpec> {
        ScaledVdda18bbW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Test Mux Control. One Hot Control Scaling Factor: 0.5 * VDD 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vfb_0p6v(&mut self) -> Vfb0p6vW<AnaRegWuTmuxCtrlLowvSpec> {
        Vfb0p6vW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Test Mux Control. One Hot Control Scaling Factor: VDDA18VCO/3 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn scaled_vdda18vco(&mut self) -> ScaledVdda18vcoW<AnaRegWuTmuxCtrlLowvSpec> {
        ScaledVdda18vcoW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Test Mux Control. One Hot Control Scaling Factor: VDDA10RF1/2 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn scaled_vdda10rf1(&mut self) -> ScaledVdda10rf1W<AnaRegWuTmuxCtrlLowvSpec> {
        ScaledVdda10rf1W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Test Mux Control. One Hot Control Scaling Factor: VDDA10RF2/2 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn scaled_vdda10rf2(&mut self) -> ScaledVdda10rf2W<AnaRegWuTmuxCtrlLowvSpec> {
        ScaledVdda10rf2W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Test Mux Control. One Hot Control VREF_0P9V 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vref_0p9v(&mut self) -> Vref0p9vW<AnaRegWuTmuxCtrlLowvSpec> {
        Vref0p9vW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Test Mux Control. One Hot Control Vref for VDDA_OSC_UV VMON 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vdda_osc_uv_vref(&mut self) -> VddaOscUvVrefW<AnaRegWuTmuxCtrlLowvSpec> {
        VddaOscUvVrefW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Test Mux Control. One Hot Control Vref for VIOIN33_UV VMON 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vioin33_uv_vref(&mut self) -> Vioin33UvVrefW<AnaRegWuTmuxCtrlLowvSpec> {
        Vioin33UvVrefW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Test Mux Control. One Hot Control Vref for VDDA18BB_UV VMON 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vdda18bb_uv_vref(&mut self) -> Vdda18bbUvVrefW<AnaRegWuTmuxCtrlLowvSpec> {
        Vdda18bbUvVrefW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Test Mux Control. One Hot Control Vref for VDD_OV VMON 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_ov_vref(&mut self) -> VddOvVrefW<AnaRegWuTmuxCtrlLowvSpec> {
        VddOvVrefW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Test Mux Control. One Hot Control Vref for VDD_UV VMON 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vdd_uv_vref(&mut self) -> VddUvVrefW<AnaRegWuTmuxCtrlLowvSpec> {
        VddUvVrefW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Test Mux Control. One Hot Control Vref for VDDA18VCO_UV VMON 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vdda18vco_uv_vref(&mut self) -> Vdda18vcoUvVrefW<AnaRegWuTmuxCtrlLowvSpec> {
        Vdda18vcoUvVrefW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Test Mux Control. One Hot Control Vref for VDDA10RF1_UV VMON 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vdda10rf1_uv_vref(&mut self) -> Vdda10rf1UvVrefW<AnaRegWuTmuxCtrlLowvSpec> {
        Vdda10rf1UvVrefW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Test Mux Control. One Hot Control Vref for VDDA10RF2_UV VMON 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vdda10rf2_uv_vref(&mut self) -> Vdda10rf2UvVrefW<AnaRegWuTmuxCtrlLowvSpec> {
        Vdda10rf2UvVrefW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Test Mux Control. One Hot Control VT Detect Signal Level on VDDA_LVDS_1P8V 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vt_ana_sig(&mut self) -> VtAnaSigW<AnaRegWuTmuxCtrlLowvSpec> {
        VtAnaSigW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Test Mux Control. One Hot Control VT Detect Signal Level on VDD12 Crude UV VMON 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vt_dig_sig_uv(&mut self) -> VtDigSigUvW<AnaRegWuTmuxCtrlLowvSpec> {
        VtDigSigUvW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Test Mux Control. One Hot Control VT Detect Signal Level on VDD12 Crude OV VMON 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vt_dig_sig_ov(&mut self) -> VtDigSigOvW<AnaRegWuTmuxCtrlLowvSpec> {
        VtDigSigOvW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Scaled VDDA_OSC supply for crude supply detect Scaling Factor: VDDA_OSC*(22/39) 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn scaled_vdda_osc_div22_39(&mut self) -> ScaledVddaOscDiv22_39W<AnaRegWuTmuxCtrlLowvSpec> {
        ScaledVddaOscDiv22_39W::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
VIOIN scaled supply for VIOIN Detect Scaling Factor: VIOIN*(52/90) 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vddsint18(&mut self) -> Vddsint18W<AnaRegWuTmuxCtrlLowvSpec> {
        Vddsint18W::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Scaled VDD 1.2V used as reference for VDDA_OSC crude supply detect 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vfb_0p85v(&mut self) -> Vfb0p85vW<AnaRegWuTmuxCtrlLowvSpec> {
        Vfb0p85vW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Test Mux Control. One Hot Control Change in TPR VDDS_3P3V IO DET reference (1.8V mode) Scaling Factor: 0.67 * VDDA_LVDS_1P8V 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn scaled_vdda_lvds_1p8v_1p2(
        &mut self,
    ) -> ScaledVddaLvds1p8v1p2W<AnaRegWuTmuxCtrlLowvSpec> {
        ScaledVddaLvds1p8v1p2W::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Test Mux Control. One Hot Control Change in TPR VDDS_3P3V IO DET reference (3.3V mode) Scaling Factor: 0.4 * VDDA_LVDS_1P8V 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn scaled_vdda_lvds_1p8v(&mut self) -> ScaledVddaLvds1p8vW<AnaRegWuTmuxCtrlLowvSpec> {
        ScaledVddaLvds1p8vW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Test Mux Control. One Hot Control Scaling Factor: 0.289 * VDDS33 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn scaled_vdds33(&mut self) -> ScaledVdds33W<AnaRegWuTmuxCtrlLowvSpec> {
        ScaledVdds33W::new(self, 24)
    }
    #[doc = "Bits 25:30 - 30:25\\]
Reserved Reads return 0x0 and writes have no effect. 0x00 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<AnaRegWuTmuxCtrlLowvSpec> {
        Reserved0W::new(self, 25)
    }
    #[doc = "Bit 31 - 31:31\\]
WU TMUX Enable 0 = TMUX Disabled 1 = TMUX Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn wu_tmux_en(&mut self) -> WuTmuxEnW<AnaRegWuTmuxCtrlLowvSpec> {
        WuTmuxEnW::new(self, 31)
    }
}
#[doc = "ANA_REG_WU_TMUX_CTRL_LOWV\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_wu_tmux_ctrl_lowv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_wu_tmux_ctrl_lowv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaRegWuTmuxCtrlLowvSpec;
impl crate::RegisterSpec for AnaRegWuTmuxCtrlLowvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_reg_wu_tmux_ctrl_lowv::R`](R) reader structure"]
impl crate::Readable for AnaRegWuTmuxCtrlLowvSpec {}
#[doc = "`write(|w| ..)` method takes [`ana_reg_wu_tmux_ctrl_lowv::W`](W) writer structure"]
impl crate::Writable for AnaRegWuTmuxCtrlLowvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANA_REG_WU_TMUX_CTRL_LOWV to value 0"]
impl crate::Resettable for AnaRegWuTmuxCtrlLowvSpec {
    const RESET_VALUE: u32 = 0;
}
