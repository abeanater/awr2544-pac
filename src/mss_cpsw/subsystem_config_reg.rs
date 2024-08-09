#[doc = "Register `SUBSYSTEM_CONFIG_REG` reader"]
pub type R = crate::R<SubsystemConfigRegSpec>;
#[doc = "Register `SUBSYSTEM_CONFIG_REG` writer"]
pub type W = crate::W<SubsystemConfigRegSpec>;
#[doc = "Field `THE_TOTAL_NUMBER` reader - 7:0\\]
The total number of ports including the host port 0"]
pub type TheTotalNumberR = crate::FieldReader;
#[doc = "Field `THE_TOTAL_NUMBER` writer - 7:0\\]
The total number of ports including the host port 0"]
pub type TheTotalNumberW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `THE_NUMBER_OF` reader - 12:8\\]
The number of CPTS GENF outputs"]
pub type TheNumberOfR = crate::FieldReader;
#[doc = "Field `THE_NUMBER_OF` writer - 12:8\\]
The number of CPTS GENF outputs"]
pub type TheNumberOfW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RMII_IS_INCLUDED` reader - 16:16\\]
RMII is included in the CPSW_NUSS"]
pub type RmiiIsIncludedR = crate::BitReader;
#[doc = "Field `RMII_IS_INCLUDED` writer - 16:16\\]
RMII is included in the CPSW_NUSS"]
pub type RmiiIsIncludedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RGMII_IS_INCLUDED` reader - 17:17\\]
RGMII is included in the CPSW_NUSS"]
pub type RgmiiIsIncludedR = crate::BitReader;
#[doc = "Field `RGMII_IS_INCLUDED` writer - 17:17\\]
RGMII is included in the CPSW_NUSS"]
pub type RgmiiIsIncludedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SGMII_IS_INCLUDED` reader - 18:18\\]
SGMII is included in the CPSW_NUSS"]
pub type SgmiiIsIncludedR = crate::BitReader;
#[doc = "Field `SGMII_IS_INCLUDED` writer - 18:18\\]
SGMII is included in the CPSW_NUSS"]
pub type SgmiiIsIncludedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSGMII_IS_INCLUDED` reader - 19:19\\]
QSGMII is included in the CPSW_NUSS"]
pub type QsgmiiIsIncludedR = crate::BitReader;
#[doc = "Field `QSGMII_IS_INCLUDED` writer - 19:19\\]
QSGMII is included in the CPSW_NUSS"]
pub type QsgmiiIsIncludedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THE_NUMBER_OF` reader - 27:20\\]
The Number of XGMII Ports included in the CPSW_NUSS"]
pub type TheNumberOfR = crate::FieldReader;
#[doc = "Field `THE_NUMBER_OF` writer - 27:20\\]
The Number of XGMII Ports included in the CPSW_NUSS"]
pub type TheNumberOfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
The total number of ports including the host port 0"]
    #[inline(always)]
    pub fn the_total_number(&self) -> TheTotalNumberR {
        TheTotalNumberR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
The number of CPTS GENF outputs"]
    #[inline(always)]
    pub fn the_number_of(&self) -> TheNumberOfR {
        TheNumberOfR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
RMII is included in the CPSW_NUSS"]
    #[inline(always)]
    pub fn rmii_is_included(&self) -> RmiiIsIncludedR {
        RmiiIsIncludedR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
RGMII is included in the CPSW_NUSS"]
    #[inline(always)]
    pub fn rgmii_is_included(&self) -> RgmiiIsIncludedR {
        RgmiiIsIncludedR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
SGMII is included in the CPSW_NUSS"]
    #[inline(always)]
    pub fn sgmii_is_included(&self) -> SgmiiIsIncludedR {
        SgmiiIsIncludedR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
QSGMII is included in the CPSW_NUSS"]
    #[inline(always)]
    pub fn qsgmii_is_included(&self) -> QsgmiiIsIncludedR {
        QsgmiiIsIncludedR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:27 - 27:20\\]
The Number of XGMII Ports included in the CPSW_NUSS"]
    #[inline(always)]
    pub fn the_number_of(&self) -> TheNumberOfR {
        TheNumberOfR::new(((self.bits >> 20) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
The total number of ports including the host port 0"]
    #[inline(always)]
    #[must_use]
    pub fn the_total_number(&mut self) -> TheTotalNumberW<SubsystemConfigRegSpec> {
        TheTotalNumberW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
The number of CPTS GENF outputs"]
    #[inline(always)]
    #[must_use]
    pub fn the_number_of(&mut self) -> TheNumberOfW<SubsystemConfigRegSpec> {
        TheNumberOfW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
RMII is included in the CPSW_NUSS"]
    #[inline(always)]
    #[must_use]
    pub fn rmii_is_included(&mut self) -> RmiiIsIncludedW<SubsystemConfigRegSpec> {
        RmiiIsIncludedW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
RGMII is included in the CPSW_NUSS"]
    #[inline(always)]
    #[must_use]
    pub fn rgmii_is_included(&mut self) -> RgmiiIsIncludedW<SubsystemConfigRegSpec> {
        RgmiiIsIncludedW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
SGMII is included in the CPSW_NUSS"]
    #[inline(always)]
    #[must_use]
    pub fn sgmii_is_included(&mut self) -> SgmiiIsIncludedW<SubsystemConfigRegSpec> {
        SgmiiIsIncludedW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
QSGMII is included in the CPSW_NUSS"]
    #[inline(always)]
    #[must_use]
    pub fn qsgmii_is_included(&mut self) -> QsgmiiIsIncludedW<SubsystemConfigRegSpec> {
        QsgmiiIsIncludedW::new(self, 19)
    }
    #[doc = "Bits 20:27 - 27:20\\]
The Number of XGMII Ports included in the CPSW_NUSS"]
    #[inline(always)]
    #[must_use]
    pub fn the_number_of(&mut self) -> TheNumberOfW<SubsystemConfigRegSpec> {
        TheNumberOfW::new(self, 20)
    }
}
#[doc = "Subsystem Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`subsystem_config_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subsystem_config_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SubsystemConfigRegSpec;
impl crate::RegisterSpec for SubsystemConfigRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`subsystem_config_reg::R`](R) reader structure"]
impl crate::Readable for SubsystemConfigRegSpec {}
#[doc = "`write(|w| ..)` method takes [`subsystem_config_reg::W`](W) writer structure"]
impl crate::Writable for SubsystemConfigRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUBSYSTEM_CONFIG_REG to value 0"]
impl crate::Resettable for SubsystemConfigRegSpec {
    const RESET_VALUE: u32 = 0;
}
