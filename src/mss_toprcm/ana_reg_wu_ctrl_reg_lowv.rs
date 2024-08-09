#[doc = "Register `ANA_REG_WU_CTRL_REG_LOWV` reader"]
pub type R = crate::R<AnaRegWuCtrlRegLowvSpec>;
#[doc = "Register `ANA_REG_WU_CTRL_REG_LOWV` writer"]
pub type W = crate::W<AnaRegWuCtrlRegLowvSpec>;
#[doc = "Field `INT_CLK_EN` reader - 0:0\\]
WU Internal Clock (RCOSC) ENABLE 0 = Internal CLK Disabled 1 = Internal CLK Enabled 0x1 = Functional Reset"]
pub type IntClkEnR = crate::BitReader;
#[doc = "Field `INT_CLK_EN` writer - 0:0\\]
WU Internal Clock (RCOSC) ENABLE 0 = Internal CLK Disabled 1 = Internal CLK Enabled 0x1 = Functional Reset"]
pub type IntClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_CLK_STOP` reader - 1:1\\]
WU Internal Clock (RCOSC) STOP 0 = Internal CLK can be enabled 1 = Internal CLK is OFF 0x0 = Functional Reset"]
pub type IntClkStopR = crate::BitReader;
#[doc = "Field `INT_CLK_STOP` writer - 1:1\\]
WU Internal Clock (RCOSC) STOP 0 = Internal CLK can be enabled 1 = Internal CLK is OFF 0x0 = Functional Reset"]
pub type IntClkStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_CLK_SW_SEL` reader - 2:2\\]
WU Internal Clock (RCOSC) SW_SEL 0 = TBD 1 = TBD 0x1 = Functional Reset"]
pub type IntClkSwSelR = crate::BitReader;
#[doc = "Field `INT_CLK_SW_SEL` writer - 2:2\\]
WU Internal Clock (RCOSC) SW_SEL 0 = TBD 1 = TBD 0x1 = Functional Reset"]
pub type IntClkSwSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_CLK_TRIM_6_0` reader - 9:3\\]
WU lnternal Clock (RCOSC) Trim 0x4B = Functional Reset (If not trimmed)"]
pub type IntClkTrim6_0R = crate::FieldReader;
#[doc = "Field `INT_CLK_TRIM_6_0` writer - 9:3\\]
WU lnternal Clock (RCOSC) Trim 0x4B = Functional Reset (If not trimmed)"]
pub type IntClkTrim6_0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ECO_SLICER_CLK_DLY_DIS` reader - 10:10\\]
Disable slicer clock delay ECO (mapped to eFuse) 0x0 = Functional Reset"]
pub type EcoSlicerClkDlyDisR = crate::BitReader;
#[doc = "Field `ECO_SLICER_CLK_DLY_DIS` writer - 10:10\\]
Disable slicer clock delay ECO (mapped to eFuse) 0x0 = Functional Reset"]
pub type EcoSlicerClkDlyDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_CLK_FREQ_SEL_3_0` reader - 14:11\\]
WU Internal Clock (RCOSC) Frequency Select Bit&lt;3> is used as override for VMON on Untrimmed devices. Bit &lt;3> is '1' if device REFSYS_TOP is trimmed. Changed on 1642 ES2P0 Change Name : Newly added mux for CLK MON EN options When Bit&lt;2> = 0, MASK_CPU_CLK_OUT_CTRL_LOWV == WU_CTRL_REG&lt;12>, essentially Bit&lt;1> of this field When Bit&lt;2> = 1, MASK_CPU_CLK_OUT_CTRL_LOWV == (original function) INTER_MASK_CPU_CLK_OUT_CTRL_LOWV Change Name : Newly added OR gates to provide options to bypass crude VDD DET (also refer to &lt;23>) Bit &lt;0> of this field when HIGH over rides the crude VDD_DET 0x0 = Functional Reset"]
pub type IntClkFreqSel3_0R = crate::FieldReader;
#[doc = "Field `INT_CLK_FREQ_SEL_3_0` writer - 14:11\\]
WU Internal Clock (RCOSC) Frequency Select Bit&lt;3> is used as override for VMON on Untrimmed devices. Bit &lt;3> is '1' if device REFSYS_TOP is trimmed. Changed on 1642 ES2P0 Change Name : Newly added mux for CLK MON EN options When Bit&lt;2> = 0, MASK_CPU_CLK_OUT_CTRL_LOWV == WU_CTRL_REG&lt;12>, essentially Bit&lt;1> of this field When Bit&lt;2> = 1, MASK_CPU_CLK_OUT_CTRL_LOWV == (original function) INTER_MASK_CPU_CLK_OUT_CTRL_LOWV Change Name : Newly added OR gates to provide options to bypass crude VDD DET (also refer to &lt;23>) Bit &lt;0> of this field when HIGH over rides the crude VDD_DET 0x0 = Functional Reset"]
pub type IntClkFreqSel3_0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WU_CPU_CLK_CTRL` reader - 15:15\\]
WU CLK Control 0 = CLK Monitor Function in Dig Sequencer is disabled 1 = REF CLK Monitor Function is Enabled 0x1 = Functional Reset"]
pub type WuCpuClkCtrlR = crate::BitReader;
#[doc = "Field `WU_CPU_CLK_CTRL` writer - 15:15\\]
WU CLK Control 0 = CLK Monitor Function in Dig Sequencer is disabled 1 = REF CLK Monitor Function is Enabled 0x1 = Functional Reset"]
pub type WuCpuClkCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_EN_OVERRIDE` reader - 16:16\\]
XTAL EN Override (WU-SEQ) Control 0 = XTAL Enable is driven by WU-SEQ detection 1 = Override XTAL Enable if disabled by default 0x0 = Functional Reset"]
pub type XtalEnOverrideR = crate::BitReader;
#[doc = "Field `XTAL_EN_OVERRIDE` writer - 16:16\\]
XTAL EN Override (WU-SEQ) Control 0 = XTAL Enable is driven by WU-SEQ detection 1 = Override XTAL Enable if disabled by default 0x0 = Functional Reset"]
pub type XtalEnOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WU_UV_DET_CTRL` reader - 17:17\\]
WU Under Voltage Detect Control 0 = UV Detect is disabled 1 = UV Detect is Enabled 0x1 = Functional Reset"]
pub type WuUvDetCtrlR = crate::BitReader;
#[doc = "Field `WU_UV_DET_CTRL` writer - 17:17\\]
WU Under Voltage Detect Control 0 = UV Detect is disabled 1 = UV Detect is Enabled 0x1 = Functional Reset"]
pub type WuUvDetCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WU_OV_DET_CTRL` reader - 18:18\\]
WU Over Voltage Detect Control Changed for 1243 ES3P0 (Metal only change from 1642 ES2P0) Change Name : FW control of VDD OV DET EN 1 = OV Detect is disabled 0 = OV Detect is Enabled 0x1 = Functional Reset"]
pub type WuOvDetCtrlR = crate::BitReader;
#[doc = "Field `WU_OV_DET_CTRL` writer - 18:18\\]
WU Over Voltage Detect Control Changed for 1243 ES3P0 (Metal only change from 1642 ES2P0) Change Name : FW control of VDD OV DET EN 1 = OV Detect is disabled 0 = OV Detect is Enabled 0x1 = Functional Reset"]
pub type WuOvDetCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WU_XTAL_DLY_CTRL` reader - 19:19\\]
Introduce additional delay for XTAL settling 0 = Default delay as per WU-SEQ 1 = Introduce additional delay as per WU-SEQ 0x0 = Functional Reset"]
pub type WuXtalDlyCtrlR = crate::BitReader;
#[doc = "Field `WU_XTAL_DLY_CTRL` writer - 19:19\\]
Introduce additional delay for XTAL settling 0 = Default delay as per WU-SEQ 1 = Introduce additional delay as per WU-SEQ 0x0 = Functional Reset"]
pub type WuXtalDlyCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WU_SUPP_VMON_EN` reader - 20:20\\]
WU VMON Enable Control 0 = VMON Control Disabled 1 = VMON Control Enabled 0x1 = Functional Reset"]
pub type WuSuppVmonEnR = crate::BitReader;
#[doc = "Field `WU_SUPP_VMON_EN` writer - 20:20\\]
WU VMON Enable Control 0 = VMON Control Disabled 1 = VMON Control Enabled 0x1 = Functional Reset"]
pub type WuSuppVmonEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WU_VRAM_VMON_EN` reader - 21:21\\]
WU VRAM VMON Enable Control 0 = SRAM UV Detect Disabled 1 = SRAM UV Detect Enabled 0x1 = Functional Reset"]
pub type WuVramVmonEnR = crate::BitReader;
#[doc = "Field `WU_VRAM_VMON_EN` writer - 21:21\\]
WU VRAM VMON Enable Control 0 = SRAM UV Detect Disabled 1 = SRAM UV Detect Enabled 0x1 = Functional Reset"]
pub type WuVramVmonEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WU_SUPP_DET_CTRL` reader - 22:22\\]
WU VMON Detect Status Override Disable in Functional Test SOP 0 = VMON Det Status Override Disabled 1 = VMON Det Status Override Enabled 0x1 = Functional Reset"]
pub type WuSuppDetCtrlR = crate::BitReader;
#[doc = "Field `WU_SUPP_DET_CTRL` writer - 22:22\\]
WU VMON Detect Status Override Disable in Functional Test SOP 0 = VMON Det Status Override Disabled 1 = VMON Det Status Override Enabled 0x1 = Functional Reset"]
pub type WuSuppDetCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WU_SPARE_IN` reader - 24:23\\]
WU Spare Control Change for 1642 ES2P0 Change Name : Newly added OR gates to provide options to bypass crude VDD DET (also refer to &lt;11>) Bit &lt;0> of this field when HIGH over rides the crude VDD_DET, this control is using firmware Bit&lt;0> of this field is WU_CTRL_REG&lt;23> Change for 2243 ES1P0 Using Bit&lt;1>, is WU_CTRL_REG&lt;24> This bit is used to make the reset fix SW controllable. Manshul's email notes-- Since the default value of WU_SPARE_IN&lt;1> is ΓÇÿ0ΓÇÖ, we will have the fix active by default. To disable it, firmware can write this bit to ΓÇÿ1ΓÇÖ before enabling VMON. 0x0 = Functional Reset"]
pub type WuSpareInR = crate::FieldReader;
#[doc = "Field `WU_SPARE_IN` writer - 24:23\\]
WU Spare Control Change for 1642 ES2P0 Change Name : Newly added OR gates to provide options to bypass crude VDD DET (also refer to &lt;11>) Bit &lt;0> of this field when HIGH over rides the crude VDD_DET, this control is using firmware Bit&lt;0> of this field is WU_CTRL_REG&lt;23> Change for 2243 ES1P0 Using Bit&lt;1>, is WU_CTRL_REG&lt;24> This bit is used to make the reset fix SW controllable. Manshul's email notes-- Since the default value of WU_SPARE_IN&lt;1> is ΓÇÿ0ΓÇÖ, we will have the fix active by default. To disable it, firmware can write this bit to ΓÇÿ1ΓÇÖ before enabling VMON. 0x0 = Functional Reset"]
pub type WuSpareInW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WU_VDDS_3P3V_UV_VMON_EN` reader - 25:25\\]
WU VDDS 3.3V UV VMON Enable Control 0 = VDDS 3.3V UV Detect Disabled 1 = VDDS 3.3V UV Detect Enabled 0x0 = Functional Reset"]
pub type WuVdds3p3vUvVmonEnR = crate::BitReader;
#[doc = "Field `WU_VDDS_3P3V_UV_VMON_EN` writer - 25:25\\]
WU VDDS 3.3V UV VMON Enable Control 0 = VDDS 3.3V UV Detect Disabled 1 = VDDS 3.3V UV Detect Enabled 0x0 = Functional Reset"]
pub type WuVdds3p3vUvVmonEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WU_VDDA_OSC_UV_VMON_EN` reader - 26:26\\]
WU VDDA OSC UV VMON Enable Control 0 = VDDA OSC UV Detect Disabled 1 = VDDA OSC UV Detect Enabled 0x0 = Functional Reset"]
pub type WuVddaOscUvVmonEnR = crate::BitReader;
#[doc = "Field `WU_VDDA_OSC_UV_VMON_EN` writer - 26:26\\]
WU VDDA OSC UV VMON Enable Control 0 = VDDA OSC UV Detect Disabled 1 = VDDA OSC UV Detect Enabled 0x0 = Functional Reset"]
pub type WuVddaOscUvVmonEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WU_VDD_UV_VMON_EN` reader - 27:27\\]
WU VDD UV VMON Enable Control 0 = VDD UV Detect Disabled 1 = VDD UV Detect Enabled 0x0 = Functional Reset"]
pub type WuVddUvVmonEnR = crate::BitReader;
#[doc = "Field `WU_VDD_UV_VMON_EN` writer - 27:27\\]
WU VDD UV VMON Enable Control 0 = VDD UV Detect Disabled 1 = VDD UV Detect Enabled 0x0 = Functional Reset"]
pub type WuVddUvVmonEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WU_VDD_OV_VMON_EN` reader - 28:28\\]
WU VDD OV VMON Enable Control 0 = VDD OV Detect Disabled 1 = VDD OV Detect Enabled 0x0 = Functional Reset"]
pub type WuVddOvVmonEnR = crate::BitReader;
#[doc = "Field `WU_VDD_OV_VMON_EN` writer - 28:28\\]
WU VDD OV VMON Enable Control 0 = VDD OV Detect Disabled 1 = VDD OV Detect Enabled 0x0 = Functional Reset"]
pub type WuVddOvVmonEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WU_SPARE_IN_2` reader - 30:29\\]
WU Spare Control 0x3 = Functional Reset"]
pub type WuSpareIn2R = crate::FieldReader;
#[doc = "Field `WU_SPARE_IN_2` writer - 30:29\\]
WU Spare Control 0x3 = Functional Reset"]
pub type WuSpareIn2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED0` reader - 31:31\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `RESERVED0` writer - 31:31\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
pub type Reserved0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
WU Internal Clock (RCOSC) ENABLE 0 = Internal CLK Disabled 1 = Internal CLK Enabled 0x1 = Functional Reset"]
    #[inline(always)]
    pub fn int_clk_en(&self) -> IntClkEnR {
        IntClkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
WU Internal Clock (RCOSC) STOP 0 = Internal CLK can be enabled 1 = Internal CLK is OFF 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn int_clk_stop(&self) -> IntClkStopR {
        IntClkStopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
WU Internal Clock (RCOSC) SW_SEL 0 = TBD 1 = TBD 0x1 = Functional Reset"]
    #[inline(always)]
    pub fn int_clk_sw_sel(&self) -> IntClkSwSelR {
        IntClkSwSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:9 - 9:3\\]
WU lnternal Clock (RCOSC) Trim 0x4B = Functional Reset (If not trimmed)"]
    #[inline(always)]
    pub fn int_clk_trim_6_0(&self) -> IntClkTrim6_0R {
        IntClkTrim6_0R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bit 10 - 10:10\\]
Disable slicer clock delay ECO (mapped to eFuse) 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn eco_slicer_clk_dly_dis(&self) -> EcoSlicerClkDlyDisR {
        EcoSlicerClkDlyDisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - 14:11\\]
WU Internal Clock (RCOSC) Frequency Select Bit&lt;3> is used as override for VMON on Untrimmed devices. Bit &lt;3> is '1' if device REFSYS_TOP is trimmed. Changed on 1642 ES2P0 Change Name : Newly added mux for CLK MON EN options When Bit&lt;2> = 0, MASK_CPU_CLK_OUT_CTRL_LOWV == WU_CTRL_REG&lt;12>, essentially Bit&lt;1> of this field When Bit&lt;2> = 1, MASK_CPU_CLK_OUT_CTRL_LOWV == (original function) INTER_MASK_CPU_CLK_OUT_CTRL_LOWV Change Name : Newly added OR gates to provide options to bypass crude VDD DET (also refer to &lt;23>) Bit &lt;0> of this field when HIGH over rides the crude VDD_DET 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn int_clk_freq_sel_3_0(&self) -> IntClkFreqSel3_0R {
        IntClkFreqSel3_0R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
WU CLK Control 0 = CLK Monitor Function in Dig Sequencer is disabled 1 = REF CLK Monitor Function is Enabled 0x1 = Functional Reset"]
    #[inline(always)]
    pub fn wu_cpu_clk_ctrl(&self) -> WuCpuClkCtrlR {
        WuCpuClkCtrlR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
XTAL EN Override (WU-SEQ) Control 0 = XTAL Enable is driven by WU-SEQ detection 1 = Override XTAL Enable if disabled by default 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn xtal_en_override(&self) -> XtalEnOverrideR {
        XtalEnOverrideR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
WU Under Voltage Detect Control 0 = UV Detect is disabled 1 = UV Detect is Enabled 0x1 = Functional Reset"]
    #[inline(always)]
    pub fn wu_uv_det_ctrl(&self) -> WuUvDetCtrlR {
        WuUvDetCtrlR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
WU Over Voltage Detect Control Changed for 1243 ES3P0 (Metal only change from 1642 ES2P0) Change Name : FW control of VDD OV DET EN 1 = OV Detect is disabled 0 = OV Detect is Enabled 0x1 = Functional Reset"]
    #[inline(always)]
    pub fn wu_ov_det_ctrl(&self) -> WuOvDetCtrlR {
        WuOvDetCtrlR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Introduce additional delay for XTAL settling 0 = Default delay as per WU-SEQ 1 = Introduce additional delay as per WU-SEQ 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn wu_xtal_dly_ctrl(&self) -> WuXtalDlyCtrlR {
        WuXtalDlyCtrlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
WU VMON Enable Control 0 = VMON Control Disabled 1 = VMON Control Enabled 0x1 = Functional Reset"]
    #[inline(always)]
    pub fn wu_supp_vmon_en(&self) -> WuSuppVmonEnR {
        WuSuppVmonEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
WU VRAM VMON Enable Control 0 = SRAM UV Detect Disabled 1 = SRAM UV Detect Enabled 0x1 = Functional Reset"]
    #[inline(always)]
    pub fn wu_vram_vmon_en(&self) -> WuVramVmonEnR {
        WuVramVmonEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
WU VMON Detect Status Override Disable in Functional Test SOP 0 = VMON Det Status Override Disabled 1 = VMON Det Status Override Enabled 0x1 = Functional Reset"]
    #[inline(always)]
    pub fn wu_supp_det_ctrl(&self) -> WuSuppDetCtrlR {
        WuSuppDetCtrlR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:24 - 24:23\\]
WU Spare Control Change for 1642 ES2P0 Change Name : Newly added OR gates to provide options to bypass crude VDD DET (also refer to &lt;11>) Bit &lt;0> of this field when HIGH over rides the crude VDD_DET, this control is using firmware Bit&lt;0> of this field is WU_CTRL_REG&lt;23> Change for 2243 ES1P0 Using Bit&lt;1>, is WU_CTRL_REG&lt;24> This bit is used to make the reset fix SW controllable. Manshul's email notes-- Since the default value of WU_SPARE_IN&lt;1> is ΓÇÿ0ΓÇÖ, we will have the fix active by default. To disable it, firmware can write this bit to ΓÇÿ1ΓÇÖ before enabling VMON. 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn wu_spare_in(&self) -> WuSpareInR {
        WuSpareInR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - 25:25\\]
WU VDDS 3.3V UV VMON Enable Control 0 = VDDS 3.3V UV Detect Disabled 1 = VDDS 3.3V UV Detect Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn wu_vdds_3p3v_uv_vmon_en(&self) -> WuVdds3p3vUvVmonEnR {
        WuVdds3p3vUvVmonEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
WU VDDA OSC UV VMON Enable Control 0 = VDDA OSC UV Detect Disabled 1 = VDDA OSC UV Detect Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn wu_vdda_osc_uv_vmon_en(&self) -> WuVddaOscUvVmonEnR {
        WuVddaOscUvVmonEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
WU VDD UV VMON Enable Control 0 = VDD UV Detect Disabled 1 = VDD UV Detect Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn wu_vdd_uv_vmon_en(&self) -> WuVddUvVmonEnR {
        WuVddUvVmonEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
WU VDD OV VMON Enable Control 0 = VDD OV Detect Disabled 1 = VDD OV Detect Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn wu_vdd_ov_vmon_en(&self) -> WuVddOvVmonEnR {
        WuVddOvVmonEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - 30:29\\]
WU Spare Control 0x3 = Functional Reset"]
    #[inline(always)]
    pub fn wu_spare_in_2(&self) -> WuSpareIn2R {
        WuSpareIn2R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
WU Internal Clock (RCOSC) ENABLE 0 = Internal CLK Disabled 1 = Internal CLK Enabled 0x1 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn int_clk_en(&mut self) -> IntClkEnW<AnaRegWuCtrlRegLowvSpec> {
        IntClkEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
WU Internal Clock (RCOSC) STOP 0 = Internal CLK can be enabled 1 = Internal CLK is OFF 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn int_clk_stop(&mut self) -> IntClkStopW<AnaRegWuCtrlRegLowvSpec> {
        IntClkStopW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
WU Internal Clock (RCOSC) SW_SEL 0 = TBD 1 = TBD 0x1 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn int_clk_sw_sel(&mut self) -> IntClkSwSelW<AnaRegWuCtrlRegLowvSpec> {
        IntClkSwSelW::new(self, 2)
    }
    #[doc = "Bits 3:9 - 9:3\\]
WU lnternal Clock (RCOSC) Trim 0x4B = Functional Reset (If not trimmed)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clk_trim_6_0(&mut self) -> IntClkTrim6_0W<AnaRegWuCtrlRegLowvSpec> {
        IntClkTrim6_0W::new(self, 3)
    }
    #[doc = "Bit 10 - 10:10\\]
Disable slicer clock delay ECO (mapped to eFuse) 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eco_slicer_clk_dly_dis(&mut self) -> EcoSlicerClkDlyDisW<AnaRegWuCtrlRegLowvSpec> {
        EcoSlicerClkDlyDisW::new(self, 10)
    }
    #[doc = "Bits 11:14 - 14:11\\]
WU Internal Clock (RCOSC) Frequency Select Bit&lt;3> is used as override for VMON on Untrimmed devices. Bit &lt;3> is '1' if device REFSYS_TOP is trimmed. Changed on 1642 ES2P0 Change Name : Newly added mux for CLK MON EN options When Bit&lt;2> = 0, MASK_CPU_CLK_OUT_CTRL_LOWV == WU_CTRL_REG&lt;12>, essentially Bit&lt;1> of this field When Bit&lt;2> = 1, MASK_CPU_CLK_OUT_CTRL_LOWV == (original function) INTER_MASK_CPU_CLK_OUT_CTRL_LOWV Change Name : Newly added OR gates to provide options to bypass crude VDD DET (also refer to &lt;23>) Bit &lt;0> of this field when HIGH over rides the crude VDD_DET 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn int_clk_freq_sel_3_0(&mut self) -> IntClkFreqSel3_0W<AnaRegWuCtrlRegLowvSpec> {
        IntClkFreqSel3_0W::new(self, 11)
    }
    #[doc = "Bit 15 - 15:15\\]
WU CLK Control 0 = CLK Monitor Function in Dig Sequencer is disabled 1 = REF CLK Monitor Function is Enabled 0x1 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn wu_cpu_clk_ctrl(&mut self) -> WuCpuClkCtrlW<AnaRegWuCtrlRegLowvSpec> {
        WuCpuClkCtrlW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
XTAL EN Override (WU-SEQ) Control 0 = XTAL Enable is driven by WU-SEQ detection 1 = Override XTAL Enable if disabled by default 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_en_override(&mut self) -> XtalEnOverrideW<AnaRegWuCtrlRegLowvSpec> {
        XtalEnOverrideW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
WU Under Voltage Detect Control 0 = UV Detect is disabled 1 = UV Detect is Enabled 0x1 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn wu_uv_det_ctrl(&mut self) -> WuUvDetCtrlW<AnaRegWuCtrlRegLowvSpec> {
        WuUvDetCtrlW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
WU Over Voltage Detect Control Changed for 1243 ES3P0 (Metal only change from 1642 ES2P0) Change Name : FW control of VDD OV DET EN 1 = OV Detect is disabled 0 = OV Detect is Enabled 0x1 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn wu_ov_det_ctrl(&mut self) -> WuOvDetCtrlW<AnaRegWuCtrlRegLowvSpec> {
        WuOvDetCtrlW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Introduce additional delay for XTAL settling 0 = Default delay as per WU-SEQ 1 = Introduce additional delay as per WU-SEQ 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn wu_xtal_dly_ctrl(&mut self) -> WuXtalDlyCtrlW<AnaRegWuCtrlRegLowvSpec> {
        WuXtalDlyCtrlW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
WU VMON Enable Control 0 = VMON Control Disabled 1 = VMON Control Enabled 0x1 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn wu_supp_vmon_en(&mut self) -> WuSuppVmonEnW<AnaRegWuCtrlRegLowvSpec> {
        WuSuppVmonEnW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
WU VRAM VMON Enable Control 0 = SRAM UV Detect Disabled 1 = SRAM UV Detect Enabled 0x1 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn wu_vram_vmon_en(&mut self) -> WuVramVmonEnW<AnaRegWuCtrlRegLowvSpec> {
        WuVramVmonEnW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
WU VMON Detect Status Override Disable in Functional Test SOP 0 = VMON Det Status Override Disabled 1 = VMON Det Status Override Enabled 0x1 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn wu_supp_det_ctrl(&mut self) -> WuSuppDetCtrlW<AnaRegWuCtrlRegLowvSpec> {
        WuSuppDetCtrlW::new(self, 22)
    }
    #[doc = "Bits 23:24 - 24:23\\]
WU Spare Control Change for 1642 ES2P0 Change Name : Newly added OR gates to provide options to bypass crude VDD DET (also refer to &lt;11>) Bit &lt;0> of this field when HIGH over rides the crude VDD_DET, this control is using firmware Bit&lt;0> of this field is WU_CTRL_REG&lt;23> Change for 2243 ES1P0 Using Bit&lt;1>, is WU_CTRL_REG&lt;24> This bit is used to make the reset fix SW controllable. Manshul's email notes-- Since the default value of WU_SPARE_IN&lt;1> is ΓÇÿ0ΓÇÖ, we will have the fix active by default. To disable it, firmware can write this bit to ΓÇÿ1ΓÇÖ before enabling VMON. 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn wu_spare_in(&mut self) -> WuSpareInW<AnaRegWuCtrlRegLowvSpec> {
        WuSpareInW::new(self, 23)
    }
    #[doc = "Bit 25 - 25:25\\]
WU VDDS 3.3V UV VMON Enable Control 0 = VDDS 3.3V UV Detect Disabled 1 = VDDS 3.3V UV Detect Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn wu_vdds_3p3v_uv_vmon_en(&mut self) -> WuVdds3p3vUvVmonEnW<AnaRegWuCtrlRegLowvSpec> {
        WuVdds3p3vUvVmonEnW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
WU VDDA OSC UV VMON Enable Control 0 = VDDA OSC UV Detect Disabled 1 = VDDA OSC UV Detect Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn wu_vdda_osc_uv_vmon_en(&mut self) -> WuVddaOscUvVmonEnW<AnaRegWuCtrlRegLowvSpec> {
        WuVddaOscUvVmonEnW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
WU VDD UV VMON Enable Control 0 = VDD UV Detect Disabled 1 = VDD UV Detect Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn wu_vdd_uv_vmon_en(&mut self) -> WuVddUvVmonEnW<AnaRegWuCtrlRegLowvSpec> {
        WuVddUvVmonEnW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
WU VDD OV VMON Enable Control 0 = VDD OV Detect Disabled 1 = VDD OV Detect Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn wu_vdd_ov_vmon_en(&mut self) -> WuVddOvVmonEnW<AnaRegWuCtrlRegLowvSpec> {
        WuVddOvVmonEnW::new(self, 28)
    }
    #[doc = "Bits 29:30 - 30:29\\]
WU Spare Control 0x3 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn wu_spare_in_2(&mut self) -> WuSpareIn2W<AnaRegWuCtrlRegLowvSpec> {
        WuSpareIn2W::new(self, 29)
    }
    #[doc = "Bit 31 - 31:31\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<AnaRegWuCtrlRegLowvSpec> {
        Reserved0W::new(self, 31)
    }
}
#[doc = "ANA_REG_WU_CTRL_REG_LOWV\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_wu_ctrl_reg_lowv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_wu_ctrl_reg_lowv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaRegWuCtrlRegLowvSpec;
impl crate::RegisterSpec for AnaRegWuCtrlRegLowvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_reg_wu_ctrl_reg_lowv::R`](R) reader structure"]
impl crate::Readable for AnaRegWuCtrlRegLowvSpec {}
#[doc = "`write(|w| ..)` method takes [`ana_reg_wu_ctrl_reg_lowv::W`](W) writer structure"]
impl crate::Writable for AnaRegWuCtrlRegLowvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANA_REG_WU_CTRL_REG_LOWV to value 0"]
impl crate::Resettable for AnaRegWuCtrlRegLowvSpec {
    const RESET_VALUE: u32 = 0;
}
