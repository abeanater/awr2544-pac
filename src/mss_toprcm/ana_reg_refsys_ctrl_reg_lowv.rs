#[doc = "Register `ANA_REG_REFSYS_CTRL_REG_LOWV` reader"]
pub type R = crate::R<AnaRegRefsysCtrlRegLowvSpec>;
#[doc = "Register `ANA_REG_REFSYS_CTRL_REG_LOWV` writer"]
pub type W = crate::W<AnaRegRefsysCtrlRegLowvSpec>;
#[doc = "Field `REFSYS_BGAP_EN_CTRL` reader - 0:0\\]
REFSYS Enable Control 0 = Disable REFSYS 1 = Enable REFSYS 0x1 = Functional Reset"]
pub type RefsysBgapEnCtrlR = crate::BitReader;
#[doc = "Field `REFSYS_BGAP_EN_CTRL` writer - 0:0\\]
REFSYS Enable Control 0 = Disable REFSYS 1 = Enable REFSYS 0x1 = Functional Reset"]
pub type RefsysBgapEnCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFSYS_V2I_EN_CTRL` reader - 1:1\\]
REFSYS Enable Control 0 = Disable V2I REFSYS 1 = Enable V2I REFSYS 0x1 = Functional Reset"]
pub type RefsysV2iEnCtrlR = crate::BitReader;
#[doc = "Field `REFSYS_V2I_EN_CTRL` writer - 1:1\\]
REFSYS Enable Control 0 = Disable V2I REFSYS 1 = Enable V2I REFSYS 0x1 = Functional Reset"]
pub type RefsysV2iEnCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFSYS_CAP_SW_CTRLZ` reader - 2:2\\]
REFSYS Cap Switch Control 0 = Switch External Cap to reference output 1 = Disconnect External Cap to Reference output 0x0 = Functional Reset"]
pub type RefsysCapSwCtrlzR = crate::BitReader;
#[doc = "Field `REFSYS_CAP_SW_CTRLZ` writer - 2:2\\]
REFSYS Cap Switch Control 0 = Switch External Cap to reference output 1 = Disconnect External Cap to Reference output 0x0 = Functional Reset"]
pub type RefsysCapSwCtrlzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFSYS_PRE_CHARGE` reader - 3:3\\]
REFSYS Pre Charge Control 0 = Disable Pre Charge Block 1 = Enable Pre Charge Block 0x0 = Functional Reset"]
pub type RefsysPreChargeR = crate::BitReader;
#[doc = "Field `REFSYS_PRE_CHARGE` writer - 3:3\\]
REFSYS Pre Charge Control 0 = Disable Pre Charge Block 1 = Enable Pre Charge Block 0x0 = Functional Reset"]
pub type RefsysPreChargeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOPE_TRIM_4_0` reader - 8:4\\]
Default Slope Trim for NOM LOT 0x0D = Functional Reset"]
pub type SlopeTrim4_0R = crate::FieldReader;
#[doc = "Field `SLOPE_TRIM_4_0` writer - 8:4\\]
Default Slope Trim for NOM LOT 0x0D = Functional Reset"]
pub type SlopeTrim4_0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MAG_TRIM_4_0` reader - 13:9\\]
Default Magnitude Trim for NOM LOT 0x00 = Functional Reset"]
pub type MagTrim4_0R = crate::FieldReader;
#[doc = "Field `MAG_TRIM_4_0` writer - 13:9\\]
Default Magnitude Trim for NOM LOT 0x00 = Functional Reset"]
pub type MagTrim4_0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IREF_TRIM_4_0` reader - 18:14\\]
Default Resistor Trim for NOM LOT 0x02 = Functional Reset"]
pub type IrefTrim4_0R = crate::FieldReader;
#[doc = "Field `IREF_TRIM_4_0` writer - 18:14\\]
Default Resistor Trim for NOM LOT 0x02 = Functional Reset"]
pub type IrefTrim4_0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BGAP_ISW` reader - 19:19\\]
&lt;0> BGAP ISW STARTUP 0x0 = Functional Reset"]
pub type BgapIswR = crate::BitReader;
#[doc = "Field `BGAP_ISW` writer - 19:19\\]
&lt;0> BGAP ISW STARTUP 0x0 = Functional Reset"]
pub type BgapIswW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `V2I_STARTUP` reader - 20:20\\]
&lt;1> V2I Startup 0x0 = Functional Reset"]
pub type V2iStartupR = crate::BitReader;
#[doc = "Field `V2I_STARTUP` writer - 20:20\\]
&lt;1> V2I Startup 0x0 = Functional Reset"]
pub type V2iStartupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKTOP_IBIAS_EN` reader - 21:21\\]
&lt;2> CLK TOP IBIAS EN 0x1 = Functional Reset"]
pub type ClktopIbiasEnR = crate::BitReader;
#[doc = "Field `CLKTOP_IBIAS_EN` writer - 21:21\\]
&lt;2> CLK TOP IBIAS EN 0x1 = Functional Reset"]
pub type ClktopIbiasEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DO_NOT_USE0` reader - 22:22\\]
Do not use this bit --> mapped to REFSYS_CTRL_REG&lt;22> 0x0 = Functional Reset"]
pub type DoNotUse0R = crate::BitReader;
#[doc = "Field `DO_NOT_USE0` writer - 22:22\\]
Do not use this bit --> mapped to REFSYS_CTRL_REG&lt;22> 0x0 = Functional Reset"]
pub type DoNotUse0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DO_NOT_USE1` reader - 23:23\\]
Do not use this bit --> mapped to REFSYS_CTRL_REG&lt;23> 0x0 = Functional Reset"]
pub type DoNotUse1R = crate::BitReader;
#[doc = "Field `DO_NOT_USE1` writer - 23:23\\]
Do not use this bit --> mapped to REFSYS_CTRL_REG&lt;23> 0x0 = Functional Reset"]
pub type DoNotUse1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFSYS_BYPASS_EN` reader - 24:24\\]
&lt;5> REFSYS By-Pass Enable 0x0 = Functional Reset"]
pub type RefsysBypassEnR = crate::BitReader;
#[doc = "Field `REFSYS_BYPASS_EN` writer - 24:24\\]
&lt;5> REFSYS By-Pass Enable 0x0 = Functional Reset"]
pub type RefsysBypassEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DO_NOT_USE2` reader - 25:25\\]
Do not use this bit --> mapped to REFSYS_CTRL_REG&lt;25> 0x0 = Functional Reset"]
pub type DoNotUse2R = crate::BitReader;
#[doc = "Field `DO_NOT_USE2` writer - 25:25\\]
Do not use this bit --> mapped to REFSYS_CTRL_REG&lt;25> 0x0 = Functional Reset"]
pub type DoNotUse2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DO_NOT_USE3` reader - 26:26\\]
Do not use this bit --> mapped to REFSYS_CTRL_REG&lt;26> 0x0 = Functional Reset"]
pub type DoNotUse3R = crate::BitReader;
#[doc = "Field `DO_NOT_USE3` writer - 26:26\\]
Do not use this bit --> mapped to REFSYS_CTRL_REG&lt;26> 0x0 = Functional Reset"]
pub type DoNotUse3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTRIM_3_0` reader - 30:27\\]
Filter TRIM Control 0x0 = Functional Reset"]
pub type Ftrim3_0R = crate::FieldReader;
#[doc = "Field `FTRIM_3_0` writer - 30:27\\]
Filter TRIM Control 0x0 = Functional Reset"]
pub type Ftrim3_0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED0` reader - 31:31\\]
Reserved 0x0 = Functional Reset"]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `RESERVED0` writer - 31:31\\]
Reserved 0x0 = Functional Reset"]
pub type Reserved0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
REFSYS Enable Control 0 = Disable REFSYS 1 = Enable REFSYS 0x1 = Functional Reset"]
    #[inline(always)]
    pub fn refsys_bgap_en_ctrl(&self) -> RefsysBgapEnCtrlR {
        RefsysBgapEnCtrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
REFSYS Enable Control 0 = Disable V2I REFSYS 1 = Enable V2I REFSYS 0x1 = Functional Reset"]
    #[inline(always)]
    pub fn refsys_v2i_en_ctrl(&self) -> RefsysV2iEnCtrlR {
        RefsysV2iEnCtrlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
REFSYS Cap Switch Control 0 = Switch External Cap to reference output 1 = Disconnect External Cap to Reference output 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn refsys_cap_sw_ctrlz(&self) -> RefsysCapSwCtrlzR {
        RefsysCapSwCtrlzR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
REFSYS Pre Charge Control 0 = Disable Pre Charge Block 1 = Enable Pre Charge Block 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn refsys_pre_charge(&self) -> RefsysPreChargeR {
        RefsysPreChargeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:8 - 8:4\\]
Default Slope Trim for NOM LOT 0x0D = Functional Reset"]
    #[inline(always)]
    pub fn slope_trim_4_0(&self) -> SlopeTrim4_0R {
        SlopeTrim4_0R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:13 - 13:9\\]
Default Magnitude Trim for NOM LOT 0x00 = Functional Reset"]
    #[inline(always)]
    pub fn mag_trim_4_0(&self) -> MagTrim4_0R {
        MagTrim4_0R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 14:18 - 18:14\\]
Default Resistor Trim for NOM LOT 0x02 = Functional Reset"]
    #[inline(always)]
    pub fn iref_trim_4_0(&self) -> IrefTrim4_0R {
        IrefTrim4_0R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bit 19 - 19:19\\]
&lt;0> BGAP ISW STARTUP 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn bgap_isw(&self) -> BgapIswR {
        BgapIswR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
&lt;1> V2I Startup 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn v2i_startup(&self) -> V2iStartupR {
        V2iStartupR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
&lt;2> CLK TOP IBIAS EN 0x1 = Functional Reset"]
    #[inline(always)]
    pub fn clktop_ibias_en(&self) -> ClktopIbiasEnR {
        ClktopIbiasEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Do not use this bit --> mapped to REFSYS_CTRL_REG&lt;22> 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn do_not_use0(&self) -> DoNotUse0R {
        DoNotUse0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Do not use this bit --> mapped to REFSYS_CTRL_REG&lt;23> 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn do_not_use1(&self) -> DoNotUse1R {
        DoNotUse1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
&lt;5> REFSYS By-Pass Enable 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn refsys_bypass_en(&self) -> RefsysBypassEnR {
        RefsysBypassEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Do not use this bit --> mapped to REFSYS_CTRL_REG&lt;25> 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn do_not_use2(&self) -> DoNotUse2R {
        DoNotUse2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Do not use this bit --> mapped to REFSYS_CTRL_REG&lt;26> 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn do_not_use3(&self) -> DoNotUse3R {
        DoNotUse3R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:30 - 30:27\\]
Filter TRIM Control 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn ftrim_3_0(&self) -> Ftrim3_0R {
        Ftrim3_0R::new(((self.bits >> 27) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Reserved 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
REFSYS Enable Control 0 = Disable REFSYS 1 = Enable REFSYS 0x1 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn refsys_bgap_en_ctrl(&mut self) -> RefsysBgapEnCtrlW<AnaRegRefsysCtrlRegLowvSpec> {
        RefsysBgapEnCtrlW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
REFSYS Enable Control 0 = Disable V2I REFSYS 1 = Enable V2I REFSYS 0x1 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn refsys_v2i_en_ctrl(&mut self) -> RefsysV2iEnCtrlW<AnaRegRefsysCtrlRegLowvSpec> {
        RefsysV2iEnCtrlW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
REFSYS Cap Switch Control 0 = Switch External Cap to reference output 1 = Disconnect External Cap to Reference output 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn refsys_cap_sw_ctrlz(&mut self) -> RefsysCapSwCtrlzW<AnaRegRefsysCtrlRegLowvSpec> {
        RefsysCapSwCtrlzW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
REFSYS Pre Charge Control 0 = Disable Pre Charge Block 1 = Enable Pre Charge Block 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn refsys_pre_charge(&mut self) -> RefsysPreChargeW<AnaRegRefsysCtrlRegLowvSpec> {
        RefsysPreChargeW::new(self, 3)
    }
    #[doc = "Bits 4:8 - 8:4\\]
Default Slope Trim for NOM LOT 0x0D = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn slope_trim_4_0(&mut self) -> SlopeTrim4_0W<AnaRegRefsysCtrlRegLowvSpec> {
        SlopeTrim4_0W::new(self, 4)
    }
    #[doc = "Bits 9:13 - 13:9\\]
Default Magnitude Trim for NOM LOT 0x00 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn mag_trim_4_0(&mut self) -> MagTrim4_0W<AnaRegRefsysCtrlRegLowvSpec> {
        MagTrim4_0W::new(self, 9)
    }
    #[doc = "Bits 14:18 - 18:14\\]
Default Resistor Trim for NOM LOT 0x02 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn iref_trim_4_0(&mut self) -> IrefTrim4_0W<AnaRegRefsysCtrlRegLowvSpec> {
        IrefTrim4_0W::new(self, 14)
    }
    #[doc = "Bit 19 - 19:19\\]
&lt;0> BGAP ISW STARTUP 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn bgap_isw(&mut self) -> BgapIswW<AnaRegRefsysCtrlRegLowvSpec> {
        BgapIswW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
&lt;1> V2I Startup 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn v2i_startup(&mut self) -> V2iStartupW<AnaRegRefsysCtrlRegLowvSpec> {
        V2iStartupW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
&lt;2> CLK TOP IBIAS EN 0x1 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn clktop_ibias_en(&mut self) -> ClktopIbiasEnW<AnaRegRefsysCtrlRegLowvSpec> {
        ClktopIbiasEnW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Do not use this bit --> mapped to REFSYS_CTRL_REG&lt;22> 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn do_not_use0(&mut self) -> DoNotUse0W<AnaRegRefsysCtrlRegLowvSpec> {
        DoNotUse0W::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Do not use this bit --> mapped to REFSYS_CTRL_REG&lt;23> 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn do_not_use1(&mut self) -> DoNotUse1W<AnaRegRefsysCtrlRegLowvSpec> {
        DoNotUse1W::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
&lt;5> REFSYS By-Pass Enable 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn refsys_bypass_en(&mut self) -> RefsysBypassEnW<AnaRegRefsysCtrlRegLowvSpec> {
        RefsysBypassEnW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Do not use this bit --> mapped to REFSYS_CTRL_REG&lt;25> 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn do_not_use2(&mut self) -> DoNotUse2W<AnaRegRefsysCtrlRegLowvSpec> {
        DoNotUse2W::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Do not use this bit --> mapped to REFSYS_CTRL_REG&lt;26> 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn do_not_use3(&mut self) -> DoNotUse3W<AnaRegRefsysCtrlRegLowvSpec> {
        DoNotUse3W::new(self, 26)
    }
    #[doc = "Bits 27:30 - 30:27\\]
Filter TRIM Control 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ftrim_3_0(&mut self) -> Ftrim3_0W<AnaRegRefsysCtrlRegLowvSpec> {
        Ftrim3_0W::new(self, 27)
    }
    #[doc = "Bit 31 - 31:31\\]
Reserved 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<AnaRegRefsysCtrlRegLowvSpec> {
        Reserved0W::new(self, 31)
    }
}
#[doc = "ANA_REG_REFSYS_CTRL_REG_LOWV\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_refsys_ctrl_reg_lowv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_refsys_ctrl_reg_lowv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaRegRefsysCtrlRegLowvSpec;
impl crate::RegisterSpec for AnaRegRefsysCtrlRegLowvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_reg_refsys_ctrl_reg_lowv::R`](R) reader structure"]
impl crate::Readable for AnaRegRefsysCtrlRegLowvSpec {}
#[doc = "`write(|w| ..)` method takes [`ana_reg_refsys_ctrl_reg_lowv::W`](W) writer structure"]
impl crate::Writable for AnaRegRefsysCtrlRegLowvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANA_REG_REFSYS_CTRL_REG_LOWV to value 0"]
impl crate::Resettable for AnaRegRefsysCtrlRegLowvSpec {
    const RESET_VALUE: u32 = 0;
}
