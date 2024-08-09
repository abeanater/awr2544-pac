#[doc = "Register `ANA_REG_TW_CTRL_REG_LOWV` reader"]
pub type R = crate::R<AnaRegTwCtrlRegLowvSpec>;
#[doc = "Register `ANA_REG_TW_CTRL_REG_LOWV` writer"]
pub type W = crate::W<AnaRegTwCtrlRegLowvSpec>;
#[doc = "Field `ADC_EN` reader - 0:0\\]
TW MSS ADC Control 0 = ADC Disable 1 = ADC Enable 0x0 = Functional Reset"]
pub type AdcEnR = crate::BitReader;
#[doc = "Field `ADC_EN` writer - 0:0\\]
TW MSS ADC Control 0 = ADC Disable 1 = ADC Enable 0x0 = Functional Reset"]
pub type AdcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_RESET` reader - 2:2\\]
MSS TW ADC Reset (Active High) 0 = ADC Out of Reset 1 = ADC In Reset 0x1 = Functional Reset"]
pub type AdcResetR = crate::BitReader;
#[doc = "Field `ADC_RESET` writer - 2:2\\]
MSS TW ADC Reset (Active High) 0 = ADC Out of Reset 1 = ADC In Reset 0x1 = Functional Reset"]
pub type AdcResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_INP_BUF_EN` reader - 3:3\\]
MSS TW ADC Input Buffer Enable 0 = Input Buffer disabled 1 = Input Buffer Enabled (Default) 0x1 = Functional Reset"]
pub type AdcInpBufEnR = crate::BitReader;
#[doc = "Field `ADC_INP_BUF_EN` writer - 3:3\\]
MSS TW ADC Input Buffer Enable 0 = Input Buffer disabled 1 = Input Buffer Enabled (Default) 0x1 = Functional Reset"]
pub type AdcInpBufEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_REF_BUF_EN` reader - 4:4\\]
MSS TW ADC Reference Buffer Enable 0 = Input Buffer disabled 1 = Input Buffer Enabled (Default) 0x1 = Functional Reset"]
pub type AdcRefBufEnR = crate::BitReader;
#[doc = "Field `ADC_REF_BUF_EN` writer - 4:4\\]
MSS TW ADC Reference Buffer Enable 0 = Input Buffer disabled 1 = Input Buffer Enabled (Default) 0x1 = Functional Reset"]
pub type AdcRefBufEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_REF_SEL_2_0` reader - 7:5\\]
MSS TW ADC Reference Select 0b001 = Reference from Top Refsys 0b010 = Reference from RX Refsys 0b100 = Reference from External Test Pin (CZ/ Trim) 0x001 = Functional Reset"]
pub type AdcRefSel2_0R = crate::FieldReader;
#[doc = "Field `ADC_REF_SEL_2_0` writer - 7:5\\]
MSS TW ADC Reference Select 0b001 = Reference from Top Refsys 0b010 = Reference from RX Refsys 0b100 = Reference from External Test Pin (CZ/ Trim) 0x001 = Functional Reset"]
pub type AdcRefSel2_0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TS_DIFF_INP_BUF_EN` reader - 8:8\\]
MSS TW ADC TS DIFF Inp Buffer Enable 0 = Input Buffer disabled 1 = Input Buffer Enabled 0x1 = Functional Reset"]
pub type TsDiffInpBufEnR = crate::BitReader;
#[doc = "Field `TS_DIFF_INP_BUF_EN` writer - 8:8\\]
MSS TW ADC TS DIFF Inp Buffer Enable 0 = Input Buffer disabled 1 = Input Buffer Enabled 0x1 = Functional Reset"]
pub type TsDiffInpBufEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS_SE_INP_BUF_EN` reader - 9:9\\]
MSS TW ADC TS SE Inp Buffer Enable 0 = Input Buffer disabled 1 = Input Buffer Enabled 0x0 = Functional Reset"]
pub type TsSeInpBufEnR = crate::BitReader;
#[doc = "Field `TS_SE_INP_BUF_EN` writer - 9:9\\]
MSS TW ADC TS SE Inp Buffer Enable 0 = Input Buffer disabled 1 = Input Buffer Enabled 0x0 = Functional Reset"]
pub type TsSeInpBufEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFORCE_EXT_CTRL` reader - 10:10\\]
MSS TW Iforce Control from External Source 0 = IFORCE Control Disabled 1 = IFORCE Control Enabled 0x0 = Functional Reset"]
pub type IforceExtCtrlR = crate::BitReader;
#[doc = "Field `IFORCE_EXT_CTRL` writer - 10:10\\]
MSS TW Iforce Control from External Source 0 = IFORCE Control Disabled 1 = IFORCE Control Enabled 0x0 = Functional Reset"]
pub type IforceExtCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREF_EXT_CTRL` reader - 11:11\\]
MSS TW VREF Control from External SOurce 0 = External VREF Control Disabled 1 = External VREF Control Enabled 0x0 = Functional Reset"]
pub type VrefExtCtrlR = crate::BitReader;
#[doc = "Field `VREF_EXT_CTRL` writer - 11:11\\]
MSS TW VREF Control from External SOurce 0 = External VREF Control Disabled 1 = External VREF Control Enabled 0x0 = Functional Reset"]
pub type VrefExtCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VIN_EXT_CTRL` reader - 12:12\\]
MSS TW VIN Control from External Source 0 = External VIN Control Disabled 1 = External VIN Control Enabled 0x0 = Functional Reset"]
pub type VinExtCtrlR = crate::BitReader;
#[doc = "Field `VIN_EXT_CTRL` writer - 12:12\\]
MSS TW VIN Control from External Source 0 = External VIN Control Disabled 1 = External VIN Control Enabled 0x0 = Functional Reset"]
pub type VinExtCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANA_TMUX_BUF_BYPASS` reader - 13:13\\]
MSS TW ANA TMUX Buffer Bypass 0 = ANA TMUX Buffer By-pass Disabled 1 = ANA TMUX Buffer By-pass Enabled 0x0 = Functional Reset"]
pub type AnaTmuxBufBypassR = crate::BitReader;
#[doc = "Field `ANA_TMUX_BUF_BYPASS` writer - 13:13\\]
MSS TW ANA TMUX Buffer Bypass 0 = ANA TMUX Buffer By-pass Disabled 1 = ANA TMUX Buffer By-pass Enabled 0x0 = Functional Reset"]
pub type AnaTmuxBufBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANA_TMUX_BUF_EN` reader - 14:14\\]
MSS TW ANA TMUX Buffer Enabled 0 = ANA TMUX Buffer Disabled 1 = ANA TMUX Buffer Enabled 0x0 = Functional Reset"]
pub type AnaTmuxBufEnR = crate::BitReader;
#[doc = "Field `ANA_TMUX_BUF_EN` writer - 14:14\\]
MSS TW ANA TMUX Buffer Enabled 0 = ANA TMUX Buffer Disabled 1 = ANA TMUX Buffer Enabled 0x0 = Functional Reset"]
pub type AnaTmuxBufEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - 31:15\\]
Reserved 0x00000 = Functional Reset"]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `Reserved0` writer - 31:15\\]
Reserved 0x00000 = Functional Reset"]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
TW MSS ADC Control 0 = ADC Disable 1 = ADC Enable 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn adc_en(&self) -> AdcEnR {
        AdcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
MSS TW ADC Reset (Active High) 0 = ADC Out of Reset 1 = ADC In Reset 0x1 = Functional Reset"]
    #[inline(always)]
    pub fn adc_reset(&self) -> AdcResetR {
        AdcResetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
MSS TW ADC Input Buffer Enable 0 = Input Buffer disabled 1 = Input Buffer Enabled (Default) 0x1 = Functional Reset"]
    #[inline(always)]
    pub fn adc_inp_buf_en(&self) -> AdcInpBufEnR {
        AdcInpBufEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
MSS TW ADC Reference Buffer Enable 0 = Input Buffer disabled 1 = Input Buffer Enabled (Default) 0x1 = Functional Reset"]
    #[inline(always)]
    pub fn adc_ref_buf_en(&self) -> AdcRefBufEnR {
        AdcRefBufEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
MSS TW ADC Reference Select 0b001 = Reference from Top Refsys 0b010 = Reference from RX Refsys 0b100 = Reference from External Test Pin (CZ/ Trim) 0x001 = Functional Reset"]
    #[inline(always)]
    pub fn adc_ref_sel_2_0(&self) -> AdcRefSel2_0R {
        AdcRefSel2_0R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
MSS TW ADC TS DIFF Inp Buffer Enable 0 = Input Buffer disabled 1 = Input Buffer Enabled 0x1 = Functional Reset"]
    #[inline(always)]
    pub fn ts_diff_inp_buf_en(&self) -> TsDiffInpBufEnR {
        TsDiffInpBufEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
MSS TW ADC TS SE Inp Buffer Enable 0 = Input Buffer disabled 1 = Input Buffer Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn ts_se_inp_buf_en(&self) -> TsSeInpBufEnR {
        TsSeInpBufEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
MSS TW Iforce Control from External Source 0 = IFORCE Control Disabled 1 = IFORCE Control Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn iforce_ext_ctrl(&self) -> IforceExtCtrlR {
        IforceExtCtrlR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
MSS TW VREF Control from External SOurce 0 = External VREF Control Disabled 1 = External VREF Control Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vref_ext_ctrl(&self) -> VrefExtCtrlR {
        VrefExtCtrlR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
MSS TW VIN Control from External Source 0 = External VIN Control Disabled 1 = External VIN Control Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vin_ext_ctrl(&self) -> VinExtCtrlR {
        VinExtCtrlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
MSS TW ANA TMUX Buffer Bypass 0 = ANA TMUX Buffer By-pass Disabled 1 = ANA TMUX Buffer By-pass Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn ana_tmux_buf_bypass(&self) -> AnaTmuxBufBypassR {
        AnaTmuxBufBypassR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
MSS TW ANA TMUX Buffer Enabled 0 = ANA TMUX Buffer Disabled 1 = ANA TMUX Buffer Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn ana_tmux_buf_en(&self) -> AnaTmuxBufEnR {
        AnaTmuxBufEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Reserved 0x00000 = Functional Reset"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
TW MSS ADC Control 0 = ADC Disable 1 = ADC Enable 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc_en(&mut self) -> AdcEnW<AnaRegTwCtrlRegLowvSpec> {
        AdcEnW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
MSS TW ADC Reset (Active High) 0 = ADC Out of Reset 1 = ADC In Reset 0x1 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc_reset(&mut self) -> AdcResetW<AnaRegTwCtrlRegLowvSpec> {
        AdcResetW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
MSS TW ADC Input Buffer Enable 0 = Input Buffer disabled 1 = Input Buffer Enabled (Default) 0x1 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc_inp_buf_en(&mut self) -> AdcInpBufEnW<AnaRegTwCtrlRegLowvSpec> {
        AdcInpBufEnW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
MSS TW ADC Reference Buffer Enable 0 = Input Buffer disabled 1 = Input Buffer Enabled (Default) 0x1 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ref_buf_en(&mut self) -> AdcRefBufEnW<AnaRegTwCtrlRegLowvSpec> {
        AdcRefBufEnW::new(self, 4)
    }
    #[doc = "Bits 5:7 - 7:5\\]
MSS TW ADC Reference Select 0b001 = Reference from Top Refsys 0b010 = Reference from RX Refsys 0b100 = Reference from External Test Pin (CZ/ Trim) 0x001 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ref_sel_2_0(&mut self) -> AdcRefSel2_0W<AnaRegTwCtrlRegLowvSpec> {
        AdcRefSel2_0W::new(self, 5)
    }
    #[doc = "Bit 8 - 8:8\\]
MSS TW ADC TS DIFF Inp Buffer Enable 0 = Input Buffer disabled 1 = Input Buffer Enabled 0x1 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ts_diff_inp_buf_en(&mut self) -> TsDiffInpBufEnW<AnaRegTwCtrlRegLowvSpec> {
        TsDiffInpBufEnW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
MSS TW ADC TS SE Inp Buffer Enable 0 = Input Buffer disabled 1 = Input Buffer Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ts_se_inp_buf_en(&mut self) -> TsSeInpBufEnW<AnaRegTwCtrlRegLowvSpec> {
        TsSeInpBufEnW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
MSS TW Iforce Control from External Source 0 = IFORCE Control Disabled 1 = IFORCE Control Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn iforce_ext_ctrl(&mut self) -> IforceExtCtrlW<AnaRegTwCtrlRegLowvSpec> {
        IforceExtCtrlW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
MSS TW VREF Control from External SOurce 0 = External VREF Control Disabled 1 = External VREF Control Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vref_ext_ctrl(&mut self) -> VrefExtCtrlW<AnaRegTwCtrlRegLowvSpec> {
        VrefExtCtrlW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
MSS TW VIN Control from External Source 0 = External VIN Control Disabled 1 = External VIN Control Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vin_ext_ctrl(&mut self) -> VinExtCtrlW<AnaRegTwCtrlRegLowvSpec> {
        VinExtCtrlW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
MSS TW ANA TMUX Buffer Bypass 0 = ANA TMUX Buffer By-pass Disabled 1 = ANA TMUX Buffer By-pass Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ana_tmux_buf_bypass(&mut self) -> AnaTmuxBufBypassW<AnaRegTwCtrlRegLowvSpec> {
        AnaTmuxBufBypassW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
MSS TW ANA TMUX Buffer Enabled 0 = ANA TMUX Buffer Disabled 1 = ANA TMUX Buffer Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ana_tmux_buf_en(&mut self) -> AnaTmuxBufEnW<AnaRegTwCtrlRegLowvSpec> {
        AnaTmuxBufEnW::new(self, 14)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Reserved 0x00000 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<AnaRegTwCtrlRegLowvSpec> {
        Reserved0W::new(self, 15)
    }
}
#[doc = "ANA_REG_TW_CTRL_REG_LOWV\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_tw_ctrl_reg_lowv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_tw_ctrl_reg_lowv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaRegTwCtrlRegLowvSpec;
impl crate::RegisterSpec for AnaRegTwCtrlRegLowvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_reg_tw_ctrl_reg_lowv::R`](R) reader structure"]
impl crate::Readable for AnaRegTwCtrlRegLowvSpec {}
#[doc = "`write(|w| ..)` method takes [`ana_reg_tw_ctrl_reg_lowv::W`](W) writer structure"]
impl crate::Writable for AnaRegTwCtrlRegLowvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANA_REG_TW_CTRL_REG_LOWV to value 0"]
impl crate::Resettable for AnaRegTwCtrlRegLowvSpec {
    const RESET_VALUE: u32 = 0;
}
