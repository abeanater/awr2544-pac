#[doc = "Register `ANA_REG_REFSYS_TMUX_CTRL_LOWV` reader"]
pub type R = crate::R<AnaRegRefsysTmuxCtrlLowvSpec>;
#[doc = "Register `ANA_REG_REFSYS_TMUX_CTRL_LOWV` writer"]
pub type W = crate::W<AnaRegRefsysTmuxCtrlLowvSpec>;
#[doc = "Field `VREF_0P45V` reader - 0:0\\]
&lt;0> - VREF 0P45 (TMUX One-Hot) 0x0 = Functional Reset"]
pub type Vref0p45vR = crate::BitReader;
#[doc = "Field `VREF_0P45V` writer - 0:0\\]
&lt;0> - VREF 0P45 (TMUX One-Hot) 0x0 = Functional Reset"]
pub type Vref0p45vW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREF_0P9V` reader - 1:1\\]
&lt;1> - VREF 0P9V (Cap Node) (TMUX One-Hot) 0x0 = Functional Reset"]
pub type Vref0p9vR = crate::BitReader;
#[doc = "Field `VREF_0P9V` writer - 1:1\\]
&lt;1> - VREF 0P9V (Cap Node) (TMUX One-Hot) 0x0 = Functional Reset"]
pub type Vref0p9vW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBG_1P22V` reader - 2:2\\]
&lt;2> - VBG 1.22V (TMUX One-Hot) 0x0 = Functional Reset"]
pub type Vbg1p22vR = crate::BitReader;
#[doc = "Field `VBG_1P22V` writer - 2:2\\]
&lt;2> - VBG 1.22V (TMUX One-Hot) 0x0 = Functional Reset"]
pub type Vbg1p22vW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_IBIASP_20u` reader - 3:3\\]
&lt;3> RX IBG BIASP 20uA (TMUX One-Hot) 0x0 = Functional Reset"]
pub type RxIbiasp20uR = crate::BitReader;
#[doc = "Field `RX_IBIASP_20u` writer - 3:3\\]
&lt;3> RX IBG BIASP 20uA (TMUX One-Hot) 0x0 = Functional Reset"]
pub type RxIbiasp20uW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBE_WEAK` reader - 4:4\\]
&lt;4> - VBE Weak (TMUX One-Hot) 0x0 = Functional Reset"]
pub type VbeWeakR = crate::BitReader;
#[doc = "Field `VBE_WEAK` writer - 4:4\\]
&lt;4> - VBE Weak (TMUX One-Hot) 0x0 = Functional Reset"]
pub type VbeWeakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED0` reader - 5:5\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `RESERVED0` writer - 5:5\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
pub type Reserved0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBIASP_20U` reader - 6:6\\]
&lt;6> CLK IBG BIASP 20uA (TMUX One-Hot) 0x0 = Functional Reset"]
pub type Ibiasp20uR = crate::BitReader;
#[doc = "Field `IBIASP_20U` writer - 6:6\\]
&lt;6> CLK IBG BIASP 20uA (TMUX One-Hot) 0x0 = Functional Reset"]
pub type Ibiasp20uW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBIASP_TS_6U` reader - 7:7\\]
&lt;7> IBG BIASP TS 6uA (TMUX One-Hot) 0x0 = Functional Reset"]
pub type IbiaspTs6uR = crate::BitReader;
#[doc = "Field `IBIASP_TS_6U` writer - 7:7\\]
&lt;7> IBG BIASP TS 6uA (TMUX One-Hot) 0x0 = Functional Reset"]
pub type IbiaspTs6uW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 8:8\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `RESERVED1` writer - 8:8\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDIODEP_100U` reader - 9:9\\]
&lt;9> Idiode BIASP 100uA (TMUX One-Hot) 0x0 = Functional Reset"]
pub type Idiodep100uR = crate::BitReader;
#[doc = "Field `IDIODEP_100U` writer - 9:9\\]
&lt;9> Idiode BIASP 100uA (TMUX One-Hot) 0x0 = Functional Reset"]
pub type Idiodep100uW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IREFP_10UA` reader - 10:10\\]
&lt;10> IREFP 10uA (TMUX One-Hot) 0x0 = Functional Reset"]
pub type Irefp10uaR = crate::BitReader;
#[doc = "Field `IREFP_10UA` writer - 10:10\\]
&lt;10> IREFP 10uA (TMUX One-Hot) 0x0 = Functional Reset"]
pub type Irefp10uaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 11:11\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `RESERVED2` writer - 11:11\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2V_SENSE` reader - 12:12\\]
Sense Voltage from the BIST I2V conversion of 20u and 12u bias current paths Sense voltage of 1V for BIST select&lt;6> Sense voltage of 0.6V for BIST select&lt;7> 0x0 = Functional Reset"]
pub type I2vSenseR = crate::BitReader;
#[doc = "Field `I2V_SENSE` writer - 12:12\\]
Sense Voltage from the BIST I2V conversion of 20u and 12u bias current paths Sense voltage of 1V for BIST select&lt;6> Sense voltage of 0.6V for BIST select&lt;7> 0x0 = Functional Reset"]
pub type I2vSenseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPASS_MIRR_VPBIAS` reader - 13:13\\]
VPBIAS Control for IREF Gen Test Mode V2I By-Pass Feature 0x0 = Functional Reset"]
pub type BypassMirrVpbiasR = crate::BitReader;
#[doc = "Field `BYPASS_MIRR_VPBIAS` writer - 13:13\\]
VPBIAS Control for IREF Gen Test Mode V2I By-Pass Feature 0x0 = Functional Reset"]
pub type BypassMirrVpbiasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_IBIASP_20u` reader - 14:14\\]
&lt;14> TX IBG BIASP 20uA (TMUX One-Hot) 0x0 = Functional Reset"]
pub type TxIbiasp20uR = crate::BitReader;
#[doc = "Field `TX_IBIASP_20u` writer - 14:14\\]
&lt;14> TX IBG BIASP 20uA (TMUX One-Hot) 0x0 = Functional Reset"]
pub type TxIbiasp20uW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LO_IBIASP_20u` reader - 15:15\\]
&lt;15> LO IBG BIASP 20uA (TMUX One-Hot) 0x0 = Functional Reset"]
pub type LoIbiasp20uR = crate::BitReader;
#[doc = "Field `LO_IBIASP_20u` writer - 15:15\\]
&lt;15> LO IBG BIASP 20uA (TMUX One-Hot) 0x0 = Functional Reset"]
pub type LoIbiasp20uW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 30:16\\]
Reserved Reads return 0x0 and writes have no effect. 0x0000 = Functional Reset"]
pub type Reserved3R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED3` writer - 30:16\\]
Reserved Reads return 0x0 and writes have no effect. 0x0000 = Functional Reset"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `REFSYS_CTRL_8` reader - 31:31\\]
REFSYS Test Mux Enable. Other bits in Bus are One-hot. This control enabled in sync with other one hot control bits in Reg 0 = TMUX Disabled 1 = TMUX Enabled 0x0 = Functional Reset"]
pub type RefsysCtrl8R = crate::BitReader;
#[doc = "Field `REFSYS_CTRL_8` writer - 31:31\\]
REFSYS Test Mux Enable. Other bits in Bus are One-hot. This control enabled in sync with other one hot control bits in Reg 0 = TMUX Disabled 1 = TMUX Enabled 0x0 = Functional Reset"]
pub type RefsysCtrl8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
&lt;0> - VREF 0P45 (TMUX One-Hot) 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vref_0p45v(&self) -> Vref0p45vR {
        Vref0p45vR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
&lt;1> - VREF 0P9V (Cap Node) (TMUX One-Hot) 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vref_0p9v(&self) -> Vref0p9vR {
        Vref0p9vR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
&lt;2> - VBG 1.22V (TMUX One-Hot) 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vbg_1p22v(&self) -> Vbg1p22vR {
        Vbg1p22vR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
&lt;3> RX IBG BIASP 20uA (TMUX One-Hot) 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn rx_ibiasp_20u(&self) -> RxIbiasp20uR {
        RxIbiasp20uR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
&lt;4> - VBE Weak (TMUX One-Hot) 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn vbe_weak(&self) -> VbeWeakR {
        VbeWeakR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
&lt;6> CLK IBG BIASP 20uA (TMUX One-Hot) 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn ibiasp_20u(&self) -> Ibiasp20uR {
        Ibiasp20uR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
&lt;7> IBG BIASP TS 6uA (TMUX One-Hot) 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn ibiasp_ts_6u(&self) -> IbiaspTs6uR {
        IbiaspTs6uR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
&lt;9> Idiode BIASP 100uA (TMUX One-Hot) 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn idiodep_100u(&self) -> Idiodep100uR {
        Idiodep100uR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
&lt;10> IREFP 10uA (TMUX One-Hot) 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn irefp_10ua(&self) -> Irefp10uaR {
        Irefp10uaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Sense Voltage from the BIST I2V conversion of 20u and 12u bias current paths Sense voltage of 1V for BIST select&lt;6> Sense voltage of 0.6V for BIST select&lt;7> 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn i2v_sense(&self) -> I2vSenseR {
        I2vSenseR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
VPBIAS Control for IREF Gen Test Mode V2I By-Pass Feature 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn bypass_mirr_vpbias(&self) -> BypassMirrVpbiasR {
        BypassMirrVpbiasR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
&lt;14> TX IBG BIASP 20uA (TMUX One-Hot) 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn tx_ibiasp_20u(&self) -> TxIbiasp20uR {
        TxIbiasp20uR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
&lt;15> LO IBG BIASP 20uA (TMUX One-Hot) 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn lo_ibiasp_20u(&self) -> LoIbiasp20uR {
        LoIbiasp20uR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:30 - 30:16\\]
Reserved Reads return 0x0 and writes have no effect. 0x0000 = Functional Reset"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
REFSYS Test Mux Enable. Other bits in Bus are One-hot. This control enabled in sync with other one hot control bits in Reg 0 = TMUX Disabled 1 = TMUX Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    pub fn refsys_ctrl_8(&self) -> RefsysCtrl8R {
        RefsysCtrl8R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
&lt;0> - VREF 0P45 (TMUX One-Hot) 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vref_0p45v(&mut self) -> Vref0p45vW<AnaRegRefsysTmuxCtrlLowvSpec> {
        Vref0p45vW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
&lt;1> - VREF 0P9V (Cap Node) (TMUX One-Hot) 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vref_0p9v(&mut self) -> Vref0p9vW<AnaRegRefsysTmuxCtrlLowvSpec> {
        Vref0p9vW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
&lt;2> - VBG 1.22V (TMUX One-Hot) 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vbg_1p22v(&mut self) -> Vbg1p22vW<AnaRegRefsysTmuxCtrlLowvSpec> {
        Vbg1p22vW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
&lt;3> RX IBG BIASP 20uA (TMUX One-Hot) 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rx_ibiasp_20u(&mut self) -> RxIbiasp20uW<AnaRegRefsysTmuxCtrlLowvSpec> {
        RxIbiasp20uW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
&lt;4> - VBE Weak (TMUX One-Hot) 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn vbe_weak(&mut self) -> VbeWeakW<AnaRegRefsysTmuxCtrlLowvSpec> {
        VbeWeakW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<AnaRegRefsysTmuxCtrlLowvSpec> {
        Reserved0W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
&lt;6> CLK IBG BIASP 20uA (TMUX One-Hot) 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ibiasp_20u(&mut self) -> Ibiasp20uW<AnaRegRefsysTmuxCtrlLowvSpec> {
        Ibiasp20uW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
&lt;7> IBG BIASP TS 6uA (TMUX One-Hot) 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ibiasp_ts_6u(&mut self) -> IbiaspTs6uW<AnaRegRefsysTmuxCtrlLowvSpec> {
        IbiaspTs6uW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<AnaRegRefsysTmuxCtrlLowvSpec> {
        Reserved1W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
&lt;9> Idiode BIASP 100uA (TMUX One-Hot) 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn idiodep_100u(&mut self) -> Idiodep100uW<AnaRegRefsysTmuxCtrlLowvSpec> {
        Idiodep100uW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
&lt;10> IREFP 10uA (TMUX One-Hot) 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn irefp_10ua(&mut self) -> Irefp10uaW<AnaRegRefsysTmuxCtrlLowvSpec> {
        Irefp10uaW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Reserved Reads return 0x0 and writes have no effect. 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<AnaRegRefsysTmuxCtrlLowvSpec> {
        Reserved2W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Sense Voltage from the BIST I2V conversion of 20u and 12u bias current paths Sense voltage of 1V for BIST select&lt;6> Sense voltage of 0.6V for BIST select&lt;7> 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2v_sense(&mut self) -> I2vSenseW<AnaRegRefsysTmuxCtrlLowvSpec> {
        I2vSenseW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
VPBIAS Control for IREF Gen Test Mode V2I By-Pass Feature 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn bypass_mirr_vpbias(&mut self) -> BypassMirrVpbiasW<AnaRegRefsysTmuxCtrlLowvSpec> {
        BypassMirrVpbiasW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
&lt;14> TX IBG BIASP 20uA (TMUX One-Hot) 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn tx_ibiasp_20u(&mut self) -> TxIbiasp20uW<AnaRegRefsysTmuxCtrlLowvSpec> {
        TxIbiasp20uW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
&lt;15> LO IBG BIASP 20uA (TMUX One-Hot) 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn lo_ibiasp_20u(&mut self) -> LoIbiasp20uW<AnaRegRefsysTmuxCtrlLowvSpec> {
        LoIbiasp20uW::new(self, 15)
    }
    #[doc = "Bits 16:30 - 30:16\\]
Reserved Reads return 0x0 and writes have no effect. 0x0000 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<AnaRegRefsysTmuxCtrlLowvSpec> {
        Reserved3W::new(self, 16)
    }
    #[doc = "Bit 31 - 31:31\\]
REFSYS Test Mux Enable. Other bits in Bus are One-hot. This control enabled in sync with other one hot control bits in Reg 0 = TMUX Disabled 1 = TMUX Enabled 0x0 = Functional Reset"]
    #[inline(always)]
    #[must_use]
    pub fn refsys_ctrl_8(&mut self) -> RefsysCtrl8W<AnaRegRefsysTmuxCtrlLowvSpec> {
        RefsysCtrl8W::new(self, 31)
    }
}
#[doc = "ANA_REG_REFSYS_TMUX_CTRL_LOWV\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_reg_refsys_tmux_ctrl_lowv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_reg_refsys_tmux_ctrl_lowv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaRegRefsysTmuxCtrlLowvSpec;
impl crate::RegisterSpec for AnaRegRefsysTmuxCtrlLowvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_reg_refsys_tmux_ctrl_lowv::R`](R) reader structure"]
impl crate::Readable for AnaRegRefsysTmuxCtrlLowvSpec {}
#[doc = "`write(|w| ..)` method takes [`ana_reg_refsys_tmux_ctrl_lowv::W`](W) writer structure"]
impl crate::Writable for AnaRegRefsysTmuxCtrlLowvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANA_REG_REFSYS_TMUX_CTRL_LOWV to value 0"]
impl crate::Resettable for AnaRegRefsysTmuxCtrlLowvSpec {
    const RESET_VALUE: u32 = 0;
}
