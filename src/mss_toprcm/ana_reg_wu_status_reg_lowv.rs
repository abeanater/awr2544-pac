#[doc = "Register `ANA_REG_WU_STATUS_REG_LOWV` reader"]
pub type R = crate::R<AnaRegWuStatusRegLowvSpec>;
#[doc = "Register `ANA_REG_WU_STATUS_REG_LOWV` writer"]
pub type W = crate::W<AnaRegWuStatusRegLowvSpec>;
#[doc = "Field `CORE_OVDET_LAT` reader - 0:0\\]
Latched Value of OV Detect 0 = OV Detect Not Triggered 1 = OV Detect has Triggered"]
pub type CoreOvdetLatR = crate::BitReader;
#[doc = "Field `CORE_OVDET_LAT` writer - 0:0\\]
Latched Value of OV Detect 0 = OV Detect Not Triggered 1 = OV Detect has Triggered"]
pub type CoreOvdetLatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_UVDET_LAT` reader - 1:1\\]
Latched Value of UV Detect 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
pub type CoreUvdetLatR = crate::BitReader;
#[doc = "Field `CORE_UVDET_LAT` writer - 1:1\\]
Latched Value of UV Detect 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
pub type CoreUvdetLatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDA18BB_UV_DET_LAT` reader - 2:2\\]
Latched Value of UV Detect for 1.8V Baseband 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
pub type Vdda18bbUvDetLatR = crate::BitReader;
#[doc = "Field `VDDA18BB_UV_DET_LAT` writer - 2:2\\]
Latched Value of UV Detect for 1.8V Baseband 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
pub type Vdda18bbUvDetLatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUPP_OK_CLK18` reader - 3:3\\]
Supp Detect output of CLK 1.8V 0 = Supply Not detected 1 = Supply Detected"]
pub type SuppOkClk18R = crate::BitReader;
#[doc = "Field `SUPP_OK_CLK18` writer - 3:3\\]
Supp Detect output of CLK 1.8V 0 = Supply Not detected 1 = Supply Detected"]
pub type SuppOkClk18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUPP_OK_IO18` reader - 4:4\\]
Supp Detect output of IO 1.8V 0 = Supply Not detected 1 = Supply Detected"]
pub type SuppOkIo18R = crate::BitReader;
#[doc = "Field `SUPP_OK_IO18` writer - 4:4\\]
Supp Detect output of IO 1.8V 0 = Supply Not detected 1 = Supply Detected"]
pub type SuppOkIo18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUPP_OK_IO33` reader - 5:5\\]
Supp Detect output of IO 3.3V 0 = Supply Not detected 1 = Supply Detected"]
pub type SuppOkIo33R = crate::BitReader;
#[doc = "Field `SUPP_OK_IO33` writer - 5:5\\]
Supp Detect output of IO 3.3V 0 = Supply Not detected 1 = Supply Detected"]
pub type SuppOkIo33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUPP_OK_RF10` reader - 6:6\\]
Can be made spare (TiedLO internally)"]
pub type SuppOkRf10R = crate::BitReader;
#[doc = "Field `SUPP_OK_RF10` writer - 6:6\\]
Can be made spare (TiedLO internally)"]
pub type SuppOkRf10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDA10RF1_UVDET_LAT` reader - 7:7\\]
Latched Value of UV Detect for 1V RF1 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
pub type Vdda10rf1UvdetLatR = crate::BitReader;
#[doc = "Field `VDDA10RF1_UVDET_LAT` writer - 7:7\\]
Latched Value of UV Detect for 1V RF1 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
pub type Vdda10rf1UvdetLatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDA10RF2_UVDET_LAT` reader - 8:8\\]
Latched Value of UV Detect for 1V RF2 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
pub type Vdda10rf2UvdetLatR = crate::BitReader;
#[doc = "Field `VDDA10RF2_UVDET_LAT` writer - 8:8\\]
Latched Value of UV Detect for 1V RF2 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
pub type Vdda10rf2UvdetLatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUPP_OK_SRAM12` reader - 9:9\\]
Can be made spare (TiedLO internally)"]
pub type SuppOkSram12R = crate::BitReader;
#[doc = "Field `SUPP_OK_SRAM12` writer - 9:9\\]
Can be made spare (TiedLO internally)"]
pub type SuppOkSram12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUPP_OK_VDDD18` reader - 10:10\\]
Can be made spare (TiedLO internally)"]
pub type SuppOkVddd18R = crate::BitReader;
#[doc = "Field `SUPP_OK_VDDD18` writer - 10:10\\]
Can be made spare (TiedLO internally)"]
pub type SuppOkVddd18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF_CLK_STATUS` reader - 11:11\\]
Ref CLK status at Wake-up 0 = REF CLK absent 1 = REF CLK Present"]
pub type RefClkStatusR = crate::BitReader;
#[doc = "Field `REF_CLK_STATUS` writer - 11:11\\]
Ref CLK status at Wake-up 0 = REF CLK absent 1 = REF CLK Present"]
pub type RefClkStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCOSC_CLK_STATUS` reader - 12:12\\]
RCOSC status at Wake-up 0 = RCOSC CLK absent 1 = RCOSC CLK Present"]
pub type RcoscClkStatusR = crate::BitReader;
#[doc = "Field `RCOSC_CLK_STATUS` writer - 12:12\\]
RCOSC status at Wake-up 0 = RCOSC CLK absent 1 = RCOSC CLK Present"]
pub type RcoscClkStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_DET_STATUS` reader - 13:13\\]
XTAL Detect status at Wake-up 0 = XTAL absent 1 = XTAL Present"]
pub type XtalDetStatusR = crate::BitReader;
#[doc = "Field `XTAL_DET_STATUS` writer - 13:13\\]
XTAL Detect status at Wake-up 0 = XTAL absent 1 = XTAL Present"]
pub type XtalDetStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LIMP_MODE_STATUS` reader - 14:14\\]
Ref CLK status at Wake-up 0 = REF CLK is present 1 = REF CLK is absent and CPU CLK Switched to RCOSC"]
pub type LimpModeStatusR = crate::BitReader;
#[doc = "Field `LIMP_MODE_STATUS` writer - 14:14\\]
Ref CLK status at Wake-up 0 = REF CLK is present 1 = REF CLK is absent and CPU CLK Switched to RCOSC"]
pub type LimpModeStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HVMODE` reader - 15:15\\]
HVMODE Status from VMON 1 = 3.3V VIO 0 = 1.8V VIO"]
pub type HvmodeR = crate::BitReader;
#[doc = "Field `HVMODE` writer - 15:15\\]
HVMODE Status from VMON 1 = 3.3V VIO 0 = 1.8V VIO"]
pub type HvmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APLLVCO18_UVDET_LAT` reader - 16:16\\]
Latched Value of UV Detect for 1.8V VCO 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
pub type Apllvco18UvdetLatR = crate::BitReader;
#[doc = "Field `APLLVCO18_UVDET_LAT` writer - 16:16\\]
Latched Value of UV Detect for 1.8V VCO 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
pub type Apllvco18UvdetLatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDA_OSC_UVDET_LAT` reader - 17:17\\]
Latched value of UV detect for 1.8V CLK 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
pub type VddaOscUvdetLatR = crate::BitReader;
#[doc = "Field `VDDA_OSC_UVDET_LAT` writer - 17:17\\]
Latched value of UV detect for 1.8V CLK 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
pub type VddaOscUvdetLatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDS_3P3V_UVDET_LAT` reader - 18:18\\]
Latched Value of 3.3V IO UV Detect 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
pub type Vdds3p3vUvdetLatR = crate::BitReader;
#[doc = "Field `VDDS_3P3V_UVDET_LAT` writer - 18:18\\]
Latched Value of 3.3V IO UV Detect 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
pub type Vdds3p3vUvdetLatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - 31:19\\]
Reserved Reads return 0x0 and writes have no effect."]
pub type Reserved0R = crate::FieldReader<u16>;
#[doc = "Field `Reserved0` writer - 31:19\\]
Reserved Reads return 0x0 and writes have no effect."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Latched Value of OV Detect 0 = OV Detect Not Triggered 1 = OV Detect has Triggered"]
    #[inline(always)]
    pub fn core_ovdet_lat(&self) -> CoreOvdetLatR {
        CoreOvdetLatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Latched Value of UV Detect 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
    #[inline(always)]
    pub fn core_uvdet_lat(&self) -> CoreUvdetLatR {
        CoreUvdetLatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Latched Value of UV Detect for 1.8V Baseband 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
    #[inline(always)]
    pub fn vdda18bb_uv_det_lat(&self) -> Vdda18bbUvDetLatR {
        Vdda18bbUvDetLatR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Supp Detect output of CLK 1.8V 0 = Supply Not detected 1 = Supply Detected"]
    #[inline(always)]
    pub fn supp_ok_clk18(&self) -> SuppOkClk18R {
        SuppOkClk18R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Supp Detect output of IO 1.8V 0 = Supply Not detected 1 = Supply Detected"]
    #[inline(always)]
    pub fn supp_ok_io18(&self) -> SuppOkIo18R {
        SuppOkIo18R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Supp Detect output of IO 3.3V 0 = Supply Not detected 1 = Supply Detected"]
    #[inline(always)]
    pub fn supp_ok_io33(&self) -> SuppOkIo33R {
        SuppOkIo33R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Can be made spare (TiedLO internally)"]
    #[inline(always)]
    pub fn supp_ok_rf10(&self) -> SuppOkRf10R {
        SuppOkRf10R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Latched Value of UV Detect for 1V RF1 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
    #[inline(always)]
    pub fn vdda10rf1_uvdet_lat(&self) -> Vdda10rf1UvdetLatR {
        Vdda10rf1UvdetLatR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Latched Value of UV Detect for 1V RF2 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
    #[inline(always)]
    pub fn vdda10rf2_uvdet_lat(&self) -> Vdda10rf2UvdetLatR {
        Vdda10rf2UvdetLatR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Can be made spare (TiedLO internally)"]
    #[inline(always)]
    pub fn supp_ok_sram12(&self) -> SuppOkSram12R {
        SuppOkSram12R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Can be made spare (TiedLO internally)"]
    #[inline(always)]
    pub fn supp_ok_vddd18(&self) -> SuppOkVddd18R {
        SuppOkVddd18R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Ref CLK status at Wake-up 0 = REF CLK absent 1 = REF CLK Present"]
    #[inline(always)]
    pub fn ref_clk_status(&self) -> RefClkStatusR {
        RefClkStatusR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
RCOSC status at Wake-up 0 = RCOSC CLK absent 1 = RCOSC CLK Present"]
    #[inline(always)]
    pub fn rcosc_clk_status(&self) -> RcoscClkStatusR {
        RcoscClkStatusR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
XTAL Detect status at Wake-up 0 = XTAL absent 1 = XTAL Present"]
    #[inline(always)]
    pub fn xtal_det_status(&self) -> XtalDetStatusR {
        XtalDetStatusR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Ref CLK status at Wake-up 0 = REF CLK is present 1 = REF CLK is absent and CPU CLK Switched to RCOSC"]
    #[inline(always)]
    pub fn limp_mode_status(&self) -> LimpModeStatusR {
        LimpModeStatusR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
HVMODE Status from VMON 1 = 3.3V VIO 0 = 1.8V VIO"]
    #[inline(always)]
    pub fn hvmode(&self) -> HvmodeR {
        HvmodeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Latched Value of UV Detect for 1.8V VCO 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
    #[inline(always)]
    pub fn apllvco18_uvdet_lat(&self) -> Apllvco18UvdetLatR {
        Apllvco18UvdetLatR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Latched value of UV detect for 1.8V CLK 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
    #[inline(always)]
    pub fn vdda_osc_uvdet_lat(&self) -> VddaOscUvdetLatR {
        VddaOscUvdetLatR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Latched Value of 3.3V IO UV Detect 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
    #[inline(always)]
    pub fn vdds_3p3v_uvdet_lat(&self) -> Vdds3p3vUvdetLatR {
        Vdds3p3vUvdetLatR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Reserved Reads return 0x0 and writes have no effect."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Latched Value of OV Detect 0 = OV Detect Not Triggered 1 = OV Detect has Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn core_ovdet_lat(&mut self) -> CoreOvdetLatW<AnaRegWuStatusRegLowvSpec> {
        CoreOvdetLatW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Latched Value of UV Detect 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn core_uvdet_lat(&mut self) -> CoreUvdetLatW<AnaRegWuStatusRegLowvSpec> {
        CoreUvdetLatW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Latched Value of UV Detect for 1.8V Baseband 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn vdda18bb_uv_det_lat(&mut self) -> Vdda18bbUvDetLatW<AnaRegWuStatusRegLowvSpec> {
        Vdda18bbUvDetLatW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Supp Detect output of CLK 1.8V 0 = Supply Not detected 1 = Supply Detected"]
    #[inline(always)]
    #[must_use]
    pub fn supp_ok_clk18(&mut self) -> SuppOkClk18W<AnaRegWuStatusRegLowvSpec> {
        SuppOkClk18W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Supp Detect output of IO 1.8V 0 = Supply Not detected 1 = Supply Detected"]
    #[inline(always)]
    #[must_use]
    pub fn supp_ok_io18(&mut self) -> SuppOkIo18W<AnaRegWuStatusRegLowvSpec> {
        SuppOkIo18W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Supp Detect output of IO 3.3V 0 = Supply Not detected 1 = Supply Detected"]
    #[inline(always)]
    #[must_use]
    pub fn supp_ok_io33(&mut self) -> SuppOkIo33W<AnaRegWuStatusRegLowvSpec> {
        SuppOkIo33W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Can be made spare (TiedLO internally)"]
    #[inline(always)]
    #[must_use]
    pub fn supp_ok_rf10(&mut self) -> SuppOkRf10W<AnaRegWuStatusRegLowvSpec> {
        SuppOkRf10W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Latched Value of UV Detect for 1V RF1 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn vdda10rf1_uvdet_lat(&mut self) -> Vdda10rf1UvdetLatW<AnaRegWuStatusRegLowvSpec> {
        Vdda10rf1UvdetLatW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Latched Value of UV Detect for 1V RF2 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn vdda10rf2_uvdet_lat(&mut self) -> Vdda10rf2UvdetLatW<AnaRegWuStatusRegLowvSpec> {
        Vdda10rf2UvdetLatW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Can be made spare (TiedLO internally)"]
    #[inline(always)]
    #[must_use]
    pub fn supp_ok_sram12(&mut self) -> SuppOkSram12W<AnaRegWuStatusRegLowvSpec> {
        SuppOkSram12W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Can be made spare (TiedLO internally)"]
    #[inline(always)]
    #[must_use]
    pub fn supp_ok_vddd18(&mut self) -> SuppOkVddd18W<AnaRegWuStatusRegLowvSpec> {
        SuppOkVddd18W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Ref CLK status at Wake-up 0 = REF CLK absent 1 = REF CLK Present"]
    #[inline(always)]
    #[must_use]
    pub fn ref_clk_status(&mut self) -> RefClkStatusW<AnaRegWuStatusRegLowvSpec> {
        RefClkStatusW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
RCOSC status at Wake-up 0 = RCOSC CLK absent 1 = RCOSC CLK Present"]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_clk_status(&mut self) -> RcoscClkStatusW<AnaRegWuStatusRegLowvSpec> {
        RcoscClkStatusW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
XTAL Detect status at Wake-up 0 = XTAL absent 1 = XTAL Present"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_det_status(&mut self) -> XtalDetStatusW<AnaRegWuStatusRegLowvSpec> {
        XtalDetStatusW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Ref CLK status at Wake-up 0 = REF CLK is present 1 = REF CLK is absent and CPU CLK Switched to RCOSC"]
    #[inline(always)]
    #[must_use]
    pub fn limp_mode_status(&mut self) -> LimpModeStatusW<AnaRegWuStatusRegLowvSpec> {
        LimpModeStatusW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
HVMODE Status from VMON 1 = 3.3V VIO 0 = 1.8V VIO"]
    #[inline(always)]
    #[must_use]
    pub fn hvmode(&mut self) -> HvmodeW<AnaRegWuStatusRegLowvSpec> {
        HvmodeW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Latched Value of UV Detect for 1.8V VCO 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn apllvco18_uvdet_lat(&mut self) -> Apllvco18UvdetLatW<AnaRegWuStatusRegLowvSpec> {
        Apllvco18UvdetLatW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Latched value of UV detect for 1.8V CLK 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn vdda_osc_uvdet_lat(&mut self) -> VddaOscUvdetLatW<AnaRegWuStatusRegLowvSpec> {
        VddaOscUvdetLatW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Latched Value of 3.3V IO UV Detect 0 = UV Detect Not Triggered 1 = UV Detect has Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn vdds_3p3v_uvdet_lat(&mut self) -> Vdds3p3vUvdetLatW<AnaRegWuStatusRegLowvSpec> {
        Vdds3p3vUvdetLatW::new(self, 18)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Reserved Reads return 0x0 and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<AnaRegWuStatusRegLowvSpec> {
        Reserved0W::new(self, 19)
    }
}
#[doc = "ANA_REG_WU_STATUS_REG_LOWV\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_wu_status_reg_lowv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_wu_status_reg_lowv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaRegWuStatusRegLowvSpec;
impl crate::RegisterSpec for AnaRegWuStatusRegLowvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_reg_wu_status_reg_lowv::R`](R) reader structure"]
impl crate::Readable for AnaRegWuStatusRegLowvSpec {}
#[doc = "`write(|w| ..)` method takes [`ana_reg_wu_status_reg_lowv::W`](W) writer structure"]
impl crate::Writable for AnaRegWuStatusRegLowvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANA_REG_WU_STATUS_REG_LOWV to value 0"]
impl crate::Resettable for AnaRegWuStatusRegLowvSpec {
    const RESET_VALUE: u32 = 0;
}
