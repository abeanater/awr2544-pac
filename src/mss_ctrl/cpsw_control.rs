#[doc = "Register `CPSW_CONTROL` reader"]
pub type R = crate::R<CpswControlSpec>;
#[doc = "Register `CPSW_CONTROL` writer"]
pub type W = crate::W<CpswControlSpec>;
#[doc = "Field `port1_mode_sel` reader - 2:0\\]
Port 1 Interface 00 = GMII/MII 01 = RMII 10 = RGMII 11 = Not Supported"]
pub type Port1ModeSelR = crate::FieldReader;
#[doc = "Field `port1_mode_sel` writer - 2:0\\]
Port 1 Interface 00 = GMII/MII 01 = RMII 10 = RGMII 11 = Not Supported"]
pub type Port1ModeSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rmii_ref_clk_oe_n` reader - 8:8\\]
To select the rmii_ref_clk from PAD or from MSS_RCM. 0: clock will be from mss_rcm through IO internal loopback 1: will be from"]
pub type RmiiRefClkOeNR = crate::BitReader;
#[doc = "Field `rmii_ref_clk_oe_n` writer - 8:8\\]
To select the rmii_ref_clk from PAD or from MSS_RCM. 0: clock will be from mss_rcm through IO internal loopback 1: will be from"]
pub type RmiiRefClkOeNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rgmii1_id_mode` reader - 16:16\\]
writing 1'b1 would disable the internal clock delays. And those delays need to be handled on board."]
pub type Rgmii1IdModeR = crate::BitReader;
#[doc = "Field `rgmii1_id_mode` writer - 16:16\\]
writing 1'b1 would disable the internal clock delays. And those delays need to be handled on board."]
pub type Rgmii1IdModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Port 1 Interface 00 = GMII/MII 01 = RMII 10 = RGMII 11 = Not Supported"]
    #[inline(always)]
    pub fn port1_mode_sel(&self) -> Port1ModeSelR {
        Port1ModeSelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
To select the rmii_ref_clk from PAD or from MSS_RCM. 0: clock will be from mss_rcm through IO internal loopback 1: will be from"]
    #[inline(always)]
    pub fn rmii_ref_clk_oe_n(&self) -> RmiiRefClkOeNR {
        RmiiRefClkOeNR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
writing 1'b1 would disable the internal clock delays. And those delays need to be handled on board."]
    #[inline(always)]
    pub fn rgmii1_id_mode(&self) -> Rgmii1IdModeR {
        Rgmii1IdModeR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Port 1 Interface 00 = GMII/MII 01 = RMII 10 = RGMII 11 = Not Supported"]
    #[inline(always)]
    #[must_use]
    pub fn port1_mode_sel(&mut self) -> Port1ModeSelW<CpswControlSpec> {
        Port1ModeSelW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
To select the rmii_ref_clk from PAD or from MSS_RCM. 0: clock will be from mss_rcm through IO internal loopback 1: will be from"]
    #[inline(always)]
    #[must_use]
    pub fn rmii_ref_clk_oe_n(&mut self) -> RmiiRefClkOeNW<CpswControlSpec> {
        RmiiRefClkOeNW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
writing 1'b1 would disable the internal clock delays. And those delays need to be handled on board."]
    #[inline(always)]
    #[must_use]
    pub fn rgmii1_id_mode(&mut self) -> Rgmii1IdModeW<CpswControlSpec> {
        Rgmii1IdModeW::new(self, 16)
    }
}
#[doc = "CPSW_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswControlSpec;
impl crate::RegisterSpec for CpswControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_control::R`](R) reader structure"]
impl crate::Readable for CpswControlSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_control::W`](W) writer structure"]
impl crate::Writable for CpswControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_CONTROL to value 0"]
impl crate::Resettable for CpswControlSpec {
    const RESET_VALUE: u32 = 0;
}
