#[doc = "Register `CSI2_TX_CFG1` reader"]
pub type R = crate::R<Csi2TxCfg1Spec>;
#[doc = "Register `CSI2_TX_CFG1` writer"]
pub type W = crate::W<Csi2TxCfg1Spec>;
#[doc = "Field `OCPCLKEN` reader - 0:0\\]
Connected to dsi_protocol port name csi_pi_ocp_clken"]
pub type OcpclkenR = crate::BitReader;
#[doc = "Field `OCPCLKEN` writer - 0:0\\]
Connected to dsi_protocol port name csi_pi_ocp_clken"]
pub type OcpclkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOPCLKDIVL3SCP` reader - 2:1\\]
Connected to dsi_protocol port name csi_pi_top_clkdiv_l3scp"]
pub type Topclkdivl3scpR = crate::FieldReader;
#[doc = "Field `TOPCLKDIVL3SCP` writer - 2:1\\]
Connected to dsi_protocol port name csi_pi_top_clkdiv_l3scp"]
pub type Topclkdivl3scpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOPCLKDIVDSI` reader - 4:3\\]
Connected to dsi_protocol port name csi_pi_top_clkdiv_dsi"]
pub type TopclkdivdsiR = crate::FieldReader;
#[doc = "Field `TOPCLKDIVDSI` writer - 4:3\\]
Connected to dsi_protocol port name csi_pi_top_clkdiv_dsi"]
pub type TopclkdivdsiW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXBYTECLKHS` reader - 5:5\\]
Connected to dsi_protocol port name csi_pi_top_clkdiv_txbyteclkhs"]
pub type TxbyteclkhsR = crate::BitReader;
#[doc = "Field `TXBYTECLKHS` writer - 5:5\\]
Connected to dsi_protocol port name csi_pi_top_clkdiv_txbyteclkhs"]
pub type TxbyteclkhsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCPBUSY` reader - 6:6\\]
Connected to dsi_protocol port name csi_pi_scpbusy"]
pub type ScpbusyR = crate::BitReader;
#[doc = "Field `SCPBUSY` writer - 6:6\\]
Connected to dsi_protocol port name csi_pi_scpbusy"]
pub type ScpbusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VPPCLKEN` reader - 7:7\\]
Connected to dsi_protocol port name mem_vp_pclk_en"]
pub type VppclkenR = crate::BitReader;
#[doc = "Field `VPPCLKEN` writer - 7:7\\]
Connected to dsi_protocol port name mem_vp_pclk_en"]
pub type VppclkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VPCLKEN` reader - 8:8\\]
Connected to dsi_protocol port name mem_vp_clk_en"]
pub type VpclkenR = crate::BitReader;
#[doc = "Field `VPCLKEN` writer - 8:8\\]
Connected to dsi_protocol port name mem_vp_clk_en"]
pub type VpclkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIDLEREQ` reader - 9:9\\]
Connected to dsi_protocol port name csi_pi_midlereq"]
pub type MidlereqR = crate::BitReader;
#[doc = "Field `MIDLEREQ` writer - 9:9\\]
Connected to dsi_protocol port name csi_pi_midlereq"]
pub type MidlereqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUMOFLANES` reader - 11:10\\]
Connected to dsi_protocol port name csi_numoflanes"]
pub type NumoflanesR = crate::FieldReader;
#[doc = "Field `NUMOFLANES` writer - 11:10\\]
Connected to dsi_protocol port name csi_numoflanes"]
pub type NumoflanesW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LANEENABLE` reader - 16:12\\]
Connected to dsi_protocol port name csi_laneenable"]
pub type LaneenableR = crate::FieldReader;
#[doc = "Field `LANEENABLE` writer - 16:12\\]
Connected to dsi_protocol port name csi_laneenable"]
pub type LaneenableW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SIDLEACK` reader - 17:17\\]
Connected to dsi_protocol port name csi_po_sidleack"]
pub type SidleackR = crate::BitReader;
#[doc = "Field `SIDLEACK` writer - 17:17\\]
Connected to dsi_protocol port name csi_po_sidleack"]
pub type SidleackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAKEUP` reader - 18:18\\]
Connected to dsi_protocol port name csi_po_swakeup"]
pub type SwakeupR = crate::BitReader;
#[doc = "Field `SWAKEUP` writer - 18:18\\]
Connected to dsi_protocol port name csi_po_swakeup"]
pub type SwakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIPD0` reader - 19:19\\]
Connected to dsi_protocol port name csi_pipd0"]
pub type Pipd0R = crate::BitReader;
#[doc = "Field `PIPD0` writer - 19:19\\]
Connected to dsi_protocol port name csi_pipd0"]
pub type Pipd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIPD1` reader - 20:20\\]
Connected to dsi_protocol port name csi_pipd1"]
pub type Pipd1R = crate::BitReader;
#[doc = "Field `PIPD1` writer - 20:20\\]
Connected to dsi_protocol port name csi_pipd1"]
pub type Pipd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIPD2` reader - 21:21\\]
Connected to dsi_protocol port name csi_pipd2"]
pub type Pipd2R = crate::BitReader;
#[doc = "Field `PIPD2` writer - 21:21\\]
Connected to dsi_protocol port name csi_pipd2"]
pub type Pipd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIPD3` reader - 22:22\\]
Connected to dsi_protocol port name csi_pipd3"]
pub type Pipd3R = crate::BitReader;
#[doc = "Field `PIPD3` writer - 22:22\\]
Connected to dsi_protocol port name csi_pipd3"]
pub type Pipd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIPD4` reader - 23:23\\]
Connected to dsi_protocol port name csi_pipd4"]
pub type Pipd4R = crate::BitReader;
#[doc = "Field `PIPD4` writer - 23:23\\]
Connected to dsi_protocol port name csi_pipd4"]
pub type Pipd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFFMODE` reader - 24:24\\]
Connected to dsi_protocol port name csi_offmode"]
pub type OffmodeR = crate::BitReader;
#[doc = "Field `OFFMODE` writer - 24:24\\]
Connected to dsi_protocol port name csi_offmode"]
pub type OffmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HHV` reader - 25:25\\]
Connected to dsi_protocol port name csi_hhv"]
pub type HhvR = crate::BitReader;
#[doc = "Field `HHV` writer - 25:25\\]
Connected to dsi_protocol port name csi_hhv"]
pub type HhvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPASSEN` reader - 26:26\\]
Connected to dsi_protocol port name csi_bypassen"]
pub type BypassenR = crate::BitReader;
#[doc = "Field `BYPASSEN` writer - 26:26\\]
Connected to dsi_protocol port name csi_bypassen"]
pub type BypassenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE0LINE` reader - 27:27\\]
Connected to dsi_protocol port name csi_te0line"]
pub type Te0lineR = crate::BitReader;
#[doc = "Field `TE0LINE` writer - 27:27\\]
Connected to dsi_protocol port name csi_te0line"]
pub type Te0lineW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE1LINE` reader - 28:28\\]
Connected to dsi_protocol port name csi_te1line"]
pub type Te1lineR = crate::BitReader;
#[doc = "Field `TE1LINE` writer - 28:28\\]
Connected to dsi_protocol port name csi_te1line"]
pub type Te1lineW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISPCUPDATESYNC` reader - 29:29\\]
Connected to dsi_protocol port name csi_dispcupdatesync"]
pub type DispcupdatesyncR = crate::BitReader;
#[doc = "Field `DISPCUPDATESYNC` writer - 29:29\\]
Connected to dsi_protocol port name csi_dispcupdatesync"]
pub type DispcupdatesyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIOLDOPWRGOOD` reader - 30:30\\]
Connected to dsi_protocol port name csi_cioldopwrgood"]
pub type CioldopwrgoodR = crate::BitReader;
#[doc = "Field `CIOLDOPWRGOOD` writer - 30:30\\]
Connected to dsi_protocol port name csi_cioldopwrgood"]
pub type CioldopwrgoodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIOUSELDOEXTERNAL` reader - 31:31\\]
Connected to dsi_protocol port name csi_ciouseldoexternal"]
pub type CiouseldoexternalR = crate::BitReader;
#[doc = "Field `CIOUSELDOEXTERNAL` writer - 31:31\\]
Connected to dsi_protocol port name csi_ciouseldoexternal"]
pub type CiouseldoexternalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Connected to dsi_protocol port name csi_pi_ocp_clken"]
    #[inline(always)]
    pub fn ocpclken(&self) -> OcpclkenR {
        OcpclkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Connected to dsi_protocol port name csi_pi_top_clkdiv_l3scp"]
    #[inline(always)]
    pub fn topclkdivl3scp(&self) -> Topclkdivl3scpR {
        Topclkdivl3scpR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Connected to dsi_protocol port name csi_pi_top_clkdiv_dsi"]
    #[inline(always)]
    pub fn topclkdivdsi(&self) -> TopclkdivdsiR {
        TopclkdivdsiR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Connected to dsi_protocol port name csi_pi_top_clkdiv_txbyteclkhs"]
    #[inline(always)]
    pub fn txbyteclkhs(&self) -> TxbyteclkhsR {
        TxbyteclkhsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Connected to dsi_protocol port name csi_pi_scpbusy"]
    #[inline(always)]
    pub fn scpbusy(&self) -> ScpbusyR {
        ScpbusyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Connected to dsi_protocol port name mem_vp_pclk_en"]
    #[inline(always)]
    pub fn vppclken(&self) -> VppclkenR {
        VppclkenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Connected to dsi_protocol port name mem_vp_clk_en"]
    #[inline(always)]
    pub fn vpclken(&self) -> VpclkenR {
        VpclkenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Connected to dsi_protocol port name csi_pi_midlereq"]
    #[inline(always)]
    pub fn midlereq(&self) -> MidlereqR {
        MidlereqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Connected to dsi_protocol port name csi_numoflanes"]
    #[inline(always)]
    pub fn numoflanes(&self) -> NumoflanesR {
        NumoflanesR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:16 - 16:12\\]
Connected to dsi_protocol port name csi_laneenable"]
    #[inline(always)]
    pub fn laneenable(&self) -> LaneenableR {
        LaneenableR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bit 17 - 17:17\\]
Connected to dsi_protocol port name csi_po_sidleack"]
    #[inline(always)]
    pub fn sidleack(&self) -> SidleackR {
        SidleackR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Connected to dsi_protocol port name csi_po_swakeup"]
    #[inline(always)]
    pub fn swakeup(&self) -> SwakeupR {
        SwakeupR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Connected to dsi_protocol port name csi_pipd0"]
    #[inline(always)]
    pub fn pipd0(&self) -> Pipd0R {
        Pipd0R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Connected to dsi_protocol port name csi_pipd1"]
    #[inline(always)]
    pub fn pipd1(&self) -> Pipd1R {
        Pipd1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Connected to dsi_protocol port name csi_pipd2"]
    #[inline(always)]
    pub fn pipd2(&self) -> Pipd2R {
        Pipd2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Connected to dsi_protocol port name csi_pipd3"]
    #[inline(always)]
    pub fn pipd3(&self) -> Pipd3R {
        Pipd3R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Connected to dsi_protocol port name csi_pipd4"]
    #[inline(always)]
    pub fn pipd4(&self) -> Pipd4R {
        Pipd4R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Connected to dsi_protocol port name csi_offmode"]
    #[inline(always)]
    pub fn offmode(&self) -> OffmodeR {
        OffmodeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Connected to dsi_protocol port name csi_hhv"]
    #[inline(always)]
    pub fn hhv(&self) -> HhvR {
        HhvR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Connected to dsi_protocol port name csi_bypassen"]
    #[inline(always)]
    pub fn bypassen(&self) -> BypassenR {
        BypassenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Connected to dsi_protocol port name csi_te0line"]
    #[inline(always)]
    pub fn te0line(&self) -> Te0lineR {
        Te0lineR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Connected to dsi_protocol port name csi_te1line"]
    #[inline(always)]
    pub fn te1line(&self) -> Te1lineR {
        Te1lineR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Connected to dsi_protocol port name csi_dispcupdatesync"]
    #[inline(always)]
    pub fn dispcupdatesync(&self) -> DispcupdatesyncR {
        DispcupdatesyncR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Connected to dsi_protocol port name csi_cioldopwrgood"]
    #[inline(always)]
    pub fn cioldopwrgood(&self) -> CioldopwrgoodR {
        CioldopwrgoodR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Connected to dsi_protocol port name csi_ciouseldoexternal"]
    #[inline(always)]
    pub fn ciouseldoexternal(&self) -> CiouseldoexternalR {
        CiouseldoexternalR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Connected to dsi_protocol port name csi_pi_ocp_clken"]
    #[inline(always)]
    #[must_use]
    pub fn ocpclken(&mut self) -> OcpclkenW<Csi2TxCfg1Spec> {
        OcpclkenW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Connected to dsi_protocol port name csi_pi_top_clkdiv_l3scp"]
    #[inline(always)]
    #[must_use]
    pub fn topclkdivl3scp(&mut self) -> Topclkdivl3scpW<Csi2TxCfg1Spec> {
        Topclkdivl3scpW::new(self, 1)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Connected to dsi_protocol port name csi_pi_top_clkdiv_dsi"]
    #[inline(always)]
    #[must_use]
    pub fn topclkdivdsi(&mut self) -> TopclkdivdsiW<Csi2TxCfg1Spec> {
        TopclkdivdsiW::new(self, 3)
    }
    #[doc = "Bit 5 - 5:5\\]
Connected to dsi_protocol port name csi_pi_top_clkdiv_txbyteclkhs"]
    #[inline(always)]
    #[must_use]
    pub fn txbyteclkhs(&mut self) -> TxbyteclkhsW<Csi2TxCfg1Spec> {
        TxbyteclkhsW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Connected to dsi_protocol port name csi_pi_scpbusy"]
    #[inline(always)]
    #[must_use]
    pub fn scpbusy(&mut self) -> ScpbusyW<Csi2TxCfg1Spec> {
        ScpbusyW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Connected to dsi_protocol port name mem_vp_pclk_en"]
    #[inline(always)]
    #[must_use]
    pub fn vppclken(&mut self) -> VppclkenW<Csi2TxCfg1Spec> {
        VppclkenW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Connected to dsi_protocol port name mem_vp_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn vpclken(&mut self) -> VpclkenW<Csi2TxCfg1Spec> {
        VpclkenW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Connected to dsi_protocol port name csi_pi_midlereq"]
    #[inline(always)]
    #[must_use]
    pub fn midlereq(&mut self) -> MidlereqW<Csi2TxCfg1Spec> {
        MidlereqW::new(self, 9)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Connected to dsi_protocol port name csi_numoflanes"]
    #[inline(always)]
    #[must_use]
    pub fn numoflanes(&mut self) -> NumoflanesW<Csi2TxCfg1Spec> {
        NumoflanesW::new(self, 10)
    }
    #[doc = "Bits 12:16 - 16:12\\]
Connected to dsi_protocol port name csi_laneenable"]
    #[inline(always)]
    #[must_use]
    pub fn laneenable(&mut self) -> LaneenableW<Csi2TxCfg1Spec> {
        LaneenableW::new(self, 12)
    }
    #[doc = "Bit 17 - 17:17\\]
Connected to dsi_protocol port name csi_po_sidleack"]
    #[inline(always)]
    #[must_use]
    pub fn sidleack(&mut self) -> SidleackW<Csi2TxCfg1Spec> {
        SidleackW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Connected to dsi_protocol port name csi_po_swakeup"]
    #[inline(always)]
    #[must_use]
    pub fn swakeup(&mut self) -> SwakeupW<Csi2TxCfg1Spec> {
        SwakeupW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Connected to dsi_protocol port name csi_pipd0"]
    #[inline(always)]
    #[must_use]
    pub fn pipd0(&mut self) -> Pipd0W<Csi2TxCfg1Spec> {
        Pipd0W::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Connected to dsi_protocol port name csi_pipd1"]
    #[inline(always)]
    #[must_use]
    pub fn pipd1(&mut self) -> Pipd1W<Csi2TxCfg1Spec> {
        Pipd1W::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Connected to dsi_protocol port name csi_pipd2"]
    #[inline(always)]
    #[must_use]
    pub fn pipd2(&mut self) -> Pipd2W<Csi2TxCfg1Spec> {
        Pipd2W::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Connected to dsi_protocol port name csi_pipd3"]
    #[inline(always)]
    #[must_use]
    pub fn pipd3(&mut self) -> Pipd3W<Csi2TxCfg1Spec> {
        Pipd3W::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Connected to dsi_protocol port name csi_pipd4"]
    #[inline(always)]
    #[must_use]
    pub fn pipd4(&mut self) -> Pipd4W<Csi2TxCfg1Spec> {
        Pipd4W::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Connected to dsi_protocol port name csi_offmode"]
    #[inline(always)]
    #[must_use]
    pub fn offmode(&mut self) -> OffmodeW<Csi2TxCfg1Spec> {
        OffmodeW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Connected to dsi_protocol port name csi_hhv"]
    #[inline(always)]
    #[must_use]
    pub fn hhv(&mut self) -> HhvW<Csi2TxCfg1Spec> {
        HhvW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Connected to dsi_protocol port name csi_bypassen"]
    #[inline(always)]
    #[must_use]
    pub fn bypassen(&mut self) -> BypassenW<Csi2TxCfg1Spec> {
        BypassenW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Connected to dsi_protocol port name csi_te0line"]
    #[inline(always)]
    #[must_use]
    pub fn te0line(&mut self) -> Te0lineW<Csi2TxCfg1Spec> {
        Te0lineW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Connected to dsi_protocol port name csi_te1line"]
    #[inline(always)]
    #[must_use]
    pub fn te1line(&mut self) -> Te1lineW<Csi2TxCfg1Spec> {
        Te1lineW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Connected to dsi_protocol port name csi_dispcupdatesync"]
    #[inline(always)]
    #[must_use]
    pub fn dispcupdatesync(&mut self) -> DispcupdatesyncW<Csi2TxCfg1Spec> {
        DispcupdatesyncW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Connected to dsi_protocol port name csi_cioldopwrgood"]
    #[inline(always)]
    #[must_use]
    pub fn cioldopwrgood(&mut self) -> CioldopwrgoodW<Csi2TxCfg1Spec> {
        CioldopwrgoodW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Connected to dsi_protocol port name csi_ciouseldoexternal"]
    #[inline(always)]
    #[must_use]
    pub fn ciouseldoexternal(&mut self) -> CiouseldoexternalW<Csi2TxCfg1Spec> {
        CiouseldoexternalW::new(self, 31)
    }
}
#[doc = "CSI2_TX_CFG1\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_tx_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_tx_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2TxCfg1Spec;
impl crate::RegisterSpec for Csi2TxCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_tx_cfg1::R`](R) reader structure"]
impl crate::Readable for Csi2TxCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`csi2_tx_cfg1::W`](W) writer structure"]
impl crate::Writable for Csi2TxCfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_TX_CFG1 to value 0"]
impl crate::Resettable for Csi2TxCfg1Spec {
    const RESET_VALUE: u32 = 0;
}
