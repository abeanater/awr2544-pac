#[doc = "Register `CSI2_TX_CFG2` reader"]
pub type R = crate::R<Csi2TxCfg2Spec>;
#[doc = "Register `CSI2_TX_CFG2` writer"]
pub type W = crate::W<Csi2TxCfg2Spec>;
#[doc = "Field `SCPINPLLCRTL` reader - 0:0\\]
Connected to dsi_protocol port name csi_scpinpllcrtl"]
pub type ScpinpllcrtlR = crate::BitReader;
#[doc = "Field `SCPINPLLCRTL` writer - 0:0\\]
Connected to dsi_protocol port name csi_scpinpllcrtl"]
pub type ScpinpllcrtlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSILOCK` reader - 1:1\\]
Connected to dsi_protocol port name csi_dsilock"]
pub type DsilockR = crate::BitReader;
#[doc = "Field `DSILOCK` writer - 1:1\\]
Connected to dsi_protocol port name csi_dsilock"]
pub type DsilockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSIRECAL` reader - 2:2\\]
Connected to dsi_protocol port name csi_dsirecal"]
pub type DsirecalR = crate::BitReader;
#[doc = "Field `DSIRECAL` writer - 2:2\\]
Connected to dsi_protocol port name csi_dsirecal"]
pub type DsirecalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLPWRACK` reader - 3:3\\]
Connected to dsi_protocol port name csi_pllpwrack"]
pub type PllpwrackR = crate::BitReader;
#[doc = "Field `PLLPWRACK` writer - 3:3\\]
Connected to dsi_protocol port name csi_pllpwrack"]
pub type PllpwrackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKIN4DDRGOODBAR` reader - 6:4\\]
Connected to DSI PHY csi_clkin4ddrgoodbar"]
pub type Clkin4ddrgoodbarR = crate::FieldReader;
#[doc = "Field `CLKIN4DDRGOODBAR` writer - 6:4\\]
Connected to DSI PHY csi_clkin4ddrgoodbar"]
pub type Clkin4ddrgoodbarW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLLPWRCMDOFF` reader - 12:12\\]
Connected to dsi_protocol port name csi_pllpwrcmdoff"]
pub type PllpwrcmdoffR = crate::BitReader;
#[doc = "Field `PLLPWRCMDOFF` writer - 12:12\\]
Connected to dsi_protocol port name csi_pllpwrcmdoff"]
pub type PllpwrcmdoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLPWRCMDONHSCLK` reader - 13:13\\]
Connected to dsi_protocol port name csi_pllpwrcmdonhsclk"]
pub type PllpwrcmdonhsclkR = crate::BitReader;
#[doc = "Field `PLLPWRCMDONHSCLK` writer - 13:13\\]
Connected to dsi_protocol port name csi_pllpwrcmdonhsclk"]
pub type PllpwrcmdonhsclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLPWRCMDONDIV` reader - 14:14\\]
Connected to dsi_protocol port name csi_pllpwrcmdondiv"]
pub type PllpwrcmdondivR = crate::BitReader;
#[doc = "Field `PLLPWRCMDONDIV` writer - 14:14\\]
Connected to dsi_protocol port name csi_pllpwrcmdondiv"]
pub type PllpwrcmdondivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLPWRCMDONALL` reader - 15:15\\]
Connected to dsi_protocol port name csi_pllpwrcmdonall"]
pub type PllpwrcmdonallR = crate::BitReader;
#[doc = "Field `PLLPWRCMDONALL` writer - 15:15\\]
Connected to dsi_protocol port name csi_pllpwrcmdonall"]
pub type PllpwrcmdonallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPASSACKZ` reader - 16:16\\]
Connected to DSI PHY csi_bypassackz"]
pub type BypassackzR = crate::BitReader;
#[doc = "Field `BYPASSACKZ` writer - 16:16\\]
Connected to DSI PHY csi_bypassackz"]
pub type BypassackzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKINENOVRCTL` reader - 17:17\\]
Override control for CLKINEN port of DSI PHY"]
pub type ClkinenovrctlR = crate::BitReader;
#[doc = "Field `CLKINENOVRCTL` writer - 17:17\\]
Override control for CLKINEN port of DSI PHY"]
pub type ClkinenovrctlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKINENOVRVAL` reader - 18:18\\]
Override value for CLKINEN port of DSI PHY. Invert of this value is connected to DSI PHY"]
pub type ClkinenovrvalR = crate::BitReader;
#[doc = "Field `CLKINENOVRVAL` writer - 18:18\\]
Override value for CLKINEN port of DSI PHY. Invert of this value is connected to DSI PHY"]
pub type ClkinenovrvalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORCLKSELOVRCTL` reader - 19:19\\]
Override control for CLKLANEADDR port of DSI PHY."]
pub type PorclkselovrctlR = crate::BitReader;
#[doc = "Field `PORCLKSELOVRCTL` writer - 19:19\\]
Override control for CLKLANEADDR port of DSI PHY."]
pub type PorclkselovrctlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORCLKSELOVRVAL` reader - 22:20\\]
Override value for CLKLANEADDR port of DSI PHY."]
pub type PorclkselovrvalR = crate::FieldReader;
#[doc = "Field `PORCLKSELOVRVAL` writer - 22:20\\]
Override value for CLKLANEADDR port of DSI PHY."]
pub type PorclkselovrvalW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CSIB2BWREADYEN` reader - 25:23\\]
Enable for the burst mode support of VBUSP 2 OCP module. 000 -->Disable, 111-->Enable, Others are reserved."]
pub type Csib2bwreadyenR = crate::FieldReader;
#[doc = "Field `CSIB2BWREADYEN` writer - 25:23\\]
Enable for the burst mode support of VBUSP 2 OCP module. 000 -->Disable, 111-->Enable, Others are reserved."]
pub type Csib2bwreadyenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CSIBURSTEN` reader - 28:26\\]
Enable to Mux the Burst related signals onto the MSB address bits to the VBUS2OCP module. 000 -->Disable, 111-->Enable, Others are reserved."]
pub type CsiburstenR = crate::FieldReader;
#[doc = "Field `CSIBURSTEN` writer - 28:26\\]
Enable to Mux the Burst related signals onto the MSB address bits to the VBUS2OCP module. 000 -->Disable, 111-->Enable, Others are reserved."]
pub type CsiburstenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DSIADDRREMAPEN` reader - 31:29\\]
Enable to Mux the re-mapped addresses for bypass access path to CSI. 000 -->Disable, 111-->Enable, Others are reserved."]
pub type DsiaddrremapenR = crate::FieldReader;
#[doc = "Field `DSIADDRREMAPEN` writer - 31:29\\]
Enable to Mux the re-mapped addresses for bypass access path to CSI. 000 -->Disable, 111-->Enable, Others are reserved."]
pub type DsiaddrremapenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Connected to dsi_protocol port name csi_scpinpllcrtl"]
    #[inline(always)]
    pub fn scpinpllcrtl(&self) -> ScpinpllcrtlR {
        ScpinpllcrtlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Connected to dsi_protocol port name csi_dsilock"]
    #[inline(always)]
    pub fn dsilock(&self) -> DsilockR {
        DsilockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Connected to dsi_protocol port name csi_dsirecal"]
    #[inline(always)]
    pub fn dsirecal(&self) -> DsirecalR {
        DsirecalR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Connected to dsi_protocol port name csi_pllpwrack"]
    #[inline(always)]
    pub fn pllpwrack(&self) -> PllpwrackR {
        PllpwrackR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Connected to DSI PHY csi_clkin4ddrgoodbar"]
    #[inline(always)]
    pub fn clkin4ddrgoodbar(&self) -> Clkin4ddrgoodbarR {
        Clkin4ddrgoodbarR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Connected to dsi_protocol port name csi_pllpwrcmdoff"]
    #[inline(always)]
    pub fn pllpwrcmdoff(&self) -> PllpwrcmdoffR {
        PllpwrcmdoffR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Connected to dsi_protocol port name csi_pllpwrcmdonhsclk"]
    #[inline(always)]
    pub fn pllpwrcmdonhsclk(&self) -> PllpwrcmdonhsclkR {
        PllpwrcmdonhsclkR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Connected to dsi_protocol port name csi_pllpwrcmdondiv"]
    #[inline(always)]
    pub fn pllpwrcmdondiv(&self) -> PllpwrcmdondivR {
        PllpwrcmdondivR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Connected to dsi_protocol port name csi_pllpwrcmdonall"]
    #[inline(always)]
    pub fn pllpwrcmdonall(&self) -> PllpwrcmdonallR {
        PllpwrcmdonallR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Connected to DSI PHY csi_bypassackz"]
    #[inline(always)]
    pub fn bypassackz(&self) -> BypassackzR {
        BypassackzR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Override control for CLKINEN port of DSI PHY"]
    #[inline(always)]
    pub fn clkinenovrctl(&self) -> ClkinenovrctlR {
        ClkinenovrctlR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Override value for CLKINEN port of DSI PHY. Invert of this value is connected to DSI PHY"]
    #[inline(always)]
    pub fn clkinenovrval(&self) -> ClkinenovrvalR {
        ClkinenovrvalR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Override control for CLKLANEADDR port of DSI PHY."]
    #[inline(always)]
    pub fn porclkselovrctl(&self) -> PorclkselovrctlR {
        PorclkselovrctlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - 22:20\\]
Override value for CLKLANEADDR port of DSI PHY."]
    #[inline(always)]
    pub fn porclkselovrval(&self) -> PorclkselovrvalR {
        PorclkselovrvalR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - 25:23\\]
Enable for the burst mode support of VBUSP 2 OCP module. 000 -->Disable, 111-->Enable, Others are reserved."]
    #[inline(always)]
    pub fn csib2bwreadyen(&self) -> Csib2bwreadyenR {
        Csib2bwreadyenR::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:28 - 28:26\\]
Enable to Mux the Burst related signals onto the MSB address bits to the VBUS2OCP module. 000 -->Disable, 111-->Enable, Others are reserved."]
    #[inline(always)]
    pub fn csibursten(&self) -> CsiburstenR {
        CsiburstenR::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Enable to Mux the re-mapped addresses for bypass access path to CSI. 000 -->Disable, 111-->Enable, Others are reserved."]
    #[inline(always)]
    pub fn dsiaddrremapen(&self) -> DsiaddrremapenR {
        DsiaddrremapenR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Connected to dsi_protocol port name csi_scpinpllcrtl"]
    #[inline(always)]
    #[must_use]
    pub fn scpinpllcrtl(&mut self) -> ScpinpllcrtlW<Csi2TxCfg2Spec> {
        ScpinpllcrtlW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Connected to dsi_protocol port name csi_dsilock"]
    #[inline(always)]
    #[must_use]
    pub fn dsilock(&mut self) -> DsilockW<Csi2TxCfg2Spec> {
        DsilockW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Connected to dsi_protocol port name csi_dsirecal"]
    #[inline(always)]
    #[must_use]
    pub fn dsirecal(&mut self) -> DsirecalW<Csi2TxCfg2Spec> {
        DsirecalW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Connected to dsi_protocol port name csi_pllpwrack"]
    #[inline(always)]
    #[must_use]
    pub fn pllpwrack(&mut self) -> PllpwrackW<Csi2TxCfg2Spec> {
        PllpwrackW::new(self, 3)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Connected to DSI PHY csi_clkin4ddrgoodbar"]
    #[inline(always)]
    #[must_use]
    pub fn clkin4ddrgoodbar(&mut self) -> Clkin4ddrgoodbarW<Csi2TxCfg2Spec> {
        Clkin4ddrgoodbarW::new(self, 4)
    }
    #[doc = "Bit 12 - 12:12\\]
Connected to dsi_protocol port name csi_pllpwrcmdoff"]
    #[inline(always)]
    #[must_use]
    pub fn pllpwrcmdoff(&mut self) -> PllpwrcmdoffW<Csi2TxCfg2Spec> {
        PllpwrcmdoffW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Connected to dsi_protocol port name csi_pllpwrcmdonhsclk"]
    #[inline(always)]
    #[must_use]
    pub fn pllpwrcmdonhsclk(&mut self) -> PllpwrcmdonhsclkW<Csi2TxCfg2Spec> {
        PllpwrcmdonhsclkW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Connected to dsi_protocol port name csi_pllpwrcmdondiv"]
    #[inline(always)]
    #[must_use]
    pub fn pllpwrcmdondiv(&mut self) -> PllpwrcmdondivW<Csi2TxCfg2Spec> {
        PllpwrcmdondivW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Connected to dsi_protocol port name csi_pllpwrcmdonall"]
    #[inline(always)]
    #[must_use]
    pub fn pllpwrcmdonall(&mut self) -> PllpwrcmdonallW<Csi2TxCfg2Spec> {
        PllpwrcmdonallW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Connected to DSI PHY csi_bypassackz"]
    #[inline(always)]
    #[must_use]
    pub fn bypassackz(&mut self) -> BypassackzW<Csi2TxCfg2Spec> {
        BypassackzW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Override control for CLKINEN port of DSI PHY"]
    #[inline(always)]
    #[must_use]
    pub fn clkinenovrctl(&mut self) -> ClkinenovrctlW<Csi2TxCfg2Spec> {
        ClkinenovrctlW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Override value for CLKINEN port of DSI PHY. Invert of this value is connected to DSI PHY"]
    #[inline(always)]
    #[must_use]
    pub fn clkinenovrval(&mut self) -> ClkinenovrvalW<Csi2TxCfg2Spec> {
        ClkinenovrvalW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Override control for CLKLANEADDR port of DSI PHY."]
    #[inline(always)]
    #[must_use]
    pub fn porclkselovrctl(&mut self) -> PorclkselovrctlW<Csi2TxCfg2Spec> {
        PorclkselovrctlW::new(self, 19)
    }
    #[doc = "Bits 20:22 - 22:20\\]
Override value for CLKLANEADDR port of DSI PHY."]
    #[inline(always)]
    #[must_use]
    pub fn porclkselovrval(&mut self) -> PorclkselovrvalW<Csi2TxCfg2Spec> {
        PorclkselovrvalW::new(self, 20)
    }
    #[doc = "Bits 23:25 - 25:23\\]
Enable for the burst mode support of VBUSP 2 OCP module. 000 -->Disable, 111-->Enable, Others are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn csib2bwreadyen(&mut self) -> Csib2bwreadyenW<Csi2TxCfg2Spec> {
        Csib2bwreadyenW::new(self, 23)
    }
    #[doc = "Bits 26:28 - 28:26\\]
Enable to Mux the Burst related signals onto the MSB address bits to the VBUS2OCP module. 000 -->Disable, 111-->Enable, Others are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn csibursten(&mut self) -> CsiburstenW<Csi2TxCfg2Spec> {
        CsiburstenW::new(self, 26)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Enable to Mux the re-mapped addresses for bypass access path to CSI. 000 -->Disable, 111-->Enable, Others are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn dsiaddrremapen(&mut self) -> DsiaddrremapenW<Csi2TxCfg2Spec> {
        DsiaddrremapenW::new(self, 29)
    }
}
#[doc = "CSI2_TX_CFG2\n\nYou can [`read`](crate::Reg::read) this register and get [`csi2_tx_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csi2_tx_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csi2TxCfg2Spec;
impl crate::RegisterSpec for Csi2TxCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_tx_cfg2::R`](R) reader structure"]
impl crate::Readable for Csi2TxCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`csi2_tx_cfg2::W`](W) writer structure"]
impl crate::Writable for Csi2TxCfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_TX_CFG2 to value 0"]
impl crate::Resettable for Csi2TxCfg2Spec {
    const RESET_VALUE: u32 = 0;
}
